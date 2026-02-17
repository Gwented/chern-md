pub mod lexer;
pub mod parser;
pub mod storage;
mod token;

#[cfg(test)]
mod tests {

    use std::fs;

    use common::intern::Intern;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn lex_test() {
        let path = "../chrn_tests/main.chrn";
        let text = fs::read_to_string(path).unwrap();

        let mut interner = Intern::new();
        let toks = Lexer::new(text.as_bytes()).tokenize(&mut interner);

        dbg!(toks);
        panic!("I'm panicking");
    }

    // #[test]
    // fn parse_test() {
    //     let path = "../chrn_tests/main.chrn";
    //     let text = fs::read_to_string(path).unwrap();
    //
    //     let (start_offset, toks) = Lexer::new(text.as_bytes()).tokenize();
    //
    //     let table = parser::parse(&toks);
    //
    //     dbg!(table);
    //     panic!("I'm panicking");
    // }
}
