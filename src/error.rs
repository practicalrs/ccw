use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    OllamaHostAddresMissing,
    OllamaRequestProblem,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error!")
    }
}
