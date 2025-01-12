use async_trait::async_trait;
use crate::domain::models::hash::Hash;
use crate::domain::models::transaction::Transaction;

#[async_trait]
pub trait MemPool: Send + Sync {
    async fn add(&self, transaction: Transaction);
    async fn get(&self, hash: &Hash) -> Option<Transaction>;
    async fn release(&self, limit: usize) -> Vec<Transaction>;
    async fn count(&self) -> usize;
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use crate::domain::models::address::Address;
    use crate::domain::models::app_data::AppData;
    use crate::domain::models::signature::{Signature, VerifyKey};
    use crate::domain::models::token::Token;
    use super::*;

    pub struct MockMemPool {
        transactions: Arc<RwLock<HashMap<Hash, Transaction>>>
    }

    impl MockMemPool {
        pub fn new() -> Self {
            MockMemPool { transactions: Arc::new(RwLock::new(HashMap::new())) }
        }
    }

    #[async_trait]
    impl MemPool for MockMemPool {
        async fn add(&self, transaction: Transaction) {
            self.transactions.write().await.insert(transaction.hash.clone(), transaction);
        }

        async fn get(&self, hash: &Hash) -> Option<Transaction> {
            self.transactions.read().await.get(hash).cloned()
        }

        async fn release(&self, limit: usize) -> Vec<Transaction> {
            let mut transactions = Vec::with_capacity(limit);
            let mut tx_guard = self.transactions.write().await;

            let hashes_to_remove: Vec<Hash> = tx_guard
                .keys()
                .take(limit)
                .cloned()
                .collect();

            for hash in hashes_to_remove {
                if let Some(transaction) = tx_guard.remove(&hash) {
                    transactions.push(transaction);
                }
            }

            transactions
        }

        async fn count(&self) -> usize {
            self.transactions.read().await.len()
        }
    }

    #[tokio::test]
    async fn test_mempool() {
        let mempool = MockMemPool { transactions: Arc::new(RwLock::new(HashMap::new())) };
        let hash = Hash([0u8; 32]);
        let transaction = Transaction::new(
            hash.clone(),
            Address {
                network: "lokichain".to_string(),
                vk: VerifyKey([0; 32])
            },
            AppData {
                app: "bank".to_string(),
                operation: "transfer".to_string(),
                payload: serde_json::Value::Null
            },
            Token {
                value: 10,
                denom: "LOKI".to_string()
            },
            10,
            0,
            Signature([0u8; 64])
        );

        mempool.add(transaction.clone()).await;
        assert_eq!(mempool.count().await, 1);
        assert_eq!(mempool.get(&hash).await.unwrap().hash.clone(), hash.clone());
        mempool.release(1).await;
        assert_eq!(mempool.count().await, 0);
    }
}