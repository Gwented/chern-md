use crate::token::{SpannedToken, TokenKind};

// Has a lifetime because of previous clone concerns in instantiation
#[derive(Debug)]
pub struct ParseError<'a> {
    expected: TokenKind,
    found: &'a SpannedToken,
    branch: Branch,
    prev_tok: SpannedToken,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Branch {
    Searching,
    Bind,
    Var,
    Nest,
    ComplexRules,
}

impl ParseError<'_> {
    pub fn new<'a>(
        expected: TokenKind,
        found: &'a SpannedToken,
        branch: Branch,
        prev_tok: &SpannedToken,
    ) -> ParseError<'a> {
        ParseError {
            expected,
            found,
            branch,
            prev_tok: prev_tok.clone(),
        }
    }
}
