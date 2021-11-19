use tokio::io::AsyncWriteExt;
use async_trait::async_trait;

use crate::error::Result;
use crate::{AsyncWrite, VERSION, Method, Reply, Addr};

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