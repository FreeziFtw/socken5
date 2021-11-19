use tokio::io::AsyncReadExt;
use async_trait::async_trait;

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
        if buf.read_u8().await? != VERSION {
            return Err(Error::InvalidVersion);
        }

        let len = usize::from(buf.read_u8().await?);
        let mut methods = Vec::with_capacity(len);

        for _ in 0..len {
            methods.push(Method::from(buf.read_u8().await?));
        }

        Ok(Self(methods))
    }
}