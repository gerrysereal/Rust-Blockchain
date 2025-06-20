pub mod block;
pub mod network;
pub mod mining;

pub use block::Block;
pub use network::add_transaction;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
    pub target_block_time: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let tx = transaction::Transaction::new(
            "genesis".to_string(),
            "genesis".to_string(),
            0.0,
            "0USD".to_string(),
            "BTC".to_string(), // Network default
            0.0,
        );
        let genesis = Block::new(0, vec![tx], String::from("0"));
        Blockchain {
            chain: vec![genesis],
            difficulty: 2,
            target_block_time: 10,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<transaction::Transaction>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);
        crate::mining::mine_block(block.index, block.previous_hash.clone(), block.transactions.clone(), self.difficulty);
        block.calculate_hash();
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
