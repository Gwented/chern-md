pub mod context;
pub mod error;
pub mod symbols;

use crate::parser::error::Branch;
use crate::parser::symbols::{Bind, Cond, SymbolTable, TypeDef};
use crate::token::{ActualType, InnerArgs};
use crate::{
    parser::{context::Context, symbols::Symbol},
    token::{SpannedToken, Token, TokenKind},
};
use common::intern::Intern;
use common::primitives::PrimitiveKeywords;

pub fn parse(
    original_text: &[u8],
    tokens: &Vec<SpannedToken>,
    interner: &mut Intern,
) -> SymbolTable {
    let mut sym_table = SymbolTable::new();

    let mut ctx = Context::new(original_text, tokens);

    while ctx.pos < ctx.tokens.len() {
        if ctx.err_vec.len() > 10 {
            break;
        }

        // Should this be cloned?
        let tok = ctx.peek_tok();

        if let Token::Id(id) = tok {
            let section = interner.search(id as usize);
            dbg!(section, &tok);
        }

        match tok {
            Token::Id(id) => match id {
                id if id == PrimitiveKeywords::Bind as u32 => {
                    // BUG: Consumes EOF but
                    ctx.advance_tok();

                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `bind`, found ",
                        "",
                        Some("Change to \"bind->\""),
                        Branch::Searching,
                        interner,
                    )
                    .ok();

                    parse_bind_section(&mut ctx, &mut sym_table, interner).ok();
                }
                id if id == PrimitiveKeywords::Var as u32 => {
                    // Will index out of bounds without match because
                    // errors cannot propogate from here in the scenario 'var-'
                    ctx.advance_tok();

                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `var`, found ",
                        "",
                        Some("Change to \"var->\""),
                        Branch::Searching,
                        interner,
                    )
                    .ok();

                    while let Token::Id(current_id) = ctx.peek_tok() {
                        //FIX: TECHNICAL DEBT
                        dbg!(&ctx.tokens[ctx.pos - 1]);
                        // COULD NOT BE A SECTION
                        if ctx.peek_kind() == TokenKind::EOF || interner.is_section(current_id) {
                            break;
                        }

                        parse_var_section(&mut ctx, &mut sym_table, interner).ok();
                    }
                }
                id if id == PrimitiveKeywords::Nest as u32 => {
                    ctx.advance_tok();

                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `nest`, found ",
                        "",
                        Some("Change to \"nest->\""),
                        Branch::Searching,
                        interner,
                    )
                    .ok();
                }
                id if id == PrimitiveKeywords::ComplexRules as u32 => {
                    ctx.advance_tok();

                    ctx.expect_verbose(
                        TokenKind::SlimArrow,
                        "Expected a '->' after section `complex_rules`, found ",
                        "",
                        Some("Change to \"complex_rules->\""),
                        Branch::Searching,
                        interner,
                    )
                    .ok();
                }
                id => {
                    //FIX: CHECK FOR SIMILARITY
                    //WHAT IF IT WAS A MACRO?
                    ctx.advance_tok();

                    let name_id = interner.search(id as usize);
                    let fmsg = format!("identifier \"{name_id}\"");
                    // let fmsg = format!(
                    //     "\nOk — looks like you got a syntax error.\nBut honestly — it happens to the best of us. I saw your code before this, and you know what? It shows you're not blindly making mistakes — you're just so focused innovating that the syntax can't catch up to your great ideas.\nHere's what went wrong:\n\tWhile you were casting spells to manipulate your computer (which was awesome) you missed the most important part — section names. You typed \"{name_id}\" instead.\nThe Fix:\n\tNext time while you're innovating — don't forget the '->'."
                    // );

                    ctx.report_template("a section with a '->' after", &fmsg, Branch::Searching);
                    break;
                }
            },
            Token::Illegal(id) => {
                ctx.advance_tok();

                let err_str = interner.search(id as usize);

                let msg = format!("Found illegal token {err_str}");

                ctx.report_verbose(&msg, Branch::Broken);
            }
            Token::EOF => break,
            t => {
                match t {
                    Token::Id(id) | Token::Literal(id) | Token::Number(id) => {
                        ctx.advance_tok();

                        let name = interner.search(id as usize);
                        let fmsg = format!("{} \"{}\"", t.kind(), name);

                        ctx.report_template(
                            "a section or type definition",
                            &fmsg,
                            Branch::Searching,
                        );
                    }
                    _ => {
                        ctx.advance_tok();
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

    if !ctx.err_vec.is_empty() {
        dbg!(sym_table);
        //TODO: Ok I don't actually have the file path

        //FIX: ANSI
        eprint!("\x1b[31mError\x1b[0m: ");

        for err in ctx.err_vec.iter() {
            eprintln!("{}\n", err.msg);
        }

        // panic!("I'm new to thinking. Does anyone have beginner thoughts?");
        std::process::exit(1);
    }

    sym_table
}

fn parse_bind_section(
    ctx: &mut Context,
    sym_table: &mut SymbolTable,
    interner: &Intern,
) -> Result<(), Token> {
    let name_id = ctx.expect_id_verbose(
        TokenKind::Literal,
        "Expected a string literal within section `bind`, found ",
        "",
        Branch::Bind,
        interner,
    )?;

    let symbol = Symbol::Bind(Bind::new(name_id));

    sym_table.store_basic(symbol, name_id);

    Ok(())
}

fn parse_var_section(
    ctx: &mut Context,
    sym_table: &mut SymbolTable,
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
        "Expected ':' after identifier to declare type, found ",
        "",
        //WARN: LIAR
        None,
        Branch::VarType,
        interner,
    )?;

    let type_res = parse_type(ctx, interner);

    let mut conds: Vec<Cond> = Vec::new();

    if ctx.peek_kind() == TokenKind::OParen {
        ctx.advance_tok();
        let new_cond = parse_cond(ctx, interner)?;
        conds.push(new_cond);

        while ctx.peek_kind() == TokenKind::Comma {
            ctx.advance_tok();
            let new_cond = parse_cond(ctx, interner)?;
            conds.push(new_cond);
        }

        ctx.expect_verbose(
            TokenKind::CParen,
            "Expected ')' at end of condition, found ",
            "",
            // FIX: CONTEXT AWARE
            None,
            Branch::VarCond,
            interner,
        )
        .ok();
    }

    let mut args: Vec<InnerArgs> = Vec::new();

    while ctx.peek_kind() == TokenKind::HashSymbol {
        ctx.advance_tok();
        let arg = parse_arg(ctx, interner)?;
        args.push(arg);
    }

    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance_tok();
    }

    let raw_type = type_res?;

    let type_id = sym_table.reserve_id();

    let type_def = TypeDef::new(id, type_id, args, conds);

    sym_table.store_symbol(Symbol::Definition(type_def), id, type_id, raw_type);

    Ok(())
}

