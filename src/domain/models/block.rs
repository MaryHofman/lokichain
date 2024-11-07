use super::transaction::Transaction;


pub struct BlockHeader {
    version: u32,
    height: u32,
    timestamp: i64,
    pre_hash: String,
    merkle_root: String,
    difficulty: u32,
    nonce: u32,
}

pub struct Block {
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}
