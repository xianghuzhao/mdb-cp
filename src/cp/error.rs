use std::fmt;

pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            message: String::from(message),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
