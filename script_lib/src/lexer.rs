use std::io::Read;

use crate::token::{Span, SpannedToken, Token};

// Possible issues?
pub struct Lexer<'a> {
    bytes: &'a [u8],
    ln: usize,
    col: usize,
    pos: usize,
}

impl Lexer<'_> {
    pub fn new(text: &[u8]) -> Lexer<'_> {
        //TODO: Needs some denoter to say how I should read this so I'm not reading and entire
        //serialized file
        Lexer {
            bytes: text,
            ln: 1,
            col: 1,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> (usize, Vec<SpannedToken>) {
        let mut tokens: Vec<SpannedToken> = Vec::new();

        // For threshold of illegal tokens before just giving up. Likely 8 cap.
        let mut illegal_toks = 0;
        // In case of in md file definition
        let mut starting_point = 0;

        loop {
            self.skip_whitespace();

            //FIXME: EOF is always at a new line
            if self.peek() == b'\0' || self.peek() == b'$' {
                tokens.push(SpannedToken {
                    token: Token::EOF,
                    span: Span::new(self.ln, self.col),
                });

                break;
            }

            let byte = self.peek();

            match byte {
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    let (ln, col) = (self.ln, self.col);
                    let token = self.read_id();

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });
                    eprintln!("Peeking {}", self.peek() as char);
                }
                b if b.is_ascii_digit() => {
                    let (ln, col) = (self.ln, self.col);
                    let token = self.read_num();

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });
                }
                b':' => {
                    tokens.push(SpannedToken {
                        token: Token::Colon,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'(' => {
                    tokens.push(SpannedToken {
                        token: Token::OParen,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b')' => {
                    tokens.push(SpannedToken {
                        token: Token::CParen,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'<' => {
                    tokens.push(SpannedToken {
                        token: Token::OAngleBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'>' => {
                    tokens.push(SpannedToken {
                        token: Token::CAngleBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'[' => {
                    tokens.push(SpannedToken {
                        token: Token::OBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b']' => {
                    tokens.push(SpannedToken {
                        token: Token::CBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'{' => {
                    tokens.push(SpannedToken {
                        token: Token::OCurlyBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'}' => {
                    tokens.push(SpannedToken {
                        token: Token::CCurlyBracket,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b',' => {
                    tokens.push(SpannedToken {
                        token: Token::Comma,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'?' => {
                    tokens.push(SpannedToken {
                        token: Token::QuestionMark,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'@' => {
                    if self.is_def_start() {
                        // Known size of type def in bytes for '@def' and '@end'
                        self.pos += 4;
                    } else if self.is_def_end() {
                        //TODO: Starting point set method needed
                        starting_point = self.pos + 4;

                        tokens.push(SpannedToken {
                            token: Token::EOF,
                            span: Span::new(self.ln, self.col),
                        });

                        break;
                    } else {
                        // //TODO: Unwind then put it into the illegal token but break after.
                        // todo!("Needs unwind after @ failure");
                        let tok = self.unwind();

                        tokens.push(SpannedToken {
                            token: Token::Illegal(tok),
                            span: Span::new(self.ln, self.col),
                        });
                    }

                    self.advance();
                }
                b'.' => {
                    tokens.push(SpannedToken {
                        token: Token::Dot,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'#' => {
                    tokens.push(SpannedToken {
                        token: Token::HashSymbol,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'"' => {
                    let (ln, col) = (self.ln, self.col);
                    self.advance();

                    let token = self.read_quotes();

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });
                }
                b'-' => {
                    let (ln, col) = (self.ln, self.col);

                    let token = if self.peek_ahead(1) == b'>' {
                        self.advance();
                        Token::SlimArrow
                    } else {
                        Token::Hyphen
                    };

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });

                    self.advance();
                }
                b'=' => {
                    tokens.push(SpannedToken {
                        token: Token::Equals,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'~' => {
                    tokens.push(SpannedToken {
                        token: Token::Tilde,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'/' => {
                    if self.peek_ahead(1) == b'/' {
                        self.skip(2);
                        self.handle_comment();
                    } else if self.peek_ahead(1) == b'*' {
                        self.skip(2);
                        self.handle_multi_comment();
                    } else {
                        tokens.push(SpannedToken {
                            token: Token::Slash,
                            span: Span::new(self.ln, self.col),
                        });

                        self.advance();
                    }
                }
                b'*' => {
                    tokens.push(SpannedToken {
                        token: Token::Asterisk,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'!' => {
                    tokens.push(SpannedToken {
                        token: Token::ExclamationPoint,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                b'%' => {
                    tokens.push(SpannedToken {
                        token: Token::Percent,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
                }
                t => {
                    illegal_toks += 1;
                    todo!(
                        "Found byte '{}', which is char '{}' in main lex branch",
                        t,
                        t as char
                    );
                    //FIXME: BETTER HANDLE
                    // TODO: Would call 'unwind()' here and keep count of illegal toks
                    // tokens.push(SpannedToken {
                    //     token: Token::Illegal(t as char),
                    //     span: Span::new(self.ln, self.col),
                    // });

                    self.advance();
                }
            }
        }

        dbg!(starting_point);
        (starting_point, tokens)
    }

    //TODO: Utf-8 compliance. Maybe.
    fn read_id(&mut self) -> Token {
        let mut id = Vec::with_capacity(8);

        while self.pos < self.bytes.len() && self.peek().is_ascii_alphanumeric()
            || self.peek() == b'_'
        {
            let byte = self.advance();
            id.push(byte);
        }

        //TODO: Illegal
        let id = String::from_utf8(id);

        match id {
            Ok(id) => Token::Id(id),
            Err(_) => Token::Illegal("Invalid UTF-8. Could not parse id.".to_string()),
        }
    }

    fn read_num(&mut self) -> Token {
        let mut id = String::new();

        // FIXME: CHANGE ALL PLACES TO LOSSY

        // TODO: Match specific handling for underscores for cleanliness.
        while self.pos < self.bytes.len() && self.peek().is_ascii_digit() || self.peek() == b'_' {
            let byte = self.advance();

            if byte == b'_' {
                continue;
            }

            id.push(byte as char);
        }

        //TODO: Possible "Base" enum with Number type arg
        Token::Number(id)
    }

    //TODO: WE NEED UTF-8
    fn read_quotes(&mut self) -> Token {
        let mut path: Vec<u8> = Vec::with_capacity(10);

        //FIXME: Could be more escapes
        let escape_sequence = [b'n', b'r', b'\"', b'0', b'\\'];

        while self.pos < self.bytes.len() {
            match self.peek() {
                //TODO: May remove escape. Odd handling stays for now.
                b'\\' => {
                    let b = self.advance();

                    if escape_sequence.contains(&self.peek()) {
                        let tok = self.unwind();
                        return Token::Illegal(tok);
                    }

                    path.push(b);
                }
                b'\"' => {
                    self.advance();
                    break;
                }
                _ => path.push(self.advance()),
            }
        }

        //TODO: Cleaner handle of failure to close string
        if self.pos == self.bytes.len() {
            return Token::Illegal("Failed to close string and found <eof>".to_string());
        }

        let path = String::from_utf8(path);
        match path {
            Ok(p) => Token::Literal(p),
            Err(_) => Token::Illegal("Invalid UTF-8. Could not parse id.".to_string()),
        }
        // Unsure if this needs to exist since, I have no reason to process these.
    }

    // Null byte denotes EOF as of now.
    fn peek(&self) -> u8 {
        self.bytes.get(self.pos).copied().unwrap_or(b'\0')
    }

    //FIX: MAY BE BETTER INLINE STILL
    fn is_def_start(&mut self) -> bool {
        if self.pos + 3 > self.bytes.len() {
            return false;
        }

        let possible_start = &self.bytes[self.pos..=self.pos + 3];

        if possible_start == "@def".as_bytes() {
            return true;
        }

        false
    }

    //FIX: MAY BE BETTER INLINE
    fn is_def_end(&mut self) -> bool {
        if self.pos + 3 > self.bytes.len() {
            return false;
        }

        let possible_end = &self.bytes[self.pos..=self.pos + 3];
        dbg!(str::from_utf8(&possible_end).unwrap());

        if possible_end == "@end".as_bytes() {
            return true;
        }

        false
    }

    // Intended to prevent garbage characters from being read after an illegal token.
    // Change the name
    fn unwind(&mut self) -> String {
        let mut id = String::new();

        while let Some(ch) = self.peek_char()
            && !ch.is_whitespace()
        {
            //FIXME: Unwrap call for Utf-8 compliance

            id.push(ch);
            dbg!(&id);

            // panic!("Stalls here {ch}");
        }

        id
    }

    //FIXME: INFINITE LOOP
    fn peek_char(&mut self) -> Option<char> {
        let b = self.peek();

        if b <= 127 {
            return Some(b as char);
        }

        let end = std::cmp::min(self.pos + 4, self.bytes.len());
        let chunk = &self.bytes[self.pos..end];

        // Lazy evaluation to avoid utf-8 checking entire self.bytes
        std::str::from_utf8(chunk)
            .ok()
            .and_then(|c| c.chars().next())
    }

    //FIX: I THINK THIS IS OK I DONT KNOW
    fn handle_comment(&mut self) {
        while self.peek() != b'\n' {
            self.advance();
        }
    }

    //Peek batch method?
    fn handle_multi_comment(&mut self) {
        while self.peek() != b'*' && self.peek_ahead(1) != b'/' {
            self.advance();
        }
        // To get rid of leftover slash
        self.skip(2);
    }

    // May return byte
    fn skip(&mut self, dest: usize) {
        self.pos += dest;
    }

    fn peek_ahead(&mut self, dest: usize) -> u8 {
        self.bytes.get(self.pos + dest).copied().unwrap_or(b'\0')
    }

    fn advance(&mut self) -> u8 {
        let b = self.peek();

        if b == b'\n' {
            self.ln += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        self.pos += 1;
        b
    }

    //TODO: Utf-8 compliance
    fn skip_whitespace(&mut self) {
        //FIXME: Odd handling. May just keep it in b'/'.
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }

        while let Some(ch) = self.peek_char()
            && ch.is_whitespace()
        {
            self.advance();
        }
    }
}
