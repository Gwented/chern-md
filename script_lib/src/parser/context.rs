use std::cell::RefCell;

use common::intern::Intern;

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{SpannedToken, Token, TokenKind},
};

//TODO: May give it the interner directly
pub struct Context<'a> {
    pub(crate) tokens: &'a [SpannedToken],
    pub(crate) pos: usize,
    // Needs RefCell for recursion
    pub(crate) err_vec: RefCell<Vec<Diagnostic>>,
    // pub(crate) warn_vec: Vec<ParseError<'a>>,
}

impl<'a> Context<'a> {
    pub(crate) fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos]
    }

    pub(crate) fn peek_ahead(&self, dest: usize) -> &SpannedToken {
        &self.tokens[self.pos + dest]
    }

    pub(crate) fn peek_kind(&self) -> TokenKind {
        self.tokens[self.pos].token.kind()
    }

    // Inlined several times to avoid cloning.
    pub(crate) fn advance(&mut self) -> &SpannedToken {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    /// The Intern won.
    pub(crate) fn expect_id(
        &mut self,
        expected: TokenKind,
        branch: Branch,
        interner: Intern,
    ) -> Result<u32, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        match found.token {
            Token::Id(id) if expected == TokenKind::Id => Ok(id),
            Token::Literal(id) if expected == TokenKind::Literal => Ok(id),
            Token::Number(id) if expected == TokenKind::Number => Ok(id),
            _ => {
                //FIX: Need byte ranges
                let ln = found.span.ln;
                let col = found.span.col;
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

    //Configuration for expected and found being disabled
    pub(crate) fn expect_id_verbose(
        &mut self,
        expected: TokenKind,
        bmsg: &str,
        amsg: &str,
        branch: Branch,
        // help: Option<&str>
        interner: &Intern,
    ) -> Result<u32, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        // Maybe just check each individually first so we know it is invalid after.
        let id_opt = match found.token {
            Token::Id(id) | Token::Literal(id) | Token::Number(id) => {
                if found.token.kind() == expected {
                    return Ok(id);
                }

                Some(id)
            }
            _ => None,
        };

        let ln = found.span.ln;
        let col = found.span.col;

        let msg = if let Some(id) = id_opt {
            let name_id = interner.search(id as usize);

            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}{} \"{name_id}\"{amsg}",
                found.token.kind()
            )
        } else {
            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}'{}'{amsg}",
                found.token.kind()
            )
        };

        //FIX: Need byte ranges

        self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

        self.recover();

        Err(found.token)
    }

    /// Intended for basic errors that need little context after
    pub(crate) fn expect_basic(
        &mut self,
        expected: TokenKind,
        branch: Branch,
        extra: Option<&str>,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        if found.token.kind() != expected {
            let ln = found.span.ln;
            let col = found.span.col;

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

    pub(crate) fn report_verbose(&mut self, msg: &str, branch: Branch) {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        let ln = found.span.ln;
        let col = found.span.col;

        let msg = format!("(in {})\n[{}:{}] {}", branch, ln, col, msg);

        self.recover();

        let report = Diagnostic::new(msg, branch);

        self.err_vec.borrow_mut().push(report);
    }

    /// Fully curated version of `expect_basic`
    //TODO: Primitive type recognition for printing all errors
    pub(crate) fn expect_verbose(
        &mut self,
        expected: TokenKind,
        bmsg: &str,
        amsg: &str,
        branch: Branch,
        interner: &Intern,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];
        dbg!(&found);
        self.pos += 1;
        dbg!(&self.tokens[self.pos]);

        let id_opt = match found.token {
            Token::Id(id) | Token::Literal(id) | Token::Number(id) => Some(id),
            _ => None,
        };

        if found.token.kind() == expected {
            return Ok(found.token);
        }

        let ln = found.span.ln;
        let col = found.span.col;

        let msg = if let Some(id) = id_opt {
            let name_id = interner.search(id as usize);

            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}{} \"{name_id}\"{amsg}",
                found.token.kind()
            )
        } else {
            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}'{}'{amsg}",
                found.token.kind()
            )
        };

        //FIX: Need byte ranges

        self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

        self.recover();

        Err(found.token)
    }

    /// Intended to take down horrific dependency
    // pub(crate) fn report_direct(&mut self, msg: &str, branch: Branch) {
    //     let report = Diagnostic::new(msg.to_string(), branch);
    //     self.err_vec.borrow_mut().push(report);
    // }

    /// More composable "Expected but found" error
    pub(crate) fn report_template(&mut self, emsg: &str, fmsg: &str, branch: Branch) {
        //BUG: POSSIBLY FIXED OF WRONG LN AND COL PRINT
        let found = &self.tokens[self.pos - 2];

        let ln = found.span.ln;
        let col = found.span.col;

        let msg = format!(
            "(in {})\n[{}:{}] Expected {}, found {}",
            branch, ln, col, emsg, fmsg,
        );

        self.recover();

        let report = Diagnostic::new(msg, Branch::VarTypeArgs);

        self.err_vec.borrow_mut().push(report);
    }

    //TODO: Branch specific behavior
    fn recover(&mut self) {
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
