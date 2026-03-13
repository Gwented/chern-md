use std::{io::IsTerminal, path::PathBuf};

// TEST:
// FileLoader would produce this. Maybe.
// Global metadata? Should this all be in one place?
#[derive(Debug)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub src_bytes: Vec<u8>,
    pub lex_start: usize,
    pub serial_offset: usize,
    pub can_color: bool,
}

impl FileMetadata {
    pub fn new(
        path: PathBuf,
        src_bytes: Vec<u8>,
        lex_offset: usize,
        serial_offset: usize,
    ) -> FileMetadata {
        FileMetadata {
            path,
            src_bytes,
            lex_start: lex_offset,
            serial_offset,
            //TODO: Could be env var
            can_color: std::io::stdout().is_terminal(),
        }
    }
}
