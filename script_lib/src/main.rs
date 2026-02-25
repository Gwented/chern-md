use std::fs;

use common::intern::{self, Intern};
use script_lib::{
    lexer::Lexer,
    parser::{self},
    storage::{self, FileLoader},
};

fn main() {
    let path = "./chrn_tests/main.chrn";

    let file = std::fs::File::open(path).unwrap();

    let (data, _) = match FileLoader::new(file).load_config() {
        Some((data, offset)) => (data, offset),
        None => panic!("Failed to use FileLoader."),
    };

    let mut interner = Intern::init();

    let toks = Lexer::new(&data).tokenize(&mut interner);

    let table = parser::parse(&data, &toks, &mut interner);

    dbg!(table);
}
