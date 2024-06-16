
// ===== Imports =====
use anyhow::Result;
use tokio::fs;
use std::path::PathBuf;

use crate::errors::TimberError;
// ===================

pub async fn init_instance(path: &str) -> Result<()> {
  let path = PathBuf::from(path);
  if !path.is_dir() {
    bail!(TimberError::InitPathIsNotADirectory { path })
  }

  let metadata_file_path = path.join("timber-metadata.json");
  let default_metadata = serde_json::to_string(&timber_common::metadata::Metadata::default())?;
  fs::write(metadata_file_path, default_metadata).await?;

  let workers_dir = path.join("workers");
  fs::create_dir(workers_dir).await?;

  info!("Initialized a new timber project");
  Ok(())
}