pub(crate) struct Lexer<'a> {
    text: &'a [u8],
    pos: usize,
}

impl Lexer<'_> {
    pub fn new(text: &[u8], start_offset: usize) -> Lexer<'_> {
        Lexer {
            text,
            pos: 0 + start_offset,
        }
    }
}
