use anyhow::{Context, Result};
use axum::http::StatusCode;
use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use borsh::BorshDeserialize;
use clap::Parser;
use kexa_consensus::{check_pow, merkle_root, COINBASE_MATURITY, DIFFICULTY_BITS, SUBSIDY};
use kexa_p2p::{encode_message, Message, MAX_MESSAGE_SIZE};
use kexa_proto::{
    tx_signing_hash, verify_tx_signature, Address, Block, BlockHeader, Hash32, OutPoint,
    Transaction, TxOut,
};
use kexa_storage::Storage;
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tracing::{error, info};

#[derive(Parser, Debug)]
#[command(name = "kexa-node")]
struct Args {
    #[arg(long, default_value = "127.0.0.1:8030")]
    rpc_addr: String,
    #[arg(long, default_value = "0.0.0.0:9030")]
    p2p_addr: String,
    #[arg(long, default_value = "./data")]
    data_dir: String,
    #[arg(long)]
    mine: bool,
    #[arg(long)]
    miner_address: Option<String>,
    #[arg(long, default_value = "")]
    peers: String,
}

#[derive(Clone)]
struct AppState {
    inner: Arc<Mutex<ChainState>>,
}

struct ChainState {
    storage: Storage,
    mempool: Vec<Transaction>,
    peers: Vec<String>,
}

#[derive(Serialize)]
struct TipResponse {
    height: u64,
    hash: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct MineRequest {
    count: u64,
    miner_address: String,
}

#[derive(Serialize)]
struct MineResponse {
    hashes: Vec<String>,
}

#[derive(Deserialize)]
struct SubmitRequest {
    tx: Transaction,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();

    let storage = Storage::open(&args.data_dir)?;
    init_genesis(&storage)?;
    let peers = if args.peers.is_empty() {
        Vec::new()
    } else {
        args.peers
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    let state = AppState {
        inner: Arc::new(Mutex::new(ChainState {
            storage,
            mempool: Vec::new(),
            peers,
        })),
    };

    let rpc_addr: SocketAddr = args.rpc_addr.parse()?;
    let p2p_addr: SocketAddr = args.p2p_addr.parse()?;

    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(err) = start_p2p_listener(state_clone, p2p_addr).await {
            error!("p2p listener failed: {err}");
        }
    });

    let peers_clone = state.clone();
    tokio::spawn(async move {
        connect_peers(peers_clone).await;
    });

    if args.mine {
        let miner_address = args
            .miner_address
            .context("--miner-address required when --mine")?;
        let miner_state = state.clone();
        tokio::spawn(async move {
            if let Err(err) = miner_loop(miner_state, miner_address).await {
                error!("miner failed: {err}");
            }
        });
    }

    let app = build_router(state);

    info!("rpc listening on {rpc_addr}");
    let listener = tokio::net::TcpListener::bind(rpc_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn ready(state: axum::extract::State<AppState>) -> Json<TipResponse> {
    let guard = state.inner.lock().await;
    let (height, hash) = guard.storage.get_tip().unwrap().expect("tip");
    Json(TipResponse {
        height,
        hash: hex::encode(hash.0),
    })
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/ready", get(ready))
        .route("/tip", get(get_tip))
        .route("/block/:hash", get(get_block))
        .route("/balance/:address", get(get_balance))
        .route("/utxos/:address", get(get_utxos))
        .route("/submit_tx", post(submit_tx))
        .route("/mine_blocks", post(mine_blocks))
        .route("/peers", get(get_peers))
        .with_state(state)
}

async fn get_tip(state: axum::extract::State<AppState>) -> Json<TipResponse> {
    let guard = state.inner.lock().await;
    let (height, hash) = guard.storage.get_tip().unwrap().expect("tip");
    Json(TipResponse {
        height,
        hash: hex::encode(hash.0),
    })
}

async fn get_block(
    Path(hash): Path<String>,
    state: axum::extract::State<AppState>,
) -> Result<Json<Block>, (StatusCode, Json<ErrorResponse>)> {
    let hash = parse_hash32(&hash)?;
    let guard = state.inner.lock().await;
    let block = guard
        .storage
        .get_block(&hash)
        .map_err(internal_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "block not found".to_string(),
                }),
            )
        })?;
    Ok(Json(block))
}

