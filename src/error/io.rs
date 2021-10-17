use std::{
    io::Error as IoError,
    fmt::{Display, Formatter, Result as IoResult}
};
use thiserror::Error;

#[derive(Debug, Error)]
pub struct Error(IoError);

impl Clone for Error {
    fn clone(&self) -> Self {
        Self(IoError::new(self.0.kind(), self.0.to_string()))
    }
}

impl Eq for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> IoResult {
        write!(f, "{}", self.0.to_string())
    }
}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.kind() == rhs.0.kind()
    }
}
