use async_trait::async_trait;
use tokio::io::*;

const KEEPALIVE: &[u8; 4] = &[92, 82, 192, 75];

use std::io::Result;
#[async_trait]
pub trait Comms: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin {
    async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg).await?;
        Ok(msg)
    }

    async fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length).await?;
        self.flush().await?;
        self.write(buf).await?;
        self.flush().await?;
        Ok(())
    }

    async fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive().await?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    async fn receive_keepalive(&mut self) -> Result<()> {
        let p = self.receive().await?;
        if p != KEEPALIVE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("keepalive failed. received {:?}", p),
            ));
        }
        Ok(())
    }

    async fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE).await?;
        Ok(())
    }

    async fn send_handshake(&mut self) -> Result<()> {
        self.send_keepalive().await?;
        self.receive_keepalive().await?;
        Ok(())
    }

    async fn receive_handshake(&mut self) -> Result<()> {
        self.receive_keepalive().await?;
        self.send_keepalive().await?;
        Ok(())
    }
}

impl<T: AsyncWrite + AsyncRead + AsyncWriteExt + Unpin> Comms for T {}

#[async_trait]
pub trait CommsSend: AsyncWrite + AsyncWriteExt + Unpin {
    async fn send(&mut self, buf: &[u8]) -> Result<()> {
        let length: [u8; 8] = u64::to_ne_bytes(buf.len() as u64);
        self.write(&length).await?;
        self.flush().await?;
        self.write(buf).await?;
        self.flush().await?;
        Ok(())
    }

    async fn send_keepalive(&mut self) -> Result<()> {
        self.send(KEEPALIVE).await?;
        Ok(())
    }
}

#[async_trait]
pub trait CommsReceive: AsyncRead + Unpin {
    async fn receive(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 8];
        self.read_exact(&mut buf).await?;
        let length = u64::from_ne_bytes(buf);
        let mut msg = vec![0u8; length as usize];
        self.read_exact(&mut msg).await?;
        Ok(msg)
    }

    async fn receive_to_string(&mut self) -> Result<String> {
        let v = self.receive().await?;
        Ok(std::string::String::from_utf8_lossy(&v).to_string())
    }

    async fn receive_keepalive(&mut self) -> Result<()> {
        let p = self.receive().await?;
        if p != KEEPALIVE {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("keepalive failed. received {:?}", p),
            ));
        }
        Ok(())
    }
}

impl<T: AsyncWrite + AsyncWriteExt + Unpin> CommsSend for T {}
impl<T: AsyncRead + Unpin> CommsReceive for T {}
