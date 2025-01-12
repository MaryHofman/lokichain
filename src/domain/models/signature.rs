use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Signature 64 bytes
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Signature(pub [u8; 64]);

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD_NO_PAD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = STANDARD_NO_PAD.decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 64 {
            return Err(serde::de::Error::custom("Signature must be 64 bytes"));
        }
        let mut array = [0u8; 64];
        array.copy_from_slice(&bytes);
        Ok(Signature(array))
    }
}

/// SignKey 32 bytes
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignKey(pub [u8; 32]);

impl Serialize for SignKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD_NO_PAD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for SignKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = STANDARD_NO_PAD.decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("SignKey must be 32 bytes"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(SignKey(array))
    }
}


/// VerifyKey 32 bytes
#[derive(Clone, Debug, Eq, PartialEq, core::hash::Hash)]
pub struct VerifyKey(pub [u8; 32]);

impl Serialize for VerifyKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD_NO_PAD.encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for VerifyKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = STANDARD_NO_PAD.decode(&s).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom("VerifyKey must be 32 bytes"));
        }
        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(VerifyKey(array))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;

    #[test]
    fn serialize_deserialize_signature() {
        let original_signature = Signature([(); 64].map(|_| random()));

        let serialized = serde_json::to_string(&original_signature).unwrap();
        let deserialized_signature: Signature = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_signature, deserialized_signature);

        let new_signature = Signature([(); 64].map(|_| random()));
        assert_ne!(new_signature, deserialized_signature)
    }

    #[test]
    fn serialize_deserialize_signkey() {
        let original_signkey = SignKey([(); 32].map(|_| random()));

        let serialized = serde_json::to_string(&original_signkey).unwrap();
        let deserialized_signkey: SignKey = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_signkey, deserialized_signkey);

        let new_signkey = SignKey([(); 32].map(|_| random()));
        assert_ne!(new_signkey, deserialized_signkey)
    }

    #[test]
    fn serialize_deserialize_verifykey() {
        let original_verifykey = VerifyKey([(); 32].map(|_| random()));

        let serialized = serde_json::to_string(&original_verifykey).unwrap();
        let deserialized_verifykey: VerifyKey = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original_verifykey, deserialized_verifykey);

        let new_verifykey = VerifyKey([(); 32].map(|_| random()));
        assert_ne!(new_verifykey, deserialized_verifykey)
    }
}
