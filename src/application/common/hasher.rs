use async_trait::async_trait;


#[async_trait]
pub trait Hasher {
    async fn hash(&self, value: &str) -> [u8];
    async fn verify(&self, value: &str, hash: &[u8]) -> bool;
}
