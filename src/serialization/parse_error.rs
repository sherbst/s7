use std::convert::From;
use std::fmt;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self {
            message: format!("{}", err),
        }
    }
}
