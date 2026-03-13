use std::{io::IsTerminal, path::PathBuf};

// TEST:
// FileLoader would produce this. Maybe.
// Global metadata? Should this all be in one place?
pub struct FileMetadata {
    pub path: PathBuf,
    pub lex_offset: usize,
    pub serial_offset: usize,
    pub can_color: bool,
}

impl FileMetadata {
    pub fn new(path: PathBuf, lex_offset: usize, serial_offset: usize) -> FileMetadata {
        FileMetadata {
            path,
            lex_offset,
            serial_offset,
            //TODO: Could be env var
            can_color: std::io::stdout().is_terminal(),
        }
    }
}
