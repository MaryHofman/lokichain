use async_trait::async_trait;
use crate::domain::models::hash::Hash;

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash(&self, value: &[u8]) -> Hash;
    async fn verify(&self, value: &[u8], hash: &Hash) -> bool;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::domain::models::hash::Hash;

    pub struct MockHasher;

    #[async_trait]
    impl Hasher for MockHasher {
        async fn hash(&self, value: &[u8]) -> Hash {
            let mut hash = [0u8; 32];
            if value.len() >= 32 {
                hash.copy_from_slice(&value[..32]);
            } else {
                hash[..value.len()].copy_from_slice(value);
            }
            Hash(hash)
        }

        async fn verify(&self, value: &[u8], hash: &Hash) -> bool {
            self.hash(value).await == *hash
        }
    }

    #[tokio::test]
    async fn test_hasher() {
        let hasher = MockHasher;
        let value = "hello world";
        let hash = hasher.hash(value.as_bytes()).await;
        assert!(hasher.verify(value.as_bytes(), &hash).await);
    }
}