use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: u64) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        }
    }
}
