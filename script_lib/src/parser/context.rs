use std::cell::RefCell;

use crate::{
    parser::error::{Branch, ParseError},
    token::{ActualType, InnerArgs, SpannedToken, TokenKind},
};

pub struct Context<'a> {
    pub(super) tokens: &'a [SpannedToken],
    pub(super) pos: usize,
    // Needs RefCell for recursion
    pub(super) err_vec: RefCell<Vec<ParseError<'a>>>,
    // pub(super) warn_vec: Vec<ParseError<'a>>,
}

impl Context<'_> {
    pub fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos]
    }

    pub fn peek_ahead(&self, dest: usize) -> &SpannedToken {
        &self.tokens[self.pos + dest]
    }

    pub fn skip(&mut self, dest: usize) -> &SpannedToken {
        self.pos += dest;
        &self.tokens[self.pos + dest]
    }

    pub fn peek_kind(&self) -> TokenKind {
        self.tokens[self.pos].token.kind()
    }

    pub fn advance(&mut self) -> &SpannedToken {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    //FIXME: Some custom error
    pub fn expect(&mut self, expected: TokenKind) -> Result<&SpannedToken, ParseError<'_>> {
        let found = self.advance();

        if found.token.kind() != expected {
            return Err(ParseError::new(expected, found, Branch::Searching));
        }

        Ok(found)
    }
}

#[derive(Debug)]
pub struct Word {
    // May be integer idk
    id: String,
    ty: ActualType,
    args: Vec<InnerArgs>,
    cond: Vec<Cond>,
}

#[derive(Debug)]
pub enum Cond {
    // Approximation operator is a range internally.
    Range(usize, usize),
    // Probably should just attach bool
    IsEmpty,
    Len(usize),
    // Ok this is kinda cool
    Not(Box<Cond>),
}
