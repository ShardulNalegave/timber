
// ===== Imports =====
use std::path::PathBuf;
// ===================

#[derive(Debug, Error)]
pub enum TimberError {
  #[error("The path provided to `init` subcommand is not a directory.\nPath: {path:?}")]
  InitPathIsNotADirectory { path: PathBuf },
}