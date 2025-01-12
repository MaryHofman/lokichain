use crate::domain::models::address::Address;
use crate::domain::models::hash::Hash;
use crate::domain::models::transaction::Transaction;
use async_trait::async_trait;

#[async_trait]
pub trait TxStorage: Send + Sync {
    async fn get(&self, key: &Hash) -> Option<Transaction>;
    async fn set(&self, key: Hash, acc: Transaction);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::domain::models::app_data::AppData;
    use crate::domain::models::signature::{Signature, VerifyKey};
    use crate::domain::models::token::Token;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub struct MockTxStorage {
        txs: Arc<RwLock<HashMap<Hash, Transaction>>>
    }

    impl MockTxStorage {
        pub fn new() -> Self {
            Self {
                txs: Arc::new(RwLock::new(HashMap::new()))
            }
        }
    }

    #[async_trait]
    impl TxStorage for MockTxStorage {
        async fn get(&self, key: &Hash) -> Option<Transaction> {
            self.txs.read().await.get(key).cloned()
        }

        async fn set(&self, key: Hash, acc: Transaction) {
            self.txs.write().await.insert(key, acc);
        }
    }

    #[tokio::test]
    async fn test_acc_storage() {
        let storage = MockTxStorage::new();

        let hash = Hash([0; 32]);

        let tx = Transaction::new(
            hash.clone(),
            Address {
                network: "lokichain".to_string(),
                vk: VerifyKey([1; 32])
            },
            AppData {
                app: "bank".to_string(),
                operation: "transfer".to_string(),
                payload: serde_json::Value::Null
            },
            Token { value: 100, denom: "LOKI".to_string() },
            10,
            0,
            Signature([0; 64])
        );
        storage.set(hash.clone(), tx).await;
        let result = storage.get(&hash).await.unwrap();
        assert_eq!(hash, result.hash);
    }
}
