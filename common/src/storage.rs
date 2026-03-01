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

    //WARN: The vector doesn't HAVE to be copied since the loader owns it, but not doing that yet.
    pub fn load_config(&mut self) -> Result<(Vec<u8>, usize, usize), String> {
        // Doesn't NEED definition but will error if declared and not closed
        // TODO: Add read limit.
        let mut requires_end = false;

        // <think>
        // I don't know
        // </think>
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

            match b {
                b'"' => {
                    self.read_quotes();
                }
                b'/' => {
                    self.advance();

                    if self.peek() == Some(b'/') {
                        self.advance();
                        self.handle_comment();
                    } else if self.peek_ahead(1) == Some(b'*') {
                        self.advance();
                        self.handle_multi_comment();
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
                            "Found illegal '@' usage while loading file after seeing `@def`. (line {})",
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
                            "Found illegal '@' usage while loading file (line {})",
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

    fn handle_multi_comment(&mut self) {
        while let Some(a) = self.peek() {
            if let Some(b) = self.peek_ahead(1)
                && b != b'*'
                && a != b'/'
            {
                self.advance();
            }
        }
        self.skip(2);
    }

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
