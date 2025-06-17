use wallet::Wallet;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

pub fn create_transaction(wallet: &Wallet, to: String, amount: u64, _fee: f64) -> (String, String, u64) {
    (wallet.address.clone(), to, amount)
}