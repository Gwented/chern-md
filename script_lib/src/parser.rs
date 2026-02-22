pub mod context;
pub mod error;
pub mod symbols;

use crate::parser::error::Branch;
use common::intern::Intern;
use common::primitives::PrimitiveKeywords;
use std::cell::RefCell;

use crate::parser::symbols::{Bind, Cond, SymbolTable, TypeDef};
use crate::token::{ActualType, InnerArgs};
use crate::{
    parser::{context::Context, symbols::Symbol},
    token::{SpannedToken, Token, TokenKind},
};

pub fn parse(tokens: &Vec<SpannedToken>, interner: &mut Intern) -> SymbolTable {
    let mut sym_table = SymbolTable::new();

    let mut ctx = Context {
        tokens: &tokens[..],
        pos: 0,
        err_vec: RefCell::new(Vec::new()),
    };

    while ctx.pos < ctx.tokens.len() {
        if ctx.err_vec.borrow().len() > 7 {
            break;
        }

        // Should this be cloned?
        let tok = ctx.advance();

        if let Token::Id(id) = tok.token {
            let section = interner.search(id as usize);
            dbg!(section, &tok.token);
        }

        match &tok.token {
            Token::Id(id) => match *id {
                id if id == PrimitiveKeywords::Bind as u32 => {
                    // ITS FINE. ITS COMPLETELY FINE. NOT A  TODO:
                    // BUG: WHY IS IT GETTING STUN LOCKED?
                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `bind`, found ",
                        "",
                        Branch::Searching,
                        interner,
                    )
                    .ok();

                    parse_bind_section(&mut ctx, &mut sym_table, interner).ok();
                }
                id if id == PrimitiveKeywords::Var as u32 => {
                    // Will index out of bounds without match because
                    // errors cannot propogate from here in the scenario 'var-'
                    match ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `var`, found ",
                        "",
                        Branch::Searching,
                        interner,
                    ) {
                        Ok(_) => (),
                        Err(_) => break,
                    };
                    dbg!("hello?");

                    while let Token::Id(current_id) = ctx.peek().token {
                        //FIX: TECHNICAL DEBT
                        if ctx.peek().token.kind() == TokenKind::EOF
                            || interner.is_section(current_id)
                        {
                            break;
                        }

                        parse_var_section(&mut ctx, &mut sym_table, interner).ok();
                    }
                }
                id if id == PrimitiveKeywords::Nest as u32 => {
                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `nest`, found ",
                        "",
                        Branch::Searching,
                        interner,
                    )
                    .ok();
                }
                id if id == PrimitiveKeywords::ComplexRules as u32 => {
                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `complex_rules`, found ",
                        "",
                        Branch::Searching,
                        interner,
                    )
                    .ok();
                }
                id => {
                    //FIX: CHECK FOR SIMILARITY
                    let name_id = interner.search(id as usize);
                    let fmsg =
                        format!("identifier \"{name_id}\". Use '->' before defining a section");
                    // let fmsg = format!(
                    //     "\nOk — looks like you got a syntax error.\nBut honestly — it happens to the best of us. I saw your code before this, and you know what? It shows you're not blindly making mistakes — you're just so focused innovating that the syntax can't catch up to your great ideas.\nHere's what went wrong:\n\tWhile you were casting spells to manipulate your computer (which was awesome) you missed the most important part — section names. You typed \"{name_id}\" instead.\nThe Fix:\n\tNext time while you're innovating — don't forget the '->'."
                    // );

                    ctx.report_template("a section", &fmsg, Branch::Searching);
                    break;
                }
            },
            // Token::Illegal(_) => todo!(),
            Token::EOF => break,
            // Currently assmung all errors that are propogated are program ending by default
            // since...
            t => {
                match t {
                    Token::Id(id) | Token::Literal(id) | Token::Number(id) => {
                        let name = interner.search(*id as usize);
                        let fmsg = format!("{} \"{}\"", t.kind(), name);

                        ctx.report_template(
                            "a section or type definition",
                            &fmsg,
                            Branch::Searching,
                        );
                    }
                    _ => {
                        let fmsg = format!("'{}'", t.kind());
                        ctx.report_template(
                            "a section or type definition",
                            &fmsg,
                            Branch::Searching,
                        );
                    }
                }

                break;
            }
        }
    }

    if !ctx.err_vec.borrow().is_empty() {
        dbg!(sym_table);
        eprint!("\x32Error:\x32 ");
        for err in ctx.err_vec.borrow().iter() {
            eprintln!("{}\n", err.msg);
        }

        panic!("I'm new to thinking. Does anyone have beginner thoughts?");
        // std::process::exit(1);
    }

    sym_table
}

