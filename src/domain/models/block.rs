use chrono::Utc;

use super::transaction::Transaction;
use crate::domain::models::address::Address;
use crate::domain::models::hash::Hash;
use crate::domain::models::signature::Signature;
use crate::domain::models::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockHeader {
    /// block hash
    hash: Hash,
    /// Block number
    height: u64,
    /// Time of creation
    timestamp: u64,
    /// hash of the previous block
    pre_hash: Hash,
    /// merkle root transaction hash
    merkle_root: Hash,
    /// The address of the validator that issued this block
    validator: Address,
    /// the reward this validator received for issuing a block
    reward: Token,
    /// validator signature
    signature: Signature
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>
}

#[allow(dead_code)]
impl Block {
    pub fn new(
        hash: Hash,
        height: u64,
        pre_hash: Hash,
        merkle_root: Hash,
        validator: Address,
        reward: Token,
        sign: Signature,
        transactions: Vec<Transaction>
    ) -> Self {
        Block {
            header: BlockHeader {
                hash,
                height,
                pre_hash,
                merkle_root,
                validator,
                reward,
                signature: sign,
                timestamp: Utc::now().timestamp() as u64,
            },
            transactions
        }
    }
}