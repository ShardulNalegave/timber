
// ===== Imports =====
#[macro_use] extern crate log;
use anyhow::Result;
use axum::{routing::get, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;
// ===================

#[tokio::main]
async fn main() -> Result<()> {
  dotenv()?;
  pretty_env_logger::init_custom_env("TIMBER_LOG");
  
  let addr = std::env::var("TIMBER_ADDR")?;
  
  let router = Router::new()
    .route("/", get(index));

  info!("Listening at {}", addr);
  let lis = TcpListener::bind(addr).await?;
  axum::serve(lis, router).await?;

  Ok(())
}

async fn index() -> &'static str {
  "Hello, World!"
}