use crate::domain::models::transaction::Transaction;
use sha2::{Sha256, Digest};

impl BlockService{

    // Добавляет новые транзакции в блок
    pub fn add_transactions(&mut self, transactions: Vec<Transaction>) {
        self.transactions.extend(transactions);
    }

    // Вычисляет хеш текущего блока
    pub fn calculate_block_hash(&self) -> String {
        let data = format!("{:?}{:?}{:?}", self.header, self.transactions);
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    // Проверяет, действителен ли блок
    pub fn is_valid(&self, previous_block_hash: &str) -> bool {

        if self.header.previous_hash != previous_block_hash {
            return false;
        }

        let calculated_hash = self.calculate_block_hash();
        true
    }

    // Связывает текущий блок с предыдущим
    pub fn link_to_previous_block(&mut self, previous_block_hash: &str) {
        self.header.previous_hash = previous_block_hash.to_string();
    }
    
}