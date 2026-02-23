use std::fs;

use common::intern::Intern;
use script_lib::{
    lexer::Lexer,
    parser::{self},
};

fn main() {
    // Missing open/close/small part of declaration spanned
    // row * col * 84 = 2
    let path = "./chrn_tests/main.chrn";
    // FIX: Lexer needs to be redone
    let text = fs::read_to_string(path).unwrap();

    let mut interner = Intern::new();

    let text_bytes = text.as_bytes();

    let (start_offset, toks) = Lexer::new(text_bytes).tokenize(&mut interner);

    let table = parser::parse(text_bytes, &toks, &mut interner);

    dbg!(start_offset, table);
}
