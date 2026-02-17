pub mod context;
pub mod error;

use common::intern::Intern;
use std::cell::RefCell;

use crate::parser::error::ParseError;

use crate::{
    parser::context::{Context, Word},
    token::{SpannedToken, Table, Token, TokenKind},
};

pub fn parse(tokens: &Vec<SpannedToken>) -> Table {
    let table = Table::new();

    let mut ctx = Context {
        tokens,
        pos: 0,
        err_vec: RefCell::new(Vec::new()),
    };

    // May change to basic loop
    while ctx.peek_kind() != TokenKind::EOF {
        let tok = ctx.peek();

        match &tok.token {
            Token::Id(id) => match id {
                // Unsure about this
                // 0 == bind. 1 == var. 2 == nest. 3 == complex_rules
                0 => {
                    parse_bind_section(&mut ctx);
                }
                1 => (),
                2 => (),
                3 => (),
                _ => {}
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
