use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum StribotError {
    Reqwest(reqwest::Error),
    StatusError,
    ParsingError,
}

impl fmt::Display for StribotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StribotError::Reqwest(ref err) => err.fmt(f),
            StribotError::StatusError => write!(f, "HTTP status error."),
            StribotError::ParsingError => write!(f, "HTML parsing error."),
        }
    }
}

impl Error for StribotError {
    fn description(&self) -> &str {
        match *self {
            StribotError::Reqwest(ref err) => err.description(),
            StribotError::StatusError => "status error",
            StribotError::ParsingError => "parsing error",
        }
    }
}

impl From<reqwest::Error> for StribotError {
    fn from(err: reqwest::Error) -> StribotError {
        StribotError::Reqwest(err)
    }
}
