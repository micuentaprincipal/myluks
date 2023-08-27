use std::collections::HashMap;

pub struct Block {
    pub index: u64,
    pub nonce: u64,
    pub prev_block_hash: String,
    pub block_hash: String,
    pub transactions: Vec<String>, // Listado de transacciones en el bloque.
}

pub struct Consensus {
    pub difficulty: u64,
    pub blockchain: Vec<Block>,
    pub hash_prefix: String, // Prefijo que debe tener el hash para considerarse válido.
}

impl Consensus {
    pub fn new(difficulty: u64, hash_prefix: String) -> Self {
        let mut consensus = Consensus {
            difficulty,
            blockchain: Vec::new(),
            hash_prefix,
        };

        consensus.create_genesis_block();
        consensus
    }

    pub fn create_genesis_block(&mut self) {
        if self.blockchain.is_empty() {
            let genesis_block = Block {
                index: 0,
                nonce: 0,
                prev_block_hash: String::from("0"),
                block_hash: String::from("0"),
                transactions: Vec::new(),
            };
            self.blockchain.push(genesis_block);
        }
    }

    pub fn implement_poh(&mut self, transactions: Vec<String>) -> Result<(), &'static str> {
        let prev_block = self.blockchain.last().unwrap();
        let mut new_block = Block {
            index: prev_block.index + 1,
            nonce: 0,
            prev_block_hash: prev_block.block_hash.clone(),
            block_hash: String::new(),
            transactions,
        };

        let block_content = format!("{}{:?}", new_block.prev_block_hash, new_block.transactions);
        new_block = self.implement_pow(block_content)?;

        // Antes de añadir el bloque, validar.
        if self.validate_block(&new_block) {
            self.blockchain.push(new_block);
            Ok(())
        } else {
            Err("El bloque no pasó la validación.")
        }
    }

    fn implement_pow(&self, block_content: String) -> Result<Block, &'static str> {
        let mut new_block = Block {
            index: self.blockchain.len() as u64,
            nonce: 0,
            prev_block_hash: self.blockchain.last().unwrap().block_hash.clone(),
            block_hash: String::new(),
            transactions: Vec::new(),
        };

        // Aquí es donde implementamos la lógica del Proof of Work.
        loop {
            let data = format!("{}{}{}", block_content, new_block.nonce, new_block.prev_block_hash);
            let hash = self.calculate_hash(&data);
            if hash.starts_with(&self.hash_prefix) {
                new_block.block_hash = hash;
                return Ok(new_block);
            }
            new_block.nonce += 1;
        }
    }

    fn calculate_hash(&self, data: &str) -> String {
        // En una implementación real, utilizaríamos alguna función criptográfica como SHA256.
        // Aquí, por simplicidad, sólo devolvemos el mismo dato.
        data.to_string()
    }

    fn validate_block(&self, block: &Block) -> bool {
        // Aquí puedes agregar más validaciones para el bloque, como verificar el hash, 
        // el índice, las transacciones, etc.
        block.block_hash.starts_with(&self.hash_prefix)
    }
}

// Nota: Esta implementación es un esqueleto y solo sirve como punto de partida.
// Las lógicas reales de Proof of History y Proof of Work son mucho más complejas y requieren más código.
