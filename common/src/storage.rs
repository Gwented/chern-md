use std::io::{BufRead, BufReader, Read};

const DEFINITION_SIZE: usize = 4;

// More inclusive name
pub struct FileLoader<R: Read> {
    handle: BufReader<R>,
    pos: usize,
    lines_read: usize,
}

impl<R: Read> FileLoader<R> {
    pub fn new(handle: R) -> FileLoader<R> {
        FileLoader {
            handle: BufReader::new(handle),
            pos: 0,
            lines_read: 1,
        }
    }

    /// Returns an Ok of text scanned for `@def` and `@end`, the lexing starting point if a
    /// definition was found, and where the serializing should start if a def was found. Or returns
    /// a string of error text.
    //FIX: COMMENTS ARE BROKEN
    pub fn load_config(&mut self) -> Result<(Vec<u8>, usize, usize), String> {
        // Doesn't NEED definition but will error if declared and not closed
        // TODO: Add read limit.
        let mut requires_end = false;

        let mut lex_start = 0;

        self.handle.fill_buf().or_else(|e| {
            Err(format!(
                "[Internal] Failed to fill buffer to read definition file: {e}"
            ))
        })?;

        while let Some(b) = self.peek() {
            if b == b'\0' {
                break;
            }
            dbg!(b as char);

            match b {
                b'"' => {
                    self.read_quotes();
                }
                b'/' => {
                    self.advance();

                    if self.peek() == Some(b'/') {
                        self.advance();
                        self.handle_comment();
                    } else if self.peek() == Some(b'*') {
                        self.advance();
                        self.handle_multi_comment()?;
                    }
                }
                b'@' => {
                    if requires_end
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@end"
                    {
                        let serial_start = self.pos + DEFINITION_SIZE;
                        return Ok((
                            self.handle.buffer()[..self.pos + DEFINITION_SIZE].to_vec(),
                            lex_start,
                            serial_start,
                        ));
                    } else if requires_end {
                        //FIX: Weird wording. Cut out error @ used.
                        let msg = format!(
                            "Found illegal '@' usage while loading configuration file after seeing `@def`. (line {})",
                            self.lines_read
                        );

                        return Err(msg);
                    }

                    if !requires_end
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@def"
                    {
                        requires_end = true;
                        lex_start = self.pos;
                    } else if !requires_end {
                        //WARN: Weird wording
                        let msg = format!(
                            "Found illegal '@' usage while configuration file (line {})",
                            self.lines_read
                        );
                        return Err(msg);
                    }

                    self.advance();
                }
                _ => {
                    self.advance();
                }
            }
        }
        // TODO: Assert this...

        if !requires_end {
            Ok((self.handle.buffer()[..self.pos].to_vec(), lex_start, 0))
        } else {
            let msg = format!("Could not find `@end` after `@def` from file {}", file!());
            Err(msg)
        }
    }

    fn read_quotes(&mut self) -> Option<u8> {
        // FIX: I think this is ok I don't know. Option is here because !!@
        while let Some(b) = self.peek() {
            match b {
                b'\\' => {
                    self.advance()?;
                }
                b'\"' => {
                    self.advance()?;
                    break;
                }
                _ => {
                    self.advance()?;
                }
            }
        }

        None
    }

    fn handle_comment(&mut self) {
        while let Some(b) = self.peek()
            && b != b'\n'
        {
            self.advance();
        }
    }

    //WARN: Seems to be working..
    fn handle_multi_comment(&mut self) -> Result<(), String> {
        let mut depth = 1;
        let comment_start = self.lines_read;
        while let Some(current_byte) = self.peek()
            && depth > 0
        {
            dbg!(depth);
            //FIX: Simplify this
            if let Some(next_byte) = self.peek_ahead(1) {
                if current_byte == b'/' && next_byte == b'*' {
                    depth += 1;
                    self.skip(2);
                } else if current_byte == b'*' && next_byte == b'/' {
                    depth -= 1;
                    self.skip(2);
                } else {
                    self.advance();
                }
            } else {
                break;
            }
        }

        if depth > 0 {
            return Err(format!(
                "Error: Found unclosed multi-line comment in configuration file which started at line {}",
                comment_start
            ));
        }

        dbg!(depth);
        Ok(())
    }
    // while let Some(a) = self.peek() {
    //     if let Some(b) = self.peek_ahead(1)
    //         && b != b'*'
    //         && a != b'/'
    //     {
    //         self.advance();
    //     }
    // }
    // self.skip(2);

    fn skip(&mut self, dest: usize) {
        self.pos += dest;
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.peek();

        if b == Some(b'\n') {
            self.lines_read += 1;
        }
        self.pos += 1;

        b
    }

    fn peek_ahead(&mut self, dest: usize) -> Option<u8> {
        self.handle.buffer().get(self.pos + dest).copied()
    }

    fn peek(&mut self) -> Option<u8> {
        self.handle.buffer().get(self.pos).copied()
    }
}
