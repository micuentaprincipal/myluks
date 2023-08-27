use crate::luks_main_contract::accounts::{Accounts, TransactionHistory};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq)]
pub struct Transfer;

impl Transfer {
    pub fn initiate_transfer(accounts: &mut Accounts, from: &String, to: &String, amount: u64) -> Result<(), &'static str> {
        // Validación de direcciones
        if !accounts.accounts_map.contains_key(from) {
            return Err("Dirección del remitente no válida");
        }
        if !accounts.accounts_map.contains_key(to) {
            return Err("Dirección del destinatario no válida");
        }
        
        // Verificar que el remitente tiene suficiente saldo para realizar la transferencia
        if accounts.get_balance(&from)? < amount {
            return Err("Saldo insuficiente");
        }

        // Decrementar el saldo del remitente
        accounts.adjust_balance(&from, -(amount as i64))?;
        
        // Incrementar el saldo del destinatario
        accounts.adjust_balance(&to, amount as i64)?;

        // Registrar la transacción en el historial de ambas cuentas
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let transaction = TransactionHistory {
            from: from.clone(),
            to: to.clone(),
            amount,
            timestamp,
        };

        accounts.add_transaction_history(from, transaction.clone())?;
        accounts.add_transaction_history(to, transaction)?;

        println!("Transferencia de {} a {} de {} monedas completada.", from, to, amount);

        Ok(())
    }
}
