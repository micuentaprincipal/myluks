use crate::luks_main_contract::accounts::Accounts;
use crate::security::Security;

pub struct ValidateTransaction {
    // La estructura ya tiene un HashSet para procesar transacciones; lo usaremos para la detección de doble gasto.
    security_module: Security,
}

impl ValidateTransaction {
    pub fn new(security_module: Security) -> Self {
        ValidateTransaction {
            security_module,
        }
    }

    pub fn validate_transaction(&mut self, accounts: &Accounts, from: &String, to: &String, amount: u64, signature: &String) -> Result<bool, &'static str> {
        // Límite de transacción
        const MAX_TRANSACTION_AMOUNT: u64 = 10_000; // Por ejemplo
        if amount > MAX_TRANSACTION_AMOUNT {
            return Err("La cantidad excede el límite máximo de transacción");
        }
        
        // Verificar que el remitente tenga suficiente saldo
        if accounts.get_balance(from)? < amount {
            return Err("Saldo insuficiente en la cuenta del remitente");
        }

        // Verificar que el destinatario exista
        if accounts.get_balance(to).is_err() {
            return Err("La cuenta destinataria no existe");
        }

        // Validación de firmas - Placeholder para futura implementación criptográfica
        if !Self::validate_signature(from, signature, accounts) {
            return Err("Validación de firma fallida");
        }

        // Validación contra doble gasto usando el módulo de seguridad
        let transaction_id = format!("{}-{}-{}", from, to, amount);
        if self.security_module.check_transaction_uniqueness(&transaction_id) {
            return Err("Doble gasto detectado");
        }

        // Agregar esta transacción a las transacciones procesadas usando el módulo de seguridad
        self.security_module.add_processed_transaction(transaction_id)?;

        Ok(true)
    }

    fn validate_signature(account: &String, signature: &String, accounts: &Accounts) -> bool {
        // Esta es una validación simplificada: simplemente comprobamos que la firma sea igual a la clave pública del remitente.
        // En una implementación real, se usaría criptografía de clave pública para verificar la firma.
        let account_details = accounts.accounts_map.get(account).unwrap();
        &account_details.public_key.value == signature.as_bytes()
    }
}
