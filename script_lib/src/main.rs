use std::{path::PathBuf, time::Instant};

use common::{intern::Intern, metadata::FileMetadata, storage::FileLoader};
use script_lib::{
    analyzer::Analyzer,
    lexer::Lexer,
    linter,
    parser::{self},
};

//FIXME: More general file information that is persistent throughout the program which would
//include the file name, path, etc.

fn main() {
    let start = Instant::now();

    let path = PathBuf::from("./chrn_tests/main.chrn");

    let file = std::fs::File::open(&path).unwrap();

    let metadata = match FileLoader::new(&path, file).load_config() {
        Ok(meta) => meta,
        Err(e) => {
            eprintln!("Error: {e}\nAborting...");
            std::process::exit(1);
        }
    };

    let mut interner = Intern::init();

    let toks = Lexer::new(&metadata.src_bytes, metadata.lex_start).tokenize(&mut interner);

    let program = parser::parse(&metadata, &toks, &mut interner);

    linter::print_all(&program, &interner);

    let stuff = Analyzer::new(&program, &metadata, &interner).analyze();

    println!("{} ms", start.elapsed().as_millis());
}
