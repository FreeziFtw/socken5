use std::{result, fmt, io};
pub type Result<T> = result::Result<T, Error>;

#[derive(fmt::Debug)]
pub enum Error {
    InvalidVersion,
    InvalidCommand,
    InvalidAddrType,
}