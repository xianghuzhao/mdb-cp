use std::fmt;

#[derive(Debug)]
pub struct Error {
    file_path: String,
    message: String,
}

impl Error {
    pub fn new(file_path: &str, message: &str) -> Self {
        Error {
            file_path: String::from(file_path),
            message: String::from(message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Load config file \"{}\" error -> {}",
            self.file_path, self.message
        )
    }
}
