use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Hash 32 bytes
#[derive(Clone, Debug, Eq, PartialEq, core::hash::Hash)]
pub struct Hash(pub [u8; 32]);

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let base64_string = STANDARD_NO_PAD.encode(&self.0);
        serializer.serialize_str(&base64_string)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = STANDARD_NO_PAD.decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("Hash must be 32 bytes"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Hash(array))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn serialize_deserialize_hash() {
        let original_hash = Hash(random::<[u8; 32]>());

        let serialized = serde_json::to_string(&original_hash).unwrap();
        let deserialized_hash: Hash = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_hash, deserialized_hash);

        let new_hash = Hash(random::<[u8; 32]>());
        assert_ne!(new_hash, deserialized_hash)
    }
}
