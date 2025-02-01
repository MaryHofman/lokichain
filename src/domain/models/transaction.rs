use chrono::Utc;
use crate::domain::models::address::Address;
use crate::domain::models::hash::Hash;
use crate::domain::models::signature::Signature;
use crate::domain::models::token::Token;
use serde::{Deserialize, Serialize};
use crate::domain::models::app_data::AppData;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TxState {
    Confirmed,           // Транзакция подтверждена
    Reverted,            // Транзакция отклонена
    PendingConfirmation, // Ожидает подтверждения
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub hash: Hash,
    pub sender: Address,
    pub data: AppData,
    pub amount: Token,
    pub timestamp: u64,
    pub gas: u64,
    pub nonce: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        hash: Hash,
        sender: Address,
        data: AppData,
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


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionWithState {
    pub transaction: Transaction,
    pub state: TxState,
}

impl TransactionWithState {
    pub fn new(transaction: &Transaction, state: TxState) -> Self {
        TransactionWithState {
            transaction: transaction.clone(), 
            state,
        }
    }
}