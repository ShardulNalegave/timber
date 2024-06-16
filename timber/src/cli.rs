
pub mod init;
pub mod run;

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
  Run(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
  #[arg(short, long)]
  pub master_addr: String,
  #[arg(short, long)]
  pub admin_addr: String,
}