use std::error::Error;
use std::fmt;

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
