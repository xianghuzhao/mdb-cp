use std::fmt;

use super::config;

#[derive(Debug)]
pub enum Error {
    ConfigError(config::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConfigError(err) => write!(f, "{}", err),
        }
    }
}

impl From<config::error::Error> for Error {
    fn from(err: config::error::Error) -> Self {
        Error::ConfigError(err)
    }
}
