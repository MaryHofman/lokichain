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
use crate::application::common::acc_storage::AccStorage;
use crate::application::common::app_router::AppRouter;
use crate::application::common::signer::Signer;
use crate::domain::models::account::Account;

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
    signer: &'a dyn Signer,
    acc_storage: &'a dyn AccStorage
}

#[async_trait]
impl Interactor<CreateTransactionRequest, CreateTransactionResult> for CreateTransaction<'_> {
    async fn execute(
        &self,
        data: CreateTransactionRequest
    ) -> Result<CreateTransactionResult, ApplicationError> {
        let hash = {
            let mut bytes = vec![];
            bytes.extend_from_slice(&serde_json::to_vec(&data.body).unwrap());
            self.hasher.hash(&bytes).await
        };

        if hash != data.hash {
            return Err(ApplicationError::InvalidData(
                [("hash".to_string(), "hash is not valid".to_string())].into()
            ));
        }

        if !self.signer.verify(&data.hash.0, &data.signature, &data.body.sender.vk).await {
            return Err(ApplicationError::InvalidData(
                [("signature".to_string(), "signature is not valid".to_string())].into()
            ));
        }

        if !self.app_router.is_exist(&data.body.data.app, &data.body.data.operation).await {
            return Err(ApplicationError::InvalidData(
                [("body.data".to_string(), "is not valid".to_string())].into()
            ));
        }

        if self.mempool.get(&data.hash).await.is_some() {
            return Err(ApplicationError::InvalidData(
                [("hash".to_string(), "tx is already exist".to_string())].into()
            ));
        }

        if data.body.gas == 0 {
            return Err(ApplicationError::InvalidData(
                [("body.gas".to_string(), "gas must be greater than 0".to_string())].into()
            ));
        }

        if data.body.amount.value == 0 {
            return Err(ApplicationError::InvalidData(
                [("body.amount".to_string(), "amount must be greater than 0".to_string())].into()
            ));
        }

        if let Some(acc) = self.acc_storage.get(&data.body.sender).await {
            if acc.balance.value < data.body.amount.value {
                return Err(ApplicationError::InvalidData(
                    [("body.sender".to_string(), "you dont have coins".to_string())].into()
                ));
            }

            if acc.balance.denom != data.body.amount.denom {
                return Err(ApplicationError::InvalidData(
                    [("body.amount".to_string(), "denom is not valid".to_string())].into()
                ));
            }

        } else {
            return Err(ApplicationError::InvalidData(
                [("body.sender".to_string(), "you dont have coins".to_string())].into()
            ));
        }

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
    use crate::domain::models::signature::{SignKey, Signature, VerifyKey};
    use crate::domain::models::token::Token;
    use serde_json::Value;
    use crate::application::common::acc_storage::tests::MockAccStorage;
    use crate::application::common::app_router::tests::MockAppRouter;
    use crate::application::common::hasher::tests::MockHasher;
    use crate::application::common::mempool::tests::MockMemPool;
    use crate::application::common::signer::tests::MockSigner;
    use crate::domain::models::app_data::AppData;

    #[tokio::test]
    async fn test_create_transaction_ok() {
        let acc_storage = MockAccStorage::new();
        let signer = MockSigner;
        let hasher = MockHasher;
        let interactor = CreateTransaction {
            hasher: &hasher,
            mempool: &MockMemPool::new(),
            app_router: &MockAppRouter,
            signer: &signer,
            acc_storage: &acc_storage,
        };

        let pk = SignKey([1; 32]);
        let vk = VerifyKey([1; 32]);

        let address = Address {
            network: "lokichain".to_string(),
            vk
        };

        acc_storage.set(
            address.clone(),
            Account {
                address: address.clone(),
                nonce: 0,
                balance: Token { value: 100, denom: "LOKI".to_string() }
            }
        ).await;

        let mut transaction = CreateTransactionRequest {
            body: TxBody {
                sender: address.clone(),
                amount: Token { value: 10, denom: "LOKI".to_string() },
                gas: 10,
                nonce: 0,
                data: AppData { app: "bank".to_string(), operation: "transfer".to_string(), payload: Value::Null },
            },
            hash: Hash([0; 32]),
            signature: Signature([0; 64])
        };

        let mut bytes = vec![];
        bytes.extend_from_slice(&serde_json::to_vec(&transaction.body).unwrap());

        transaction.hash = MockHasher.hash(&bytes).await;
        transaction.signature = signer.sign(&transaction.hash.0, &pk).await;

        let result = interactor.execute(transaction).await;
        assert!(result.is_ok());
    }
}
