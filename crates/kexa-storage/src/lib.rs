use anyhow::Result;
use borsh::BorshDeserialize;
use kexa_proto::{Block, BlockHeader, Hash32, OutPoint, TxOut};
use sled::Db;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    fn tree(&self, name: &str) -> sled::Tree {
        self.db.open_tree(name).expect("tree")
    }

    pub fn put_block(&self, hash: &Hash32, block: &Block) -> Result<()> {
        self.tree("blocks").insert(hash.0, borsh::to_vec(block)?)?;
        Ok(())
    }

    pub fn get_block(&self, hash: &Hash32) -> Result<Option<Block>> {
        if let Some(value) = self.tree("blocks").get(hash.0)? {
            let block = Block::try_from_slice(&value)?;
            Ok(Some(block))
        } else {
            Ok(None)
        }
    }

    pub fn put_header(&self, height: u64, header: &BlockHeader) -> Result<()> {
        self.tree("headers")
            .insert(height.to_be_bytes(), borsh::to_vec(header)?)?;
        Ok(())
    }

    pub fn put_height_hash(&self, height: u64, hash: &Hash32) -> Result<()> {
        self.tree("height_hash")
            .insert(height.to_be_bytes(), hash.0.to_vec())?;
        Ok(())
    }

    pub fn get_hash_by_height(&self, height: u64) -> Result<Option<Hash32>> {
        if let Some(value) = self.tree("height_hash").get(height.to_be_bytes())? {
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&value);
            Ok(Some(Hash32(hash)))
        } else {
            Ok(None)
        }
    }

    pub fn get_header(&self, height: u64) -> Result<Option<BlockHeader>> {
        if let Some(value) = self.tree("headers").get(height.to_be_bytes())? {
            let header = BlockHeader::try_from_slice(&value)?;
            Ok(Some(header))
        } else {
            Ok(None)
        }
    }

    pub fn set_tip(&self, height: u64, hash: &Hash32) -> Result<()> {
        let mut data = Vec::new();
        data.extend_from_slice(&height.to_be_bytes());
        data.extend_from_slice(&hash.0);
        self.tree("meta").insert(b"tip", data)?;
        Ok(())
    }

    pub fn get_tip(&self) -> Result<Option<(u64, Hash32)>> {
        if let Some(value) = self.tree("meta").get(b"tip")? {
            let height = u64::from_be_bytes(value[0..8].try_into().expect("height"));
            let mut hash = [0u8; 32];
            hash.copy_from_slice(&value[8..40]);
            Ok(Some((height, Hash32(hash))))
        } else {
            Ok(None)
        }
    }

    pub fn put_utxo(&self, outpoint: &OutPoint, output: &TxOut) -> Result<()> {
        self.tree("utxo")
            .insert(outpoint_key(outpoint), borsh::to_vec(output)?)?;
        Ok(())
    }

    pub fn get_utxo(&self, outpoint: &OutPoint) -> Result<Option<TxOut>> {
        if let Some(value) = self.tree("utxo").get(outpoint_key(outpoint))? {
            Ok(Some(TxOut::try_from_slice(&value)?))
        } else {
            Ok(None)
        }
    }

    pub fn delete_utxo(&self, outpoint: &OutPoint) -> Result<()> {
        self.tree("utxo").remove(outpoint_key(outpoint))?;
        Ok(())
    }

    pub fn list_utxos_by_address(&self, address: &[u8; 32]) -> Result<Vec<(OutPoint, TxOut)>> {
        let mut results = Vec::new();
        for item in self.tree("utxo").iter() {
            let (key, value) = item?;
            let outpoint = outpoint_from_key(&key);
            let output = TxOut::try_from_slice(&value)?;
            if &output.address == address {
                results.push((outpoint, output));
            }
        }
        Ok(results)
    }
}

fn outpoint_key(outpoint: &OutPoint) -> Vec<u8> {
    let mut key = Vec::with_capacity(36);
    key.extend_from_slice(&outpoint.txid.0);
    key.extend_from_slice(&outpoint.index.to_be_bytes());
    key
}

fn outpoint_from_key(key: &[u8]) -> OutPoint {
    let mut txid = [0u8; 32];
    txid.copy_from_slice(&key[..32]);
    let index = u32::from_be_bytes(key[32..36].try_into().expect("index"));
    OutPoint {
        txid: Hash32(txid),
        index,
    }
}
