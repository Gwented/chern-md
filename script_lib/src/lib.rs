pub mod color;
pub mod lexer;
pub mod parser;
pub mod token;

#[cfg(test)]
mod tests {

    use std::fs;

    use common::intern::Intern;

    use crate::{lexer::Lexer, parser::context::Context};

    use super::*;

    #[test]
    fn lex_tok_test() {
        let text = r#"bind->"./some/path""#;

        let mut interner = Intern::new();
        let (start_offset, toks) = Lexer::new(text.as_bytes()).tokenize(&mut interner);

        assert_eq!(
            0, start_offset,
            "start_offset without `@def` failed in lex_tok_test"
        );
        assert_eq!(4, toks.len(), "Token length exceeded 4 in lex_tok_test");
        // assert_eq!(vec![Token], toks.len(), "");
    }

    // #[test]
    // fn parse_test() {
    //     let text = "@defvar->map:Map<i32 u32>@end{test_word}";
    //
    //     let mut interner = Intern::new();
    //
    //     let bytes = text.as_bytes();
    //
    //     let (start_offset, _) = Lexer::new(bytes).tokenize(&mut interner);
    //
    //     // assert_eq!("hi", &text[start_offset..]);
    //     // assert_eq!(start_offset, text.len() - test_word.len());
    // }

    #[test]
    fn start_offset_test() {
        let test_word = "hi";

        let text = format!("@defvar->map:Map<i32 u32>@end{test_word}");

        let mut interner = Intern::new();

        let bytes = text.as_bytes();

        let (start_offset, _) = Lexer::new(bytes).tokenize(&mut interner);

        assert_eq!("hi", &text[start_offset..], "_GNU_SOURCE");
        assert_eq!(start_offset, text.len() - test_word.len(), "windows.h");
    }
}
