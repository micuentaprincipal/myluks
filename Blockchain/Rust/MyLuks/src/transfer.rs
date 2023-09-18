// Enhanced Error Handling for transfers
pub enum TransferError {
    AccountNotFound,
    InsufficientBalance,
    InvalidAmount, // New Error for invalid transfer amounts
    TransactionNotFound, // New Error for transaction reversion
    UnauthorizedRevert // New Error for unauthorized transaction reversion
}

use crate::accounts::{Accounts, AccountError, TransactionHistory};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::validate_transaction::ValidationError;

#[derive(Debug, PartialEq)]
pub enum TransferType {
    PeerToPeer,
    ToContract,
    FromContract,
}

#[derive(Debug, PartialEq)]
pub struct Transfer {
    pub transactions: Vec<TransactionHistory>, // New field to store transaction history
}

impl Transfer {
    // Constantes y estructuras
    const TRANSFER_FEE_PERCENTAGE: f32 = 0.05;

    // Función para calcular la tarifa de transferencia
    fn calculate_transfer_fee(&self, amount: u64) -> u64 {
        (Self::TRANSFER_FEE_PERCENTAGE * amount as f32) as u64
    }

    fn validate_transfer_amount(&self, amount: u64) -> Result<(), TransferError> {
        if amount <= 0 {
            return Err(TransferError::InvalidAmount);
        }
        Ok(())
    }

    fn apply_transfer_fee(&self, amount: u64) -> u64 {
        amount - self.calculate_transfer_fee(amount)
    }

    fn decrement_sender_balance(accounts: &mut Accounts, from: &String, amount: u64) -> Result<(), TransferError> {
        accounts.adjust_balance(&from, -(amount as i64)).map_err(|e| match e {
            AccountError::InsufficientBalance => TransferError::InsufficientBalance,
            _ => panic!("Unexpected error"),
        })
    }
    
    fn increment_recipient_balance(accounts: &mut Accounts, to: &String, amount: u64) -> Result<(), TransferError> {
        accounts.adjust_balance(&to, amount as i64).map_err(|_| TransferError::AccountNotFound)
    }

    fn notify_user(user: &String, message: &str) {
        println!("Notification to {}: {}", user, message);
    }

    pub fn initiate_transfer(&mut self, accounts: &mut Accounts, from: &String, to: &String, amount: u64, transfer_type: TransferType) -> Result<(), TransferError> {
        self.validate_transfer_amount(amount)?;

        let fee = self.calculate_transfer_fee(amount);
        let final_amount = self.apply_transfer_fee(amount);

        // Check if both sender and receiver exist before making any changes
        if !accounts.account_exists(from) || !accounts.account_exists(to) {
            return Err(TransferError::AccountNotFound);
        }

        self.decrement_sender_balance(accounts, from, amount + fee)?; // Include fee in sender's decrement
        self.increment_recipient_balance(accounts, to, final_amount)?;

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let transaction = TransactionHistory {
            from: from.clone(),
            to: to.clone(),
            amount: final_amount,
            timestamp,
        };

        accounts.add_transaction_history(from, transaction.clone()).unwrap();
        accounts.add_transaction_history(to, transaction.clone()).unwrap();

        self.transactions.push(transaction);
        log_transfer(from, to, final_amount, &transfer_type);

        self.notify_user(from, &format!("You transferred {} coins to {}. Fee: {} coins.", final_amount, to, fee));
        self.notify_user(to, &format!("You received {} coins from {}.", final_amount, from));

        Ok(())
    }

    pub fn revert_transaction(&mut self, accounts: &mut Accounts, transaction_id: usize, caller: &String) -> Result<(), TransferError> {
        let transaction = self.transactions.get(transaction_id).ok_or(TransferError::TransactionNotFound)?;

        // Check if the transaction was from the caller
        if &transaction.from != caller {
            return Err(TransferError::UnauthorizedRevert);
        }

        // Check if both sender and receiver exist before making any changes
        if !accounts.account_exists(&transaction.from) || !accounts.account_exists(&transaction.to) {
            return Err(TransferError::AccountNotFound);
        }

        // Revert the transaction by adjusting the balances
        self.decrement_sender_balance(accounts, &transaction.to, transaction.amount)?;  // Note the reversed roles
        self.increment_recipient_balance(accounts, &transaction.from, transaction.amount)?;

        // Remove the transaction from the history
        self.transactions.remove(transaction_id);

        Ok(())
    }

    fn log_transfer(from: &String, to: &String, amount: u64, transfer_type: &TransferType) {
        println!("Transfer of {} to {} of {} coins completed. Transfer Type: {:?}", from, to, amount, transfer_type);
    }
}
