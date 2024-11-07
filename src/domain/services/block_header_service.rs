use crate::domain::models::block_header::BlockHeader;
use crate::domain::models::transaction::Transaction;
use sha2::{Sha256, Digest};

impl BlockHeader{
    // Создает заголовок нового блока
    pub fn create_block_header(previous_block_hash: &str, difficulty: u32) -> BlockHeader {
        BlockHeader {
            timestump: get_current_timestamp(),
            nonce: 0, 
            pre_hash: previous_block_hash.to_string(),
            merkle: String::new(), 
            difficulty,
        }
    }

    // Вычисляет хеш заголовка блока
    pub fn calculate_header_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.timestump, self.nonce, self.pre_hash, self.merkle, self.difficulty
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{}", result)
    }

    // Обновляет корень дерева Меркла
    pub fn update_merkle_root(&mut self, transactions: &[Transaction]) {
        let hashes: Vec<String> = transactions.iter().map(|tx| self.hash_transaction(tx)).collect();
        self.merkle = self.calculate_merkle_root(hashes);
    }

}