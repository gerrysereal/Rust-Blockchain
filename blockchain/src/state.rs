use std::collections::HashMap;

pub struct State {
    balances: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("Genesis".to_string(), 1000); // Initial balance for genesis
        State { balances }
    }

    pub fn update_balance(&mut self, address: &str, amount: u64) {
        *self.balances.entry(address.to_string()).or_insert(0) += amount;
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }
}