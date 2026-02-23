use common::intern::Intern;

use crate::token::{Span, SpannedToken, Token};

/// Known size in bytes for `@def` and `@end`
const DEFINITION_SIZE: usize = 4;

// Possible issues?
pub struct Lexer<'a> {
    bytes: &'a [u8],
    pos: usize,
}

//FIX: Should not read to string due to it being able to be in the file itself
impl Lexer<'_> {
    pub fn new(bytes: &[u8]) -> Lexer<'_> {
        Lexer { bytes, pos: 0 }
    }

    pub fn tokenize(&mut self, interner: &mut Intern) -> (usize, Vec<SpannedToken>) {
        let mut tokens: Vec<SpannedToken> = Vec::new();

        // For threshold of illegal tokens before just giving up. Likely 8 cap.
        let mut illegal_toks: u8 = 0;
        // In case of in md file definition
        let mut start_offset: usize = 0;

        let mut in_def = false;

        loop {
            self.skip_whitespace();

            //FIXME: EOF is always at a new line
            if self.peek() == b'\0' || illegal_toks > 5 {
                tokens.push(SpannedToken {
                    token: Token::EOF,
                    span: Span::new(self.pos, self.pos),
                });

                break;
            }

            //WARN: Ascii only
            let byte = self.peek();

            dbg!(byte as char);

            match byte {
                b if b.is_ascii_alphabetic() || b == b'_' => {
                    // eprintln!(
                    //     "Peeking byte {} char {} start",
                    //     self.peek(),
                    //     self.peek() as char
                    // );

                    tokens.push(self.read_id(interner));
                }
                b if b.is_ascii_digit() => {
                    tokens.push(self.read_num(interner));
                }
                b':' => {
                    tokens.push(SpannedToken {
                        token: Token::Colon,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'(' => {
                    tokens.push(SpannedToken {
                        token: Token::OParen,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b')' => {
                    tokens.push(SpannedToken {
                        token: Token::CParen,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'<' => {
                    tokens.push(SpannedToken {
                        token: Token::OAngleBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'>' => {
                    tokens.push(SpannedToken {
                        token: Token::CAngleBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'[' => {
                    tokens.push(SpannedToken {
                        token: Token::OBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b']' => {
                    tokens.push(SpannedToken {
                        token: Token::CBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'{' => {
                    tokens.push(SpannedToken {
                        token: Token::OCurlyBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'}' => {
                    tokens.push(SpannedToken {
                        token: Token::CCurlyBracket,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b',' => {
                    tokens.push(SpannedToken {
                        token: Token::Comma,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'?' => {
                    tokens.push(SpannedToken {
                        token: Token::QuestionMark,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'@' => {
                    // Allows for same behavior even in file with serialized data
                    if self.is_def_start() {
                        in_def = true;
                        self.pos += DEFINITION_SIZE;
                    } else if self.is_def_end() {
                        in_def = false;

                        tokens.push(SpannedToken {
                            token: Token::EOF,
                            // Needs - 1 because span is inclusive exclusive
                            // due to how bytes are seen
                            span: Span::new(self.pos, self.pos + DEFINITION_SIZE - 1),
                        });

                        start_offset = self.pos + DEFINITION_SIZE;
                        break;
                    } else {
                        todo!("Implement unwind");
                        let tok = self.recover();

                        let id = interner.intern(&tok);

                        tokens.push(SpannedToken {
                            token: Token::Illegal(id),
                            span: Span::new(self.pos, self.pos),
                        });

                        self.advance();
                    }
                }
                b'.' => {
                    let (ln, col) = (self.pos, self.pos);
                    self.advance();

                    if self.peek() == b'.' && self.peek_ahead(1) == b'=' {
                        self.skip(1);

                        tokens.push(SpannedToken {
                            token: Token::DotRange,
                            span: Span::new(ln, col),
                        });
                    } else {
                        tokens.push(SpannedToken {
                            token: Token::Dot,
                            span: Span::new(ln, col),
                        });
                    }

                    self.advance();
                }
                b'#' => {
                    tokens.push(SpannedToken {
                        token: Token::HashSymbol,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'"' => {
                    //FIX:
                    self.advance();
                    tokens.push(self.read_quotes(interner));
                }
                b'-' => {
                    let (start, mut end) = (self.pos, self.pos);

                    let token = if self.peek_ahead(1) == b'>' {
                        self.advance();
                        end = self.pos;
                        Token::SlimArrow
                    } else {
                        Token::Hyphen
                    };

                    tokens.push(SpannedToken {
                        token,
                        span: Span::new(start, end),
                    });

                    self.advance();
                }
                b'=' => {
                    tokens.push(SpannedToken {
                        token: Token::Equals,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'~' => {
                    tokens.push(SpannedToken {
                        token: Token::Tilde,
                        span: Span::new(self.pos, self.pos),
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
                            span: Span::new(self.pos, self.pos),
                        });

                        self.advance();
                    }
                }
                b'*' => {
                    tokens.push(SpannedToken {
                        token: Token::Asterisk,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'!' => {
                    tokens.push(SpannedToken {
                        token: Token::ExclamationPoint,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                b'%' => {
                    tokens.push(SpannedToken {
                        token: Token::Percent,
                        span: Span::new(self.pos, self.pos),
                    });

                    self.advance();
                }
                t => {
                    illegal_toks += 1;

                    // TODO: Figure out if this should exist to avoid Java level errors
                    // if illegal_toks > 8 {
                    //
                    // }
                    todo!("Implement unwind.");
                    let tok = self.recover();

                    //FIXME: STAND IN VALUE
                    // let id = interner.intern(&t);

                    tokens.push(SpannedToken {
                        token: Token::Illegal(0x00),
                        span: Span::new(self.pos, self.pos),
                    });
                }
            }
        }

        //FIXME:
        if in_def {
            eprintln!("Missing `@end`");
        }

        (start_offset, tokens)
    }

    fn read_id(&mut self, interner: &mut Intern) -> SpannedToken {
        let mut id = Vec::with_capacity(8);

        let start = self.pos;
        dbg!(self.bytes[start] as char);

        //FIXME: Utf-8 compliance.
        while self.pos < self.bytes.len() && self.peek().is_ascii_alphanumeric()
            || self.peek() == b'_'
        {
            let byte = self.advance();
            id.push(byte);
        }

        // 9 trillion years debugging and it was an off by one error.
        let end = self.pos - 1;

        let id = str::from_utf8(id.as_slice());

        match id {
            Ok(id) => {
                let id = interner.intern(id);

                SpannedToken {
                    token: Token::Id(id),
                    span: Span::new(start, end),
                }
            }
            Err(_) => {
                let response = "Invalid UTF-8. Could not parse id.";
                let resp_id = interner.intern(response);

                SpannedToken {
                    token: Token::Illegal(resp_id),
                    span: Span::new(start, end),
                }
            }
        }
    }

    fn read_num(&mut self, interner: &mut Intern) -> SpannedToken {
        let mut id = String::new();
        let start = self.pos;

        // TODO: Match specific handling for underscores for cleanliness.
        // Clean code, clean architecture, SOLID principles
        while self.pos < self.bytes.len() && self.peek().is_ascii_digit() || self.peek() == b'_' {
            let byte = self.advance();

            if byte == b'_' {
                continue;
            }

            id.push(byte as char);
        }

        let end = self.pos - 1;
        //TODO: Possible "Base" enum with Number type arg
        let id = interner.intern(&id);

        SpannedToken {
            token: Token::Number(id),
            span: Span::new(start, end),
        }
    }

    //FIX: Currently fixing quote offset with - 1 but should likely just return start, end, tok
    fn read_quotes(&mut self, interner: &mut Intern) -> SpannedToken {
        let mut path: Vec<u8> = Vec::with_capacity(10);
        //WARN:
        let start = self.pos - 1;

        //FIXME: Could be more escapes
        let escape_sequences = [b'n', b'r', b'\"', b'0', b'\\', b'x'];

        while self.pos < self.bytes.len() {
            match self.peek() {
                b'\\' => {
                    let b = self.advance();

                    //FIXME: BROKEN WINDER. TEMPORARY VALUE
                    if escape_sequences.contains(&self.peek()) {
                        todo!("Implement wooging (name clashing)");
                        let tok = self.recover();
                        // return Token::Illegal(0x00);
                    }

                    dbg!(b);
                    path.push(b'\\');
                }
                b'\"' => {
                    self.advance();
                    break;
                }
                _ => path.push(self.advance()),
            }
        }

        //WARN:
        let end = self.pos - 1;

        //TODO: Cleaner handle of failure to close string
        if self.pos == self.bytes.len() {
            let resp_id = interner.intern("Failed to close string literal and found <eof>");
            return SpannedToken {
                token: Token::Illegal(resp_id),
                span: Span::new(start, end),
            };
        }

        dbg!(&path);
        let path_res = str::from_utf8(path.as_slice());

        match path_res {
            Ok(p) => {
                let p = interner.intern(p);
                SpannedToken {
                    token: Token::Literal(p),
                    span: Span::new(start, end),
                }
            }
            Err(_) => {
                let response = "Invalid UTF-8. Could not parse literal.";
                let id = interner.intern(response);
                SpannedToken {
                    token: Token::Illegal(id),
                    span: Span::new(start, end),
                }
            }
        }
        // Unsure if this needs to exist since, I have no reason to process these.
    }

    // Null byte denotes EOF as of now.
    fn peek(&self) -> u8 {
        self.bytes.get(self.pos).copied().unwrap_or(b'\0')
    }

    //FIX: IS THIS REQUIRED NOW?
    fn is_def_start(&self) -> bool {
        if self.pos + 3 > self.bytes.len() {
            return false;
        }

        let possible_start = &self.bytes[self.pos..=self.pos + 3];

        if possible_start == "@def".as_bytes() {
            return true;
        }

        false
    }

    //FIX: READ PREVIOUS
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

    //FIXME: Intended to prevent garbage characters from being read after an illegal token.
    fn recover(&mut self) -> String {
        let mut id = String::new();

        while let Some(ch) = self.peek_char() {
            //FIXME: Unwrap call for Utf-8 compliance
            panic!("I literally peeked");

            id.push(ch);
            panic!("Stalls here {ch}");
            self.advance();
            dbg!(&id);
        }

        dbg!(&id);
        panic!("Passed to ID");

        id
    }

    //FIXME: ENTIRELY BROKEN.
    fn peek_char(&mut self) -> Option<char> {
        let b = self.peek();

        if b <= 127 {
            return Some(b as char);
        }

        let end = std::cmp::min(self.pos + 2, self.bytes.len());
        dbg!(end);
        let chunk = &self.bytes[self.pos..end];
        // dbg!(str::from_utf8(chunk).ok().and_then(|c| c.chars().next()));
        // // Lazy evaluation to avoid utf-8 checking entire self.bytes
        std::str::from_utf8(chunk)
            .ok()
            .and_then(|c| c.chars().next())
    }

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
        // To get rid of leftover */
        self.skip(2);
    }

    // May return byte
    // BUG: This could be an issue. Many other places alike are present.
    fn skip(&mut self, dest: usize) {
        self.pos += dest;
    }

    fn peek_ahead(&mut self, dest: usize) -> u8 {
        self.bytes.get(self.pos + dest).copied().unwrap_or(b'\0')
    }

    fn advance(&mut self) -> u8 {
        let b = self.peek();
        self.pos += 1;
        b
    }

    //TODO: Utf-8 compliance
    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.advance();
        }

        //FIXME: BROKEN WINDER.
        while let Some(ch) = self.peek_char()
            && ch.is_whitespace()
        {
            self.advance();
        }
    }
}
