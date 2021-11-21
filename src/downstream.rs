use tokio::io::AsyncWriteExt;
use async_trait::async_trait;

use crate::error::Result;
use crate::{AsyncWrite, VERSION, Method, Reply, Addr};

/// ```text
/// +----+--------+
/// |VER | METHOD |
/// +----+--------+
/// | 1  |   1    |
/// +----+--------+
/// ```
#[derive(Debug)]
pub struct Handshake(pub Method);

#[async_trait]
impl AsyncWrite for Handshake {
    async fn write<W>(&self, buf: &mut W) -> Result<()>
        where
            W: AsyncWriteExt + Unpin + Send
    {
        buf.write_u8(VERSION).await?;
        buf.write_u8(u8::from(self.0)).await?;
        Ok(())
    }
}

/// ```text
/// +----+-----+-------+------+----------+----------+
/// |VER | REP |  RSV  | ATYP | BND.ADDR | BND.PORT |
/// +----+-----+-------+------+----------+----------+
/// | 1  |  1  | X'00' |  1   | Variable |    2     |
/// +----+-----+-------+------+----------+----------+
/// ```
#[derive(Debug)]
pub struct CommandResponse {
    pub reply: Reply,
    pub addr: Addr,
    pub port: u16,
}

#[async_trait]
impl AsyncWrite for CommandResponse {
    async fn write<W>(&self, buf: &mut W) -> Result<()>
        where
            W: AsyncWriteExt + Unpin + Send
    {
        buf.write_u8(VERSION).await?;
        buf.write_u8(u8::from(self.reply)).await?;
        buf.write_u8(0x00).await?;

        match &self.addr {
            Addr::V4(ip) => {
                buf.write_u8(0x01).await?;
                buf.write(&ip.octets()).await?;
            }
            Addr::Domain(domain) => {
                buf.write_u8(0x03).await?;
                buf.write_u8(u8::try_from(domain.len())?).await?;
                buf.write(domain.as_bytes()).await?;
            }
            Addr::V6(ip) => {
                buf.write_u8(0x04).await?;
                buf.write(&ip.octets()).await?;
            }
        }

        buf.write_u16(self.port).await?;
        Ok(())
    }
}

/// ```text
/// +----+--------+
/// |VER | STATUS |
/// +----+--------+
/// | 1  |   1    |
/// +----+--------+
/// ```
#[derive(Debug)]
pub struct AuthResponse(pub u8);

#[async_trait]
impl AsyncWrite for AuthResponse {
    async fn write<W>(&self, buf: &mut W) -> Result<()>
        where
            W: AsyncWriteExt + Unpin + Send
    {
        buf.write_u8(0x01).await?;
        buf.write_u8(self.0).await?;
        Ok(())
    }
}