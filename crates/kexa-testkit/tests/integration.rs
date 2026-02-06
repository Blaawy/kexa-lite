use anyhow::Result;
use ed25519_dalek::SigningKey;
use kexa_proto::{sign_tx, tx_signing_hash, Address, OutPoint, Transaction, TxIn, TxOut};
use rand::rngs::OsRng;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{collections::HashSet, fs, path::PathBuf, time::Duration};

#[derive(Deserialize)]
struct UtxoResponse {
    txid: String,
    index: u32,
    amount: u64,
}

#[derive(Deserialize)]
struct TipResponse {
    height: u64,
    hash: String,
}

fn temp_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!("kexa-{}-{}", name, rand::random::<u64>()));
    fs::create_dir_all(&path).expect("temp dir");
    path
}

fn pick_unique_port(used: &mut HashSet<u16>) -> Result<u16> {
    for _ in 0..100 {
        let p = kexa_testkit::pick_free_port()?;
        if used.insert(p) {
            return Ok(p);
        }
    }
    anyhow::bail!("could not pick unique free port");
}
#[tokio::test]
async fn devnet_flow_two_nodes() -> Result<()> {
    let bin = kexa_testkit::build_node_binary()?;
    let node1_dir = temp_dir("node1");
    let node2_dir = temp_dir("node2");
    let mut used = HashSet::new();
    let rpc1 = pick_unique_port(&mut used)?;
    let rpc2 = pick_unique_port(&mut used)?;
    let p2p1 = pick_unique_port(&mut used)?;
    let p2p2 = pick_unique_port(&mut used)?;
    let node1 = kexa_testkit::spawn_node(
        &bin,
        rpc1,
        p2p1,
        node1_dir.to_str().unwrap(),
        "",
        false,
        None,
    )?;
    let node2 = kexa_testkit::spawn_node(
        &bin,
        rpc2,
        p2p2,
        node2_dir.to_str().unwrap(),
        &format!("127.0.0.1:{}", p2p1),
        false,
        None,
    )?;

    let url1 = format!("http://127.0.0.1:{}", rpc1);
    let url2 = format!("http://127.0.0.1:{}", rpc2);
    kexa_testkit::wait_for_ready(&url1).await?;
    kexa_testkit::wait_for_ready(&url2).await?;

    let alice_key = SigningKey::generate(&mut OsRng);
    let bob_key = SigningKey::generate(&mut OsRng);
    let alice_addr = Address::from_pubkey(&alice_key.verifying_key()).to_bech32();
    let bob_addr = Address::from_pubkey(&bob_key.verifying_key()).to_bech32();

    let client = reqwest::Client::builder().no_proxy().build()?;
    client
        .post(format!("{}/mine_blocks", url1))
        .json(&serde_json::json!({"count": 1, "miner_address": alice_addr}))
        .send()
        .await?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    let balance: u64 = get_json(
        &client,
        &format!("{}/balance/{}", url1, alice_addr),
        "balance",
    )
    .await?;
    assert!(balance > 0);

    let utxos: Vec<UtxoResponse> =
        get_json(&client, &format!("{}/utxos/{}", url1, alice_addr), "utxos").await?;
    let utxo = utxos.first().expect("utxo");
    let mut txid = [0u8; 32];
    txid.copy_from_slice(&hex::decode(&utxo.txid)?);
    let outpoint = OutPoint {
        txid: kexa_proto::Hash32(txid),
        index: utxo.index,
    };
    let mut tx = Transaction {
        version: 0,
        inputs: vec![TxIn {
            outpoint,
            signature: [0u8; 64],
            pubkey: alice_key.verifying_key().to_bytes(),
        }],
        outputs: vec![
            TxOut {
                amount: 10,
                address: Address::from_bech32(&bob_addr)?.payload,
            },
            TxOut {
                amount: utxo.amount - 11,
                address: Address::from_bech32(&alice_addr)?.payload,
            },
        ],
    };
    let signing_hash = tx_signing_hash(&tx);
    tx.inputs[0].signature = sign_tx(&alice_key, &signing_hash.0);

    client
        .post(format!("{}/submit_tx", url1))
        .json(&serde_json::json!({"tx": tx}))
        .send()
        .await?;

    client
        .post(format!("{}/mine_blocks", url1))
        .json(&serde_json::json!({"count": 1, "miner_address": alice_addr}))
        .send()
        .await?;

    tokio::time::sleep(Duration::from_millis(500)).await;

    let balance_after: u64 = get_json(
        &client,
        &format!("{}/balance/{}", url1, bob_addr),
        "balance_after",
    )
    .await?;
    assert_eq!(balance_after, 10);

    let tip2 = wait_for_tip(&client, &format!("{}/tip", url2), 1).await?;
    assert!(!tip2.hash.is_empty());

    drop(node1);
    drop(node2);
    Ok(())
}

async fn get_json<T: DeserializeOwned>(
    client: &reqwest::Client,
    url: &str,
    label: &str,
) -> Result<T> {
    let response = client.get(url).send().await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        anyhow::bail!("{} failed {}: {}", label, status, body);
    }
    let value = serde_json::from_str(&body)
        .map_err(|err| anyhow::anyhow!("{} parse error: {}", label, err))?;
    Ok(value)
}

async fn wait_for_tip(client: &reqwest::Client, url: &str, min_height: u64) -> Result<TipResponse> {
    for _ in 0..20 {
        let tip: TipResponse = get_json(client, url, "tip2").await?;
        if tip.height >= min_height {
            return Ok(tip);
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
    anyhow::bail!("tip did not reach height {}", min_height);
}
