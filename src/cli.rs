
pub mod init;

#[derive(Parser)]
#[command(version = "0.1.0")]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  #[command(about = "Setup a new timber project in the current directory")]
  Init { path: String },
  #[command(about = "Runs timber")]
  Run,
}