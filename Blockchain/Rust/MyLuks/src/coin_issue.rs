use std::collections::HashSet;

/// Representa la emisión y gestión de monedas en la blockchain.
pub struct CoinIssue {
    total_supply: u64,
    reward_per_block: u64,
    inflation_rate: u64,
    central_bank: String,
    emitted_transactions: HashSet<String>, // Registra las transacciones de emisión
}

impl CoinIssue {
    /// Crea una nueva instancia de `CoinIssue`.
    pub fn new(initial_supply: u64, reward_per_block: u64, inflation_rate: u64, central_bank: String) -> Self {
        CoinIssue {
            total_supply: initial_supply,
            reward_per_block,
            inflation_rate,
            central_bank,
            emitted_transactions: HashSet::new(),
        }
    }

    /// Emite nuevas monedas, ajustando el suministro total en función de la inflación.
    pub fn issue_coins(&mut self, transaction_id: String) -> Result<u64, &'static str> {
        if self.emitted_transactions.contains(&transaction_id) {
            return Err("Esta transacción ya ha sido procesada");
        }

        let new_coins = self.reward_per_block;

        if self.total_supply != 0 {
            self.total_supply += new_coins + (self.total_supply * self.inflation_rate / 100);
        } else {
            self.total_supply += new_coins;
        }

        self.emitted_transactions.insert(transaction_id);
        Ok(new_coins)
    }

    /// Define las reglas de emisión, permitiendo ajustar la recompensa por bloque y la tasa de inflación.
    pub fn define_emission_rules(&mut self, new_reward_per_block: u64, new_inflation_rate: u64) {
        self.reward_per_block = new_reward_per_block;
        self.inflation_rate = new_inflation_rate;
    }

    /// Cambia la dirección del banco central.
    pub fn change_central_bank(&mut self, new_central_bank: String) -> Result<(), &'static str> {
        if new_central_bank.is_empty() {
            return Err("Dirección inválida");
        }
        self.central_bank = new_central_bank;
        Ok(())
    }

    /// Obtiene el suministro total de monedas en circulación.
    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }
}

// Nota: Por simplicidad, este código no incluye la emisión real de monedas a una blockchain, 
// sino que solo ajusta las variables internas de la estructura.