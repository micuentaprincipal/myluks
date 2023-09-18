use crate::luks_main_contract::accounts::{Accounts, AccountError};
use crate::security::{Security, SecurityError};  // Import modified to integrate the Security module

/// Enum for handling validation errors in transactions.
pub enum ValidationError {
    ExceedsMaxAmount,
    InsufficientSenderBalance,
    InvalidRecipient,
    SignatureFailure,
    DoubleSpend,
    SenderAccountLocked,
    InvalidSignature,  // New error type for invalid signatures
    InvalidTransactionHash,  // New error type for invalid transaction hashes
}

/// Main structure for handling transaction validation.
pub struct ValidateTransaction {
    security_module: Security,  // Changed to Security
}

impl ValidateTransaction {
    /// Constructor for initializing a new ValidateTransaction instance.
    pub fn new(security_module: Security) -> Self {
        ValidateTransaction { security_module }
    }

    /// Generate a unique transaction hash (New Function)
    pub fn generate_transaction_hash(&self, from: &String, to: &String, amount: u64) -> String {
        let data = format!("{}-{}-{}", from, to, amount);
        self.security_module.hash(&data)
    }

    /// Main function to validate a transaction based on various conditions.
    pub fn validate_transaction(
        &mut self,
        accounts: &Accounts,
        from: &String,
        to: &String,
        amount: u64,
        signature: &String,
    ) -> Result<bool, ValidationError> {
        const MAX_TRANSACTION_AMOUNT: u64 = 10_000;

        // Check if the amount exceeds the maximum allowed transaction amount.
        if amount > MAX_TRANSACTION_AMOUNT {
            return Err(ValidationError::ExceedsMaxAmount);
        }

        // Check if the sender's account is locked.
        let sender_details = accounts.get_account_details(from).map_err(|_| ValidationError::InvalidRecipient)?;
        if sender_details.locked {
            return Err(ValidationError::SenderAccountLocked);
        }

        // Check if the sender has enough balance to proceed with the transaction.
        if accounts.get_balance(from).unwrap_or(0) < amount {
            return Err(ValidationError::InsufficientSenderBalance);
        }

        // Check if the recipient exists.
        if accounts.get_balance(to).is_err() {
            return Err(ValidationError::InvalidRecipient);
        }

        // Validate the signature of the transaction using the Security module. (Modified)
        match self.security_module.validate_signature(from, &amount.to_string(), signature) {
            Ok(true) => {},
            Ok(false) | Err(_) => return Err(ValidationError::InvalidSignature),
        }

        // Check for double spending.
        let transaction_hash = self.generate_transaction_hash(from, to, amount);  // Modified to use new function
        if self.security_module.has_transaction_been_processed(&transaction_hash) {
            return Err(ValidationError::DoubleSpend);
        }

        // Validate transaction hash (New Check)
        if self.generate_transaction_hash(from, to, amount) != transaction_hash {
            return Err(ValidationError::InvalidTransactionHash);
        }

        // Mark the transaction as processed.
        self.security_module.add_processed_transaction(transaction_hash).unwrap();
        Ok(true)
    }
}
