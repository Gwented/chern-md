pub mod lexer;
pub mod parser;
mod token;

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn testing() {
        let path = "../chrn_tests/main.chrn";
        let text = fs::read_to_string(path).unwrap();
        let toks = Lexer::new(text.as_bytes()).tokenize();
        dbg!(toks);
        panic!("I'm panicking");
    }
}
