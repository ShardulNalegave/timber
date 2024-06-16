
pub mod errors;
pub mod cli;
pub mod admin;
pub mod master;

// ===== Imports =====
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate thiserror;

use anyhow::Result;
use clap::Parser;
use cli::Commands;
// ===================

#[tokio::main]
async fn main() -> Result<()> {
  pretty_env_logger::init_custom_env("TIMBER_LOG");
  let cli_args = cli::Cli::parse();

  match &cli_args.command {
    Commands::Run(args) => cli::run::run(args).await?,
    Commands::Init { path } => cli::init::init_instance(path).await?,
  };

  Ok(())
}