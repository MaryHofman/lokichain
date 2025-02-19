use anyhow::Error;
use crate::application::common::acc_storage::AccStorage;
use crate::application::common::app_router::AppRouter;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::interactor::Interactor;
use crate::application::common::mempool::MemPool;
use crate::application::common::signer::Signer;
use crate::application::common::tx_storage::TxStorage;
use crate::domain::models::address::Address;
use crate::domain::models::app_data::AppData;
use crate::domain::models::hash::Hash;
use crate::domain::models::signature::Signature;
use crate::domain::models::token::Token;
use crate::domain::models::transaction::Transaction;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct GetTransactionByHashRequest {
    pub hash: Hash,
}

pub struct GetTransactionByHash<'a> {
    tx_storage: &'a dyn TxStorage,
}

#[async_trait]
impl Interactor<GetTransactionByHashRequest, Transaction> for GetTransactionByHash<'_> {
    async fn execute(
        &self,
        data: GetTransactionByHashRequest
    ) -> Result<Transaction, ApplicationError> {

        todo!("Implement get transaction by hash");

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::tx_storage::tests::MockTxStorage;
    use crate::domain::models::address::Address;
    use crate::domain::models::app_data::AppData;
    use crate::domain::models::hash::Hash;
    use crate::domain::models::signature::{Signature, VerifyKey};
    use crate::domain::models::token::Token;
    use crate::domain::models::transaction::{Transaction, TxState};

    fn make_transaction_stub() -> Transaction {
        Transaction::new(
            Hash([0; 32]),
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
        )
    }

    #[tokio::test]
    async fn test_get_transaction_by_hash_ok() {
        let tx_storage = MockTxStorage::new();
        let tx = make_transaction_stub();
        tx_storage.set(tx.hash.clone(), tx.clone()).await;


        let interactor = GetTransactionByHash {
            tx_storage: &tx_storage
        };
        let request = GetTransactionByHashRequest {
            hash: tx.hash.clone()
        };
        let result = interactor.execute(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().hash, tx.hash);
    }

    #[tokio::test]
    async fn test_get_transaction_by_hash_not_found() {
        let interactor = GetTransactionByHash {
            tx_storage: &MockTxStorage::new()
        };
        let request = GetTransactionByHashRequest {
            hash: Hash([1; 32])
        };
        let result = interactor.execute(request).await;
        assert!(result.is_err());
        assert!(match result.err().unwrap() {
            ApplicationError::NotFound(msg) => msg == "Transaction not found",
            _ => panic!("Unexpected error")
        });
    }
}
