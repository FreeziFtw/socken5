use std::{result, io};
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid socks version (expected 5, found {0})")]
    InvalidVersion(u8),
    #[error("invalid command")]
    InvalidCommand,
    #[error("invalid address type")]
    InvalidAddrType,
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Utf(#[from] FromUtf8Error),
    #[error("{0}")]
    Int(#[from] TryFromIntError),
}