async fn get_balance(
    Path(address): Path<String>,
    state: axum::extract::State<AppState>,
) -> Result<Json<u64>, (StatusCode, Json<ErrorResponse>)> {
    let address = parse_address(&address)?;
    let guard = state.inner.lock().await;
    let utxos = guard
        .storage
        .list_utxos_by_address(&address.payload)
        .map_err(internal_error)?;
    let total: u64 = utxos.into_iter().map(|(_, out)| out.amount).sum();
    Ok(Json(total))
}

#[derive(Serialize)]
struct UtxoResponse {
    txid: String,
    index: u32,
    amount: u64,
}

async fn get_utxos(
    Path(address): Path<String>,
    state: axum::extract::State<AppState>,
) -> Result<Json<Vec<UtxoResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let address = parse_address(&address)?;
    let guard = state.inner.lock().await;
    let utxos = guard
        .storage
        .list_utxos_by_address(&address.payload)
        .map_err(internal_error)?;
    let response = utxos
        .into_iter()
        .map(|(outpoint, output)| UtxoResponse {
            txid: hex::encode(outpoint.txid.0),
            index: outpoint.index,
            amount: output.amount,
        })
        .collect();
    Ok(Json(response))
}

async fn submit_tx(
    state: axum::extract::State<AppState>,
    Json(req): Json<SubmitRequest>,
) -> Json<String> {
    let mut guard = state.inner.lock().await;
    if let Err(err) = validate_tx(&guard.storage, &guard.mempool, &req.tx) {
        return Json(format!("error: {err}"));
    }
    guard.mempool.push(req.tx.clone());
    Json(hex::encode(req.tx.txid().0))
}

async fn mine_blocks(
    state: axum::extract::State<AppState>,
    Json(req): Json<MineRequest>,
) -> Result<Json<MineResponse>, (StatusCode, Json<ErrorResponse>)> {
    let _ = parse_address(&req.miner_address).map_err(|_| bad_request("invalid miner address"))?;
    let mut hashes = Vec::new();
    let state = state.0;
    for _ in 0..req.count {
        let hash = mine_one_block(state.clone(), &req.miner_address)
            .await
            .map_err(|err| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: format!("mining failed: {err}"),
                    }),
                )
            })?;
        hashes.push(hex::encode(hash.0));
    }
    Ok(Json(MineResponse { hashes }))
}

async fn get_peers(state: axum::extract::State<AppState>) -> Json<Vec<String>> {
    let guard = state.inner.lock().await;
    Json(guard.peers.clone())
}

fn parse_hash32(hash: &str) -> Result<Hash32, (StatusCode, Json<ErrorResponse>)> {
    let bytes = hex::decode(hash).map_err(|_| bad_request("invalid hash"))?;
    if bytes.len() != 32 {
        return Err(bad_request("invalid hash"));
    }
    let mut h = [0u8; 32];
    h.copy_from_slice(&bytes);
    Ok(Hash32(h))
}

fn parse_address(address: &str) -> Result<Address, (StatusCode, Json<ErrorResponse>)> {
    Address::from_bech32(address).map_err(|_| bad_request("invalid address"))
}

fn bad_request(message: &str) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: message.to_string(),
        }),
    )
}

fn internal_error(err: anyhow::Error) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse {
            error: format!("internal error: {err}"),
        }),
    )
}

fn now_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_secs()
}

fn init_genesis(storage: &Storage) -> Result<()> {
    if storage.get_tip()?.is_some() {
        return Ok(());
    }
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
        timestamp: now_timestamp(),
        bits: DIFFICULTY_BITS,
        nonce: 0,
        height: 0,
    };
    let block = Block {
        header,
        txs: vec![coinbase],
    };
    let hash = block.header.hash();
    storage.put_block(&hash, &block)?;
    storage.put_header(0, &block.header)?;
    storage.put_height_hash(0, &hash)?;
    storage.set_tip(0, &hash)?;
    Ok(())
}

async fn miner_loop(state: AppState, miner_address: String) -> Result<()> {
    loop {
        mine_one_block(state.clone(), &miner_address).await?;
    }
}

