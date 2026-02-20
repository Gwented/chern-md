pub mod context;
pub mod error;
pub mod symbols;

use common::intern::{Intern, ReservedKeyword};
use std::cell::RefCell;

use crate::parser::error::Branch;

use crate::parser::symbols::Table;
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
        if ctx.err_vec.borrow().len() > 5 {
            break;
        }

        // Should this be cloned?
        let tok = ctx.advance();

        if let Token::Id(id) = tok.token {
            let name = interner.search(id);
            dbg!(name, &tok.token);
        }

        match &tok.token {
            Token::Id(id) => match *id {
                id if id == ReservedKeyword::Bind as usize => {
                    // ITS FINE. ITS COMPLETELY FINE. NOT A  TODO:
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();

                    if let Ok(Symbol::Path { id }) = parse_bind_section(&mut ctx) {
                        table.symbols.insert(id, Symbol::Path { id });
                    };
                }
                id if id == ReservedKeyword::Var as usize => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();
                }
                id if id == ReservedKeyword::Nest as usize => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();
                }
                id if id == ReservedKeyword::ComplexRules as usize => {
                    ctx.expect_basic(TokenKind::SlimArrow, Branch::Searching)
                        .ok();
                }
                t => unimplemented!("Failed in main parse branch with tok of id '{:?}'", t),
            },
            // Token::Illegal(_) => todo!(),
            Token::EOF => break,
            t => unimplemented!("Failed in main parse branch with tok '{:?}'", t),
        }
    }

    if !ctx.err_vec.borrow().is_empty() {
        // WHERES MY ITERATOR
        for err in ctx.err_vec.borrow().iter() {
            eprintln!("{}", err.msg);
        }
        panic!("I am new to thinking");
        // std::process::exit(1);
    }

    dbg!(ctx.err_vec.borrow());
    table
}

//FIXME: NEED TO SKIP IN CONTEXT STRUCT
fn unwind() {}

//TODO: Set only to one bind allowed for now
fn parse_bind_section(ctx: &mut Context) -> Result<Symbol, ()> {
    //FIX: NEED TO HAVE UNIFIED ERROR HANDLE POINT
    if let Ok(Token::Literal(id)) = ctx.expect_basic(TokenKind::Literal, Branch::Bind) {
        return Ok(Symbol::Path { id });
    }

    Err(())
}

fn parse_var_section<'a>(ctx: &'a Context) -> Result<Symbol, ()> {
    todo!()
}

fn parse_nest_section<'a>(ctx: &'a Context) -> Result<Symbol, ()> {
    todo!()
}

fn parse_complex_section<'a>(ctx: &'a Context) -> Result<Symbol, ()> {
    todo!()
}
