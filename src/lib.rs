//! Socks5 protocol structure
//!
//! Socken5 provides an incomplete protocol structure for the socks5 protocol.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! socken5 = "0.1.0"
//! ```

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::result;
use std::net::{Ipv4Addr, Ipv6Addr};

// Public modules
/// Downstream packets.
pub mod downstream;
mod error;
/// Upstream packets.
pub mod upstream;

// Public exports
pub use crate::error::{Result, Error};

const VERSION: u8 = 0x05;

#[async_trait]
pub trait AsyncWrite {
    async fn write<W>(&self, buf: &mut W) -> Result<()> where W: AsyncWriteExt + Unpin + Send;
}

#[async_trait]
pub trait AsyncRead: Sized {
    async fn read<R>(buf: &mut R) -> Result<Self> where R: AsyncReadExt + Unpin + Send;
}

/// A socks5 method.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    /// No authentication.
    NoAuth,
    /// Gssapi authentication.
    Gssapi,
    /// Username/Password authentication.
    Auth,
    /// Other methods.
    Other(u8),
    /// No acceptable method found.
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

/// A socks5 command.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Command {
    /// Connect to target.
    Connect,
    /// Bind to socket.
    Bind,
    /// Associate Udp.
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

/// An address type, either IPv4, IPv6 or domain name.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Addr {
    /// An IPv4 address.
    V4(Ipv4Addr),
    /// A domain domain name.
    Domain(String),
    /// An IPv6 address.
    V6(Ipv6Addr),
}

/// A socks5 command reply.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reply {
    /// Successful.
    Success,
    /// Internal server error.
    ServerFailure,
    /// Connection not allowed by ruleset.
    NotAllowedByRuleset,
    /// Network unreachable.
    NetworkUnreachable,
    /// Host unreachable.
    HostUnreachable,
    /// Connection refused.
    ConnectionRefused,
    /// Ttl expired.
    TtlExpired,
    /// Command not supported.
    CommandNotSupported,
    /// Address type not supported.
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