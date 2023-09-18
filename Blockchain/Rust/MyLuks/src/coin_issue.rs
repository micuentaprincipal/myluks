use std::collections::HashSet;
use std::time::Instant; // Para medir el rendimiento
use crypto::digest::Digest; // Para hash verification
use crypto::sha2::Sha256; // Para hash verification

pub enum CoinIssueError {
    TransactionAlreadyProcessed,
    MaxSupplyReached,
    InvalidAddress,
    NotAuthorized,
    IntegrityViolation, // Nuevo: Para verificación de hash
}

pub struct CoinIssue {
    total_supply: u64,
    reward_per_block: u64,
    inflation_rate: u64,
    max_supply: u64,
    central_bank: String,
    emitted_transactions: HashSet<String>,
    accounts_hash: String, // Nuevo: Para la verificación de hash
}

impl CoinIssue {
    const INITIAL_HASH: &'static str = "some_real_initial_hash_value"; 

    pub fn new(
        initial_supply: u64,
        reward_per_block: u64,
        inflation_rate: u64,
        max_supply: u64,
        central_bank: String,
    ) -> Self {
        CoinIssue {
            total_supply: initial_supply,
            reward_per_block,
            inflation_rate,
            max_supply,
            central_bank,
            emitted_transactions: HashSet::new(),
            accounts_hash: Self::INITIAL_HASH.to_string(),
        }
    }

    pub fn can_issue_coins(&self, requesting_address: &String) -> bool {
        &self.central_bank == requesting_address
    }

    fn update_and_verify_hash(&mut self, new_transaction_id: &String) -> bool {
        let mut hasher = Sha256::new();
        hasher.input_str(&format!("{}{}", &self.accounts_hash, new_transaction_id));
        let new_hash = hasher.result_str();

        if new_hash == self.accounts_hash {
            return false;
        }

        self.accounts_hash = new_hash;
        true
    }

    fn recalculate_reward(&mut self) {
        self.reward_per_block = self.reward_per_block + (self.total_supply * self.inflation_rate / 100);
    }

    pub fn issue_coins(
        &mut self,
        transaction_id: String,
        requesting_address: &String,
    ) -> Result<u64, CoinIssueError> {
        let start_time = Instant::now(); 

        if !self.can_issue_coins(requesting_address) {
            return Err(CoinIssueError::NotAuthorized);
        }

        if !self.update_and_verify_hash(&transaction_id) {
            return Err(CoinIssueError::IntegrityViolation);
        }

        if self.emitted_transactions.contains(&transaction_id) {
            return Err(CoinIssueError::TransactionAlreadyProcessed);
        }

        self.recalculate_reward();

        let emission = self.reward_per_block;
        if self.total_supply + emission > self.max_supply {
            return Err(CoinIssueError::MaxSupplyReached);
        }

        self.total_supply += emission;
        self.emitted_transactions.insert(transaction_id);

        let elapsed_time = start_time.elapsed(); 
        println!("Tiempo tomado para emitir monedas: {:?}", elapsed_time);

        Ok(emission)
    }
}
