use std::io::IsTerminal;

use crate::{parser::error::Diagnostic, symbols::SpannedToken};

/// Amount of '-' to print for multiple error separation
const TOTAL_SEPARATORS: usize = 60;

#[derive(Debug)]
pub(super) struct Context<'a> {
    src_text: &'a [u8],
    pub(super) tokens: &'a [SpannedToken],
    pub(super) pos: usize,
    pub(super) err_vec: Vec<Diagnostic>,
    can_color: bool,
}

// Fuzzy find?
// I'm NOT having context switch branches manually. Please.
impl<'a> Context<'a> {
    pub(super) fn new(original_text: &'a [u8], tokens: &'a [SpannedToken]) -> Context<'a> {
        Context {
            src_text: original_text,
            tokens,
            pos: 0,
            err_vec: Vec::new(),
            can_color: std::io::stdout().is_terminal(),
        }
    }
}
