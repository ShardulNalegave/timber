
// ===== Imports =====
use std::net::SocketAddr;
use anyhow::Result;
use tokio::net::TcpListener;
// ===================

pub async fn run_master(master_addr: SocketAddr) -> Result<()> {
  let lis = TcpListener::bind(master_addr).await?;
  info!("Master listening at {}", master_addr);

  loop {
    let (_conn, _addr) = lis.accept().await?;
    info!("New connection!");
  }
}