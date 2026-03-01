pub mod color;
pub mod lexer;
pub mod linter;
pub mod parser;
pub mod token;

#[cfg(test)]
mod tests {

    use common::{intern::Intern, primitives::PrimitiveKeywords, storage::FileLoader};

    use crate::{lexer::Lexer, parser};

    #[test]
    fn lex_tok_test() {
        // let text = r#"bind-> "./some/path""#;
        let text = r#"bind-> "./some/path""#;
        dbg!(&text);

        let (cfg, lex_start, start_offset) =
            FileLoader::new(text.as_bytes()).load_config().unwrap();

        let mut interner = Intern::init();

        let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);

        assert_eq!(0, start_offset, "start_offset without `@def` failed");
        assert_eq!(4, toks.len(), "Token length exceeded 4 in lex_tok_test");
        // assert_eq!(vec![Token], toks.len(), "");
    }

    #[test]
    fn lex_tok_test_rev() {
        // let text = r#"bind-> "./some/path""#;
        let text = r#"@defbind-> "./some/path""#;
        dbg!(&text);

        let opt = FileLoader::new(text.as_bytes()).load_config();

        assert_eq!(opt.is_err(), true);
    }

    //utf8 broke

    // #[ignore = "Hi"]
    #[test]
    fn primitives_test() {
        // let text = "
        // i8 u8 i16 u16 f16 i32 u32 f32 i64
        // u64 f64 i128 u128 f128 sized unsized
        // char str bool nil BigInt BigFloat List
        // Map Set bind var nest complex_rules";

        let interner = Intern::init();

        assert_eq!("i8", interner.search(PrimitiveKeywords::I8 as usize));
        assert_eq!("u8", interner.search(1));
        assert_eq!("i16", interner.search(2));
        assert_eq!("u16", interner.search(3));
        assert_eq!("f16", interner.search(4));
        assert_eq!("i32", interner.search(5));
        assert_eq!("u32", interner.search(6));
        assert_eq!("f32", interner.search(7));
        assert_eq!("i64", interner.search(8));
        assert_eq!("u64", interner.search(9));
        assert_eq!("f64", interner.search(10));
        assert_eq!("i128", interner.search(11));
        assert_eq!("u128", interner.search(12));
        assert_eq!("f128", interner.search(13));
        assert_eq!("sized", interner.search(14));
        assert_eq!("unsized", interner.search(15));
        assert_eq!("char", interner.search(16));
        assert_eq!("str", interner.search(17));
        assert_eq!("bool", interner.search(18));
        assert_eq!("nil", interner.search(19));
        assert_eq!("BigInt", interner.search(20));
        assert_eq!("BigFloat", interner.search(21));
        assert_eq!("List", interner.search(22));
        assert_eq!("Map", interner.search(23));
        assert_eq!("Set", interner.search(24));
        assert_eq!("bind", interner.search(25));
        assert_eq!("var", interner.search(26));
        assert_eq!("nest", interner.search(27));
        assert_eq!("complex_rules", interner.search(28));
        assert_eq!("Range", interner.search(29));
        assert_eq!("IsEmpty", interner.search(30));
    }

    // #[test]
    // fn parse_test() {
    //     let text = r#"
    //         bind->"./some/寒しい/path"
    //         var-> name: str[IsEmpty(), Len(~5)] #warn
    //         "#;
    //
    //     let mut interner = Intern::init();
    //
    //     let bytes = text.as_bytes();
    //
    //     let toks = Lexer::new(bytes).tokenize(&mut interner);
    //
    //     let sym_table = parser::parse(bytes, &toks, &mut interner);
    //
    //     for symbol in sym_table.symbols().values() {
    //         match symbol {
    //             Symbol::Bind(bind) => {
    //                 assert_eq!(interner.search(bind.name_id as usize), "./some/寒しい/path");
    //             }
    //             Symbol::Definition(type_def) => {
    //                 assert_eq!(interner.search(type_def.name_id as usize), "name");
    //                 assert_eq!(
    //                     true,
    //                     matches!(
    //                         *sym_table.search_type(type_def.type_id as usize),
    //                         ActualType::Str
    //                     )
    //                 );
    //
    //                 assert_eq!(2, type_def.cond.len());
    //
    //                 let is_empty_func =
    //                     FunctionDef::new(PrimitiveKeywords::IsEmpty as u32, Vec::new());
    //
    //                 let len_func = FunctionDef::new(
    //                     PrimitiveKeywords::Len as u32,
    //                     vec![FuncArgs::Num(0), FuncArgs::Num(5)],
    //                 );
    //
    //                 //Not doing what it looks like but green text == good
    //                 assert_eq!(true, matches!(&type_def.cond[0], is_empty_func));
    //                 assert_eq!(true, matches!(&type_def.cond[1], len_func));
    //
    //                 assert_eq!(type_def.args[0], InnerArgs::Warn);
    //                 assert_eq!(1, type_def.args.len());
    //             }
    //             Symbol::Func(function_def) => todo!(),
    //         }
    //     }
    // }

    #[test]
    fn start_offset_test() {
        // 24
        let text = format!("adwh@def var-> int: i32 @endhi");

        let (_, lex_start, start_offset) = FileLoader::new(text.as_bytes()).load_config().unwrap();

        assert_eq!(&text[4..], &text[lex_start..]);
        assert_eq!("hi", &text[start_offset..]);
        assert_eq!(start_offset, 28, "windows.h");
    }

    // #[test]
    // fn template_test() {
    //     let text = r#"
    //             person: S|Person,
    //             nest->
    //                 .person {
    //                     name: str
    //                     age: u8
    //                     things: List<i32>
    //                 }
    //             "#;
    //
    //     let (cfg, lex_start, _) = FileLoader::new(text.as_bytes()).load_config().unwrap();
    //
    //     let mut interner = Intern::init();
    //
    //     let toks = Lexer::new(&cfg, lex_start).tokenize(&mut interner);
    //
    //     let sym_table = parser::parse(&cfg, &toks, &mut interner);
    //
    //     dbg!(sym_table);
    //     // assert_eq!("hi", &text[start_offset..]);
    //     // assert_eq!(start_offset, 24, "windows.h");
    // }
}
