use std::collections::{HashSet, HashMap};
use sha2::{Sha256, Digest};
use std::time::Instant; // Para medir el rendimiento

/// Security module for handling transaction processing and cryptographic operations.
pub enum SecurityError {
    TransactionAlreadyProcessed,
    InvalidSignature,
    KeyNotFound,
    FailedToSign,
    InvalidKey,
}

pub struct Security {
    processed_transactions: HashSet<String>,
    public_keys: HashMap<String, Vec<u8>>,
    private_keys: HashMap<String, Vec<u8>>,
}

impl Security {
    pub fn new() -> Self {
        Security {
            processed_transactions: HashSet::new(),
            public_keys: HashMap::new(),
            private_keys: HashMap::new(),
        }
    }

    pub fn add_processed_transaction(&mut self, transaction_hash: String) -> Result<(), SecurityError> {
        if !self.processed_transactions.insert(transaction_hash) {
            return Err(SecurityError::TransactionAlreadyProcessed);
        }
        Ok(())
    }

    pub fn has_transaction_been_processed(&self, transaction_hash: &String) -> bool {
        self.processed_transactions.contains(transaction_hash)
    }

    pub fn sign(&self, account: &String, data: &String) -> Result<String, SecurityError> {
        let private_key = self.private_keys.get(account).ok_or(SecurityError::KeyNotFound)?;
        let hashed_data = self.hash(data);
        // En un escenario real, usaríamos el `private_key` para firmar los datos, pero para esta simulación, simplemente devolvemos el hash.
        Ok(hashed_data)
    }

    pub fn validate_signature(&self, account: &String, data: &String, signature: &String) -> Result<bool, SecurityError> {
        let public_key = self.public_keys.get(account).ok_or(SecurityError::KeyNotFound)?;
        let hashed_data = self.hash(data);
        // En un escenario real, usaríamos el `public_key` para verificar la firma.
        if &hashed_data == signature {
            Ok(true)
        } else {
            Err(SecurityError::InvalidSignature)
        }
    }

    fn validate_keys(public_key: &Vec<u8>, private_key: &Vec<u8>) -> Result<(), SecurityError> {
        if public_key.is_empty() || private_key.is_empty() {
            return Err(SecurityError::InvalidKey);
        }
        Ok(())
    }

    pub fn add_keys(&mut self, account: String, public_key: Vec<u8>, private_key: Vec<u8>) -> Result<(), SecurityError> {
        Self::validate_keys(&public_key, &private_key)?;
        self.public_keys.insert(account.clone(), public_key);
        self.private_keys.insert(account, private_key);
        Ok(())
    }

    fn hash(&self, data: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn measure_performance() {
        let start = Instant::now();
        // Simulamos alguna operación costosa, como hash.
        let mut hasher = Sha256::new();
        for _ in 0..1000 {
            hasher.update("performance_test");
        }
        let _ = hasher.finalize();
        let duration = start.elapsed();
        println!("Tiempo tomado para 1000 hashes: {:?}", duration);
    }

    pub fn verify_hash(data: &str, expected_hash: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize()) == expected_hash
    }

    /// Handles errors and prints custom messages based on error type.
    pub fn handle_errors(error: SecurityError) {
        match error {
            SecurityError::TransactionAlreadyProcessed => println!("Error: La transacción ya ha sido procesada."),
            SecurityError::InvalidSignature => println!("Error: Firma inválida."),
            SecurityError::KeyNotFound => println!("Error: Llave no encontrada."),
            SecurityError::FailedToSign => println!("Error: Fallo al firmar."),
            SecurityError::InvalidKey => println!("Error: Llave inválida."),
        }
    }

    /// Validates a transaction by checking its signature and if it's already processed.
    pub fn validate_transaction(&self, account: &String, data: &String, signature: &String) -> Result<bool, SecurityError> {
        // Check if the transaction has been processed
        if self.has_transaction_been_processed(data) {
            return Err(SecurityError::TransactionAlreadyProcessed);
        }

        // Validate signature
        match self.validate_signature(account, data, signature) {
            Ok(true) => Ok(true),
            Ok(false) => Err(SecurityError::InvalidSignature),
            Err(e) => Err(e),
        }
    }
}
