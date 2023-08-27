// Declarando módulos
mod accounts;
mod transfer;
mod validate_transaction;
mod consensus;
mod coin_issue;
mod security;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Address {
    pub value: String, // Representa la dirección en la blockchain.
}

#[derive(Debug, PartialEq)]
pub struct Account {
    pub balance: u64,
    // Otros detalles de la cuenta podrían ir aquí (por ejemplo, llaves públicas, historial de transacciones, etc.)
}

#[derive(Debug, PartialEq)]
pub struct LuksMainContract {
    pub owner: Address,
    pub accounts: HashMap<Address, Account>,
}

impl LuksMainContract {
    pub fn create_account(&mut self, address: Address, initial_balance: u64) -> Result<(), &'static str> {
        if self.accounts.contains_key(&address) {
            return Err("La cuenta ya existe");
        }
        self.accounts.insert(address, Account { balance: initial_balance });
        Ok(())
    }

    pub fn delete_account(&mut self, address: &Address) -> Result<(), &'static str> {
        self.accounts.remove(address).ok_or("Error al eliminar la cuenta")?;
        Ok(())
    }

    pub fn get_account_details(&self, address: &Address) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn initiate_transfer(&mut self, from: &Address, to: &Address, amount: u64) -> Result<(), &'static str> {
        // Validamos la transacción antes de procesarla
        let is_valid = self.validate_transaction(from, to, amount)?;
        if !is_valid {
            return Err("Transacción no válida");
        }
        
        let sender = self.accounts.get_mut(from).ok_or("Cuenta del remitente no encontrada")?;
        let receiver = self.accounts.get_mut(to).ok_or("Cuenta del destinatario no encontrada")?;
        
        if sender.balance < amount {
            return Err("Saldo insuficiente");
        }
        sender.balance -= amount;
        receiver.balance += amount;

        Ok(())
    }

    pub fn validate_transaction(&self, from: &Address, to: &Address, amount: u64) -> Result<bool, &'static str> {
        // Llamamos al módulo validate_transaction para validar la transacción
        // Aquí es donde se verificarán las firmas, el doble gasto, etc.
        // Para este ejemplo, sólo comprobamos el saldo
        let sender = self.accounts.get(from).ok_or("Cuenta del remitente no encontrada")?;
        if sender.balance < amount {
            return Err("Saldo insuficiente");
        }
        Ok(true)
    }
}
