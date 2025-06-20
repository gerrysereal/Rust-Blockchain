// blockchain/src/mining.rs
use crate::Block;
use transaction::Transaction;
use sha2::{Digest, Sha256};

pub fn calculate_hash(index: u64, timestamp: u64, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp.to_string());
    for tx in transactions {
        hasher.update(format!("{}", tx.amount));
        hasher.update(format!("{}", tx.fee));
        hasher.update(tx.sender.as_bytes());
        hasher.update(tx.receiver.as_bytes());
        hasher.update(tx.network.as_bytes());
        hasher.update(tx.peg_value.as_bytes());
        hasher.update(tx.status.as_bytes());
        hasher.update(tx.txid.as_bytes());
        hasher.update(tx.date.as_bytes());
    }
    hasher.update(previous_hash);
    hasher.update(nonce.to_string());
    format!("{:x}", hasher.finalize())
}

pub fn mine_block(index: u64, previous_hash: String, mut transactions: Vec<Transaction>, difficulty: usize) -> Block {
    let reward_tx = Transaction::new(
        "network".to_string(),
        "miner_address".to_string(),
        50.0,
        "50USD".to_string(),
        "BTC".to_string(), // Network default
        0.0,
    );
    transactions.push(reward_tx);

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut nonce = 0;

    loop {
        let hash = calculate_hash(index, timestamp, &transactions, &previous_hash, nonce);
        if hash.starts_with(&"0".repeat(difficulty)) {
            println!("Block mined: {}", hash);
            return Block {
                index,
                timestamp,
                transactions,
                previous_hash,
                hash,
                nonce,
            };
        }
        nonce += 1;
    }
}