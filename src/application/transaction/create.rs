use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::interactor::Interactor;
use crate::application::common::mempool::MemPool;
use crate::domain::models::address::Address;
use crate::domain::models::app_data::AppData;
use crate::domain::models::hash::Hash;
use crate::domain::models::signature::Signature;
use crate::domain::models::token::Token;
use crate::domain::models::transaction::Transaction;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use crate::application::common::app_router::AppRouter;
use crate::application::common::signer::Signer;

#[derive(Debug, Deserialize, Serialize)]
pub struct TxBody {
    pub sender: Address,
    pub data: AppData,
    pub amount: Token,
    pub gas: u64,
    pub nonce: u64
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub body: TxBody,
    pub hash: Hash,
    pub signature: Signature
}

#[derive(Debug, Serialize)]
pub struct CreateTransactionResult {
    pub hash: Hash
}

pub struct CreateTransaction<'a> {
    hasher: &'a dyn Hasher,
    mempool: &'a dyn MemPool,
    app_router: &'a dyn AppRouter,
    signer: &'a dyn Signer
}

#[async_trait]
impl Interactor<CreateTransactionRequest, CreateTransactionResult> for CreateTransaction<'_> {
    async fn execute(&self, data: CreateTransactionRequest) -> Result<CreateTransactionResult, ApplicationError> {
        let hash = {
            let mut bytes = vec![];
            bytes.extend_from_slice(&serde_json::to_vec(&data.body).unwrap());
            self.hasher.hash(&bytes).await
        };

        if hash != data.hash {
            return Err(ApplicationError::InvalidData([("hash".to_string(), "hash is not valid".to_string())].into()));
        }

        let is_valid = self.signer.verify(&data.hash.0, &data.signature, &data.body.sender.hash).await;

        let transaction = Transaction::new(
            data.hash,
            data.body.sender,
            data.body.data,
            data.body.amount,
            data.body.gas,
            data.body.nonce,
            data.signature
        );

        self.mempool.add(transaction).await;

        Ok(CreateTransactionResult { hash })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::address::Address;
    use crate::domain::models::hash::Hash;
    use crate::domain::models::signature::Signature;
    use crate::domain::models::token::Token;
    use serde_json::Value;

    use crate::application::common::app_router::tests::MockAppRouter;
    use crate::application::common::hasher::tests::MockHasher;
    use crate::application::common::mempool::tests::MockMemPool;
    use crate::domain::models::app_data::AppData;

    #[tokio::test]
    async fn test_create_transaction_ok() {
        let interactor = CreateTransaction {
            hasher: &MockHasher,
            mempool: &MockMemPool::new(),
            app_router: &MockAppRouter
        };

        let mut transaction = CreateTransactionRequest {
            body: TxBody {
                sender: Address { network: "lokichain".to_string(), hash: vec![] },
                amount: Token { value: 0, denom: "LOKI".to_string() },
                gas: 10,
                nonce: 0,
                data: AppData { app: "bank".to_string(), operation: "transfer".to_string(), payload: Value::Null },
            },
            hash: Hash([0; 32]),
            signature: Signature([0; 64])
        };

        let mut bytes = vec![];
        bytes.extend_from_slice(&serde_json::to_vec(&transaction.body).unwrap());

        let hash = MockHasher.hash(&bytes).await;
        transaction.hash = hash;

        let result = interactor.execute(transaction).await;
        assert!(result.is_ok());
    }
}
