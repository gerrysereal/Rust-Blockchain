pub mod block;
pub mod network;
pub mod mining;

pub use block::Block;
pub use network::{add_transaction, get_wallet, get_history};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Wallet {
    balances: HashMap<String, f64>,
}

impl Wallet {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("genesis".to_string(), 1000.0);
        Wallet { balances }
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        *self.balances.get(address).unwrap_or(&0.0)
    }

    pub fn update_balance(&mut self, address: &str, amount: f64) {
        let current = self.get_balance(address);
        self.balances.insert(address.to_string(), current + amount);
    }
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub target_block_time: u64,
    pub wallet: Wallet,
    pub history: Vec<transaction::Transaction>, // add history transaction
}

impl Blockchain {
    pub fn new() -> Self {
        let wallet = Wallet::new();
        let tx = transaction::Transaction::new(
            "genesis".to_string(),
            "genesis".to_string(),
            0.0,
            "0USD".to_string(),
            "BTC".to_string(),
            0.0,
        );
        let genesis = Block::new(0, vec![tx.clone()], String::from("0"));
        let mut blockchain = Blockchain {
            chain: vec![genesis],
            difficulty: 2,
            target_block_time: 10,
            wallet,
            history: vec![tx], // Inisialisasi
        };
        // Sinkronisasi
        for block in &blockchain.chain {
            for tx in &block.transactions {
                blockchain.wallet.update_balance(&tx.sender, -tx.amount - tx.fee);
                blockchain.wallet.update_balance(&tx.receiver, tx.amount);
            }
        }
        blockchain
    }

    pub fn add_block(&mut self, transactions: Vec<transaction::Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions.clone(), previous_hash);
        crate::mining::mine_block(block.index, block.previous_hash.clone(), block.transactions.clone(), self.difficulty);
        block.calculate_hash();
        self.chain.push(block);

        for block in &self.chain {
            for tx in &block.transactions {
                self.wallet.update_balance(&tx.sender, -tx.amount - tx.fee);
                self.wallet.update_balance(&tx.receiver, tx.amount);
                self.history.push(tx.clone()); // add history
                println!("Updating balance: {} -> {}, {} -> {}", tx.sender, -tx.amount - tx.fee, tx.receiver, tx.amount);
            }
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}