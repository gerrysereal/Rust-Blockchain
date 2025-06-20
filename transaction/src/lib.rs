use serde::{Serialize, Deserialize};
use chrono::{Utc, FixedOffset};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub peg_value: String,
    pub network: String,
    pub fee: f64,
    pub status: String,
    pub txid: String,
    pub date: String,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64, peg_value: String, network: String, fee: f64) -> Self {
        let txid = Uuid::new_v4().to_string();
        let date = Utc::now()
            .with_timezone(&FixedOffset::east_opt(7 * 3600).unwrap())
            .format("%Y-%m-%d %H:%M:%S WIB")
            .to_string();
        Transaction {
            sender,
            receiver,
            amount,
            peg_value,
            network,
            fee,
            status: "pending".to_string(),
            txid,
            date,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        }
        if self.fee <= 0.0 {
            return Err("Fee must be greater than zero".to_string());
        }

        let supported_networks = ["BTC", "ETH", "SOL", "BNB", "ADA", "XRP", "DOT", "MATIC", "USDT"];
        let network_trim = self.network.trim();
        if !supported_networks.contains(&network_trim) {
            return Err(format!("Unsupported network: {}. Supported: {:?}", network_trim, supported_networks));
        }

        let peg_value_trim = self.peg_value.trim().to_uppercase();
        if !peg_value_trim.ends_with("IDR") && !peg_value_trim.ends_with("USD") && !peg_value_trim.ends_with("USDT") {
            return Err(format!("Unsupported peg value unit: {}. Use IDR, USD, or USDT.", peg_value_trim));
        }

        Ok(())
    }
}
