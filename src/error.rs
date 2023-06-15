use std::fmt;

use super::config;
use super::cp;

pub enum Error {
    ConfigError(config::error::Error),
    CopyError(cp::error::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ConfigError(err) => err.fmt(f),
            Error::CopyError(err) => err.fmt(f),
        }
    }
}

impl From<config::error::Error> for Error {
    fn from(err: config::error::Error) -> Self {
        Error::ConfigError(err)
    }
}

impl From<cp::error::Error> for Error {
    fn from(err: cp::error::Error) -> Self {
        Error::CopyError(err)
    }
}
