//TODO: GET NAME ID SPANS FOR REGISTER PHASE
use std::io::IsTerminal;

use common::{builtins, reporter, symbols::Span};

use crate::analyzer::error::Diagnostic;

/// Amount of '-' to print for multiple error separation
const TOTAL_SEPARATORS: usize = 60;

//TODO: Find out if this is needed
#[derive(Debug)]
pub(super) struct SemanticReporter<'a> {
    pub(super) src_text: &'a [u8],
    pub(super) err_vec: Vec<Diagnostic>,
    pub(super) can_color: bool,
}

impl<'a> SemanticReporter<'a> {
    pub(super) fn new(src_text: &'a [u8]) -> SemanticReporter<'a> {
        SemanticReporter {
            src_text,
            err_vec: Vec::new(),
            can_color: std::io::stdout().is_terminal(),
        }
    }

    pub(super) fn report_basic(&mut self, msg: &str) {
        let diag = Diagnostic::new(msg.to_owned());
        self.err_vec.push(diag);
    }

    /// Draws red arrows under the span given. err_name represents whether or not a keyword that
    /// could be similar in name should be looked for.
    pub(super) fn report_spanned(&mut self, msg: &str, err_name: Option<&str>, span: &Span) {
        let (ln, col, segment) = reporter::form_err_diag(self.src_text, span, self.can_color);

        let separators = "-".repeat(TOTAL_SEPARATORS);

        let help = if let Some(name) = err_name {
            self.try_help(name).unwrap_or_default()
        } else {
            "".to_string()
        };

        let msg = format!("{msg}\n\n[{ln}:{col}]\n{segment}\n\n{help}{separators}\n");

        let diag = Diagnostic::new(msg.to_owned());

        self.err_vec.push(diag);
    }

    // Kiwi
    fn try_help(&self, err_name: &str) -> Option<String> {
        let kw_index = builtins::fuzzy_find_kw(err_name.as_bytes())?;
        let found_kw = builtins::KEYWORDS_ARRAY[kw_index];

        let msg = format!("Found similar keyword \"{}\"", found_kw);

        let help = reporter::form_help(&msg, self.can_color);

        Some(help)
    }

    pub(super) fn emit_errors(&self) {
        //FIX: Get file path
        let initial_err = if self.can_color {
            format!("{}Error{}", reporter::RED, reporter::NC)
        } else {
            format!("Error")
        };

        println!("From path => {{}}");

        for err in &self.err_vec {
            println!("{initial_err}: {}", err.msg);
        }

        eprintln!("\nReported {} error(s)\n", self.err_vec.len());
    }
}
