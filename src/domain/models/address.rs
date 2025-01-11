use bech32::{Bech32m, Hrp};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};


#[derive(Clone)]
pub struct RawAddress {
    /// network tag; ex: lokichain
    pub network: String,
    /// 44 for bip-44
    pub purpose: u32,
    /// 0 for Bitcoin, 60 for Ethereum
    pub coin_type: u32,
    /// Number of account
    ///
    /// Each user can have multiple accounts
    pub account: u32,
    /// 0 for external addresses, 1 for internal
    pub change: u32,
    /// Address index
    ///
    /// Each account can have multiple addresses of a specific currency,
    /// for example Alice can have 3 bitcoin addresses
    pub address_index: u32
}


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Address {
    /// network tag; ex: lokichain
    pub network: String,
    /// Public key hash
    pub hash: Vec<u8>
}


impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hrp = Hrp::parse(&self.network).map_err(|_| std::fmt::Error)?;
        let encoded = bech32::encode::<Bech32m>(hrp, &self.hash).map_err(serde::ser::Error::custom)?;
        write!(f, "{}", encoded)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (hrp, hash) = bech32::decode(&s).map_err(serde::de::Error::custom)?;
        Ok(Address { network: hrp.to_string(), hash })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_address() {
        let original_address = Address {
            network: "lokichain".to_string(),
            hash: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
        };

        let serialized = serde_json::to_string(&original_address).unwrap();
        let deserialized_address: Address = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_address, deserialized_address);

        let new_address = Address {
            network: "lokichain".to_string(),
            hash: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        };
        assert_ne!(new_address, deserialized_address);
    }
}