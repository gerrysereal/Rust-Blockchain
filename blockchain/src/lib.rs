use serde::{Serialize, Deserialize};

pub mod block;
pub mod network;

pub use block::Block; // Export Block dari modul block
pub use network::start_server; // Export start_server dari modul network

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub target_block_time: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, vec![Transaction {
            from: "genesis".to_string(),
            to: "genesis".to_string(),
            amount: 0,
        }], String::from("0"));
        Blockchain {
            chain: vec![genesis],
            difficulty: 4,
            target_block_time: 10,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        self.chain.push(block);
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