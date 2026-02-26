use std::io::IsTerminal;

use common::intern::Intern;

use crate::{
    parser::error::{Branch, Diagnostic},
    token::{Span, SpannedToken, Token, TokenKind},
};
//FIX: ANSI terminal specific
const RED: &str = "\x1b[31m";
const ORANGE: &str = "\x1b[33m";
const NC: &str = "\x1b[0m";

//TODO: A struct that contains something like the branch, and error type instead of params

/// Amount of '-' to print for multiple error separation
const TOTAL_SEPARATORS: usize = 60;

//UNUSED:
const MAX_RETRIES: u8 = 1;

#[derive(Debug)]
pub struct Context<'a> {
    //TEST:
    pub(crate) should_exit: bool,
    //TEST:
    pub(crate) original_text: &'a [u8],
    pub(crate) tokens: &'a [SpannedToken],
    pub(crate) pos: usize,
    pub(crate) err_vec: Vec<Diagnostic>,
    pub(crate) can_color: bool,
}

// Make more composable or something
// Make "missing" report that covers common wrong characters to help parser in data structures
impl<'a> Context<'a> {
    pub fn new(original_text: &'a [u8], tokens: &'a [SpannedToken]) -> Context<'a> {
        Context {
            should_exit: false,
            original_text,
            tokens,
            pos: 0,
            err_vec: Vec::new(),
            can_color: std::io::stdout().is_terminal(),
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

        // Leads to EOF being skipped and index out of bounds unless this is done.
        // WARN: This is a fail safe for logic errors
        // if self.peek_kind() != TokenKind::EOF {
        // }
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
            format!("(in {branch})\n{bmsg}\"{name_id}\"{amsg}\n|\n|[{ln}:{col}]\n|\n{segment}",)
        } else {
            format!(
                "(in {branch})\n{bmsg}'{}'{amsg}\n|\n|[{ln}:{col}]\n|\n{segment}",
                found.token.kind()
            )
        };

        self.err_vec.push(Diagnostic::new(msg, branch));

        if self.should_exit {
            self.recover(branch);
            self.should_exit = false;
        } else {
            self.should_exit = true;
        }

