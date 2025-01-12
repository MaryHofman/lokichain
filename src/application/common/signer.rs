use crate::domain::models::signature::{SignKey, Signature, VerifyKey};
use async_trait::async_trait;

#[async_trait]
pub trait Signer: Send + Sync {
    async fn sign(&self, data: &[u8], sign_key: &SignKey) -> Signature;
    async fn verify(&self, data: &[u8], signature: &Signature, verify_key: &VerifyKey) -> bool;
}

#[cfg(test)]
pub mod tests {
    use super::*;


    /// Simple mock signer implementation
    ///
    /// For verification to work correctly, `[SignKey]` must be equal to `[VerifyKey]`
    pub struct MockSigner;

    #[async_trait]
    impl Signer for MockSigner {
        async fn sign(&self, data: &[u8], sign_key: &SignKey) -> Signature {
            let mut signature = [0u8; 64];
            let data_len = std::cmp::min(32, data.len());
            signature[..data_len].copy_from_slice(&data[..data_len]);
            signature[32..].copy_from_slice(&sign_key.0);
            Signature(signature)
        }

        async fn verify(&self, data: &[u8], signature: &Signature, verify_key: &VerifyKey) -> bool {
            let check_sign = self.sign(data, &SignKey(verify_key.0)).await;
            check_sign == *signature
        }
    }

    #[tokio::test]
    async fn test_signer() {
        let signer = MockSigner;
        let data = b"hello world";
        let sign_key = SignKey([0u8; 32]);
        let verify_key = VerifyKey([0u8; 32]);
        let signature = signer.sign(data, &sign_key).await;
        assert!(signer.verify(data, &signature, &verify_key).await);
    }
}
