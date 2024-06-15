
// ===== Imports =====
use anyhow::Result;
use bytes::BytesMut;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use crate::message::Message;
// ===================

#[async_trait]
pub trait RWMessage {
  async fn read_message(&mut self) -> Result<Message>;
  async fn write_message(&mut self, msg: &Message) -> Result<()>;
}

#[async_trait]
impl RWMessage for TcpStream {
  async fn read_message(&mut self) -> Result<Message> {
    let size = self.read_u128().await?;

    let mut buf = BytesMut::with_capacity(size as usize);
    self.read(&mut buf).await?;

    let msg = Message::from_bytes(buf.freeze())?;
    Ok(msg)
  }

  async fn write_message(&mut self, msg: &Message) -> Result<()> {
    let byts = msg.into_bytes()?;
    let size = byts.len() as u128;

    self.write_u128(size).await?;
    self.write(&byts).await?;
    
    Ok(())
  }
}