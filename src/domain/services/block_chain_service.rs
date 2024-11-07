use crate::domain::models::block_header::BlockHeader;
use crate::domain::models::block::Block;
use crate::domain::models::transaction::Transaction;

impl BlockChainService {

    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    pub fn create_block(&mut self) -> Block {
        let previous_block = self.chain.last().unwrap();
        let block_header = BlockHeader::create_block_header(
            &previous_block.header.calculate_header_hash(),
            self.difficulty,
        );
        let block = Block {
            header: block_header,
            count: self.chain.len() as u32 + 1,
            transactions: self.curr_trans.clone(),
        };
       
        block
    }

    pub fn validate_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let previous_block = &self.chain[i - 1];
            let current_block = &self.chain[i];
            if current_block.header.pre_hash != previous_block.header.calculate_header_hash() {
                return false;
            }
            if !self.validate_transactions(&current_block.transactions) {
                return false;
            }
        }
        true
    }

    fn validate_transactions(&self, transactions: &[Transaction]) -> bool {
        transactions.iter().all(|tx| tx.is_valid())
    }

    pub fn get_block_by_index(&self, index: usize) -> Option<&Block> {
        self.chain.get(index)
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.is_valid() {
            self.curr_trans.push(tx);
        } else {
            println!("Invalid transaction!");
        }
    }

    pub fn verify_transactions_in_blocks(&self) -> bool {
        self.chain.iter().all(|block| {
            block.transactions.iter().all(|tx| tx.is_valid())
        })
    }
}