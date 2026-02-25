use std::io::{BufRead, BufReader, ErrorKind, Read};

// I didn't want to do this but it has state
const DEFINITION_SIZE: usize = 4;

// ~FileReader() : { delete ALL }
pub struct FileLoader<R: Read> {
    handle: BufReader<R>,
    pos: usize,
}

impl<R: Read> FileLoader<R> {
    pub fn new(handle: R) -> FileLoader<R> {
        FileLoader {
            handle: BufReader::new(handle),
            pos: 0,
        }
    }

    //TODO: Swap to err
    pub fn load_config(&mut self) -> Option<(Vec<u8>, usize)> {
        // Doesn't NEED definition but will error if declared and not closed
        // TODO: Add read limit.
        let mut requires_def = false;

        self.handle.fill_buf().ok().expect("Failed to fill buffer");

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

                    if self.peek()? == b'/' {
                        self.advance();
                        self.handle_comment();
                    } else if self.peek_ahead(1)? == b'*' {
                        self.advance();
                        self.handle_multi_comment();
                    }
                }
                b'@' => {
                    if !requires_def
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@def"
                    {
                        requires_def = true;
                    }

                    if requires_def
                        && &self.handle.buffer()[self.pos..self.pos + DEFINITION_SIZE] == b"@end"
                    {
                        let start_offset = self.pos + DEFINITION_SIZE;
                        return Some((
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

        if !requires_def {
            dbg!(str::from_utf8(self.handle.buffer()).unwrap(), self.pos);
            Some((self.handle.buffer()[..self.pos].to_vec(), 0))
        } else {
            None
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
        // To get rid of leftover */
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
