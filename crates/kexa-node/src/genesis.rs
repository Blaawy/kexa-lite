use anyhow::{Context, Result};
use kexa_consensus::{merkle_root, DIFFICULTY_BITS, SUBSIDY};
use kexa_proto::{Address, Block, BlockHeader, Hash32, Transaction, TxOut};
use serde::{Deserialize, Serialize};

pub const TESTNET_GENESIS_HASH_HEX: &str =
    "1b9c1803328d95518a0fd921ce8fd1d5f93c9a88ca02c0b1440248effc763159";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisSpec {
    pub network: String,
    pub header: GenesisHeaderSpec,
    pub coinbase_outputs: Vec<GenesisOutputSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisHeaderSpec {
    pub version: u32,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisOutputSpec {
    pub amount: u64,
    pub address_bech32: String,
}

pub fn build_testnet_genesis() -> (Block, Hash32) {
    let coinbase = Transaction {
        version: 0,
        inputs: vec![],
        outputs: vec![TxOut {
            amount: SUBSIDY,
            address: [0u8; 32],
        }],
    };
    let merkle = merkle_root(std::slice::from_ref(&coinbase));
    let header = BlockHeader {
        version: 0,
        prev_hash: Hash32::zero(),
        merkle_root: merkle,
        timestamp: 0,
        bits: DIFFICULTY_BITS,
        nonce: 0,
        height: 0,
    };
    let block = Block {
        header,
        txs: vec![coinbase],
    };
    let hash = block.header.hash();
    (block, hash)
}

pub fn build_genesis_from_spec(spec: &GenesisSpec) -> Result<(Block, Hash32)> {
    if spec.network != "mainnet" {
        anyhow::bail!("genesis spec network must be 'mainnet'");
    }
    if spec.coinbase_outputs.is_empty() {
        anyhow::bail!("genesis spec must include at least one coinbase output");
    }

    let mut outputs = Vec::with_capacity(spec.coinbase_outputs.len());
    for output in &spec.coinbase_outputs {
        let address = Address::from_bech32(&output.address_bech32)
            .with_context(|| format!("invalid genesis address: {}", output.address_bech32))?;
        outputs.push(TxOut {
            amount: output.amount,
            address: address.payload,
        });
    }

    let coinbase = Transaction {
        version: 0,
        inputs: vec![],
        outputs,
    };
    let version =
        u8::try_from(spec.header.version).context("genesis header.version out of range")?;
    let header = BlockHeader {
        version,
        prev_hash: Hash32::zero(),
        merkle_root: merkle_root(std::slice::from_ref(&coinbase)),
        timestamp: spec.header.timestamp,
        bits: spec.header.bits,
        nonce: spec.header.nonce,
        height: 0,
    };
    let block = Block {
        header,
        txs: vec![coinbase],
    };
    let hash = block.header.hash();
    Ok((block, hash))
}

pub fn load_genesis_spec(path: &str) -> Result<GenesisSpec> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed reading genesis spec: {path}"))?;
    let spec = serde_json::from_str(&raw)
        .with_context(|| format!("failed parsing genesis spec json: {path}"))?;
    Ok(spec)
}
