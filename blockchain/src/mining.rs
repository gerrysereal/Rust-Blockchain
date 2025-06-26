use sha2::{Digest, Sha256};
use crate::Block;
use crate::Transaction;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;
use std::fs::{File, OpenOptions};
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};

// Tambah struct buat mining stats
#[derive(Serialize, Deserialize, Debug)]
struct MiningStats {
    block_index: u64,
    timestamp: u64,
    nonce: u64,
    hash: String,
    mining_time_sec: u64,
    difficulty: usize,
    estimated_hashrate: f64,
}

pub fn calculate_hash(index: u32, timestamp: i64, transactions: &Vec<Transaction>, previous_hash: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(index.to_string());
    hasher.update(timestamp.to_string());
    for tx in transactions {
        hasher.update(format!("{:?}", tx));
    }
    hasher.update(previous_hash);
    hasher.update(nonce.to_string());
    format!("{:x}", hasher.finalize())
}

fn load_mining_stats() -> Vec<MiningStats> {
    match File::open("mining_stats.json") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                serde_json::from_str(&contents).unwrap_or_default()
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}

fn save_mining_stats(stats: &[MiningStats]) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(stats)?;
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("mining_stats.json")?;
    file.write_all(json.as_bytes())?;
    println!("Mining stats saved to mining_stats.json"); // Tambah logging
    Ok(())
}

use crate::{load_wallet_balances, save_wallet_balances};

pub fn mine_block(index: u32, previous_hash: String, mut transactions: Vec<Transaction>, difficulty: usize) -> Block {
    let base_reward = 50.0;
    let halving_interval = 100_000;
    let halvings = index / halving_interval;
    let adjusted_reward = base_reward / 2f64.powf(halvings as f64);

    let mut network_rewards: HashMap<String, f64> = HashMap::new();
    for tx in &transactions {
        *network_rewards.entry(tx.network.clone()).or_insert(0.0) += tx.fee * 0.5;
    }

    let peg_values: HashMap<String, String> = network_rewards.iter()
        .map(|(k, v)| (k.clone(), format!("{} {}", v + adjusted_reward, k)))
        .collect();

    for (network, reward_total) in network_rewards.iter() {
        transactions.push(Transaction {
            from: "network".to_string(),
            to: "miner_address".to_string(),
            amount: reward_total + adjusted_reward,
            peg_value: peg_values.get(network).unwrap().clone(),
            network: network.clone(),
            fee: 0.0,
            signature: None,
            status: "pending".to_string(),
            txid: format!("reward_{}_{}", network, Utc::now().timestamp()),
            timestamp: Utc::now().timestamp() as u64,
        });
    }

    let timestamp = Utc::now().timestamp();
    let found = Arc::new(Mutex::new(None));
    let threads: Vec<_> = (0..4).map(|i| {
        let found = Arc::clone(&found);
        let txs = transactions.clone();
        let prev_hash = previous_hash.clone();
        thread::spawn(move || {
            let mut local_nonce = i * 1_000_000;
            loop {
                if found.lock().unwrap().is_some() {
                    break;
                }
                let hash = calculate_hash(index, timestamp, &txs, &prev_hash, local_nonce);
                if hash.starts_with(&"0".repeat(difficulty)) {
                    *found.lock().unwrap() = Some((hash, local_nonce));
                    break;
                }
                local_nonce += 1;
            }
        })
    }).collect();

    for t in threads {
        t.join().unwrap();
    }

    let start = SystemTime::now();
    let (hash, nonce) = found.lock().unwrap().clone().unwrap();
    let duration = start.elapsed().unwrap().as_secs();

    let stats = MiningStats {
        block_index: index as u64,
        timestamp: timestamp as u64,
        nonce,
        hash: hash.clone(),
        mining_time_sec: duration,
        difficulty,
        estimated_hashrate: nonce as f64 / duration as f64,
    };

    let mut mining_stats = load_mining_stats();
    mining_stats.push(stats);
    save_mining_stats(&mining_stats).unwrap_or_else(|e| eprintln!("Failed to save mining stats: {}", e));

    let mut balances = load_wallet_balances();
    for tx in &transactions {
        if tx.to == "miner_address" {
            let entry = balances.entry("miner_address".to_string()).or_insert(HashMap::new());
            *entry.entry(tx.network.clone()).or_insert(0.0) += tx.amount;
        }
    }
    save_wallet_balances(&balances).unwrap_or_else(|e| eprintln!("Failed to save wallet balances: {}", e));

    println!("Block mined: {} (nonce: {}, time: {}s)", hash, nonce, duration);
    Block {
        index: index as u64,
        timestamp: timestamp as u64,
        transactions,
        previous_hash,
        nonce,
        hash,
    }
}