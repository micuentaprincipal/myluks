use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use crate::validate_transaction::ValidationError;

pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_block_hash: String,
    pub block_hash: String,
    pub transactions: Vec<Transaction>,
}

pub enum ConsensusError {
    DuplicateTransaction,
    BlockValidationError,
}

pub struct Consensus {
    pub blockchain: Vec<Block>,
    pub hash_prefix: String,
    processed_transactions: HashSet<String>,
}

impl Consensus {
    pub fn new(hash_prefix: String) -> Self {
        let mut consensus = Consensus {
            blockchain: Vec::new(),
            hash_prefix,
            processed_transactions: HashSet::new(),
        };

        consensus.create_genesis_block();
        consensus
    }

    pub fn create_genesis_block(&mut self) {
        if self.blockchain.is_empty() {
            let genesis_block = Block {
                index: 0,
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                prev_block_hash: String::from("0"),
                block_hash: String::from("0"),
                transactions: Vec::new(),
            };
            self.blockchain.push(genesis_block);
        }
    }

    pub fn implement_poh(&mut self, transactions: Vec<Transaction>) -> Result<(), ConsensusError> {
        // Check for duplicate transactions
        for tx in &transactions {
            let tx_id = self.calculate_transaction_id(tx);
            if self.processed_transactions.contains(&tx_id) {
                return Err(ConsensusError::DuplicateTransaction);
            }
        }

        let prev_block = self.blockchain.last().unwrap();
        let mut new_block = Block {
            index: prev_block.index + 1,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            prev_block_hash: prev_block.block_hash.clone(),
            block_hash: String::new(),
            transactions: transactions.clone(),
        };

        new_block.block_hash = self.calculate_block_hash(&new_block);

        // Validate the new block
        if !self.validate_block(&new_block) {
            return Err(ConsensusError::BlockValidationError);
        }

        // If block is valid, add to blockchain and update processed transactions
        for tx in &transactions {
            let tx_id = self.calculate_transaction_id(tx);
            self.processed_transactions.insert(tx_id);
        }
        self.blockchain.push(new_block);
        Ok(())
    }

    fn calculate_block_hash(&self, block: &Block) -> String {
        let input = format!("{}{}{}{:?}", block.index, block.timestamp, block.prev_block_hash, block.transactions);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn calculate_transaction_id(&self, tx: &Transaction) -> String {
        let input = format!("{}{}{}{}", tx.from, tx.to, tx.amount, tx.signature);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn validate_block(&self, block: &Block) -> bool {
        // Validate that the block's timestamp is greater than the last block in the chain.
        let prev_block = self.blockchain.last().unwrap();
        if block.timestamp <= prev_block.timestamp {
            return false;
        }
        
        // Validate that the block hash is correct.
        if block.block_hash != self.calculate_block_hash(block) {
            return false;
        }

        true
    }
}
