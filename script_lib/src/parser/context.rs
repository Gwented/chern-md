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
        // dbg!(&self.tokens[self.pos - 2]);
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
        self.tokens[self.pos].token.kind()
    }

    pub(crate) fn advance(&mut self) -> &SpannedToken {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    //TODO: (Possibly) REASON FOR EMPTY ERROR. I DO NOT REPORT IT, THE METHOD DOES.
    //REPORTING TWICE WHEN THE BRANCH WAS GIVEN IS REDUNDANT. (probably)

    //FIXME: GET RID OF THIS
    //May want to return Option<usize>. Or get rid of it.
    //This is a horrible dependency
    //Add an Option<&str> side note.
    pub(crate) fn expect_id(&mut self, expected: TokenKind, branch: Branch) -> Result<u32, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        match found.token {
            Token::Id(id) if expected == TokenKind::Id => Ok(id),
            Token::Literal(id) if expected == TokenKind::Literal => Ok(id),
            Token::Number(id) if expected == TokenKind::Number => Ok(id),
            // Token::EOF => todo!(),
            _ => {
                //FIX: Needs to be removed or some type
                let ln = found.span.ln();
                let col = found.span.col();
                dbg!(&branch);

                //TODO: TEMP ERR MSG
                let msg = format!(
                    "(in {})\n[{}:{}] Expected '{}' but found '{}'",
                    branch,
                    ln,
                    col,
                    expected,
                    found.token.kind(),
                );

                self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

                self.recover();

                return Err(found.token);
            }
        }
    }

    // IT WORKS

    // FIXME: Maybe just clone the SpannedToken...
    // pub fn expect_type(&mut self, expected: TokenKind, branch: Branch) -> Result<ActualType, ()> {
    //     let found = &self.tokens[self.pos];
    //     self.pos += 1;
    //
    // }

    //FIX:
    //Maybe this method should just be more composite inherently, or have a more composite version
    pub(crate) fn expect_basic(
        &mut self,
        expected: TokenKind,
        branch: Branch,
        extra: Option<&str>,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        if found.token.kind() != expected {
            let ln = found.span.ln();
            let col = found.span.col();

            //TODO: TEMP ERR MSG
            //NEEDS IDS

            let msg = format!(
                "(in {})\n[{}:{}] Expected '{}' but found '{}'. {}",
                //  Scared of space here                       ^^^^
                branch,
                ln,
                col,
                expected,
                found.token.kind(),
                extra.unwrap_or_default()
            );

            self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

            self.recover();

            return Err(found.token);
        }

        Ok(found.token.clone())
    }

    pub(crate) fn report_template(&mut self, emsg: &str, fmsg: &str, branch: Branch) {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        let ln = found.span.ln();
        let col = found.span.col();

        let msg = format!(
            "(in {})\n[{}:{}] Expected {} but found {}",
            branch, ln, col, emsg, fmsg,
        );

        self.recover();

        let report = Diagnostic::new(msg, Branch::VarInnerArgs);

        self.err_vec.borrow_mut().push(report);
    }

    pub(crate) fn recover(&mut self) {
        dbg!(self.pos, self.tokens.len());
        if self.pos < self.tokens.len() && self.peek_kind() != TokenKind::EOF {
            while self.pos < self.tokens.len() + 2
                && self.peek_ahead(1).token.kind() != TokenKind::EOF
                && self.peek_ahead(1).token.kind() != TokenKind::SlimArrow
                && self.peek_ahead(1).token.kind() != TokenKind::Colon
            {
                // dbg!(self.peek_ahead(1).token.kind());
                // dbg!("Recovering in");
                self.advance();
            }
            // dbg!("Recovered");
        }
    }
}
