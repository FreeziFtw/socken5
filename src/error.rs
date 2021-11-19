use std::{result, fmt, io};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(fmt::Debug)]
pub enum Error {
    InvalidVersion,
    InvalidCommand,
    InvalidAddrType,
    Io(io::Error),
    Utf8(FromUtf8Error),
    Int(TryFromIntError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::Utf8(err)
    }
}

impl From<TryFromIntError> for Error {
    fn from(err: TryFromIntError) -> Self {
        Self::Int(err)
    }
}