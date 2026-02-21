pub mod context;
pub mod error;
pub mod symbols;

use common::intern::Intern;
use common::primitives::PrimitiveKeywords;
use std::cell::RefCell;
// Mistmatched types
// X is required here, did you meant to.
use crate::parser::error::Branch;

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
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching, None)
                        .ok();

                    parse_bind_section(&mut ctx, &mut sym_table, interner).ok();
                }
                id if id == PrimitiveKeywords::Var as u32 => {
                    // Will index out of bounds without match because
                    // errors cannot propogate from here in the scenario 'var-'
                    match ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching, None) {
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
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching, None)
                        .ok();
                }
                id if id == PrimitiveKeywords::ComplexRules as u32 => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching, None)
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
        // WHERES MY ITERATOR
        dbg!(sym_table);
        for err in ctx.err_vec.borrow().iter() {
            eprintln!("{}\n", err.msg);
        }

        panic!("I'm new to thinking. Does anyone have beginner thoughts?");
        // std::process::exit(1);
    }

    sym_table
}

//FIXME: NEED TO SKIP IN CONTEXT STRUCT

//TODO: Just return id
//And error need to return something useful eventually to know that we have to error
fn parse_bind_section(
    ctx: &mut Context,
    sym_table: &mut SymbolTable,
    interner: &Intern,
) -> Result<(), Token> {
    let name_id = ctx.expect_id(TokenKind::Literal, Branch::Bind)?;

    dbg!(interner.search(name_id as usize));

    let symbol = Symbol::Bind(Bind::new(name_id));

    sym_table.store_bind(symbol, name_id);

    Ok(())
}

fn parse_var_section(
    ctx: &mut Context,
    table: &mut SymbolTable,
    interner: &Intern,
) -> Result<(), Token> {
    let id = ctx.expect_id(TokenKind::Id, Branch::Var)?;

    ctx.expect_basic(
        TokenKind::Colon,
        Branch::Var,
        Some("\nFailed to parse type."),
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
        ctx.expect_basic(
            TokenKind::CParen,
            Branch::VarCond,
            // FIX: Should likely be header
            Some(
                "\nFailed to find closing parenthesis inside condition. Is there a missing comma?",
            ),
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

    let type_id = table.reserve_t_id();

    let type_def = TypeDef::new(id, type_id, args, conds);

    // Maybe just match?
    table.store_symbol(Symbol::Definition(type_def), id, type_id, raw_type);

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
                //FIX: Type is known of course but should likely still be composed
                let fmt_tok = format!("identifier \"{name}\"");

                //FIX: CHECK IF WE WE GOT WAS SIMILAR TO something?
                //It's going to be a macro because <>
                ctx.report_template("a type", &fmt_tok, Branch::Var);

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
            ctx.report_template("a type after identifier declaration", &fmt_tok, Branch::Var);

            Err(Token::Literal(id))
        }
        Token::EOF => panic!("Got eof during parse type (Later)"),
        t => {
            //FIX: I FORGOT.  Forgot what??? I actually don't know.

            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a type after identifier declaration", &fmt_tok, Branch::Var);

            Err(t)
        }
    }
}

fn parse_arg(ctx: &mut Context, interner: &Intern) -> Result<InnerArgs, Token> {
    let id = ctx.expect_id(TokenKind::Id, Branch::VarInnerArgs)?;

    //TODO: ODD HANDLING
    InnerArgs::try_from(interner.search(id as usize)).or(Err(Token::Illegal(id)))

    //FIX: CHECK FOR NOTATION ON CORRECT TYPE BY OTHER PASSING IN TYPE OR OUTSIDE
}

fn parse_array(ctx: &mut Context, interner: &Intern) -> Result<ActualType, Token> {
    ctx.expect_basic(
        TokenKind::OAngleBracket,
        Branch::Var,
        Some("Could not create 'List' type."),
    )?;

    let ty = parse_type(ctx, interner)?;

    ctx.expect_basic(
        TokenKind::CAngleBracket,
        Branch::Var,
        Some("Could not find end of 'List' type."),
    )?;

    Ok(ty)
}

fn parse_map(ctx: &mut Context, interner: &Intern) -> Result<(ActualType, ActualType), Token> {
    ctx.expect_basic(
        TokenKind::OAngleBracket,
        Branch::Var,
        Some("Could not create 'Map' type."),
    )?;

    let key = parse_type(ctx, interner)?;

    //Bold
    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance();
    }

    let val = parse_type(ctx, interner)?;

    ctx.expect_basic(
        TokenKind::CAngleBracket,
        Branch::Var,
        Some("Could not find '<' in list type."),
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
                    let fmt_tok = format!("{} \"{n}\"", TokenKind::Id);
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
            ctx.expect_id(TokenKind::Number, Branch::VarCond)?
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

            ctx.expect_id(TokenKind::Number, Branch::VarCond)?
        }
        Token::Id(id) | Token::Literal(id) => {
            let err_tok = ctx.peek().token;
            let name = interner.search(id as usize);

            let fmt_tok = format!("{} \"{}\" while parsing condition.", TokenKind::Id, name);
            ctx.report_template("a (range) or number", &fmt_tok, Branch::Var);
            return Err(err_tok);
        }
        t => {
            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a range or numeric literal", &fmt_tok, Branch::Var);
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
        ctx.report_template(
            "a valid range",
            "a start '{start}' greater than the end {end}'",
            Branch::VarCond,
        );
    }

    ctx.expect_basic(TokenKind::CParen, Branch::VarCond, None)?;

    Ok(Cond::Range(start, end))
}

fn parse_nest_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}

fn parse_complex_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}
