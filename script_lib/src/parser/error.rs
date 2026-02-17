use crate::token::{SpannedToken, TokenKind};

pub struct ParseError<'a> {
    expected: TokenKind,
    found: &'a SpannedToken,
    branch: Branch,
}

pub enum Branch {
    Searching,
    Bind,
    Var,
    Nest,
    ComplexRules,
}

impl ParseError<'_> {
    pub fn new(expected: TokenKind, found: &SpannedToken, branch: Branch) -> ParseError<'_> {
        ParseError {
            expected,
            found,
            branch,
        }
    }
}
