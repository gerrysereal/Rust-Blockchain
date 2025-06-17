use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub balance: u64,
    pub address: String,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            balance: 100,
            address: "default_wallet_address".to_string(),
        }
    }

    pub fn can_transfer(&self, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            Ok(())
        } else {
            Err("Insufficient balance".to_string())
        }
    }

    pub fn address(&self) -> Option<String> {
        Some(self.address.clone())
    }
}