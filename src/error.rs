use std::{result, fmt, io};
#[derive(fmt::Debug)]
pub enum Error {
    InvalidVersion,
    InvalidCommand,
    InvalidAddrType,
}