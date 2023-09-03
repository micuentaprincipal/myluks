// Importing necessary modules
mod accounts;
mod transfer;
mod validate_transaction;
mod consensus;
mod coin_issue;
mod security;

use std::collections::HashMap;

// Enum for error handling
pub enum LuksError {
    AccountAlreadyExists,
    AccountDeletionError,
    AccountNotFound,
    InsufficientBalance,
    InvalidTransaction,
    Unauthorized
}

#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    pub value: String, // Represents the address on the blockchain.
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub balance: u64,
    // Additional details could go here (public keys, transaction history, etc.)
}

#[derive(Debug, PartialEq)]
pub struct LuksMainContract {
    pub owner: Address,
    pub accounts: HashMap<Address, Account>,
    // Adding a consensus property
    pub consensus_module: consensus::Consensus,
}

impl LuksMainContract {
    // Function to create a new account
    pub fn create_account(&mut self, address: Address, initial_balance: u64) -> Result<(), LuksError> {
        if self.accounts.contains_key(&address) {
            return Err(LuksError::AccountAlreadyExists);
        }
        self.accounts.insert(address, Account { balance: initial_balance });
        Ok(())
    }

    // Function to delete an account
    pub fn delete_account(&mut self, address: &Address, caller: &Address) -> Result<(), LuksError> {
        if &self.owner != caller {
            return Err(LuksError::Unauthorized);
        }
        self.accounts.remove(address).ok_or(LuksError::AccountDeletionError)?;
        Ok(())
    }

    // Function to get account details
    pub fn get_account_details(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    // Function to initiate a transfer
    pub fn initiate_transfer(&mut self, from: &Address, to: &Address, amount: u64) -> Result<(), LuksError> {
        let is_valid = validate_transaction::validate(from, to, amount)?;
        if !is_valid {
            return Err(LuksError::InvalidTransaction);
        }
        
        let sender = self.accounts.get_mut(from).ok_or(LuksError::AccountNotFound)?;
        let receiver = self.accounts.get_mut(to).ok_or(LuksError::AccountNotFound)?;
        
        if sender.balance < amount {
            return Err(LuksError::InsufficientBalance);
        }
        sender.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }
}
