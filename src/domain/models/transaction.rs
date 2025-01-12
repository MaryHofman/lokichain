use chrono::Utc;
use crate::domain::models::address::Address;
use crate::domain::models::hash::Hash;
use crate::domain::models::signature::Signature;
use crate::domain::models::token::Token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    /// Transaction hash (not including signature)
    pub hash: Hash,
    /// Address of the sender
    pub sender: Address,
    /// Data field (for smart contract calls or arbitrary logic)
    pub data: Vec<u8>,
    /// Amount of tokens to transfer (if applicable)
    pub amount: Token,
    /// Timestamp of the transaction
    pub timestamp: u64,
    /// The commission (gas) value is calculated as *gas_price* * *gas_limit*
    ///
    /// * gas_price - the price of calculation, depends on the network load
    /// * gas_limit - the maximum amount of "gas" that the user is willing
    /// to spend to perform a transaction
    ///
    /// The more gas, the faster the transaction will be included in the block
    pub gas: u64,
    /// Sender transaction number
    ///
    /// This number is stored in the state. It is important to keep the transactions
    /// in order and make them unique to protect against replay
    pub nonce: u64,
    /// Signature of the transaction
    pub signature: Signature
}

impl Transaction {
    pub fn new(
        hash: Hash,
        sender: Address,
        data: Vec<u8>,
        amount: Token,
        gas: u64,
        nonce: u64,
        signature: Signature
    ) -> Self {
        Transaction {
            hash,
            sender,
            data,
            amount,
            timestamp: Utc::now().timestamp() as u64,
            gas,
            nonce,
            signature
        }
    }
}
