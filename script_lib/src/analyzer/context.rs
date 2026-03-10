//TODO: GET NAME ID SPANS FOR REGISTER PHASE
use std::io::IsTerminal;

use common::{reporter, symbols::Span};

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

    //WARN: Odd
    pub(super) fn report_basic(&mut self, msg: &str) {
        let diag = Diagnostic::new(msg.to_owned());
        self.err_vec.push(diag);
    }

    // Should spans stay addresses or would that conflict with !!!<><
    pub(super) fn report_spanned(&mut self, msg: &str, span: &Span) {
        let (ln, col, segment) = reporter::form_err_diag(self.src_text, span, self.can_color);

        let separators = "-".repeat(TOTAL_SEPARATORS);

        //TODO:
        let help = "";

        let msg = format!("{msg}\n\n[{ln}:{col}]\n{segment}{help}\n{separators}\n");

        let diag = Diagnostic::new(msg.to_owned());

        self.err_vec.push(diag);
    }

    pub(super) fn emit_errors(&self) {
        let initial_err = if self.can_color {
            format!("{}Error:{}", reporter::RED, reporter::NC)
        } else {
            format!("Error:")
        };

        println!("{initial_err}");

        for err in &self.err_vec {
            println!("{}", err.msg);
        }

        eprintln!("Reported {} error(s)\n", self.err_vec.len());
    }
}
