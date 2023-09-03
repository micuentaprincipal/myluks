use crate::luks_main_contract::accounts::{Accounts, AccountError};
use crate::security::Security;

/// Enum for handling validation errors in transactions.
pub enum ValidationError {
    ExceedsMaxAmount,
    InsufficientSenderBalance,
    InvalidRecipient,
    SignatureFailure,
    DoubleSpend,
    SenderAccountLocked // New error type for locked accounts
}

/// Main structure for handling transaction validation.
pub struct ValidateTransaction {
    security_module: Security,
}

impl ValidateTransaction {
    /// Constructor for initializing a new ValidateTransaction instance.
    pub fn new(security_module: Security) -> Self {
        ValidateTransaction { security_module }
    }

    /// Main function to validate a transaction based on various conditions.
    pub fn validate_transaction(
        &mut self,
        accounts: &Accounts,
        from: &String,
        to: &String,
        amount: u64,
        signature: &String
    ) -> Result<bool, ValidationError> {
        const MAX_TRANSACTION_AMOUNT: u64 = 10_000;

        // Check if the amount exceeds the maximum allowed transaction amount.
        if amount > MAX_TRANSACTION_AMOUNT {
            return Err(ValidationError::ExceedsMaxAmount);
        }

        // Check if the sender's account is locked (New Check).
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

        // Validate the signature of the transaction.
        if !self.validate_signature(from, signature, accounts) {
            return Err(ValidationError::SignatureFailure);
        }

        // Check for double spending.
        let transaction_id = format!("{}-{}-{}", from, to, amount);
        if self.security_module.has_transaction_been_processed(&transaction_id) {
            return Err(ValidationError::DoubleSpend);
        }

        // Mark the transaction as processed.
        self.security_module.add_processed_transaction(transaction_id).unwrap();
        Ok(true)
    }

    /// Function to validate the signature of a transaction.
    fn validate_signature(&self, account: &String, signature: &String, accounts: &Accounts) -> bool {
        let account_details = accounts.get_account_details(account).unwrap();
        &account_details.public_key.value == signature.as_bytes()
    }
}