async fn mine_one_block(state: AppState, miner_address: &str) -> Result<Hash32> {
    let address = Address::from_bech32(miner_address)?;
    let (height, prev_hash, mempool) = {
        let mut guard = state.inner.lock().await;
        let (height, prev_hash) = guard.storage.get_tip()?.expect("tip");
        let mempool = guard.mempool.drain(..).collect::<Vec<_>>();
        (height, prev_hash, mempool)
    };
    let mut fee_total = 0u64;
    for tx in &mempool {
        fee_total = fee_total.saturating_add(calculate_fee(state.clone(), tx).await?);
    }
    let coinbase = Transaction {
        version: 0,
        inputs: vec![],
        outputs: vec![TxOut {
            amount: SUBSIDY.saturating_add(fee_total),
            address: address.payload,
        }],
    };
    let mut txs = Vec::with_capacity(1 + mempool.len());
    txs.push(coinbase);
    txs.extend(mempool);

    let merkle = merkle_root(&txs);
    let mut header = BlockHeader {
        version: 0,
        prev_hash,
        merkle_root: merkle,
        timestamp: now_timestamp(),
        bits: DIFFICULTY_BITS,
        nonce: 0,
        height: height + 1,
    };

    loop {
        if check_pow(&header) {
            break;
        }
        header.nonce = header.nonce.wrapping_add(1);
    }

    let block = Block {
        header: header.clone(),
        txs,
    };
    apply_block(state.clone(), block.clone()).await?;
    sync_with_peers(state.clone()).await?;
    Ok(block.header.hash())
}

async fn calculate_fee(state: AppState, tx: &Transaction) -> Result<u64> {
    let guard = state.inner.lock().await;
    let mut input_sum = 0u64;
    for input in &tx.inputs {
        if let Some(utxo) = guard.storage.get_utxo(&input.outpoint)? {
            input_sum = input_sum.saturating_add(utxo.amount);
        }
    }
    let output_sum: u64 = tx.outputs.iter().map(|o| o.amount).sum();
    Ok(input_sum.saturating_sub(output_sum))
}

async fn apply_block(state: AppState, block: Block) -> Result<()> {
    let guard = state.inner.lock().await;
    validate_block(&guard.storage, &block)?;
    for (idx, tx) in block.txs.iter().enumerate() {
        if idx == 0 {
            if !tx.inputs.is_empty() {
                anyhow::bail!("coinbase has inputs");
            }
        } else {
            for input in &tx.inputs {
                if guard.storage.get_utxo(&input.outpoint)?.is_none() {
                    anyhow::bail!("missing utxo when applying block");
                }
                guard.storage.delete_utxo(&input.outpoint)?;
            }
        }
        for (index, output) in tx.outputs.iter().enumerate() {
            let outpoint = OutPoint {
                txid: tx.txid(),
                index: index as u32,
            };
            guard.storage.put_utxo(&outpoint, output)?;
        }
    }
    let hash = block.header.hash();
    guard.storage.put_block(&hash, &block)?;
    guard
        .storage
        .put_header(block.header.height, &block.header)?;
    guard.storage.put_height_hash(block.header.height, &hash)?;
    guard.storage.set_tip(block.header.height, &hash)?;
    Ok(())
}

fn validate_tx(storage: &Storage, mempool: &[Transaction], tx: &Transaction) -> Result<()> {
    if tx.inputs.is_empty() {
        anyhow::bail!("non-coinbase tx must have inputs");
    }
    let mut input_sum = 0u64;
    let mut seen = std::collections::HashSet::new();
    for input in &tx.inputs {
        if !seen.insert((input.outpoint.txid.0, input.outpoint.index)) {
            anyhow::bail!("double spend in tx");
        }
        let utxo = storage.get_utxo(&input.outpoint)?.context("missing utxo")?;
        input_sum = input_sum.saturating_add(utxo.amount);
        let input_address = Address::from_pubkey_bytes(&input.pubkey).context("invalid pubkey")?;
        if input_address.payload != utxo.address {
            anyhow::bail!("pubkey does not match utxo address");
        }
        let tx_hash = tx_signing_hash(tx);
        if !verify_tx_signature(&input.pubkey, &input.signature, &tx_hash.0) {
            anyhow::bail!("invalid signature");
        }
    }

    for existing in mempool {
        for input in &existing.inputs {
            for new_input in &tx.inputs {
                if input.outpoint.txid == new_input.outpoint.txid
                    && input.outpoint.index == new_input.outpoint.index
                {
                    anyhow::bail!("double spend in mempool");
                }
            }
        }
    }

    let output_sum: u64 = tx.outputs.iter().map(|o| o.amount).sum();
    if output_sum > input_sum {
        anyhow::bail!("outputs exceed inputs");
    }
    Ok(())
}

