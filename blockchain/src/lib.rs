use std::collections::HashMap;
use sha2::{Digest, Sha256};
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: f64,
    pub peg_value: String,
    pub network: String,
    pub fee: f64,
    pub signature: Option<String>,
    pub status: String,
    pub txid: String,
    pub timestamp: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64, peg_value: String, network: String, fee: f64) -> Self {
        let mut hasher = Sha256::new();
        let nonce: u64 = rand::thread_rng().random();
        let data = format!("{}{}{}{}{}{}{}", from, to, amount, peg_value, network, fee, nonce);
        hasher.update(data.as_bytes());
        let txid = format!("{:x}", hasher.finalize());
        Transaction {
            from,
            to,
            amount,
            peg_value,
            network,
            fee,
            signature: None,
            status: "pending".to_string(),
            txid,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.amount <= 0.0 || self.fee < 0.0 {
            Err("Invalid amount or fee".to_string())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    balances: HashMap<String, HashMap<String, f64>>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut balances = load_wallet_balances();
        if balances.is_empty() || !balances.contains_key("genesis") {
            balances.insert("genesis".to_string(), HashMap::from([("SOL".to_string(), 100.0)]));
            if let Err(e) = save_wallet_balances(&balances) {
                eprintln!("Failed to save initial balances: {}", e);
            }
        }
        Wallet { balances }
    }

    pub fn get_balance(&self, address: &str, network: &str) -> f64 {
        self.balances.get(address).and_then(|n| n.get(network)).copied().unwrap_or(0.0)
    }

    pub fn update_balance(&mut self, address: &str, network: &str, amount: f64) {
        let entry = self.balances.entry(address.to_string()).or_insert(HashMap::new());
        *entry.entry(network.to_string()).or_insert(0.0) += amount;
        if let Err(e) = save_wallet_balances(&self.balances) {
            eprintln!("Error saving balances: {}", e);
        }
    }
}

fn load_wallet_balances() -> HashMap<String, HashMap<String, f64>> {
    match File::open("wallet_balances.json") {
        Ok(mut file) => {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                serde_json::from_str(&contents).unwrap_or_default()
            } else {
                HashMap::new()
            }
        }
        Err(_) => HashMap::new(),
    }
}

fn save_wallet_balances(balances: &HashMap<String, HashMap<String, f64>>) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(balances)?;
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open("wallet_balances.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub mod mining;
pub mod network;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub target_block_time: u64,
    pub wallet: Wallet,
    pub history: Vec<Transaction>,
    pub network_fees: HashMap<String, f64>,
}

impl Blockchain {
    pub fn new() -> Self {
        let blockchain = Blockchain {
            chain: vec![],
            difficulty: 2,
            target_block_time: 10,
            wallet: Wallet::new(),
            history: vec![],
            network_fees: HashMap::from([
                ("SOL".to_string(), 0.1),
                ("BTC".to_string(), 0.001),
                ("ETH".to_string(), 0.01),
                ("BNB".to_string(), 2.0),
            ]),
        };
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().map(|b| b.hash.clone()).unwrap_or_else(|| String::from("0"));
        let mut block = Block::new(self.chain.len() as u64, transactions.clone(), previous_hash);
        block = crate::mining::mine_block(block.index as u32, block.previous_hash.clone(), block.transactions.clone(), self.difficulty);
        self.chain.push(block);

        if let Some(last_block) = self.chain.last_mut() {
            for tx in &mut last_block.transactions {
                if tx.status == "pending" {
                    tx.status = "berhasil".to_string();
                }
            }
        }

        for block in &self.chain {
            for tx in &block.transactions {
                if !self.history.iter().any(|h| h.txid == tx.txid) {
                    self.history.push(tx.clone());
                    if tx.status == "berhasil" {
                        let fee = self.network_fees.get(&tx.network).unwrap_or(&0.0);
                        self.wallet.update_balance(&tx.from, &tx.network, -(tx.amount + *fee));
                        self.wallet.update_balance(&tx.to, &tx.network, tx.amount);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp() as u64;
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.calculate_hash();
        block
    }

    pub fn calculate_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{:?}{}", self.index, self.transactions, self.previous_hash).as_bytes());
        self.hash = format!("{:x}", hasher.finalize());
    }
}