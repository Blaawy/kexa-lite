use anyhow::Result;
use kexa_proto::{Block, BlockHeader, Hash32, Transaction};
use sha2::{Digest, Sha256};

pub const DIFFICULTY_BITS: u32 = 16; // leading zero bits required in devnet
pub const SUBSIDY: u64 = 50;
pub const COINBASE_MATURITY: u64 = 0;

pub fn merkle_root(txs: &[Transaction]) -> Hash32 {
    if txs.is_empty() {
        return Hash32::zero();
    }
    let mut layer: Vec<Hash32> = txs.iter().map(|tx| tx.txid()).collect();
    while layer.len() > 1 {
        let mut next = Vec::with_capacity(layer.len().div_ceil(2));
        for pair in layer.chunks(2) {
            let left = pair[0];
            let right = if pair.len() == 2 { pair[1] } else { pair[0] };
            let mut hasher = Sha256::new();
            hasher.update(left.0);
            hasher.update(right.0);
            next.push(Hash32(hasher.finalize().into()));
        }
        layer = next;
    }
    layer[0]
}

pub fn check_pow(header: &BlockHeader) -> bool {
    let hash = header.hash().0;
    let mut remaining = header.bits;
    for byte in hash.iter() {
        if remaining >= 8 {
            if *byte != 0 {
                return false;
            }
            remaining -= 8;
        } else if remaining == 0 {
            return true;
        } else {
            let mask = 0xFFu8 << (8 - remaining);
            return byte & mask == 0;
        }
    }
    true
}

pub fn validate_block(block: &Block) -> Result<()> {
    if block.txs.is_empty() {
        anyhow::bail!("block has no transactions");
    }
    let root = merkle_root(&block.txs);
    if root != block.header.merkle_root {
        anyhow::bail!("merkle root mismatch");
    }
    if !check_pow(&block.header) {
        anyhow::bail!("pow invalid");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kexa_proto::{Transaction, TxOut};

    #[test]
    fn merkle_root_golden() {
        let tx = Transaction {
            version: 0,
            inputs: vec![],
            outputs: vec![TxOut {
                amount: 1,
                address: [2u8; 32],
            }],
        };
        let root = merkle_root(&[tx]);
        assert_eq!(
            hex::encode(root.0),
            "f17fa62d5443ba6f40363093a346f426c65a96095c6e88580d263b721a07c20d"
        );
    }
}