//TODO: Just return id
//And error need to return something useful eventually to know that we have to error
fn parse_bind_section(
    ctx: &mut Context,
    sym_table: &mut SymbolTable,
    interner: &Intern,
) -> Result<(), Token> {
    let name_id = ctx.expect_id_verbose(
        TokenKind::Literal,
        "Expected a string literal within section `bind`, found ",
        "",
        Branch::Var,
        interner,
    )?;

    dbg!(interner.search(name_id as usize));

    let symbol = Symbol::Bind(Bind::new(name_id));

    sym_table.store_basic(symbol, name_id);

    Ok(())
}

fn parse_var_section(
    ctx: &mut Context,
    table: &mut SymbolTable,
    interner: &Intern,
) -> Result<(), Token> {
    let id = ctx.expect_id_verbose(
        TokenKind::Id,
        "Expected an identifier within `var`, found '",
        "'. |e.g. name: str'|",
        Branch::Var,
        interner,
    )?;

    ctx.expect_verbose(
        TokenKind::Colon,
        //TODO: Have a 'help' option that goes under the span shown
        "Insert a ':' after ",
        " to declare it's type.",
        Branch::Var,
        interner,
    )?;

    let raw_type = parse_type(ctx, interner)?;

    let mut conds: Vec<Cond> = Vec::new();

    if ctx.peek_kind() == TokenKind::OParen {
        ctx.advance();
        let new_cond = parse_cond(ctx, interner)?;
        conds.push(new_cond);

        while ctx.peek_kind() == TokenKind::Comma {
            ctx.advance();
            let new_cond = parse_cond(ctx, interner)?;
            conds.push(new_cond);
        }
        ctx.expect_verbose(
            TokenKind::CParen,
            "Expected ')' at end of condition, found ",
            ". Is there a missing comma?",
            Branch::VarCond,
            interner,
        )?;
    }

    let mut args: Vec<InnerArgs> = Vec::new();

    while ctx.peek_kind() == TokenKind::HashSymbol {
        ctx.advance();
        let arg = parse_arg(ctx, interner)?;
        args.push(arg);
    }

    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance();
    }

    let type_id = table.reserve_id();

    let type_def = TypeDef::new(id, type_id, args, conds);

    // Maybe just match?
    table.store_complex(Symbol::Definition(type_def), id, type_id, raw_type);

    Ok(())
}

// macro_rules! check_similar {
//     ($x:ident) => {
//
//     };
// }

