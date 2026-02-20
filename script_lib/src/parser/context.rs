use std::cell::RefCell;

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{Span, SpannedToken, Token, TokenKind},
};

pub struct Context<'a> {
    pub(super) tokens: &'a [SpannedToken],
    pub(super) pos: usize,
    // Needs RefCell for recursion
    pub(super) err_vec: RefCell<Vec<Diagnostic>>,
    // pub(super) warn_vec: Vec<ParseError<'a>>,
}

impl<'a> Context<'a> {
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
        dbg!(&self.tokens[self.pos]);
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    //TODO: (Possibly) REASON FOR EMPTY ERROR. I DO NOT REPORT IT, THE METHOD DOES.
    //REPORTING TWICE WHEN THE BRANCH WAS GIVEN IS REDUNDANT. (probably)

    pub fn expect_basic(&mut self, expected: TokenKind, branch: Branch) -> Result<Token, ()> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        if found.token.kind() != expected {
            //FIX: NEEDS TO BE SOME OR NONE
            let prev_tok = &self.tokens[self.pos - 2];
            let ln = found.span.ln();
            let col = found.span.col();

            //TODO: TEMP ERR MSG
            let msg = format!(
                "(in {})\n[{}:{}] Expected '{}' but found '{}'.",
                branch, ln, col, expected, found.token
            );

            self.err_vec
                .borrow_mut()
                .push(Diagnostic::new(msg, branch, &prev_tok));

            return Err(());
        }

        Ok(found.token.clone())
    }

    pub fn expect_template(span: Span, emsg: &str, fmsg: &str) -> Result<Token, ()> {
        let msg = format!("Expected {emsg} but found {fmsg}");
        unimplemented!()
    }
}
