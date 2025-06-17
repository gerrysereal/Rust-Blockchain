use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub peg_value: f64, // Stablecoin simulation
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: u64, peg_value: f64) -> Self {
        Transaction { sender, receiver, amount, peg_value }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.amount == 0 {
            return Err("Amount cannot be zero".to_string());
        }
        Ok(())
    }
}