// macro_rules! check_similar {
//     ($x:ident) => {
//
//     };
// }

//FIXME: Give ActualType the function instead
//The ActualType should USE the ReservedKeyword to GET the type to avoid misdirection
//Or not I don't know
fn parse_type(ctx: &mut Context, interner: &Intern) -> Result<ActualType, Token> {
    match ctx.peek_tok() {
        Token::Id(id) => match PrimitiveKeywords::try_from(id) {
            Ok(p) => match p {
                PrimitiveKeywords::List => {
                    ctx.advance_tok();

                    let ty = parse_array(ctx, interner)?;

                    let list = ActualType::List(Box::new(ty));

                    Ok(list)
                }
                PrimitiveKeywords::Set => {
                    ctx.advance_tok();

                    let ty = parse_array(ctx, interner)?;

                    let set = ActualType::Set(Box::new(ty));

                    Ok(set)
                }
                PrimitiveKeywords::Map => {
                    ctx.advance_tok();

                    let (key, val) = parse_map(ctx, interner)?;

                    let map = ActualType::Map(Box::new(key), Box::new(val));

                    Ok(map)
                }
                p => {
                    //TODO: Can this fail?
                    let ty = ActualType::try_from(id).or_else(|_| {
                        ctx.advance_tok();

                        let name = interner.search(id as usize);

                        let msg = format!(
                            "Expected compatible type, found primitive \"{name}\", Branch::VarType"
                        );
                        ctx.report_verbose(&msg, Branch::VarType);
                        Err(Token::Poison)
                    })?;

                    ctx.advance_tok();

                    Ok(ty)
                }
            },
            Err(_) => {
                let name = interner.search(id as usize);

                // let primitive = ctx.try_rewind(10);

                let msg = format!("Expected a type, found identifier \"{name}\"");
                //FIX: CHECK IF WE WE WE WE GOT WAS SIMILAR TO something?
                ctx.advance_tok();
                ctx.report_verbose(&msg, Branch::VarType);

                // A little weird since internally, this is meaningless
                Err(Token::Illegal(id))
            }
        },
        Token::QuestionMark => {
            ctx.advance_tok();
            Ok(ActualType::Any(None))
        }
        Token::Literal(id) | Token::Number(id) => {
            let name = interner.search(id as usize);

            let kind = ctx.peek_kind();

            ctx.advance_tok();
            let fmt_tok = format!("{} \"{name}\"", kind);
            ctx.report_template("a type", &fmt_tok, Branch::VarType);

            Err(Token::Literal(id))
        }
        Token::EOF => {
            //FIX: Points to EOF since it is technically the error.
            ctx.advance_tok();

            ctx.report_verbose("Expected type, found '<eof>'", Branch::VarType);

            Err(Token::EOF)
        }
        Token::Poison => {
            panic!("Touched <poison>");
        }
        Token::SlimArrow => todo!(),
        Token::Slash => todo!(),
        Token::HashSymbol => todo!(),
        Token::Percent => todo!(),
        Token::Colon => todo!(),
        Token::ExclamationPoint => todo!(),
        Token::Asterisk => todo!(),
        Token::DoubleQuotes => todo!(),
        Token::Dot => todo!(),
        Token::VerticalBar => todo!(),
        Token::Illegal(_) => todo!(),
        t => {
            ctx.advance_tok();
            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a type", &fmt_tok, Branch::VarType);

            Err(Token::Poison)
        }
    }
}

