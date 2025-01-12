use crate::domain::models::address::Address;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use crate::domain::models::token::Token;

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Account {
    /// account address
    pub address: Address,
    /// last transaction number
    pub nonce: u64,
    /// account balance
    pub balance: Token,
}
