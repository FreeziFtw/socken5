use tokio::io::AsyncReadExt;
use async_trait::async_trait;
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::{Result, Error};
use crate::{AsyncRead, Command, VERSION, Method, Addr};

#[derive(Debug)]
pub struct Handshake(pub Vec<Method>);

#[async_trait]
impl AsyncRead for Handshake {
    async fn read<R>(buf: &mut R) -> Result<Self>
        where
            R: AsyncReadExt + Unpin + Send
    {
        let version = buf.read_u8()
            .await?;

        if version != VERSION {
            return Err(Error::InvalidVersion(version));
        }

        let len = usize::from(buf.read_u8().await?);
        let mut methods = Vec::with_capacity(len);

        for _ in 0..len {
            methods.push(Method::from(buf.read_u8().await?));
        }

        Ok(Self(methods))
    }
}

#[derive(Debug)]
pub struct CommandRequest {
    pub cmd: Command,
    pub addr: Addr,
    pub port: u16,
}

#[async_trait]
impl AsyncRead for CommandRequest {
    async fn read<R>(buf: &mut R) -> Result<Self>
        where
            R: AsyncReadExt + Unpin + Send
    {
        let version = buf.read_u8()
            .await?;

        if version != VERSION {
            return Err(Error::InvalidVersion(version));
        }

        let cmd = Command::try_from(buf.read_u8().await?)?;
        buf.read_u8().await?;

        let addr = match buf.read_u8().await? {
            0x01 => {
                let mut octets = [0u8; 4];
                buf.read(&mut octets).await?;
                Addr::V4(Ipv4Addr::from(octets))
            }
            0x03 => {
                let len = usize::from(buf.read_u8().await?);
                let mut octets = Vec::with_capacity(len);

                for _ in 0..len {
                    octets.push(buf.read_u8().await?);
                }

                Addr::Domain(String::from_utf8(octets)?)
            }
            0x04 => {
                let mut octets = [0u8; 16];
                buf.read(&mut octets).await?;
                Addr::V6(Ipv6Addr::from(octets))
            }
            _ => return Err(Error::InvalidAddrType),
        };

        Ok(
            Self {
                cmd,
                addr,
                port: buf.read_u16().await?
            }
        )
    }
}