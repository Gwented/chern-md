use std::cell::RefCell;

use common::{intern::Intern, primitives::PrimitiveKeywords};

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{Span, SpannedToken, Token, TokenKind},
};
//FIX: ANSI terminal specific
const RED: &str = "\x1b[31m";
const ORANGE: &str = "\x1b[33m";
const NC: &str = "\x1b[0m";

/// Amount of '-' to print for multiple error separation
const TOTAL_SEPARATORS: usize = 60;

/// This must remain greater than 1 or everything will break. This was the goal.
/// Check if recover not peeking causes crash without this or if it's just the tree walking
///
/// The EOF issue is because of tokens now being able to just ignore EOF and force crashes,
/// but it's from there being no unified agreement on if it's EOF, quit. I will speak with the
/// workers.
///
/// It actually does what it's supposed to do now that poisoned as be sprinkled
/// The point of this is to act as retry logic so it stops hallucinating. Especially EOF
const MAX_TOLERANCE: u8 = 3;

#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) tolerance: u8,
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

        // Leads to EOF being skipped and index out of bounds unless this is done.
        // TODO: See if changes fixed this bug
        if self.peek_kind() != TokenKind::EOF {
            self.pos += 1;
        }

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

            format!("(in {branch})\n{bmsg}\"{name_id}\"{amsg}\n|\n|[{ln}:{col}]\n|\n{segment}",)
        } else {
            format!(
                "(in {branch})\n{bmsg}'{}'{amsg}\n|\n|[{ln}:{col}]\n|\n{segment}",
                found.token.kind()
            )
        };

        self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

        self.recover(branch);

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
        dbg!(&found);

        // Leads to EOF being skipped and index out of bounds unless this is done.
        if self.peek_kind() != TokenKind::EOF {
            self.pos += 1;
        }

        // Are we serious?
        if found.token.kind() != expected {
            let (ln, col, segment) = self.get_location(&found.span);

            let separator = "-".repeat(TOTAL_SEPARATORS);

            let msg = format!(
                "(in {branch})\n Expected '{expected}', found '{}'. {}\n|\n|\n[{ln}:{col}]\n{segment}\n{separator}",
                found.token.kind(),
                extra.unwrap_or_default()
            );

            self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

            self.tolerance += 1;

            if self.tolerance >= MAX_TOLERANCE {
                self.recover(branch);
                self.tolerance = 0;
            }

            return Err(found.token);
        }

        Ok(found.token.clone())
    }

    /// ALWAYS advance before using this or ensure an advance happened before
    pub(crate) fn report_verbose(&mut self, msg: &str, branch: Branch) {
        let found = &self.tokens[self.pos - 1];

        // MAJOR BUG: EOF is never stopped here
        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = format!("(in {branch})\n{msg}\n|\n|\n[{ln}:{col}]\n{segment}\n{separator}");

        self.tolerance += 1;

        if self.tolerance > MAX_TOLERANCE {
            self.recover(branch);
            self.tolerance = 0;
        }

        let report = Diagnostic::new(msg, branch);

        self.err_vec.borrow_mut().push(report);
    }

    /// Fully curated version of `expect_basic`
    //TODO: Primitive type recognition for printing all errors

    //
    //FIX: BECOME MORE CONTEXT AWARE IN HELP FROM INSIDE
    pub(crate) fn expect_verbose(
        &mut self,
        expected: TokenKind,
        bmsg: &str,
        amsg: &str,
        help: Option<&str>,
        branch: Branch,
        interner: &Intern,
    ) -> Result<Token, Token> {
        let found = &self.tokens[self.pos];

        // Leads to EOF being skipped and index out of bounds unless this is done.
        if self.peek_kind() != TokenKind::EOF {
            self.pos += 1;
        }

        let id_opt = match found.token {
            Token::Id(id) | Token::Literal(id) | Token::Number(id) => Some(id),
            _ => None,
        };

        if found.token.kind() == expected {
            return Ok(found.token);
        }

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let help = if let Some(msg) = help {
            format!("{ORANGE}Help{NC}: {msg}\n")
        } else {
            "".to_string()
        };

        let msg = if let Some(id) = id_opt {
            let name = interner.search(id as usize);

            // New line is after since no help would space it out by default
            // WARN:

            format!(
                "(in {branch})\n{bmsg}{} \"{name}\"{amsg}\n|\n|\n[{ln}:{col}]\n{segment}\n{help}{separator}",
                found.token.kind()
            )
        } else {
            format!(
                "(in {branch})\n{bmsg}'{}'{amsg}\n|\n|\n[{ln}:{col}]\n{segment}\n{help}{separator}",
                found.token.kind()
            )
        };

        self.err_vec.borrow_mut().push(Diagnostic::new(msg, branch));

        self.tolerance += 1;

        if self.tolerance >= MAX_TOLERANCE {
            self.recover(branch);
            self.tolerance = 0;
        }

        Err(found.token)
    }

    /// More composable "Expected but found" error.
    /// [...] Expected [emsg], found [fmsg]
    pub(crate) fn report_template(&mut self, emsg: &str, fmsg: &str, branch: Branch) {
        let found = &self.tokens[self.pos - 1];

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = format!(
            "(in {branch})\n Expected {emsg}, found {fmsg}\n|\n|\n[{ln}:{col}]\n{segment}\n{separator}",
        );

        self.tolerance += 1;

        if self.tolerance > MAX_TOLERANCE {
            self.recover(branch);
        }

        let report = Diagnostic::new(msg, Branch::VarTypeArgs);

        self.err_vec.borrow_mut().push(report);
    }

    //FIX: Should likely return helper struct of `Segment`
    fn get_location(&self, span: &Span) -> (usize, usize, String) {
        let mut ln = 1;
        let mut col = 1;

        for (i, t) in self.tokens.iter().enumerate() {
            println!("index {i}: {t:?}");
        }

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

        // Needs offset or will print span.end when span.start is more informational
        col -= span.end - span.start;

        let seg_end = self.get_line_end(seg_start);

        let segment = &self.original_text[seg_start..seg_end];

        //FIX:
        let segment = str::from_utf8(segment)
            .expect("[temp] Invalid UTF-8 although would be impossible after lexer");

        // Span range is inclusive exclusive so final character is missed otherwise
        // Has no other mathematical outside of this
        let span_diff_offset = span.end - span.start + 1;

        let arrows = "^".repeat(span_diff_offset);

        // Spaces need to be proportional to the current line's size therefore it must
        // stay inside the range. THIS SHOULD HAVE BEEN OBVIOUS.
        // WARN: UTF-8 issues possible
        let space_offset = self.original_text[seg_start..span.start].len();

        let spaces = " ".repeat(space_offset);

        //FIX: VIOLATES SRP JAVA EE PSPRIBOOT

        let fmt_segment = format!("\t{segment}\n\t{spaces}{RED}{arrows}{NC}");

        (ln, col, fmt_segment)
    }

    fn get_line_end(&self, start: usize) -> usize {
        for i in start..self.original_text.len() {
            let b = self.original_text[i];

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
    //WARN: WATCH THIS CLOSELY
    fn recover(&mut self, branch: Branch) {
        let target = self.match_target(branch);
        //FIX: SEE IF SELF.POS CHECK IS STILL NEEDED. WAS REMOVED.
        if self.peek_kind() != TokenKind::EOF {
            while self.pos < self.tokens.len() + 2
                && self.peek_kind() != TokenKind::EOF
                && self.peek_ahead(1).token.kind() != TokenKind::SlimArrow
                && self.peek_ahead(1).token.kind() != TokenKind::Colon
                && self.peek_kind() != target
            {
                self.advance();
            }
        }
    }

    //TEST:
    // MAKE THEM APPLY SOFTMAX FUNCTIONS BY SCANNING PAST TOKENS TO PICK MOST PROBABLY TOKEN
    fn match_target(&self, branch: Branch) -> TokenKind {
        match branch {
            Branch::Searching => TokenKind::Id,
            Branch::Bind => TokenKind::Colon,
            Branch::Var => TokenKind::OParen,
            Branch::VarType => TokenKind::HashSymbol,
            // Or OParen
            Branch::VarCond => TokenKind::Comma,
            // Or Comma
            Branch::VarTypeArgs => TokenKind::HashSymbol,
            Branch::Nest => todo!(),
            Branch::ComplexRules => todo!(),
        }
    }

    // const EULERS_NUMBER: f32 = 2.71828;
    //
    // // Wait I don't have probabilities
    // fn softmax(&self) {
    //     let mut n = 0;
    //
    //     for tok in self.tokens.iter().map(|t| t.token).into_iter() {}
    // }

    fn peek(&self) -> &SpannedToken {
        dbg!(&self.tokens[self.pos]);
        &self.tokens[self.pos]
    }

    pub(crate) fn peek_tok(&mut self) -> Token {
        self.tokens
            .get(self.pos)
            .map(|t| t.token)
            .unwrap_or(Token::EOF)
    }

    pub(crate) fn peek_ahead(&self, dest: usize) -> &SpannedToken {
        &self.tokens[self.pos + dest]
    }

    pub(crate) fn peek_kind(&self) -> TokenKind {
        self.tokens
            .get(self.pos)
            .map(|t| t.token.kind())
            .unwrap_or(TokenKind::EOF)
    }

    pub(crate) fn advance_tok(&mut self) -> Token {
        let t = self.tokens[self.pos].token;
        self.pos += 1;
        t
    }

    fn advance(&mut self) -> &SpannedToken {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }
}