fn validate_block(storage: &Storage, block: &Block) -> Result<()> {
    if block.txs.is_empty() {
        anyhow::bail!("block empty");
    }
    if block.header.height == 0 {
        if let Some((tip_height, tip_hash)) = storage.get_tip()? {
            if tip_height == 0 && tip_hash == block.header.hash() {
                return Ok(());
            }
            anyhow::bail!("unexpected genesis block");
        }
        return Ok(());
    }
    let (tip_height, tip_hash) = storage.get_tip()?.context("tip missing")?;
    if block.header.height != tip_height + 1 {
        anyhow::bail!("unexpected height");
    }
    if block.header.prev_hash != tip_hash {
        anyhow::bail!("prev hash mismatch");
    }
    if merkle_root(&block.txs) != block.header.merkle_root {
        anyhow::bail!("merkle mismatch");
    }
    if !check_pow(&block.header) {
        anyhow::bail!("pow invalid");
    }
    if COINBASE_MATURITY != 0 {
        anyhow::bail!("coinbase maturity not supported in v0");
    }
    let coinbase = &block.txs[0];
    if !coinbase.inputs.is_empty() {
        anyhow::bail!("coinbase inputs present");
    }
    let mut spent_in_block = std::collections::HashSet::new();
    let coinbase_total: u64 = coinbase.outputs.iter().map(|o| o.amount).sum();
    let mut total_fees = 0u64;
    for (idx, tx) in block.txs.iter().enumerate() {
        if idx != 0 {
            for input in &tx.inputs {
                if !spent_in_block.insert((input.outpoint.txid.0, input.outpoint.index)) {
                    anyhow::bail!("intra-block double spend");
                }
            }
            validate_tx(storage, &[], tx)?;
            total_fees = total_fees.saturating_add(tx_fee(storage, tx)?);
        }
    }
    let max_reward = SUBSIDY.saturating_add(total_fees);
    if coinbase_total > max_reward {
        anyhow::bail!("coinbase exceeds subsidy+fees");
    }
    Ok(())
}

fn tx_fee(storage: &Storage, tx: &Transaction) -> Result<u64> {
    let mut input_sum = 0u64;
    for input in &tx.inputs {
        let utxo = storage.get_utxo(&input.outpoint)?.context("missing utxo")?;
        input_sum = input_sum.saturating_add(utxo.amount);
    }
    let output_sum: u64 = tx.outputs.iter().map(|o| o.amount).sum();
    Ok(input_sum.saturating_sub(output_sum))
}

async fn start_p2p_listener(state: AppState, addr: SocketAddr) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("p2p listening on {addr}");
    loop {
        let (stream, _) = listener.accept().await?;
        let peer_state = state.clone();
        tokio::spawn(async move {
            if let Err(err) = handle_peer(peer_state, stream).await {
                error!("peer error: {err}");
            }
        });
    }
}

