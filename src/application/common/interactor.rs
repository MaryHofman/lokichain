use async_trait::async_trait;
use super::exceptions::ApplicationError;

#[async_trait]
pub trait Interactor<I = (), O = ()> {
    async fn execute(&self, data: I) -> Result<O, ApplicationError>;
}
