// Should this be pub(crate)?
pub mod analyzer;
pub mod lexer;
pub mod linter;
pub mod parser;
pub mod symbols;
pub mod token;

#[cfg(test)]
mod tests {
    use common::{intern::Intern, storage::FileLoader};

    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn lex_tok_test() {
        let text = r#"bind-> "./some/path""#;
        dbg!(&text);

        let (cfg, lex_start, start_offset) =
            FileLoader::new(text.as_bytes()).load_config().unwrap();

        let mut interner = Intern::init();

        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(0, start_offset, "start_offset without `@def` failed");
        assert_eq!(4, toks.len(), "Token length exceeded 4 in lex_tok_test");
    }

    #[test]
    fn lex_tok_test_rev() {
        // Properly closed @def and @end
        let correct = r#"@defbind-> "./some/path"@end"#;

        let opt = FileLoader::new(correct.as_bytes()).load_config();

        assert_eq!(true, opt.is_ok());

        // Improper @def without an @end
        let wrong = r#"@defbind-> "./some/path""#;

        let opt = FileLoader::new(wrong.as_bytes()).load_config();

        assert_eq!(true, opt.is_err());
    }

    //utf8 broke

    #[test]
    fn char_literal_test() {
        // Valid single character
        let text = "'a'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Char(c) => assert_eq!('a', c),
            _ => panic!("Expected character 'a', found {:?}", toks[0].token),
        }

        // Valid escaped character
        let text = "'\\n'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Char(c) => assert_eq!('\n', c),
            _ => panic!("Expected character '\\n', found {:?}", toks[0].token),
        }

        // Valid hex escape
        let text = "'\\x2F'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Char(c) => assert_eq!('\x2F', c),
            _ => panic!("Expected character '\x2F', found {:?}", toks[0].token),
        }

        // Invalid character
        let text = "'aa'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Illegal(_) => {}
            _ => panic!("Expected Token::Illegal, got {:?}", toks[0].token),
        }

        // Invalid hex escape
        let text = "'\\x2'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Illegal(_) => {}
            _ => panic!("Expected Token::Illegal, got {:?}", toks[0].token),
        }

        // I can't actually read hex
        // Invalid hex digits
        let text = "'\\x255'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Illegal(_) => {}
            _ => panic!("Expected Token::Illegal, got {:?}", toks[0].token),
        }

        // Unknown escape
        let text = "'\\q'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Illegal(_) => (),
            _ => panic!("Expected Illegal token \"\\q\", found {:?}", toks[0].token),
        }

        // Out of range escape
        let text = "'\\x1Y'";
        let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
        let mut interner = Intern::init();
        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(2, toks.len());
        match toks[0].token {
            Token::Illegal(_) => (),
            _ => panic!("Expected Illegal token \"\\1Y\", found {:?}", toks[0].token),
        }
    }

    #[test]
    fn multi_line_comment_test() {
        // Properly closed multi-line comment
        let correct = "
            /* /* */ */
        "
        .as_bytes();

        // Unclosed multi-line comment
        let wrong = "
            /* /* */ 
        "
        .as_bytes();

        let correct = FileLoader::new(correct).load_config();
        let wrong = FileLoader::new(wrong).load_config();

        assert_eq!(true, correct.is_ok());
        assert_eq!(true, wrong.is_err());
    }

    #[test]
    fn start_offset_test() {
        let text = format!("adwh@def var-> int: i32 @endhi");

        let (_, lex_start, serial_offset) = FileLoader::new(text.as_bytes()).load_config().unwrap();

        assert_eq!(&text[4..], &text[lex_start..]);
        assert_eq!("hi", &text[serial_offset..]);
        assert_eq!(28, serial_offset);
    }
}
