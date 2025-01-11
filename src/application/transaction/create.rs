use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::domain::models::transaction::Transaction;

pub struct CreateTransactionResult {

}

pub struct CreateTransaction {}

#[async_trait]
impl Interactor<Transaction, CreateTransactionResult> for CreateTransaction {
    async fn execute(&self, data: Transaction) -> Result<CreateTransactionResult, ApplicationError> {
        Ok(CreateTransactionResult {})
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::models::address::Address;
    use crate::domain::models::hash::Hash;
    use crate::domain::models::signature::Signature;
    use crate::domain::models::token::Token;
    use super::*;
    use crate::domain::models::transaction::Transaction;

    #[tokio::test]
    async fn test_create_transaction() {
        let interactor = CreateTransaction {};
        let transaction = Transaction {
            hash: Hash([0; 32]),
            sender: Address { network: "lokichain".to_string(), hash: vec![] },
            amount: Token { value: 0, denom: "LOKI".to_string() },
            timestamp: 0,
            gas: 0,
            nonce: 0,
            data: vec![],
            signature: Signature([0; 64])
        };
        let result = interactor.execute(transaction).await;
        assert!(result.is_ok());
    }
}