use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
pub struct PublicKey {
    pub value: Vec<u8>, // Representa la clave pública.
}

#[derive(Debug, PartialEq)]
pub struct TransactionHistory {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: u64, // Representa el tiempo en el que se realizó la transacción.
}

#[derive(Debug, PartialEq)]
pub struct AccountDetails {
    pub balance: u64,
    pub public_key: PublicKey,
    pub transactions: VecDeque<TransactionHistory>, // Almacenará las últimas N transacciones.
}

#[derive(Debug, PartialEq)]
pub struct Accounts {
    pub accounts_map: HashMap<String, AccountDetails>,
}

impl Accounts {
    pub fn new() -> Self {
        Accounts {
            accounts_map: HashMap::new(),
        }
    }

    pub fn create_account(&mut self, account_id: String, initial_balance: u64, public_key: PublicKey) -> Result<(), &'static str> {
        if self.accounts_map.contains_key(&account_id) {
            return Err("La cuenta ya existe");
        }

        let account_details = AccountDetails {
            balance: initial_balance,
            public_key,
            transactions: VecDeque::new(),
        };

        self.accounts_map.insert(account_id, account_details);
        Ok(())
    }

    pub fn get_balance(&self, account_id: &String) -> Result<u64, &'static str> {
        match self.accounts_map.get(account_id) {
            Some(account) => Ok(account.balance),
            None => Err("Cuenta no encontrada"),
        }
    }

    pub fn adjust_balance(&mut self, account_id: &String, amount: i64) -> Result<(), &'static str> {
        let account = self.accounts_map.get_mut(account_id).ok_or("Cuenta no encontrada")?;

        if amount < 0 && account.balance < (-amount) as u64 {
            return Err("Saldo insuficiente");
        }

        account.balance = (account.balance as i64 + amount) as u64;
        Ok(())
    }

    pub fn get_public_key(&self, account_id: &String) -> Result<&PublicKey, &'static str> {
        self.accounts_map
            .get(account_id)
            .map(|account| &account.public_key)
            .ok_or("Cuenta no encontrada")
    }

    pub fn set_public_key(&mut self, account_id: &String, new_key: PublicKey) -> Result<(), &'static str> {
        let account = self.accounts_map.get_mut(account_id).ok_or("Cuenta no encontrada")?;
        account.public_key = new_key;
        Ok(())
    }

    pub fn add_transaction_history(&mut self, account_id: &String, transaction: TransactionHistory) -> Result<(), &'static str> {
        let account = self.accounts_map.get_mut(account_id).ok_or("Cuenta no encontrada")?;
        if account.transactions.len() >= 10 { // Suponiendo que queremos mantener las últimas 10 transacciones.
            account.transactions.pop_front();
        }
        account.transactions.push_back(transaction);
        Ok(())
    }
}