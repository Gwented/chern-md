pub mod context;
pub mod error;

use common::intern::{Intern, ReservedKeyword};
use std::cell::RefCell;

use crate::parser::error::ParseError;

use crate::{
    parser::context::{Context, Word},
    token::{SpannedToken, Table, Token, TokenKind},
};

pub fn parse(tokens: &Vec<SpannedToken>, interner: &mut Intern) -> Table {
    let table = Table::new();

    let mut ctx = Context {
        tokens,
        pos: 0,
        err_vec: RefCell::new(Vec::new()),
    };

    // May change to basic loop
    while ctx.peek_kind() != TokenKind::EOF {
        let tok = ctx.peek();
        // if let Token::Id(id) = tok.token {
        //     let name = interner.search(id);
        //     dbg!(name, &tok.token);
        //     panic!("Not a feature");
        // }

        match &tok.token {
            Token::Id(id) => match *id {
                id if id == ReservedKeyword::Bind as usize => {
                    parse_bind_section(&mut ctx);
                }
                id if id == ReservedKeyword::Var as usize => {}
                id if id == ReservedKeyword::Nest as usize => {}
                id if id == ReservedKeyword::ComplexRules as usize => {}
                t => unimplemented!("Failed in main parse branch with tok of id '{:?}'", t),
            },
            // Token::Illegal(_) => todo!(),
            Token::EOF => break,
            t => unimplemented!("Failed in main parse branch with tok '{:?}'", t),
        }
    }

    table
}

fn parse_bind_section(ctx: &mut Context) -> Word {
    todo!("In bind")
}

fn parse_var_section<'a>(ctx: &'a Context) -> Result<Word, ParseError<'a>> {
    todo!()
}

fn parse_nest_section<'a>(ctx: &'a Context) -> Result<Word, ParseError<'a>> {
    todo!()
}

fn parse_complex_section<'a>(ctx: &'a Context) -> Result<Word, ParseError<'a>> {
    todo!()
}
