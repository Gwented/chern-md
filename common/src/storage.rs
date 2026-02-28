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
            lines_read: 0,
        }
    }

    //TODO: Should return Result Vec usize string
    pub fn load_config(&mut self) -> Result<(Vec<u8>, usize), String> {
        // Doesn't NEED definition but will error if declared and not closed
        // TODO: Add read limit.
        let mut requires_end = false;

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
                    // Remove @def?
                    if !requires_end
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@def"
                    {
                        requires_end = true;
                    }

                    if requires_end
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@end"
                    {
                        let start_offset = self.pos + DEFINITION_SIZE;
                        return Ok((
                            self.handle.buffer()[..self.pos + DEFINITION_SIZE].to_vec(),
                            start_offset,
                        ));
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
            Ok((self.handle.buffer()[..self.pos].to_vec(), 0))
        } else {
            Err("Could not find definition within ".to_string())
        }
    }

    //BUG:

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
