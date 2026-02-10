use anyhow::Result;
use kexa_proto::{Block, BlockHeader, Hash32, Transaction};
use sha2::{Digest, Sha256};

pub const DIFFICULTY_BITS: u32 = 16; // leading zero bits required in devnet
pub const SUBSIDY: u64 = 50;
pub const COINBASE_MATURITY: u64 = 0;

/// Mainnet economics (Gate M1 locked params)
pub const MAX_SUPPLY: u64 = 18_000_000;
/// 1.5% of 18,000,000 (policy-only lock/vesting in v0; enforcement decision deferred)
pub const FOUNDERS_RESERVE: u64 = 270_000;
pub const MINEABLE_SUPPLY: u64 = MAX_SUPPLY - FOUNDERS_RESERVE;
/// Number of subsidy-bearing blocks (heights 1..=MINEABLE_BLOCKS)
pub const MINEABLE_BLOCKS: u64 = MINEABLE_SUPPLY / SUBSIDY; // 354_600 at SUBSIDY=50

/// Height-based subsidy schedule for mainnet (v0):
/// - height 0 (genesis): 0
/// - heights 1..=MINEABLE_BLOCKS: SUBSIDY
/// - after that: 0 (fees only)
pub fn block_subsidy(height: u64) -> u64 {
    if height == 0 {
        0
    } else if height <= MINEABLE_BLOCKS {
        SUBSIDY
    } else {
        0
    }
}

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

    #[test]
    fn emission_schedule_params_locked() {
        // Gate M1 locked numbers
        assert_eq!(SUBSIDY, 50);
        assert_eq!(MAX_SUPPLY, 18_000_000);
        assert_eq!(FOUNDERS_RESERVE, 270_000);
        assert_eq!(MINEABLE_SUPPLY, 17_730_000);
        assert_eq!(MINEABLE_BLOCKS, 354_600);

        // Subsidy schedule checkpoints
        assert_eq!(block_subsidy(0), 0);
        assert_eq!(block_subsidy(1), SUBSIDY);
        assert_eq!(block_subsidy(MINEABLE_BLOCKS), SUBSIDY);
        assert_eq!(block_subsidy(MINEABLE_BLOCKS + 1), 0);

        // Supply identity
        let mined = SUBSIDY.saturating_mul(MINEABLE_BLOCKS);
        assert_eq!(mined, MINEABLE_SUPPLY);
        assert_eq!(mined + FOUNDERS_RESERVE, MAX_SUPPLY);
    }
}
