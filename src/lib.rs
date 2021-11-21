use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::result;
use std::net::{Ipv4Addr, Ipv6Addr};

pub use crate::error::{Result, Error};

pub mod downstream;
pub mod error;
pub mod upstream;

const VERSION: u8 = 0x05;

#[async_trait]
pub trait AsyncWrite {
    async fn write<W>(&self, buf: &mut W) -> Result<()> where W: AsyncWriteExt + Unpin + Send;
}

#[async_trait]
pub trait AsyncRead: Sized {
    async fn read<R>(buf: &mut R) -> Result<Self> where R: AsyncReadExt + Unpin + Send;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    NoAuth,
    Gssapi,
    Auth,
    Other(u8),
    NoAcceptable,
}

impl From<u8> for Method {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Self::NoAuth,
            0x01 => Self::Gssapi,
            0x02 => Self::Auth,
            _ => Self::Other(val),
        }
    }
}

impl From<Method> for u8 {
    fn from(method: Method) -> Self {
        match method {
            Method::NoAuth => 0x00,
            Method::Gssapi => 0x01,
            Method::Auth => 0x02,
            Method::Other(val) => val,
            Method::NoAcceptable => 0xFF,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    Connect,
    Bind,
    UdpAssociate,
}

impl TryFrom<u8> for Command {
    type Error = Error;

    fn try_from(val: u8) -> result::Result<Self, Self::Error> {
        match val {
            0x01 => Ok(Self::Connect),
            0x02 => Ok(Self::Bind),
            0x03 => Ok(Self::UdpAssociate),
            _ => Err(Error::InvalidCommand)
        }
    }
}

impl From<Command> for u8 {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Connect => 0x01,
            Command::Bind => 0x02,
            Command::UdpAssociate => 0x03,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Addr {
    V4(Ipv4Addr),
    Domain(String),
    V6(Ipv6Addr),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reply {
    Success,
    ServerFailure,
    NotAllowedByRuleset,
    NetworkUnreachable,
    HostUnreachable,
    ConnectionRefused,
    TtlExpired,
    CommandNotSupported,
    AddrTypeNotSupported,
}

impl From<Reply> for u8 {
    fn from(reply: Reply) -> Self {
        match reply {
            Reply::Success => 0x00,
            Reply::ServerFailure => 0x01,
            Reply::NotAllowedByRuleset => 0x02,
            Reply::NetworkUnreachable => 0x03,
            Reply::HostUnreachable => 0x04,
            Reply::ConnectionRefused => 0x05,
            Reply::TtlExpired => 0x06,
            Reply::CommandNotSupported => 0x07,
            Reply::AddrTypeNotSupported => 0x08,
        }
    }
}