        Err(found.token)
    }

    /// Intended for basic errors that need little context after
    /// ALWAYS advance before using this or ensure an advance happened before
    pub(crate) fn report_verbose(&mut self, msg: &str, branch: Branch) {
        let found = &self.tokens[self.pos - 1];

        // MAJOR BUG: EOF is never stopped here
        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = format!("(in {branch})\n{msg}\n|\n|\n[{ln}:{col}]\n{segment}\n{separator}");

        if self.should_exit {
            self.recover(branch);
            self.should_exit = false;
        } else {
            self.should_exit = true;
        }

        let report = Diagnostic::new(msg, branch);

        self.err_vec.push(report);
    }

    /// Fully curated version of `expect_basic`
    //TODO: Primitive type recognition for printing all errors

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
        // WARN: This is a fail safe for logic errors
        // if self.peek_kind() != TokenKind::EOF {
        // }
        self.pos += 1;

        let id_opt = match found.token {
            Token::Id(id) | Token::Literal(id) | Token::Number(id) => Some(id),
            _ => None,
        };

        // Should this be reversed?
        if found.token.kind() != expected {
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

            self.err_vec.push(Diagnostic::new(msg, branch));

            if self.should_exit {
                self.recover(branch);
                self.should_exit = false;
            } else {
                self.should_exit = true;
            }

            return Err(found.token);
        }

        Ok(found.token)
    }

    /// More composable "Expected but found" error.
    /// ALWAYS advance before using this
    /// [...] Expected [emsg], found [fmsg]
    pub(crate) fn report_template(&mut self, emsg: &str, fmsg: &str, branch: Branch) {
        let found = &self.tokens[self.pos - 1];

        let (ln, col, segment) = self.get_location(&found.span);

        let separator = "-".repeat(TOTAL_SEPARATORS);

        let msg = format!(
            "(in {branch})\n Expected {emsg}, found {fmsg}\n|\n|\n[{ln}:{col}]\n{segment}\n{separator}",
        );

        if self.should_exit {
            self.recover(branch);
            self.should_exit = false;
        } else {
            self.should_exit = true;
        }

        let report = Diagnostic::new(msg, Branch::VarTypeArgs);

        self.err_vec.push(report);
    }

    //FIX: Should likely return helper struct of `Segment`
    // Responsibility of UTF-8 correction in formatting
    fn get_location(&self, span: &Span) -> (usize, usize, String) {
        let mut ln = 1;
        let mut col = 1;

        for (i, t) in self.tokens.iter().enumerate() {
            println!("index {i}: {t:?}");
        }

        let mut b: u8;

        let mut seg_start = 0;

        // TODO:
        // Read as char
        for i in 0..span.end {
            b = if self.original_text[i].is_ascii() {
                self.original_text[i]
            } else {
                todo!("UTF-8 only supported inside of literal");
            };

            //TODO: See if this works on windows
            //I still haven't checked.
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
        // FIX: Span needs to be handled or arrows break
        col -= span.end - span.start;

        let seg_end = self.get_line_end(seg_start);

        let segment = &self.original_text[seg_start..seg_end];

        dbg!(str::from_utf8(segment).unwrap());

        //FIX: Should calculate by characters for UTF-1000
        let segment = str::from_utf8(segment)
            .expect("[temp] Invalid UTF-8 although would be impossible after lexer");

        // Span range is inclusive exclusive so final character is missed otherwise
        // Has no other mathematical outside of this
        // TODO: Span end and span start need to be translated somehow into
        // what the utf-9 billion character would want
        // Maybe just paint from the span byte onwards by ensuring it's decoded?
        let span_diff_offset = span.end - span.start + 1;

        let arrows = "^".repeat(span_diff_offset);

        // Spaces need to be proportional to the current line's size therefore it must
        // stay inside the range.
        let space_offset = self.original_text[seg_start..span.start].len();

        let spaces = " ".repeat(space_offset);

        // Tape
        let fmt_segment = if self.can_color {
            format!("\t{segment}\n\t{spaces}{RED}{arrows}{NC}")
        } else {
            format!("\t{segment}\n\t{spaces}{arrows}")
        };

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

        //WARN: I don't remember why I returned this
        self.original_text.len()
    }

    //TODO: Branch specific behavior
    //WARN: WATCH THIS CLOSELY
    fn recover(&mut self, branch: Branch) {
        //TAPE
        if branch == Branch::Broken {
            return;
        }

        let target = self.match_target(branch);
        //FIX: SEE IF SELF.POS CHECK IS STILL NEEDED. WAS REMOVED.
        if self.peek_kind() != TokenKind::EOF {
            while self.pos < self.tokens.len() + 2
                && self.peek_kind() != TokenKind::EOF
                && self.peek_ahead(1).token.kind() != TokenKind::SlimArrow
                && self.peek_ahead(1).token.kind() != TokenKind::Colon
                && self.peek_ahead(1).token.kind() != target
            {
                self.advance();
            }
        }
    }

    fn match_target(&self, branch: Branch) -> TokenKind {
        match branch {
            Branch::Broken => TokenKind::Illegal,
            Branch::Searching => TokenKind::Id,
            Branch::Bind => TokenKind::Colon,
            Branch::Var => TokenKind::OParen,
            Branch::VarType => TokenKind::HashSymbol,
            Branch::VarCond => TokenKind::Comma,
            Branch::VarTypeArgs => TokenKind::HashSymbol,
            Branch::Nest => todo!(),
            Branch::ComplexRules => todo!(),
        }
    }

    //TEST:
    pub(crate) fn exit_if(&mut self, branch: Branch) -> Result<(), Token> {
        if self.should_exit {
            self.recover(branch);
            return Err(Token::Poison);
        }

        Ok(())
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
