use sha2::{Sha256, Digest};


pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u64,
    pub signature: Option<String>,
    pub fee: f64,
    pub nonce: u64,
    pub status: TransactionStatus,
}

