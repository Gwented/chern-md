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
                b if b.is_ascii_digit() => {
                    let (ln, col) = (self.ln, self.col);
                    let token = self.read_num();

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });
                }
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    let (ln, col) = (self.ln, self.col);
                    let token = self.read_id();

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(ln, col),
                    });
                    eprintln!("Peeking {}", self.peek() as char);
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
                        //TODO: Unwind then put it into the illegal token but break after.
                        todo!("Needs unwind after @ failure")
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
                    tokens.push(SpannedToken {
                        token: Token::DoubleQuotes,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
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
                        self.skip_whitespace();
                        break;
                    }

                    tokens.push(SpannedToken {
                        token: Token::Slash,
                        span: Span::new(self.ln, self.col),
                    });

                    self.advance();
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

    //TODO: Utf-8 compliance
    fn read_id(&mut self) -> Token {
        let mut id = Vec::with_capacity(8);

        while self.pos < self.bytes.len() && self.peek().is_ascii_alphanumeric()
            || self.peek() == b'_'
        {
            let byte = self.advance();
            id.push(byte);
        }

        let id = String::from_utf8(id).expect("Invalid UTF-8");

        Token::Id(id)
    }

    fn read_num(&mut self) -> Token {
        let mut id = String::new();

        let mut notation = false;
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
    fn unwind() {}

    fn peek_char(&mut self) -> Option<[u8; 4]> {
        let mut bytes = [0u8; 4];
        let mut i = 0;

        while let Some(b) = self.bytes.get(self.pos).copied()
            && b > 127
        {
            if i + 1 < bytes.len() {
                bytes[i] = b;
                i += 1;
            } else {
                break;
            }
        }

        if bytes.is_empty() {
            return None;
        }

        Some(bytes)
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

    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }
}