//FIXME: Give ActualType the function instead
//The ActualType should USE the ReservedKeyword to GET the type to avoid misdirection
fn parse_type(ctx: &mut Context, interner: &Intern) -> Result<ActualType, Token> {
    match ctx.peek().token {
        Token::Id(id) => match PrimitiveKeywords::try_from(id) {
            //FIX: Specific error messages for data structure errors
            Ok(help_us_all) => match help_us_all {
                PrimitiveKeywords::List => {
                    ctx.advance();
                    let ty = parse_array(ctx, interner)?;

                    let array = ActualType::List(Box::new(ty));

                    Ok(array)
                }
                PrimitiveKeywords::Set => {
                    ctx.advance();
                    let ty = parse_array(ctx, interner)?;

                    let set = ActualType::Set(Box::new(ty));

                    Ok(set)
                }
                PrimitiveKeywords::Map => {
                    ctx.advance();
                    let (key, val) = parse_map(ctx, interner)?;

                    let map = ActualType::Map(Box::new(key), Box::new(val));
                    Ok(map)
                }
                _ => {
                    let type_res = ActualType::try_from(id).or(Err(Token::Illegal(id)));
                    ctx.advance();
                    type_res
                }
            },
            Err(_) => {
                let name = interner.search(id as usize);

                let msg =
                    // Please no more expected but found please please palsea
                    format!("Expected a type next, found identifier \"{name}\"");
                //FIX: CHECK IF WE WE GOT WAS SIMILAR TO something?
                ctx.report_verbose(&msg, Branch::Var);

                // A little weird since internally, this is meaningless
                Err(Token::Illegal(id))
            }
        },
        Token::QuestionMark => {
            ctx.advance();
            Ok(ActualType::Any(None))
        }
        // parse
        // Specific error messages here to say types were misplaeced?
        Token::OAngleBracket => todo!(),
        Token::CAngleBracket => todo!(),
        // Token::Comma => todo!(),
        // Token::OParen => todo!(),
        // gorp
        // Token::Percent => todo!(),
        // Token::Dot => todo!(),
        // Token::VerticalBar => todo!(),
        Token::Literal(id) => {
            let name = interner.search(id as usize);

            let fmt_tok = format!("{} \"{name}\"", TokenKind::Literal);
            ctx.report_template("a type after identifier", &fmt_tok, Branch::Var);

            Err(Token::Literal(id))
        }
        Token::EOF => panic!("Got eof during parse type (Later)"),
        t => {
            //FIX: I FORGOT.  Forgot what??? I actually don't know.

            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a type after identifier", &fmt_tok, Branch::Var);

            Err(t)
        }
    }
}

fn parse_arg(ctx: &mut Context, interner: &Intern) -> Result<InnerArgs, Token> {
    let id = ctx.expect_id_verbose(
        TokenKind::Id,
        "A type argument requires a '#' but '",
        "' was found. |e.g. #warn|",
        Branch::VarTypeArgs,
        interner,
    )?;

    //FIX: ODD HANDLING
    InnerArgs::try_from(interner.search(id as usize)).or_else(|invalid_id| {
        let msg = format!("The argument \"#{invalid_id}\" does not exist.");
        ctx.report_verbose(&msg, Branch::VarTypeArgs);
        return Err(Token::Illegal(id));
    })
}

fn parse_array(ctx: &mut Context, interner: &Intern) -> Result<ActualType, Token> {
    // Probably should just separate func
    ctx.expect_verbose(
        TokenKind::OAngleBracket,
        "A '<' is required to hold a type in `List`, found ",
        "",
        Branch::Var,
        interner,
    )?;

    let ty = parse_type(ctx, interner)?;

    ctx.expect_verbose(
        TokenKind::CAngleBracket,
        "Expected a '>' to close `List`, found ",
        "",
        Branch::Var,
        interner,
    )?;

    Ok(ty)
}

fn parse_map(ctx: &mut Context, interner: &Intern) -> Result<(ActualType, ActualType), Token> {
    ctx.expect_verbose(
        TokenKind::OAngleBracket,
        "Expected a '<' to define `Map`, found",
        "",
        Branch::Var,
        interner,
    )?;

    let key = parse_type(ctx, interner)?;

    //Bold
    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance();
    }

    let val = parse_type(ctx, interner)?;

    ctx.expect_verbose(
        TokenKind::CAngleBracket,
        "Expected a '>' to close `Map`, found ",
        "",
        Branch::Var,
        interner,
    )?;

    Ok((key, val))
}

