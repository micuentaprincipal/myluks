// Enhanced Error Handling
pub enum AccountError {
    AccountAlreadyExists,
    AccountNotFound,
    InsufficientBalance,
    UnauthorizedKeyChange,
    AccountLocked,
    AccountNotLocked,
}

use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct PublicKey {
    pub value: Vec<u8>, // Represents the public key.
}

impl PublicKey {
    // New function to get hex representation
    pub fn to_hex(&self) -> String {
        hex::encode(&self.value)
    }
}

#[derive(Debug, PartialEq)]
pub struct TransactionHistory {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64, // Represents the time the transaction was made.
}

#[derive(Debug, PartialEq)]
pub struct AccountDetails {
    pub balance: u64,
    pub public_key: PublicKey,
    pub transactions: VecDeque<TransactionHistory>, // Will store the last N transactions.
    pub locked: bool,
}

#[derive(Debug, PartialEq)]
pub struct Accounts {
    pub accounts_map: HashMap<String, AccountDetails>,
    pub max_transaction_history: usize,
}

// Nueva estructura para almacenar hash de las cuentas
#[derive(Debug, PartialEq)]
pub struct AccountHash {
    pub account_id: String,
    pub hash: Vec<u8>, // Hash de la cuenta
}

impl AccountHash {
    // Función para calcular el hash de una cuenta
    pub fn compute_hash(account: &AccountDetails) -> Vec<u8> {
        let data = serde_json::to_string(&account).unwrap();
        let digest = sha2::Sha256::digest(data.as_bytes());
        digest.to_vec()
    }
}

impl Accounts {
    pub fn new(max_transaction_history: usize) -> Self {
        Accounts {
            accounts_map: HashMap::new(),
            max_transaction_history,
        }
    }

    pub fn create_account(&mut self, account_id: String, initial_balance: u64, public_key: PublicKey) -> Result<(), AccountError> {
        if self.accounts_map.contains_key(&account_id) {
            return Err(AccountError::AccountAlreadyExists);
        }
        let account_details = AccountDetails {
            balance: initial_balance,
            public_key,
            transactions: VecDeque::new(),
            locked: false, // Initially, the account is not locked
        };
        self.accounts_map.insert(account_id, account_details);
        Ok(())
    }

    pub fn get_balance(&self, account_id: &String) -> Result<u64, AccountError> {
        match self.accounts_map.get(account_id) {
            Some(account) => Ok(account.balance),
            None => Err(AccountError::AccountNotFound),
        }
    }

    pub fn adjust_balance(&mut self, account_id: &String, amount: i64) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        if account.locked {
            return Err(AccountError::AccountLocked);
        }
        if amount < 0 && account.balance < (-amount) as u64 {
            return Err(AccountError::InsufficientBalance);
        }
        account.balance = (account.balance as i64 + amount) as u64;
        Ok(())
    }

    // Function to delete an account
    pub fn delete_account(&mut self, account_id: &String) -> Result<(), AccountError> {
        self.accounts_map.remove(account_id).ok_or(AccountError::AccountNotFound)?;
        Ok(())
    }

    // Function to list all accounts
    pub fn list_all_accounts(&self) -> Vec<String> {
        self.accounts_map.keys().cloned().collect()
    }

    // Function to get full account details
    pub fn get_account_details(&self, account_id: &String) -> Result<&AccountDetails, AccountError> {
        self.accounts_map.get(account_id).ok_or(AccountError::AccountNotFound)
    }

    // Function to securely set a new public key
    pub fn set_public_key_secure(&mut self, account_id: &String, old_key: &PublicKey, new_key: PublicKey) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        if &account.public_key != old_key {
            return Err(AccountError::UnauthorizedKeyChange);
        }
        account.public_key = new_key;
        Ok(())
    }

    // Nueva función para obtener el hash de una cuenta
    pub fn get_account_hash(&self, account_id: &String) -> Option<Vec<u8>> {
        self.accounts_map.get(account_id).map(|account| AccountHash::compute_hash(account))
    }

    // Nueva función para validar la integridad de una cuenta
    pub fn validate_account_integrity(&self, account_id: &String, provided_hash: Vec<u8>) -> Result<(), AccountError> {
        match self.get_account_hash(account_id) {
            Some(hash) if hash == provided_hash => Ok(()),
            Some(_) => Err(AccountError::AccountAlreadyExists), // Hashes no coinciden
            None => Err(AccountError::AccountNotFound),
        }
    }

    // New Functions
    // Function to update the account balance
    pub fn update_account_balance(&mut self, account_id: &String, new_balance: u64) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        if account.locked {
            return Err(AccountError::AccountLocked);
        }
        account.balance = new_balance;
        Ok(())
    }

    // Function to lock an account
    pub fn lock_account(&mut self, account_id: &String) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        account.locked = true;
        Ok(())
    }

    // Function to unlock an account
    pub fn unlock_account(&mut self, account_id: &String) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        account.locked = false;
        Ok(())
    }
    // New function to validate if the account is locked
    fn is_account_locked(&self, account_id: &String) -> Result<(), AccountError> {
        match self.accounts_map.get(account_id) {
            Some(account) if account.locked => Err(AccountError::AccountLocked),
            Some(_) => Ok(()),
            None => Err(AccountError::AccountNotFound),
        }
    }

    pub fn add_transaction(&mut self, account_id: &String, transaction: TransactionHistory) -> Result<(), AccountError> {
        let account = self.accounts_map.get_mut(account_id).ok_or(AccountError::AccountNotFound)?;
        if account.transactions.len() >= self.max_transaction_history {
            account.transactions.pop_front();
        }
        account.transactions.push_back(transaction);
        Ok(())
    }

    pub fn get_transactions(&self, account_id: &String) -> Result<VecDeque<TransactionHistory>, AccountError> {
        match self.accounts_map.get(account_id) {
            Some(account) => Ok(account.transactions.clone()),
            None => Err(AccountError::AccountNotFound),
        }
    }    
}
