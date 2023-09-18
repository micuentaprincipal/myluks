use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use crate::validate_transaction::ValidationError;
use secp256k1::{Secp256k1, Message, PublicKey};

pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub prev_block_hash: String,
    pub block_hash: String,
    pub transactions: Vec<Transaction>,
}

pub enum ConsensusError {
    DuplicateTransaction,
    BlockValidationError(BlockValidationError),
}

pub struct Node {
    id: String,
    is_validating: bool, // Si el nodo actualmente está validando
}

// Errores específicos relacionados con la validación del bloque
pub enum BlockValidationError {
    InvalidTimestamp,
    MismatchedPreviousHash,
    IncorrectBlockHash,
}

pub struct Consensus {
    pub blockchain: Vec<Block>,
    pub hash_prefix: String,
    processed_transactions: HashSet<String>,
    nodes: HashSet<Node>,
}

impl Consensus {
    // Nuevo campo para mantener un registro de nodos
    nodes: HashSet<Node>,
    
    pub fn new(hash_prefix: String) -> Self {
        let mut consensus = Consensus {
            blockchain: Vec::new(),
            hash_prefix,
            processed_transactions: HashSet::new(),
            nodes: HashSet::new(),
        };

        consensus.create_genesis_block();
        consensus
    }

    // Función para registrar un nuevo nodo en la red
    pub fn register_node(&mut self, node: Node) {
        self.nodes.insert(node);
    }

    // Función que simula una votación de nodos para validar un bloque
    fn nodes_vote_on_block(&self, block: &Block) -> bool {
        let mut votes_for = 0;
        for node in &self.nodes {
            if node.is_validating && self.validate_block_for_node(node, block) {
                votes_for += 1;
            }
        }
        // Considerando el caso de bajo número de nodos.
        if self.nodes.len() < 3 {
            return votes_for == self.nodes.len();
        } else {
            return votes_for > (self.nodes.len() / 2);
        }
    }

    fn validate_block_for_node(&self, _node: &Node, block: &Block) -> bool {
        // Aquí puedes agregar lógica específica de validación por nodo, por ahora simplemente reutilizamos la función validate_block
        self.validate_block(block)
    }

    pub fn implement_poh(&mut self, transactions: Vec<Transaction>) -> Result<(), ConsensusError> {
        for tx in &transactions {
            // Verificación de la firma de la transacción
            if !self.verify_transaction_signature(tx) {
                self.notify("Invalid transaction signature detected");
                return Err(ConsensusError::DuplicateTransaction);
            }
            
            let tx_id = self.calculate_transaction_id(tx);
            if self.processed_transactions.contains(&tx_id) {
                self.notify("Duplicate transaction detected");
                return Err(ConsensusError::DuplicateTransaction);
            }
        }

        if !self.validate_block(&new_block).is_ok() {
            self.notify("Block validation failed");
            return Err(ConsensusError::BlockValidationError(self.validate_block(&new_block).unwrap_err()));
        }

        let new_block = self.construct_new_block(transactions)?;

        // Antes de validar el bloque localmente, pedimos a los nodos que voten
        if !self.nodes_vote_on_block(&new_block) {
            self.notify("Block rejected by network nodes");
            return Err(ConsensusError::BlockValidationError);
        }

        if !self.validate_block(&new_block) {
            self.notify("Block validation failed");
            return Err(ConsensusError::BlockValidationError);
        }

        
        let validation_result = self.validate_block(&new_block);
        
        // Almacenar el resultado de la validación en una variable.
        if !validation_result.is_ok() {
            self.notify("Block validation failed");
            return Err(ConsensusError::BlockValidationError(validation_result.unwrap_err()));
        }

        // Si el bloque es válido, añadirlo a la cadena de bloques y actualizar las transacciones procesadas
        self.update_blockchain(new_block);
        self.notify("Block successfully added to the blockchain");

        Ok(())
    }

    fn calculate_block_hash(&self, block: &Block) -> String {
        let input = format!("{}|{}|{}|{:?}", block.index, block.timestamp, block.prev_block_hash, block.transactions);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn calculate_transaction_id(&self, tx: &Transaction) -> String {
        let input = format!("{}|{}|{}|{}", tx.from, tx.to, tx.amount, tx.signature);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    fn verify_transaction_signature(&self, tx: &Transaction) -> bool {
        // Crear una instancia del contexto de secp256k1
        let secp = Secp256k1::verification_only();

        // Construir el mensaje que se firmó originalmente. 
        // Puede variar según cómo se haya estructurado la firma en tu implementación.
        let message_data = format!("{}{}{}", tx.from, tx.to, tx.amount);
        let message = Message::from_slice(&sha256(message_data)).expect("32 bytes");

        // Convertir la firma y la clave pública de la transacción a estructuras manejables por la biblioteca.
        let signature = secp256k1::Signature::from_compact(&tx.signature.as_bytes()).expect("Signature parsing");
        let pubkey_bytes = hex::decode(&tx.from).expect("Decoding pubkey"); // Asumiendo que 'from' contiene la clave pública en formato hex
        let pubkey = PublicKey::from_slice(&pubkey_bytes).expect("Public key parsing");

        // Verificar la firma
        secp.verify(&message, &signature, &pubkey).is_ok()
    }

    fn validate_block(&self, block: &Block) -> Result<(), BlockValidationError> {
        let prev_block = self.blockchain.last().unwrap();

        if block.timestamp <= prev_block.timestamp {
            return Err(BlockValidationError::InvalidTimestamp);
        }
        
        if block.prev_block_hash != prev_block.block_hash {
            return Err(BlockValidationError::MismatchedPreviousHash);
        }

        if block.block_hash != self.calculate_block_hash(block) {
            return Err(BlockValidationError::IncorrectBlockHash);
        }

        Ok(())
    }
}