//TODO: Incredible name.
fn parse_cond(ctx: &mut Context, interner: &Intern) -> Result<Cond, Token> {
    match ctx.peek().token {
        Token::Id(id) => {
            let name = interner.search(id as usize);

            match name {
                "Len" => {
                    ctx.advance();
                    return handle_len_func(ctx, interner);
                } // "IsEmpty" =>
                "IsEmpty" => {
                    ctx.advance();
                    return Ok(Cond::IsEmpty);
                }
                // Notations
                n => {
                    let fmt_tok = format!("\"{n}\"");
                    ctx.report_template(
                        "a condition after declared type",
                        &fmt_tok,
                        Branch::VarCond,
                    );

                    Err(Token::Literal(id))
                }
            }
        }
        Token::Literal(id) | Token::Number(id) => {
            let name = interner.search(id as usize);

            let fmt_tok = format!("{} \"{name}\"", TokenKind::Literal);
            ctx.report_template("a condition after declared type", &fmt_tok, Branch::VarCond);

            Err(Token::Literal(id))
        }
        Token::ExclamationPoint => {
            //TODO: Probably should just use booleans this is a bit much
            ctx.advance();

            if ctx.peek_kind() == TokenKind::ExclamationPoint {
                ctx.report_template(
                    "a condition",
                    "another '!'. Boolean logic can only be one condition deep.",
                    Branch::VarCond,
                );
            }

            let wrapped = parse_cond(ctx, interner)?;
            Ok(Cond::Not(Box::new(wrapped)))
        }
        t => {
            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a condition after declared type", &fmt_tok, Branch::VarCond);

            Err(t)
        }
    }
}

fn handle_len_func(ctx: &mut Context, interner: &Intern) -> Result<Cond, Token> {
    ctx.expect_basic(
        TokenKind::OParen,
        Branch::VarCond,
        Some("Could not find open parenthesis of 'Len' function."),
    )?;
    let mut start: usize = 0;

    let end_id = match ctx.peek().token {
        Token::Tilde => {
            ctx.advance();
            ctx.expect_id_verbose(
                TokenKind::Number,
                "Expected a number after '~', found '",
                "' within `Len()`. Use '(~x1)' or '(x1..=x2)' to define a range.",
                Branch::VarCond,
                interner,
            )?
        }
        Token::Number(id) => {
            ctx.advance();
            let raw_num = interner.search(id as usize);

            start = match raw_num.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    panic!("[temp] Internal error. Failed to parse number in condition.");
                    // ctx.report_template(emsg, fmsg, branch);
                    // return Err(ctx.advance().token);
                }
            };

            ctx.expect_basic(
                TokenKind::DotRange,
                Branch::VarCond,
                Some("Use '~x1' or '(x1..=x2)' to define a range."),
            )?;

            ctx.expect_id_verbose(
                TokenKind::Number,
                "",
                " was given but a number is required at the end of a range. |e.g. 0..=5|",
                Branch::VarCond,
                interner,
            )?
        }
        Token::Id(id) | Token::Literal(id) => {
            let err_tok = ctx.peek().token;
            let name = interner.search(id as usize);

            let fmt_tok = format!("{} \"{}\" while parsing condition.", err_tok.kind(), name);
            ctx.report_template("a (range) or number", &fmt_tok, Branch::Var);
            return Err(err_tok);
        }
        t => {
            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a (range) or number", &fmt_tok, Branch::Var);
            return Err(t);
        }
    };

    let end = interner.search(end_id as usize);

    let end = end
        .parse()
        //TODO: report this
        .expect(format!("Failed to parse identifier {} as a number in len func", end).as_str());
    dbg!("here");

    if start > end {
        let msg = format!("The range '{start}..={end}' is invalid. Cannot have end > start.");

        ctx.report_verbose(&msg, Branch::VarCond);
    }

    ctx.expect_verbose(
        TokenKind::CParen,
        "Missing ')' after defining `Len()`, found ",
        "",
        Branch::VarCond,
        interner,
    )?;

    Ok(Cond::Range(start, end))
}

fn parse_nest_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}

fn parse_complex_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}
