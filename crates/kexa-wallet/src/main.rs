use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ed25519_dalek::SigningKey;
use kexa_proto::{sign_tx, tx_signing_hash, Address, OutPoint, Transaction, TxIn, TxOut};
use rand::rngs::OsRng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::info;

#[derive(Parser)]
#[command(name = "kexa-wallet")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New {
        name: String,
    },
    Address {
        name: String,
    },
    Balance {
        name: String,
        #[arg(long)]
        node: String,
    },
    Send {
        name: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        fee: u64,
        #[arg(long)]
        node: String,
    },
}

#[derive(Serialize, Deserialize)]
struct WalletFile {
    secret: [u8; 32],
}

#[derive(Deserialize)]
struct UtxoResponse {
    txid: String,
    index: u32,
    amount: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();

    match args.command {
        Command::New { name } => {
            let path = wallet_path(&name)?;
            fs::create_dir_all(path.parent().expect("parent"))?;
            let key = SigningKey::generate(&mut OsRng);
            let file = WalletFile {
                secret: key.to_bytes(),
            };
            fs::write(&path, serde_json::to_vec_pretty(&file)?)?;
            info!("wallet created at {}", path.display());
        }
        Command::Address { name } => {
            let key = load_wallet(&name)?;
            let address = Address::from_pubkey(&key.verifying_key());
            println!("{}", address.to_bech32());
        }
        Command::Balance { name, node } => {
            let key = load_wallet(&name)?;
            let address = Address::from_pubkey(&key.verifying_key());
            let url = format!("{}/balance/{}", node, address.to_bech32());
            let client = Client::new();
            let amount: u64 = client.get(url).send().await?.json().await?;
            println!("{}", amount);
        }
        Command::Send {
            name,
            to,
            amount,
            fee,
            node,
        } => {
            let key = load_wallet(&name)?;
            let from_address = Address::from_pubkey(&key.verifying_key());
            let to_address = Address::from_bech32(&to)?;
            let utxos = fetch_utxos(&node, &from_address).await?;
            let mut selected = Vec::new();
            let mut total = 0u64;
            for utxo in utxos {
                total = total.saturating_add(utxo.amount);
                selected.push(utxo);
                if total >= amount.saturating_add(fee) {
                    break;
                }
            }
            if total < amount.saturating_add(fee) {
                anyhow::bail!("insufficient funds");
            }
            let mut inputs = Vec::new();
            for utxo in &selected {
                let mut txid = [0u8; 32];
                txid.copy_from_slice(&hex::decode(&utxo.txid)?);
                let outpoint = OutPoint {
                    txid: kexa_proto::Hash32(txid),
                    index: utxo.index,
                };
                inputs.push(TxIn {
                    outpoint,
                    signature: [0u8; 64],
                    pubkey: key.verifying_key().to_bytes(),
                });
            }
            let mut outputs = vec![TxOut {
                amount,
                address: to_address.payload,
            }];
            let change = total - amount - fee;
            if change > 0 {
                outputs.push(TxOut {
                    amount: change,
                    address: from_address.payload,
                });
            }
            let mut tx = Transaction {
                version: 0,
                inputs,
                outputs,
            };
            let signing_hash = tx_signing_hash(&tx);
            for input in &mut tx.inputs {
                input.signature = sign_tx(&key, &signing_hash.0);
            }
            let client = Client::new();
            let url = format!("{}/submit_tx", node);
            let resp: String = client
                .post(url)
                .json(&serde_json::json!({"tx": tx}))
                .send()
                .await?
                .json()
                .await?;
            println!("{}", resp);
        }
    }

    Ok(())
}

fn wallet_path(name: &str) -> Result<PathBuf> {
    let mut path = dirs::home_dir().context("home dir")?;
    path.push(".kexa/wallets");
    path.push(format!("{}.json", name));
    Ok(path)
}

fn load_wallet(name: &str) -> Result<SigningKey> {
    let path = wallet_path(name)?;
    let data = fs::read(path)?;
    let file: WalletFile = serde_json::from_slice(&data)?;
    Ok(SigningKey::from_bytes(&file.secret))
}

async fn fetch_utxos(node: &str, address: &Address) -> Result<Vec<UtxoResponse>> {
    let url = format!("{}/utxos/{}", node, address.to_bech32());
    let client = Client::new();
    let utxos: Vec<UtxoResponse> = client.get(url).send().await?.json().await?;
    Ok(utxos)
}
