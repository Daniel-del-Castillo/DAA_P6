//! This file defines the different errors that can appear while
//! reading a problem instance from a file
use std::error::Error;
use std::fmt;

/// The enum that encapsulates the two types of error that can appear while
/// reading a problem instance from a file. It can be an IO error or a SyntaxError
#[derive(Debug)]
pub enum ProblemInstanceError {
    IOError(std::io::Error),
    SyntaxError(usize),
}

impl fmt::Display for ProblemInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProblemInstanceError::IOError(error) => {
                write!(f, "There has been an IO error: {}", error)
            }
            ProblemInstanceError::SyntaxError(line) => {
                write!(f, "A syntax error has been found at line {}", line)
            }
        }
    }
}

impl Error for ProblemInstanceError {}

impl From<std::io::Error> for ProblemInstanceError {
    fn from(error: std::io::Error) -> Self {
        ProblemInstanceError::IOError(error)
    }
}
