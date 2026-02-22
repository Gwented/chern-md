use std::cell::RefCell;

use common::{intern::Intern, primitives::PrimitiveKeywords};

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{Span, SpannedToken, Token, TokenKind},
};
//WARN: I don't know if this is ok
const RED: &str = "\x1b[31m";
const NC: &str = "\x1b[0m";
const TOTAL_SEPARATORS: usize = 40;

//TODO: May give it the interner directly
#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) original_text: &'a [u8],
    pub(crate) tokens: &'a [SpannedToken],
    pub(crate) pos: usize,
    // Needs RefCell for recursion
    pub(crate) err_vec: RefCell<Vec<Diagnostic>>,
    // pub(crate) warn_vec: Vec<ParseError<'a>>,
    // or could just have a level with diagnostic
}

impl<'a> Context<'a> {
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

        let (ln, col, segment) = self.get_location(&found.span);

        let msg = if let Some(id) = id_opt {
            let name_id = interner.search(id as usize);

            format!("(in {branch})\n[{ln}:{col}] {bmsg} \"{name_id}\"{amsg}\n\n{segment}",)
        } else {
            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}'{}'{amsg}\n\n{segment}",
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
            let (ln, col, segment) = self.get_location(&found.span);

            let separator = "-".repeat(TOTAL_SEPARATORS);

            // (in {branch})\n{msg}\n|\n[{ln}:{col}]\n{segment}\n{separator}
            let msg = format!(
                "(in {branch})\n Expected '{expected}', found '{}'. {}\n|\n[{ln}:{col}]\n{segment}\n{separator}",
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

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);
        let msg = format!("(in {branch})\n{msg}\n|\n[{ln}:{col}]\n{segment}\n{separator}");

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
        help: Option<&str>,
        interner: &Intern,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];
        self.pos += 1;

        let id_opt = match found.token {
            Token::Id(id) | Token::Literal(id) | Token::Number(id) => Some(id),
            _ => None,
        };

        if found.token.kind() == expected {
            return Ok(found.token);
        }

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = if let Some(id) = id_opt {
            let name_id = interner.search(id as usize);

            format!(
                "(in {branch})\n {bmsg}{} \"{name_id}\"{amsg}\n|\n[{ln}:{col}]\n{segment}\n{separator}",
                found.token.kind()
            )
        } else {
            format!(
                "(in {branch})\n[{ln}:{col}] {bmsg}'{}'{amsg}\n|\n[{ln}:{col}]\n{segment}\n{separator}",
                found.token.kind()
            )
        };

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

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = format!(
            "(in {branch})\n Expected {emsg}, found {fmsg}\n|\n[{ln}:{col}]\n{segment}\n{separator}",
        );

        self.recover();

        let report = Diagnostic::new(msg, Branch::VarTypeArgs);

        self.err_vec.borrow_mut().push(report);
    }

    //FIX: Should likely return helper struct of `Segment`
    fn get_location(&self, span: &Span) -> (usize, usize, String) {
        let mut ln = 1;
        let mut col = 1;
        dbg!(span);

        let mut b: u8;

        let mut seg_start = 0;

        for i in 0..span.end {
            b = self.original_text[i];

            //TODO: See if this works on windows
            if b == b'\r' && self.original_text.get(i + 1).copied() == Some(b'\n') {
                ln += 1;
                // Offset to skip new line since cannot alter for loop counter directly
                // Should likely just manually loop to avoid odditiy
                seg_start = i + 2;
                col = 1;
            } else if b == b'\n' {
                ln += 1;
                seg_start = i + 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        let seg_end = self.get_line_end(seg_start);

        let segment = &self.original_text[seg_start..seg_end];

        let segment = str::from_utf8(segment)
            .expect("[temp] Invalid UTF-8 although would be impossible after lexer");

        // `Span` range is inclusive exclusive so final character is missed otherwise
        let span_diff_offset = span.end - span.start + 1;

        let spaces = " ".repeat(segment.len() - span_diff_offset);

        // Same offset reason.
        let arrows = "^".repeat(span_diff_offset);

        let fmt_segment = format!("{segment}\n{spaces}{RED}{arrows}{NC}");
        // FIX: col stops at span.end not span.start
        //                                                    help: ^^
        (ln, col, fmt_segment)
    }

    fn get_line_end(&self, start: usize) -> usize {
        for i in start..self.original_text.len() {
            let b = self.original_text[i];
            dbg!(b as char);

            if b == b'\r' && self.original_text.get(i + 1).copied() == Some(b'\n') {
                return i;
            } else if b == b'\n' {
                return i;
            }
        }

        self.original_text.len()
    }

    /// Is inteded to override recursive information loss for better errors.
    pub(crate) fn try_rewind(&mut self, cap: usize) -> Option<PrimitiveKeywords> {
        for i in 1..=cap {
            if self.pos - i < self.tokens.len() {
                return None;
            }
            dbg!("Descending");
            dbg!(self.pos);

            panic!("Made it?");
            let tok = self.tokens[self.pos - i].token;

            match tok {
                Token::Id(id) => {
                    let res = PrimitiveKeywords::try_from(id);

                    if res.is_ok() {
                        panic!("Found it");
                        res.ok();
                    }
                }
                _ => (),
            }
        }
        panic!("Isnta");

        None
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
                dbg!("Recovering in");
                self.advance();
            }
            // dbg!("Recovered");
        }
    }

    pub(crate) fn peek(&self) -> &SpannedToken {
        dbg!(&self.tokens[self.pos]);
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
}
