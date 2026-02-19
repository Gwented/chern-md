pub(crate) struct Lexer<'a> {
    text: &'a [u8],
    pos: usize,
}

impl Lexer<'_> {
    pub fn new(text: &[u8]) -> Lexer<'_> {
        Lexer { text, pos: 0 }
    }
}
