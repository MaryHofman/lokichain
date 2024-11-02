use transaction::Transaction;

#[derive(Debug, Serrialize)]
struct BlockHeader{
    timestump: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serrialize)]
pub struct Block{
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}