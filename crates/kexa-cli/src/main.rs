use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(name = "kexa-cli")]
#[command(about = "KEXA mini-explorer CLI (testnet/devnet)", long_about = None)]
struct Args {
    /// RPC base URL (e.g. http://127.0.0.1:18030 or http://<host>:8030)
    #[arg(long, default_value = "http://127.0.0.1:18030")]
    rpc: String,

    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Check node health
    Health,
    /// Show chain tip (height + hash)
    Tip,
    /// Show last N blocks (summary)
    Blocks {
        /// Number of blocks from tip backwards
        #[arg(long, default_value_t = 20)]
        last: usize,
    },
    /// Fetch a block by hash, or by height (client-side)
    Block {
        /// Block hash (hex)
        hash: Option<String>,
        /// Block height (uses /blocks?limit= to find matching height)
        #[arg(long)]
        height: Option<u64>,
        /// When using --height, how many blocks to scan back from tip
        #[arg(long, default_value_t = 500)]
        scan: usize,
    },
}

#[derive(Deserialize, Debug)]
struct TipResp {
    height: u64,
    hash: String,
}

#[derive(Deserialize, Debug)]
struct BlockSummary {
    height: u64,
    hash: String,
    tx_count: usize,
    timestamp: u64,
}

fn join_url(base: &str, path: &str) -> String {
    let b = base.trim_end_matches('/');
    let p = path.trim_start_matches('/');
    format!("{b}/{p}")
}

fn http_get_text(url: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let status = resp.status();
    let body = resp.text()?;
    if !status.is_success() {
        return Err(anyhow!("HTTP {status} from {url}: {body}"));
    }
    Ok(body)
}

fn http_get_json<T: for<'de> Deserialize<'de>>(url: &str) -> Result<T> {
    let client = reqwest::blocking::Client::new();
    let resp = client.get(url).send()?;
    let status = resp.status();
    let text = resp.text()?;
    if !status.is_success() {
        return Err(anyhow!("HTTP {status} from {url}: {text}"));
    }
    Ok(serde_json::from_str(&text)?)
}

fn main() -> Result<()> {
    let args = Args::parse();
    let rpc = args.rpc;

    match args.cmd {
        Cmd::Health => {
            let url = join_url(&rpc, "/health");
            let out = http_get_text(&url)?;
            println!("{out}");
        }
        Cmd::Tip => {
            let url = join_url(&rpc, "/tip");
            let tip: TipResp = http_get_json(&url)?;
            println!("height: {}", tip.height);
            println!("hash:   {}", tip.hash);
        }
        Cmd::Blocks { last } => {
            if last == 0 || last > 500 {
                return Err(anyhow!("--last must be 1..=500"));
            }
            let url = join_url(&rpc, &format!("/blocks?limit={last}"));
            let blocks: Vec<BlockSummary> = http_get_json(&url)?;
            for b in blocks {
                println!(
                    "h={}  hash={}  txs={}  ts={}",
                    b.height, b.hash, b.tx_count, b.timestamp
                );
            }
        }
        Cmd::Block { hash, height, scan } => {
            match (hash, height) {
                (Some(h), None) => {
                    let url = join_url(&rpc, &format!("/block/{h}"));
                    let out = http_get_text(&url)?;
                    println!("{out}");
                }
                (None, Some(h)) => {
                    if scan == 0 || scan > 5000 {
                        return Err(anyhow!("--scan must be 1..=5000"));
                    }
                    let url = join_url(&rpc, &format!("/blocks?limit={scan}"));
                    let blocks: Vec<BlockSummary> = http_get_json(&url)?;
                    let found = blocks.into_iter().find(|b| b.height == h).ok_or_else(|| {
                        anyhow!(
                            "height {h} not found within last {scan} blocks from tip (try bigger --scan)"
                        )
                    })?;
                    let url = join_url(&rpc, &format!("/block/{}", found.hash));
                    let out = http_get_text(&url)?;
                    println!("{out}");
                }
                _ => {
                    return Err(anyhow!(
                        "Usage: kexa-cli block <hash>  OR  kexa-cli block --height <h> [--scan N]"
                    ));
                }
            }
        }
    }

    Ok(())
}