async fn connect_peers(state: AppState) {
    loop {
        if let Err(err) = sync_with_peers(state.clone()).await {
            error!("peer sync error: {err}");
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}

async fn handle_peer(state: AppState, mut stream: TcpStream) -> Result<()> {
    let (height, tip) = {
        let guard = state.inner.lock().await;
        guard.storage.get_tip()?.expect("tip")
    };
    let version = Message::Version { height, tip };
    let data = encode_message(&version)?;
    stream.write_all(&data).await?;

    loop {
        let mut len_bytes = [0u8; 4];
        if stream.read_exact(&mut len_bytes).await.is_err() {
            break;
        }
        let len = u32::from_be_bytes(len_bytes) as usize;
        if len > MAX_MESSAGE_SIZE {
            anyhow::bail!("message too large");
        }
        let mut payload = vec![0u8; len];
        stream.read_exact(&mut payload).await?;
        let message = Message::try_from_slice(&payload)?;
        match message {
            Message::Version {
                height: peer_height,
                tip: _peer_tip,
            }
            | Message::Tip {
                height: peer_height,
                tip: _peer_tip,
            } => {
                let (local_height, local_tip) = {
                    let guard = state.inner.lock().await;
                    guard.storage.get_tip()?.expect("tip")
                };
                if peer_height > local_height {
                    let msg = Message::GetBlocks {
                        start_height: local_height + 1,
                    };
                    let data = encode_message(&msg)?;
                    stream.write_all(&data).await?;
                } else if peer_height < local_height {
                    let msg = Message::Tip {
                        height: local_height,
                        tip: local_tip,
                    };
                    let data = encode_message(&msg)?;
                    stream.write_all(&data).await?;
                }
            }
            Message::GetBlocks { start_height } => {
                let tip_height = {
                    let guard = state.inner.lock().await;
                    guard.storage.get_tip()?.expect("tip").0
                };
                for height in start_height..=tip_height {
                    let hash = {
                        let guard = state.inner.lock().await;
                        guard.storage.get_hash_by_height(height)?.expect("hash")
                    };
                    let block = {
                        let guard = state.inner.lock().await;
                        guard.storage.get_block(&hash)?.expect("block")
                    };
                    let msg = Message::Block { block };
                    let data = encode_message(&msg)?;
                    stream.write_all(&data).await?;
                }
            }
            Message::Block { block } => {
                apply_block(state.clone(), block).await?;
            }
            Message::GetTip => {
                let (height, tip) = {
                    let guard = state.inner.lock().await;
                    guard.storage.get_tip()?.expect("tip")
                };
                let msg = Message::Tip { height, tip };
                let data = encode_message(&msg)?;
                stream.write_all(&data).await?;
            }
            Message::GetBlock { hash } => {
                let block = {
                    let guard = state.inner.lock().await;
                    guard.storage.get_block(&hash)?
                };
                if let Some(block) = block {
                    let msg = Message::Block { block };
                    let data = encode_message(&msg)?;
                    stream.write_all(&data).await?;
                }
            }
        }
    }
    Ok(())
}

async fn sync_with_peers(state: AppState) -> Result<()> {
    let peers = {
        let guard = state.inner.lock().await;
        guard.peers.clone()
    };
    for peer in peers {
        if let Ok(stream) = TcpStream::connect(&peer).await {
            let peer_state = state.clone();
            tokio::spawn(async move {
                if let Err(err) = handle_peer(peer_state, stream).await {
                    error!("peer error: {err}");
                }
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use ed25519_dalek::SigningKey;
    use http_body_util::BodyExt;
    use kexa_proto::TxIn;
    use rand::rngs::OsRng;
    use std::fs;
    use tower::ServiceExt;

    fn temp_storage() -> Storage {
        let mut path = std::env::temp_dir();
        path.push(format!("kexa-node-test-{}", rand::random::<u64>()));
        let _ = fs::create_dir_all(&path);
        Storage::open(path.to_str().expect("path")).expect("storage")
    }

    fn test_state() -> AppState {
        let storage = temp_storage();
        init_genesis(&storage).expect("genesis");
        AppState {
            inner: Arc::new(Mutex::new(ChainState {
                storage,
                mempool: Vec::new(),
                peers: Vec::new(),
            })),
        }
    }

    #[test]
    fn rejects_wrong_pubkey_for_utxo() {
        let storage = temp_storage();
        let alice = SigningKey::generate(&mut OsRng);
        let bob = SigningKey::generate(&mut OsRng);

        let outpoint = OutPoint {
            txid: Hash32([9u8; 32]),
            index: 0,
        };
        let output = TxOut {
            amount: 50,
            address: Address::from_pubkey(&alice.verifying_key()).payload,
        };
        storage.put_utxo(&outpoint, &output).expect("utxo");

        let mut tx = Transaction {
            version: 0,
            inputs: vec![TxIn {
                outpoint,
                signature: [0u8; 64],
                pubkey: bob.verifying_key().to_bytes(),
            }],
            outputs: vec![TxOut {
                amount: 49,
                address: Address::from_pubkey(&bob.verifying_key()).payload,
            }],
        };
        let signing_hash = tx_signing_hash(&tx);
        tx.inputs[0].signature = kexa_proto::sign_tx(&bob, &signing_hash.0);

        let err = validate_tx(&storage, &[], &tx).unwrap_err();
        assert!(err.to_string().contains("pubkey does not match utxo"));
    }

    #[test]
    fn rejects_coinbase_overpay() {
        let storage = temp_storage();
        init_genesis(&storage).expect("genesis");
        let (height, prev_hash) = storage.get_tip().expect("tip").expect("tip");

        let coinbase = Transaction {
            version: 0,
            inputs: vec![],
            outputs: vec![TxOut {
                amount: SUBSIDY + 1,
                address: [1u8; 32],
            }],
        };
        let merkle = merkle_root(std::slice::from_ref(&coinbase));
        let mut header = BlockHeader {
            version: 0,
            prev_hash,
            merkle_root: merkle,
            timestamp: now_timestamp(),
            bits: DIFFICULTY_BITS,
            nonce: 0,
            height: height + 1,
        };
        while !check_pow(&header) {
            header.nonce = header.nonce.wrapping_add(1);
        }
        let block = Block {
            header,
            txs: vec![coinbase],
        };
        let err = validate_block(&storage, &block).unwrap_err();
        assert!(err.to_string().contains("coinbase exceeds subsidy"));
    }

    #[tokio::test]
    async fn rejects_unexpected_height_zero_block() {
        let storage = temp_storage();
        init_genesis(&storage).expect("genesis");
        let (tip_height, tip_hash) = storage.get_tip().expect("tip").expect("tip");

        let coinbase = Transaction {
            version: 0,
            inputs: vec![],
            outputs: vec![TxOut {
                amount: SUBSIDY,
                address: [2u8; 32],
            }],
        };
        let merkle = merkle_root(std::slice::from_ref(&coinbase));
        let block = Block {
            header: BlockHeader {
                version: 0,
                prev_hash: Hash32::zero(),
                merkle_root: merkle,
                timestamp: now_timestamp(),
                bits: DIFFICULTY_BITS,
                nonce: 0,
                height: 0,
            },
            txs: vec![coinbase],
        };

        let state = AppState {
            inner: Arc::new(Mutex::new(ChainState {
                storage,
                mempool: Vec::new(),
                peers: Vec::new(),
            })),
        };

        let err = apply_block(state.clone(), block).await.unwrap_err();
        assert!(err.to_string().contains("unexpected genesis block"));
        let (new_height, new_hash) = {
            let guard = state.inner.lock().await;
            guard.storage.get_tip().expect("tip").expect("tip")
        };
        assert_eq!(new_height, tip_height);
        assert_eq!(new_hash, tip_hash);
    }

    #[test]
    fn rejects_intra_block_double_spend() {
        let storage = temp_storage();
        init_genesis(&storage).expect("genesis");
        let (height, prev_hash) = storage.get_tip().expect("tip").expect("tip");
        let key = SigningKey::generate(&mut OsRng);
        let outpoint = OutPoint {
            txid: Hash32([5u8; 32]),
            index: 0,
        };
        let output = TxOut {
            amount: 50,
            address: Address::from_pubkey(&key.verifying_key()).payload,
        };
        storage.put_utxo(&outpoint, &output).expect("utxo");

        let mut tx1 = Transaction {
            version: 0,
            inputs: vec![TxIn {
                outpoint: outpoint.clone(),
                signature: [0u8; 64],
                pubkey: key.verifying_key().to_bytes(),
            }],
            outputs: vec![TxOut {
                amount: 40,
                address: Address::from_pubkey(&key.verifying_key()).payload,
            }],
        };
        let signing_hash = tx_signing_hash(&tx1);
        tx1.inputs[0].signature = kexa_proto::sign_tx(&key, &signing_hash.0);

        let mut tx2 = Transaction {
            version: 0,
            inputs: vec![TxIn {
                outpoint: outpoint.clone(),
                signature: [0u8; 64],
                pubkey: key.verifying_key().to_bytes(),
            }],
            outputs: vec![TxOut {
                amount: 30,
                address: Address::from_pubkey(&key.verifying_key()).payload,
            }],
        };
        let signing_hash = tx_signing_hash(&tx2);
        tx2.inputs[0].signature = kexa_proto::sign_tx(&key, &signing_hash.0);

        let coinbase = Transaction {
            version: 0,
            inputs: vec![],
            outputs: vec![TxOut {
                amount: SUBSIDY,
                address: [9u8; 32],
            }],
        };
        let txs = vec![coinbase, tx1, tx2];
        let merkle = merkle_root(&txs);
        let mut header = BlockHeader {
            version: 0,
            prev_hash,
            merkle_root: merkle,
            timestamp: now_timestamp(),
            bits: DIFFICULTY_BITS,
            nonce: 0,
            height: height + 1,
        };
        while !check_pow(&header) {
            header.nonce = header.nonce.wrapping_add(1);
        }
        let block = Block { header, txs };
        let err = validate_block(&storage, &block).unwrap_err();
        assert!(err.to_string().contains("intra-block double spend"));
    }

    #[tokio::test]
    async fn invalid_hash_returns_4xx() {
        let app = build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/block/not-hex")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("invalid hash"));
    }

    #[tokio::test]
    async fn invalid_address_returns_4xx() {
        let app = build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/balance/not-an-address")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("invalid address"));
    }

    #[tokio::test]
    async fn invalid_miner_address_returns_4xx() {
        let app = build_router(test_state());
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/mine_blocks")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"count":1,"miner_address":"not-an-address"}"#,
                    ))
                    .unwrap(),
            )
            .await
            .expect("response");
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(body.to_vec()).unwrap();
        assert!(body.contains("invalid miner address"));
    }
}
