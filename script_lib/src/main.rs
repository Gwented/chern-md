use common::intern::Intern;
use script_lib::{
    lexer::Lexer,
    parser::{self},
    storage::FileLoader,
};

fn main() {
    let path = "./chrn_tests/main.chrn";

    let file = std::fs::File::open(path).unwrap();

    let (data, _) = match FileLoader::new(file).load_config() {
        Some((data, offset)) => (data, offset),
        None => panic!("Missing entry point (probably)."),
    };

    // dbg!(&data);

    // let data = "
    // @def
    //     bind-> \"stops here\"
    //     var->
    //         Glorp: i32 (Len(~5  ))
    //     @end"
    //     .as_bytes();
    // dbg!(&str::from_utf8(&data).unwrap());
    // dbg!(&data.len());

    let mut interner = Intern::init();

    let toks = Lexer::new(&data).tokenize(&mut interner);

    let table = parser::parse(&data, &toks, &mut interner);

    dbg!(table);
}
