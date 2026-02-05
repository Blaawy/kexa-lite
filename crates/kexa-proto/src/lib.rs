use bech32::{self, FromBase32, ToBase32, Variant};
use borsh::{BorshDeserialize, BorshSerialize};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde_big_array::BigArray;
use sha2::{Digest, Sha256};
use thiserror::Error;

pub const PROTOCOL_VERSION: u8 = 0;
pub const ADDRESS_HRP: &str = "kexa";

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Hash32(pub [u8; 32]);

impl Hash32 {
    pub fn zero() -> Self {
        Self([0u8; 32])
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Address {
    pub payload: [u8; 32],
}

impl Address {
    pub fn from_pubkey(pubkey: &VerifyingKey) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(pubkey.to_bytes());
        let digest = hasher.finalize();
        let mut payload = [0u8; 32];
        payload.copy_from_slice(&digest);
        Self { payload }
    }

    pub fn from_pubkey_bytes(pubkey: &[u8; 32]) -> Option<Self> {
        let key = VerifyingKey::from_bytes(pubkey).ok()?;
        Some(Self::from_pubkey(&key))
    }

    pub fn to_bech32(&self) -> String {
        bech32::encode(ADDRESS_HRP, self.payload.to_base32(), Variant::Bech32).expect("bech32")
    }

    pub fn from_bech32(addr: &str) -> Result<Self, AddressError> {
        let (hrp, data, variant) = bech32::decode(addr)?;
        if hrp != ADDRESS_HRP {
            return Err(AddressError::InvalidPrefix(hrp));
        }
        if variant != Variant::Bech32 {
            return Err(AddressError::InvalidVariant);
        }
        let bytes = Vec::<u8>::from_base32(&data)?;
        if bytes.len() != 32 {
            return Err(AddressError::InvalidLength(bytes.len()));
        }
        let mut payload = [0u8; 32];
        payload.copy_from_slice(&bytes);
        Ok(Self { payload })
    }
}

#[derive(Debug, Error)]
pub enum AddressError {
    #[error("bech32 decode error: {0}")]
    Bech32(#[from] bech32::Error),
    #[error("invalid prefix: {0}")]
    InvalidPrefix(String),
    #[error("invalid variant")]
    InvalidVariant,
    #[error("invalid length: {0}")]
    InvalidLength(usize),
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct OutPoint {
    pub txid: Hash32,
    pub index: u32,
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct TxIn {
    pub outpoint: OutPoint,
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
    pub pubkey: [u8; 32],
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct TxOut {
    pub amount: u64,
    pub address: [u8; 32],
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Transaction {
    pub version: u8,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
}

impl Transaction {
    pub fn txid(&self) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update(self.serialize());
        Hash32(hasher.finalize().into())
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(PROTOCOL_VERSION);
        data.extend(borsh::to_vec(self).expect("tx serialize"));
        data
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct BlockHeader {
    pub version: u8,
    pub prev_hash: Hash32,
    pub merkle_root: Hash32,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
    pub height: u64,
}

impl BlockHeader {
    pub fn hash(&self) -> Hash32 {
        let mut hasher = Sha256::new();
        hasher.update(self.serialize());
        let first = hasher.finalize_reset();
        hasher.update(first);
        Hash32(hasher.finalize().into())
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.push(PROTOCOL_VERSION);
        data.extend(borsh::to_vec(self).expect("header serialize"));
        data
    }
}

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    BorshSerialize,
    BorshDeserialize,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct Block {
    pub header: BlockHeader,
    pub txs: Vec<Transaction>,
}

pub fn sign_tx(signing_key: &SigningKey, message: &[u8]) -> [u8; 64] {
    let sig: Signature = signing_key.sign(message);
    sig.to_bytes()
}

pub fn verify_tx_signature(pubkey_bytes: &[u8; 32], signature: &[u8; 64], message: &[u8]) -> bool {
    if let Ok(pubkey) = VerifyingKey::from_bytes(pubkey_bytes) {
        let sig = Signature::from_bytes(signature);
        return pubkey.verify_strict(message, &sig).is_ok();
    }
    false
}

pub fn tx_signing_hash(tx: &Transaction) -> Hash32 {
    let mut sanitized = tx.clone();
    for input in &mut sanitized.inputs {
        input.signature = [0u8; 64];
    }
    let mut hasher = Sha256::new();
    hasher.update(sanitized.serialize());
    Hash32(hasher.finalize().into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_round_trip() {
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let addr = Address::from_pubkey(&key.verifying_key());
        let encoded = addr.to_bech32();
        let decoded = Address::from_bech32(&encoded).expect("decode");
        assert_eq!(addr, decoded);
    }

    #[test]
    fn tx_serialization_golden() {
        let tx = Transaction {
            version: 0,
            inputs: vec![],
            outputs: vec![TxOut {
                amount: 42,
                address: [1u8; 32],
            }],
        };
        let bytes = tx.serialize();
        let hex = hex::encode(bytes);
        assert_eq!(
            hex,
            "000000000000010000002a000000000000000101010101010101010101010101010101010101010101010101010101010101"
        );
    }
}
