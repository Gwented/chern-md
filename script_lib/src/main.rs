use std::time::Instant;

use common::{intern::Intern, storage::FileLoader};
use script_lib::{
    analyzer::Analyzer,
    lexer::Lexer,
    linter,
    parser::{self},
};

//FIX: More general file information that is persistent throughout the program which would
//include the file name, path, etc.

fn main() {
    let start = Instant::now();

    let path = "./chrn_tests/main.chrn";

    let file = std::fs::File::open(path).unwrap();

    let (data, lex_start, _) = match FileLoader::new(file).load_config() {
        Ok((data, lex_start, serial_start)) => (data, lex_start, serial_start),
        Err(e) => {
            eprintln!("Error: {e}\nAborting...");
            std::process::exit(1);
        }
    };

    let mut interner = Intern::init();

    let toks = Lexer::new(&data, lex_start).tokenize(&mut interner);

    let program = parser::parse(&data, &toks, &mut interner);

    linter::print_all(&program, &interner);

    let stuff = Analyzer::new(&program, &interner, &data).analyze();

    println!("{} ms", start.elapsed().as_millis());
}
