use std::cell::RefCell;

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{SpannedToken, Token, TokenKind},
};

pub struct Context<'a> {
    pub(crate) tokens: &'a [SpannedToken],
    pub(crate) pos: usize,
    // Needs RefCell for recursion
    pub(crate) err_vec: RefCell<Vec<Diagnostic>>,
    // pub(crate) warn_vec: Vec<ParseError<'a>>,
}

impl<'a> Context<'a> {
    pub(crate) fn peek(&self) -> &SpannedToken {
        dbg!(&self.tokens[self.pos - 2]);
        &self.tokens[self.pos]
    }

    pub(crate) fn peek_ahead(&self, dest: usize) -> &SpannedToken {
        &self.tokens[self.pos + dest]
    }

    // pub(crate) fn skip(&mut self, dest: usize) -> &SpannedToken {
    //     self.pos += dest;
    //     &self.tokens[self.pos + dest]
    // }

    pub(crate) fn peek_kind(&self) -> TokenKind {
        dbg!(&self.tokens[self.pos - 2].token.kind());
        self.tokens[self.pos].token.kind()
    }

    pub(crate) fn advance(&mut self) -> &SpannedToken {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    //TODO: (Possibly) REASON FOR EMPTY ERROR. I DO NOT REPORT IT, THE METHOD DOES.
    //REPORTING TWICE WHEN THE BRANCH WAS GIVEN IS REDUNDANT. (probably)

    //FIXME: Change to usize return
    //ADD IDENTIFIER SPECIFIC FUNCTIONS FOR IDS

    pub(crate) fn expect_id(
        &mut self,
        expected: TokenKind,
        branch: Branch,
    ) -> Result<usize, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        // Horrific.
        match found.token.clone() {
            Token::Id(id) if expected == TokenKind::Id => Ok(id),
            Token::Literal(id) if expected == TokenKind::Literal => Ok(id),
            // Token::EOF => todo!(),
            _ => {
                let prev_tok = &self.tokens[self.pos - 2];
                let ln = found.span.ln();
                let col = found.span.col();
                dbg!(&branch);

                //TODO: TEMP ERR MSG
                let msg = format!(
                    "(in {})\n[{}:{}] Expected '{}' but found '{}'. [{}]",
                    branch,
                    ln,
                    col,
                    expected,
                    found.token.kind(),
                    prev_tok.token.kind()
                );

                self.err_vec
                    .borrow_mut()
                    .push(Diagnostic::new(msg, branch, &prev_tok));

                self.recover();

                return Err(found.token);
            }
        }
    }

    // IT WORKS
    pub(crate) fn recover(&mut self) {
        dbg!("Recovering from", &self.tokens[self.pos - 2]);
        if self.peek_kind() != TokenKind::EOF {
            while self.pos > self.tokens.len()
                && self.peek_ahead(1).token.kind() != TokenKind::Colon
                && self.peek_ahead(1).token.kind() != TokenKind::Id
            {
                dbg!("Recovering in");
                self.advance();
            }
            dbg!("Recovered");
        }
    }

    // FIXME: Maybe just clone the SpannedToken...
    // pub fn expect_type(&mut self, expected: TokenKind, branch: Branch) -> Result<ActualType, ()> {
    //     let found = &self.tokens[self.pos];
    //     self.pos += 1;
    //
    // }

    pub fn expect_num(&mut self) -> Result<usize, ()> {
        todo!()
    }

    pub(crate) fn expect_basic(
        &mut self,
        expected: TokenKind,
        branch: Branch,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        if found.token.kind() != expected {
            //FIX: NEEDS TO BE SOME OR NONE
            let prev_tok = &self.tokens[self.pos - 2];
            let ln = found.span.ln();
            let col = found.span.col();

            //TODO: TEMP ERR MSG
            let msg = format!(
                "(in {})\n[{}:{}] Expected '{}' but found '{}'. [{:?}]",
                branch,
                ln,
                col,
                expected,
                found.token.kind(),
                prev_tok.token
            );

            self.err_vec
                .borrow_mut()
                .push(Diagnostic::new(msg, branch, &prev_tok));

            self.recover();

            return Err(found.token);
        }

        Ok(found.token.clone())
    }

    pub(crate) fn report_template(&mut self, emsg: &str, fmsg: &str, branch: Branch) {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        let prev_tok = &self.tokens[self.pos - 2];
        let ln = found.span.ln();
        let col = found.span.col();

        //TODO: TEMP ERR MSG
        let msg = format!(
            "(in {})\n[{}:{}] Expected {} but found {}.",
            branch, ln, col, emsg, fmsg,
        );

        self.recover();

        let report = Diagnostic::new(msg, Branch::InnerArgs, prev_tok);

        self.err_vec.borrow_mut().push(report);
    }
}
