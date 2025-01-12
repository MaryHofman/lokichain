use crate::domain::models::account::Account;
use crate::domain::models::address::Address;
use async_trait::async_trait;

#[async_trait]
pub trait AccStorage: Send + Sync {
    async fn get(&self, key: &Address) -> Option<Account>;
    async fn set(&self, key: Address, acc: Account);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::domain::models::token::Token;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use crate::domain::models::signature::VerifyKey;

    pub struct MockAccStorage {
        accounts: Arc<RwLock<HashMap<Address, Account>>>
    }

    impl MockAccStorage {
        pub fn new() -> Self {
            MockAccStorage {
                accounts: Arc::new(RwLock::new(HashMap::new()))
            }
        }
    }

    #[async_trait]
    impl AccStorage for MockAccStorage {
        async fn get(&self, key: &Address) -> Option<Account> {
            self.accounts.read().await.get(key).cloned()
        }

        async fn set(&self, key: Address, acc: Account) {
            self.accounts.write().await.insert(key, acc);
        }
    }

    #[tokio::test]
    async fn test_acc_storage() {
        let storage = MockAccStorage {
            accounts: Arc::new(RwLock::new(HashMap::new()))
        };

        let address = Address {
            network: "lokichain".to_string(),
            vk: VerifyKey([0; 32])
        };

        let account = Account {
            address: address.clone(),
            nonce: 0,
            balance: Token {
                value: 0,
                denom: "LOKI".to_string()
            }
        };

        storage.set(address.clone(), account.clone()).await;
        let result = storage.get(&address).await.unwrap();
        assert_eq!(account, result);
    }
}