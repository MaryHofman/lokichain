use crate::domain::models::transaction::Transaction;
use crate::domain::models::block::Block;

pub struct BlockChain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}