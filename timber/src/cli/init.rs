
// ===== Imports =====
use anyhow::Result;
use tokio::fs;
use std::{path::PathBuf, process::exit};
// ===================

pub async fn init_instance(path: &str) -> Result<()> {
  let path = PathBuf::from(path);

  if !path.exists() {
    fs::create_dir_all(path.clone()).await?;
  }

  if !path.is_dir() {
    error!("Path provided to 'init' is not a directory");
    exit(-1);
  }

  let metadata_file_path = path.join("timber-metadata.json");
  let transformers_dir = path.join("transformers");
  let rangers_dir = path.join("rangers");
  let archiver_dir = path.join("archiver");

  if metadata_file_path.exists() || transformers_dir.exists() || rangers_dir.exists() || archiver_dir.exists() {
    error!("Please check that init directory doesn't already have generated files and directorie");
    exit(-1);
  }

  let default_metadata = serde_json::to_string_pretty(&timber_common::metadata::Metadata::default())?;
  fs::write(metadata_file_path, default_metadata).await?;
  fs::create_dir(transformers_dir).await?;
  fs::create_dir(rangers_dir).await?;
  fs::create_dir(archiver_dir).await?;

  info!("Initialized a new timber project");
  Ok(())
}