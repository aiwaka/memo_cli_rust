use std::error;
use std::fmt;

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
