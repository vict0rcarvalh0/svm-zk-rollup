use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub balance: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    pub balances: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        let mut balances = HashMap::new();
        balances.insert("alice".to_string(), 100); 
        balances.insert("bob".to_string(), 50); 

        Self { balances }
    }

    pub fn apply_transaction(&mut self, sender: &str, receiver: &str, amount: u64) -> bool {
        let sender_balance = self.balances.get(sender).copied().unwrap_or(0);

        if sender_balance < amount {
            return false;
        }

        *self.balances.entry(sender.to_string()).or_insert(0) -= amount;
        *self.balances.entry(receiver.to_string()).or_insert(0) += amount;

        true
    }
}