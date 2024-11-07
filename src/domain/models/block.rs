use crate::domain::models::transaction::Transaction;
use crate::domain::models::block_header::BlockHeader;

pub struct Block{
    header: BlockHeader,
    count: u32,
    transactions: Vec<Transaction>
}