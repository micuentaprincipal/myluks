use std::collections::{HashSet, HashMap};
use sha2::{Sha256, Digest};

/// Security module for handling transaction processing and cryptographic operations.
/// Includes features like signing, validation, and key management.
pub enum SecurityError {
    TransactionAlreadyProcessed,
    InvalidSignature,
    KeyNotFound,
    FailedToSign,
    InvalidKey, // New error type for invalid keys
}

/// Main structure for the security module.
pub struct Security {
    processed_transactions: HashSet<String>,
    // Simulated storage for public and private keys.
    public_keys: HashMap<String, Vec<u8>>,
    private_keys: HashMap<String, Vec<u8>>,
}

impl Security {
    /// Constructor for initializing a new Security instance.
    pub fn new() -> Self {
        Security {
            processed_transactions: HashSet::new(),
            public_keys: HashMap::new(),
            private_keys: HashMap::new(),
        }
    }

    /// Adds a processed transaction to the set.
    pub fn add_processed_transaction(&mut self, transaction_hash: String) -> Result<(), SecurityError> {
        if !self.processed_transactions.insert(transaction_hash) {
            return Err(SecurityError::TransactionAlreadyProcessed);
        }
        Ok(())
    }

    /// Checks if a transaction has already been processed.
    pub fn has_transaction_been_processed(&self, transaction_hash: &String) -> bool {
        self.processed_transactions.contains(transaction_hash)
    }

    /// Simulates signing using a "private key".
    pub fn sign(&self, account: &String, data: &String) -> Result<String, SecurityError> {
        let private_key = self.private_keys.get(account).ok_or(SecurityError::KeyNotFound)?;
        let hashed_data = self.hash(data);
        if &hashed_data == private_key {
            Ok(hashed_data)
        } else {
            Err(SecurityError::FailedToSign)
        }
    }

    /// Simulates signature validation using a "public key".
    pub fn validate_signature(&self, account: &String, data: &String, signature: &String) -> Result<bool, SecurityError> {
        let public_key = self.public_keys.get(account).ok_or(SecurityError::KeyNotFound)?;
        let hashed_data = self.hash(data);
        if &hashed_data == public_key && &hashed_data == signature {
            Ok(true)
        } else {
            Err(SecurityError::InvalidSignature)
        }
    }

    /// Validates keys before adding them to storage. (New function)
    fn validate_keys(public_key: &Vec<u8>, private_key: &Vec<u8>) -> Result<(), SecurityError> {
        if public_key.is_empty() || private_key.is_empty() {
            return Err(SecurityError::InvalidKey);
        }
        Ok(())
    }

    /// Adds public and private keys to the simulated storage.
    pub fn add_keys(&mut self, account: String, public_key: Vec<u8>, private_key: Vec<u8>) -> Result<(), SecurityError> {
        Self::validate_keys(&public_key, &private_key)?;
        self.public_keys.insert(account.clone(), public_key);
        self.private_keys.insert(account, private_key);
        Ok(())
    }
    
    /// Simulates a hashing process.
    fn hash(&self, data: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}
