//! このソフトウェア独自のエラーを定義する.

use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct OperationCancelError;

impl fmt::Display for OperationCancelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "operation canceled.")
    }
}

impl error::Error for OperationCancelError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FileNotFoundError {
    filename: String,
}
impl FileNotFoundError {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "file '{}' not found", self.filename)
    }
}

impl error::Error for FileNotFoundError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
