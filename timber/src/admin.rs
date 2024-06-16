
// ===== Imports =====
use std::net::SocketAddr;
use anyhow::Result;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
// ===================

pub async fn run_admin(admin_addr: SocketAddr) -> Result<()> {
  let router = Router::new()
    .route("/", get(index));

  let lis = TcpListener::bind(admin_addr).await?;

  info!("Admin listening at {}", admin_addr);
  axum::serve(lis, router).await?;
  Ok(())
}

async fn index() -> &'static str {
  "Timber"
}