use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Generic(String),
    PortNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

impl<T: Display> From<T> for Error {
    fn from(e: T) -> Self {
        Self::Generic(e.to_string())
    }
}
