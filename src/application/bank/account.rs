use crate::application::common::acc_storage::AccStorage;
use crate::application::common::app_router::AppRouter;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::interactor::Interactor;
use crate::application::common::mempool::MemPool;
use crate::application::common::signer::Signer;
use crate::application::common::tx_storage::TxStorage;
use crate::domain::models::account::Account;
use crate::domain::models::address::Address;
use crate::domain::models::token::Token;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetAccountRequest {
    pub address: Address
}

pub struct GetAccount<'a> {
    acc_storage: &'a dyn AccStorage,
}

#[async_trait]
impl Interactor<GetAccountRequest, Account> for GetAccount<'_> {
    async fn execute(
        &self,
        data: GetAccountRequest
    ) -> Result<Account, ApplicationError> {

        todo!("Implement get account");

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::acc_storage::tests::MockAccStorage;
    use crate::domain::models::account::Account;
    use crate::domain::models::address::Address;
    use crate::domain::models::signature::VerifyKey;
    use crate::domain::models::token::Token;


    #[tokio::test]
    async fn test_get_account_ok() {
        let acc_storage = MockAccStorage::new();
        let address = Address {
            network: "lokichain".to_string(),
            vk: VerifyKey([1; 32])
        };
        let account = Account {
            address: address.clone(),
            nonce: 1,
            balance: Token {
                value: 100,
                denom: "LOKI".to_string()
            }
        };
        acc_storage.set(address.clone(), account.clone()).await;

        let interactor = GetAccount {
            acc_storage: &acc_storage
        };
        let result = interactor.execute(GetAccountRequest { address }).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), account);
    }

    #[tokio::test]
    async fn test_get_account_acc_not_found() {
        let address = Address {
            network: "lokichain".to_string(),
            vk: VerifyKey([1; 32])
        };

        let interactor = GetAccount {
            acc_storage: &MockAccStorage::new()
        };
        let result = interactor.execute(GetAccountRequest { address }).await;
        assert!(result.is_err());
        assert!(match result.err().unwrap() {
            ApplicationError::NotFound(msg) => msg == "Account not found",
            _ => panic!("Unexpected error")
        });
    }
}
