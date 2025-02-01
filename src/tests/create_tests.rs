#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::app_router;
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
    use mockall::predicate;
  

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


    #[tokio::test]
    async fn test_create_transaction_invalid_hash() {
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
        let expected_hash = MockHasher.hash(&bytes).await;
        transaction.hash = Hash([1; 32]); 

        let result = interactor.execute(transaction).await;
        assert!(result.is_err());

        if let Err(ApplicationError::InvalidData(ref errors)) = result {
            assert_eq!(errors.get("hash").unwrap(), "hash is not valid");
        }
    }

    #[tokio::test]
    async fn test_create_transaction_invalid_signature() {
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
        let correct_signature = signer.sign(&transaction.hash.0, &pk).await;
        transaction.signature = Signature([1; 64]); 
        let result = interactor.execute(transaction).await;
        assert!(result.is_err());

        if let Err(ApplicationError::InvalidData(ref errors)) = result {
            assert_eq!(errors.get("signature").unwrap(), "signature is not valid");
        }
    }



    
    #[tokio::test]
    async fn test_is_exist_condition() {
       
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
            vk,
        };
    
        acc_storage.set(
            address.clone(),
            Account {
                address: address.clone(),
                nonce: 0,
                balance: Token { value: 100, denom: "LOKI".to_string() },
            }
        ).await;
    
        let app_name = "test_app".to_string();
        let operation_name = "test_op".to_string();
    
        let transaction = CreateTransactionRequest {
            body: TxBody {
                sender: address.clone(),
                amount: Token { value: 10, denom: "LOKI".to_string() },
                gas: 10,
                nonce: 0,
                data: AppData {
                    app: app_name.clone(), 
                    operation: operation_name.clone(),  
                    payload: Value::Null,  
                },
            },
            hash: Hash([0; 32]),
            signature: Signature([0; 64]),
        };
    
       
        let result = interactor.execute(transaction).await;

        assert!(result.is_err(), "AppRouter should confirm that app and operation exist");
    }
    

    
    #[tokio::test]
    async fn test_exist_transaction_mempool() {
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

        let created_transaction = Transaction::new(
            transaction.hash.clone(),  
            transaction.body.sender.clone(),
            transaction.body.data.clone(),
            transaction.body.amount.clone(),
            transaction.body.gas,
            transaction.body.nonce,
            transaction.signature.clone(),
        );

        let transactionWithState = TransactionWithState::new(&created_transaction, TxState::Confirmed);

        interactor.mempool.add(transactionWithState).await;

        let result = interactor.execute(transaction).await;
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_gas() {
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
                gas: 0,
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
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_amount_value() {
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

        transaction.hash = MockHasher.hash(&bytes).await;
        transaction.signature = signer.sign(&transaction.hash.0, &pk).await;

    
        let result = interactor.execute(transaction).await;
        assert!(result.is_err());
    }




    #[tokio::test]
    async fn test_insufficient_balance() {
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
                balance: Token { value: 10, denom: "LOKI".to_string() }
            }
        ).await;

        let mut transaction = CreateTransactionRequest {
            body: TxBody {
                sender: address.clone(),
                amount: Token { value: 100, denom: "LOKI".to_string() },
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
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_denom() {
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
                amount: Token { value: 10, denom: "Bitcoin".to_string() },
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
        assert!(result.is_err());
    }


    #[tokio::test]
    async fn test_account_not_found() {
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
        assert!(result.is_err());
    }
}