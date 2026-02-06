use anyhow::{Context, Result};
use std::{
    net::TcpListener,
    path::PathBuf,
    process::{Child, Command},
    time::Duration,
};

pub fn build_node_binary() -> Result<PathBuf> {
    let status = Command::new("cargo")
        .args(["build", "-p", "kexa-node"])
        .status()
        .context("build kexa-node")?;
    if !status.success() {
        anyhow::bail!("kexa-node build failed");
    }
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.pop();
    root.pop();
    let bin = root.join("target/debug/kexa-node");
    Ok(bin)
}

/// Ask the OS for a usable local TCP port (avoids Windows excluded port ranges).
pub fn pick_free_port() -> Result<u16> {
    let l = TcpListener::bind("127.0.0.1:0").context("bind ephemeral port")?;
    Ok(l.local_addr().context("local_addr")?.port())
}
pub fn spawn_node(
    bin: &PathBuf,
    rpc: u16,
    p2p: u16,
    data_dir: &str,
    peers: &str,
    mine: bool,
    miner_address: Option<&str>,
) -> Result<Child> {
    let mut cmd = Command::new(bin);
    cmd.args([
        "--rpc-addr",
        &format!("127.0.0.1:{}", rpc),
        "--p2p-addr",
        &format!("127.0.0.1:{}", p2p),
        "--data-dir",
        data_dir,
        "--peers",
        peers,
    ]);
    if mine {
        cmd.arg("--mine");
        if let Some(addr) = miner_address {
            cmd.args(["--miner-address", addr]);
        }
    }
    let child = cmd.spawn().context("spawn kexa-node")?;
    Ok(child)
}

pub async fn wait_for_ready(url: &str) -> Result<()> {
    let client = reqwest::Client::builder().no_proxy().build()?;
    for _ in 0..50 {
        if client.get(format!("{}/ready", url)).send().await.is_ok() {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
    anyhow::bail!("node not ready");
}
