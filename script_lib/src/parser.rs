pub mod context;
pub mod error;
pub mod symbols;

use common::intern::{self, Intern, ReservedKeyword};
use std::cell::RefCell;

use crate::parser::error::Branch;

use crate::parser::symbols::{Bind, Cond, Table, TypeDef};
use crate::token::{ActualType, InnerArgs};
use crate::{
    parser::{context::Context, symbols::Symbol},
    token::{SpannedToken, Token, TokenKind},
};

pub fn parse(tokens: &Vec<SpannedToken>, interner: &mut Intern) -> Table {
    let mut table = Table::new();

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
            let section = interner.search(id);
            dbg!(section, &tok.token);
        }

        match &tok.token {
            Token::Id(id) => match *id {
                id if id == ReservedKeyword::Bind as usize => {
                    // ITS FINE. ITS COMPLETELY FINE. NOT A  TODO:
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();

                    // Odd handling so I can pattern match
                    if let Ok(b) = parse_bind_section(&mut ctx, interner) {
                        table.symbols.insert(b.id, Symbol::Bind(b));
                    };
                }
                id if id == ReservedKeyword::Var as usize => {
                    // Will index out of bounds without match because
                    // errors cannot propogate from here
                    match ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching) {
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

                        if let Ok(type_def) = parse_var_section(&mut ctx, interner) {
                            table
                                .symbols
                                .insert(type_def.id, Symbol::Definition(type_def));
                        }
                    }
                }
                id if id == ReservedKeyword::Nest as usize => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();
                }
                id if id == ReservedKeyword::ComplexRules as usize => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();
                }
                id => {
                    //FIX: CHECK FOR SIMILARITY
                    let name_id = interner.search(id);
                    let fmsg =
                        format!("identifier \"{name_id}\". Use '->' before defining a section.");

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
                    Token::Id(id) | Token::Literal(id) => {
                        let name = interner.search(*id);
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
        dbg!(table);
        for err in ctx.err_vec.borrow().iter() {
            eprintln!("{}\n", err.msg);
        }

        panic!("I'm new to thinking. Anyone have beginner thoughts?");
        // std::process::exit(1);
    }

    table
}

//FIXME: NEED TO SKIP IN CONTEXT STRUCT
fn unwind() {}

//TODO: Just return id
//And error need to return something useful eventually to know that we have to error
fn parse_bind_section(ctx: &mut Context, interner: &Intern) -> Result<Bind, Token> {
    let id = ctx.expect_id(TokenKind::Literal, Branch::Bind)?;

    dbg!(interner.search(id));

    Ok(Bind::new(id))
}

fn parse_var_section(ctx: &mut Context, interner: &Intern) -> Result<TypeDef, Token> {
    let name_id = ctx.expect_id(TokenKind::Id, Branch::Var)?;
    // dbg!(interner.search(name_id), name_id);

    ctx.expect_basic(TokenKind::Colon, Branch::Var)?;

    let ty = parse_type(ctx, interner)?;

    if ctx.peek_kind() == TokenKind::Comma {
        ctx.advance();
    }

    let conds: Vec<Cond> = Vec::new();

    if ctx.peek_kind() == TokenKind::OParen {}

    let mut args: Vec<InnerArgs> = Vec::new();
    // dbg!("Before var^ args", ctx.peek_kind());

    while ctx.peek_kind() == TokenKind::HashSymbol {
        ctx.advance();
        // dbg!(interner.search(ctx.expect_id(TokenKind::Id, Branch::Var).unwrap()));
        let arg = parse_arg(ctx, interner)?;
        args.push(arg);
    }

    // dbg!(&interner.search(name_id));

    Ok(TypeDef::new(name_id, ty, args, conds))
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
        Token::Id(id) => match ReservedKeyword::try_from(id) {
            Ok(help_us_all) => match help_us_all {
                ReservedKeyword::Array => parse_array(ctx),
                ReservedKeyword::Set => todo!(),
                ReservedKeyword::Map => todo!(),
                _ => {
                    let type_res = ActualType::try_from(id).or(Err(Token::Illegal(id)));
                    ctx.advance();
                    type_res
                }
            },
            Err(_) => {
                let name_id = interner.search(id);
                let fmt_tok = format!("identifier \"{name_id}\"");

                //FIX: CHECK IF WE WE GOT WAS SIMILAR TO something?
                //It's going to be a macro because <>
                ctx.report_template("a type", &fmt_tok, Branch::Var);

                // A little weird since internally, this is meaningless
                Err(Token::Illegal(id))
            }
        },
        Token::QuestionMark => {
            ctx.advance();
            Ok(ActualType::Any)
        }
        // parse
        // Specific error messages here to say types were misplaeced?
        Token::OAngleBracket => todo!(),
        Token::CAngleBracket => todo!(),
        // Token::Comma => todo!(),
        // Token::OParen => todo!(),
        // Token::CParen => todo!(),
        // gorp
        // Token::Equals => todo!(),
        // Token::Percent => todo!(),
        // Token::Tilde => todo!(),
        // Token::Dot => todo!(),
        // Token::VerticalBar => todo!(),
        Token::EOF => panic!("Got eof during parse type (Later)"),
        t => {
            //FIX: I FORGOT
            let fmt_tok = format!("{}", t.kind());
            ctx.report_template("a type after identifier", &fmt_tok, Branch::Var);
            Err(t)
        }
    }
}

fn parse_arg(ctx: &mut Context, interner: &Intern) -> Result<InnerArgs, Token> {
    let id = ctx.expect_id(TokenKind::Id, Branch::InnerArgs)?;

    InnerArgs::try_from(interner.search(id)).or(Err(Token::Illegal(id)))

    //FIX: CHECK FOR NOTATION ON CORRECT TYPE BY OTHER PASSING IN TYPE OR OUTSIDE
}

fn parse_array(ctx: &mut Context) -> Result<ActualType, Token> {
    todo!()
}

fn parse_nest_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}

fn parse_complex_section(ctx: &mut Context, interner: &mut Intern) -> Result<Symbol, ()> {
    todo!()
}
