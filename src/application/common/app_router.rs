use async_trait::async_trait;

/// The application router distributes transactions to target applications
///
/// For example, Bob wants to transfer 100 LOKI to Alice. He creates a `[Transaction]`
/// with a `data` field containing the name of the application and instructions for it
#[async_trait]
pub trait AppRouter: Send + Sync {
    async fn is_exist(&self, app_name: &str, operation: &str) -> bool;
}

#[cfg(test)]
pub mod tests {
    use super::AppRouter;
    use async_trait::async_trait;

    pub struct MockAppRouter;

    #[async_trait]
    impl AppRouter for MockAppRouter {
        async fn is_exist(&self, app_name: &str, operation: &str) -> bool {
            app_name == "bank" && operation == "transfer"
        }
    }

    #[tokio::test]
    async fn test_is_exist() {
        let router = MockAppRouter;
        assert!(router.is_exist("bank", "transfer").await);
        assert!(!router.is_exist("bank", "deposit").await);
        assert!(!router.is_exist("wallet", "transfer").await);
    }
}