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
use crate::validate_transaction::ValidationError;  // New import for additional validation

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
    // Function to validate transfer amount (New Function)
    fn validate_transfer_amount(&self, amount: u64) -> Result<(), TransferError> {
        if amount <= 0 {
            return Err(TransferError::InvalidAmount);
        }
        Ok(())
    }
    
    pub fn initiate_transfer(accounts: &mut Accounts, from: &String, to: &String, amount: u64, transfer_type: TransferType) -> Result<(), TransferError> {
        // New call to validate the transfer amount
        self.validate_transfer_amount(amount)?;
        
        // Validation of addresses
        if !accounts.accounts_map.contains_key(from) {
            return Err(TransferError::AccountNotFound);
        }
        if !accounts.accounts_map.contains_key(to) {
            return Err(TransferError::AccountNotFound);
        }
        
        // Verify that the sender has enough balance to make the transfer
        if accounts.get_balance(&from)? < amount {
            return Err(TransferError::InsufficientBalance);
        }

        // Decrement sender's balance
        accounts.adjust_balance(&from, -(amount as i64))?;
        
        // Increment recipient's balance
        accounts.adjust_balance(&to, amount as i64)?;

        // Record the transaction in the history of both accounts
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let transaction = TransactionHistory {
            from: from.clone(),
            to: to.clone(),
            amount,
            timestamp,
        };

        accounts.add_transaction_history(from, transaction.clone())?;
        accounts.add_transaction_history(to, transaction.clone())?;
        
        // Store the transaction in the Transfer struct (New Line)
        self.transactions.push(transaction);

        // Log the transfer
        log_transfer(from, to, amount, &transfer_type);

        Ok(())
    }

    // New Function to revert a transaction
    pub fn revert_transaction(&mut self, transaction_id: usize, caller: &String) -> Result<(), TransferError> {
        let transaction = self.transactions.get(transaction_id).ok_or(TransferError::TransactionNotFound)?;
        
        if &transaction.from != caller {
            return Err(TransferError::UnauthorizedRevert);
        }
        
        // Revert the transaction by adjusting the balances
        accounts.adjust_balance(&transaction.from, transaction.amount as i64)?;
        accounts.adjust_balance(&transaction.to, -(transaction.amount as i64))?;

        // Remove the transaction from the history
        self.transactions.remove(transaction_id);
        
        Ok(())
    }

    fn log_transfer(from: &String, to: &String, amount: u64, transfer_type: &TransferType) {
        println!("Transfer of {} to {} of {} coins completed. Transfer Type: {:?}", from, to, amount, transfer_type);
    }
}
