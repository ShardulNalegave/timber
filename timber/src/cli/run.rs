
// ===== Imports =====
use std::{net::SocketAddr, process::exit};
use anyhow::Result;
use crate::{admin::run_admin, master::run_master};
use super::RunArgs;
// ===================

pub async fn run(args: &RunArgs) -> Result<()> {
  let admin_addr: SocketAddr = match args.admin_addr.parse() {
    Ok(sock_addr) => sock_addr,
    Err(_) => {
      error!("Invalid Admin Address provided");
      exit(-1);
    },
  };

  let master_addr: SocketAddr = match args.master_addr.parse() {
    Ok(sock_addr) => sock_addr,
    Err(_) => {
      error!("Invalid Master Address provided");
      exit(-1);
    },
  };

  tokio::select! {
    res = run_admin(admin_addr) => match res {
      Ok(_) => {},
      Err(e) => anyhow::bail!(e),
    },

    res = run_master(master_addr) => match res {
      Ok(_) => {},
      Err(e) => anyhow::bail!(e),
    },
  };

  Ok(())
}