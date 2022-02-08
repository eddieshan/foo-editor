use std::io;

#[derive(Debug)]
pub enum TermError {
    CannotGetTermAttributes,
    CannotSetTermAttributes,
    InvalidTermAttributes
}

#[derive(Debug)]
pub enum EditorError {
    OsTermError(TermError), // Errors caused by OS term specific sys calls.
    IoError(io::Error) // General IO errors.
}


impl From<TermError> for EditorError {
    fn from(err: TermError) -> Self {
        EditorError::OsTermError(err)
    }
}

impl From<io::Error> for EditorError {
    fn from(err: io::Error) -> Self {
        EditorError::IoError(err)
    }
}