fn parse_arg(ctx: &mut Context, interner: &Intern) -> Result<InnerArgs, Token> {
    let id = ctx.expect_id_verbose(
        TokenKind::Id,
        "Type arguments require a '#' first but ",
        " was found. |e.g. #warn|",
        Branch::VarTypeArgs,
        interner,
    )?;

    InnerArgs::try_from(interner.search(id as usize)).or_else(|invalid_id| {
        let msg = format!("The argument \"#{invalid_id}\" does not exist.");

        ctx.report_verbose(&msg, Branch::VarTypeArgs);

        return Err(Token::Illegal(id));
    })
}

fn parse_array(ctx: &mut Context, interner: &Intern) -> Result<ActualType, Token> {
    //TODO: Probably should just separate func for Sets
    ctx.expect_verbose(
        TokenKind::OAngleBracket,
        "A '<' is required for a `List` or `Set` to take in a type, found ",
        "",
        None,
        Branch::VarType,
        interner,
    )
    .ok();

    let ty = parse_type(ctx, interner)?;

    ctx.expect_verbose(
        TokenKind::CAngleBracket,
        "Expected a '>' to close `List` or `Set`, found ",
        "",
        None,
        Branch::VarType,
        interner,
    )
    .ok();

    Ok(ty)
}

fn parse_map(ctx: &mut Context, interner: &Intern) -> Result<(ActualType, ActualType), Token> {
    // Kinda weird since the type doesn't exist without a '<'
    ctx.expect_verbose(
        TokenKind::OAngleBracket,
        "Expected a '<' after `Map`, found ",
        "",
        None,
        Branch::VarType,
        interner,
    )
    .ok();

    let key = parse_type(ctx, interner)?;

    //Not that bold
    //FIX: Expect this
    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance_tok();
    }

    let val = parse_type(ctx, interner)?;

    ctx.expect_verbose(
        TokenKind::CAngleBracket,
        "Expected a '>' to close `Map`, found ",
        "",
        None,
        Branch::VarType,
        interner,
    )
    .ok();

    // let key = key_res.or_else(|_| Err(Token::Poison))?;
    // let val = val_res.or_else(|_| Err(Token::Poison))?;

    Ok((key, val))
}

