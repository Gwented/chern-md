use common::{intern::Intern, storage::FileLoader};
use script_lib::{
    lexer::Lexer,
    parser::{self},
};

fn main() {
    let path = "./chrn_tests/main.chrn";

    let file = std::fs::File::open(path).unwrap();

    let (data, _) = match FileLoader::new(file).load_config() {
        Ok((data, offset)) => (data, offset),
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let mut interner = Intern::init();

    let toks = Lexer::new(&data).tokenize(&mut interner);

    let table = parser::parse(&data, &toks, &mut interner);

    dbg!(table);
}
