
pub mod metadata;
use metadata::Metadata;

pub mod introduction;
use introduction::WorkerIntroduction;

// ===== Imports =====
use anyhow::Result;
use bytes::Bytes;
// ===================

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "$message_kind")]
pub enum Message {
  Metadata(Metadata),
  WorkerIntroduction(WorkerIntroduction),
  SubscribeToTopic { topic: String },
  PublishToTopic { topic: String },
}

impl Message {
  pub fn from_bytes(byts: Bytes) -> Result<Self> {
    let msg = serde_json::from_slice(&byts)?;
    Ok(msg)
  }
  
  pub fn into_bytes(&self) -> Result<Bytes> {
    let data = serde_json::to_vec(&self)?;
    Ok(Bytes::from(data))
  }
}