//TODO: Incredible name.
fn parse_cond(ctx: &mut Context, interner: &Intern) -> Result<Cond, Token> {
    match ctx.peek_tok() {
        Token::Id(id) => {
            let name = interner.search(id as usize);

            match name {
                "Len" => {
                    ctx.advance_tok();
                    return handle_len_func(ctx, interner);
                } // "IsEmpty" =>
                "IsEmpty" => {
                    ctx.advance_tok();
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

                    //WARN:
                    Err(Token::Poison)
                }
            }
        }
        Token::Literal(id) | Token::Number(id) => {
            let name = interner.search(id as usize);

            let fmt_tok = format!("{} \"{name}\"", TokenKind::Literal);
            ctx.report_template("a condition after declared type", &fmt_tok, Branch::VarCond);

            //WARN:
            Err(Token::Poison)
        }
        Token::ExclamationPoint => {
            //TODO: Probably should just use booleans this is a bit much
            ctx.advance_tok();

            if ctx.peek_kind() == TokenKind::ExclamationPoint {
                ctx.report_template(
                    "a valid condition",
                    "another '!'. `Not` can only be used once in a single statement.",
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
    ctx.expect_verbose(
        TokenKind::OParen,
        "Missing '(' before `Len()`, found ",
        "",
        None,
        Branch::VarCond,
        interner,
    )?;

    let mut start: usize = 0;

    let end_id = match ctx.peek_tok() {
        Token::Tilde => {
            ctx.advance_tok();

            ctx.expect_id_verbose(
                TokenKind::Number,
                "Expected a number after '~', found ",
                " within `Len()`. Use '(~x1)' or '(x1..=x2)' to define a range.",
                Branch::VarCond,
                interner,
            )?
        }
        Token::Number(id) => {
            ctx.advance_tok();
            let raw_num = interner.search(id as usize);

            start = match raw_num.parse::<usize>() {
                Ok(n) => n,
                Err(_) => {
                    panic!("[temp] Internal error. Failed to parse number in condition.");
                    // ctx.report_template(emsg, fmsg, branch);
                    // return Err(ctx.advance().token);
                }
            };

            ctx.expect_verbose(
                TokenKind::DotRange,
                "Expected (range), found ",
                "",
                Some("Use 'Len(~x1)' or 'Len(x1..=x2)' to define a range."),
                Branch::VarCond,
                interner,
            )
            .ok();

            let user_help = format!(". |e.g. {start}..=other|");

            ctx.expect_id_verbose(
                TokenKind::Number,
                "Expected a number at the end of (range), found ",
                &user_help,
                Branch::VarCond,
                interner,
            )?
        }
        Token::Id(id) | Token::Literal(id) => {
            let err_tok = ctx.peek_tok();
            let name = interner.search(id as usize);

            //WARN: I think this should be advanced?
            let fmt_tok = format!("{} \"{}\" while parsing condition.", err_tok.kind(), name);
            ctx.report_template("a (range) or number", &fmt_tok, Branch::VarCond);
            return Err(err_tok);
        }
        t => {
            let fmt_tok = format!("'{}'", t.kind());
            ctx.report_template("a (range) or number", &fmt_tok, Branch::VarCond);
            return Err(t);
        }
    };

    let end = interner.search(end_id as usize);

    let end = end
        .parse()
        //TODO: report this
        .or_else(|_| {
            let msg = format!("Expected valid number, found {end}");

            ctx.report_verbose(
                //FIX:
                &msg,
                Branch::VarCond,
            );
            //WARN:
            return Err(Token::Poison);
        })?;

    if start > end {
        let msg = format!("The range '{start}..={end}' is invalid. Cannot have end > start.");
        ctx.report_verbose(&msg, Branch::VarCond);
    }

    ctx.expect_verbose(
        TokenKind::CParen,
        "Expected ')' after `Len()`, found ",
        "",
        Some("Individual type conditions must be closed with ')' |e.g. `(Len(~4))` |"),
        Branch::VarCond,
        interner,
    )
    .ok();

    Ok(Cond::Range(start, end))
}

fn parse_nest_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}

fn parse_complex_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}
