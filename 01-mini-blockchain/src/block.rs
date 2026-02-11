use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Structure représentant un bloc dans la blockchain
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub index: u64,
    pub timestamp: DateTime<Utc>,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    /// Créer un nouveau bloc
    ///
    /// # Arguments
    /// * `index` - Position du bloc dans la chaîne
    /// * `data` - Données/transactions du bloc
    /// * `previous_hash` - Hash du bloc précédent
    pub fn new(index: u64, data: String, previous_hash: String) -> Self {
        Self {
            index,
            timestamp: Utc::now(),
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    /// Calculer le hash SHA-256 du bloc
    ///
    /// # Returns
    /// String hexadécimal de 64 caractères représentant le hash
    pub fn calculate_hash(&self) -> String {
        let input = format!("{}{}{}{}{}", self.index, self.timestamp.to_rfc3339(), self.data, self.previous_hash, self.nonce);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let hash = hex::encode(hasher.finalize());
        return hash;
    }

    /// Créer le bloc genesis (premier bloc de la chaîne)
    ///
    /// # Returns
    /// Le bloc genesis avec index 0 et previous_hash "0"
    pub fn genesis() -> Self {
         let mut genesis = Block::new(0, "Genesis Block".to_string(), "0".to_string());
         genesis.hash = genesis.calculate_hash();
         genesis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        // Créer un bloc de test
        let block = Block::new(1, "Test data".to_string(), "previous_hash_123".to_string());

        // Vérifier les champs
        assert_eq!(block.index, 1);
        assert_eq!(block.data, "Test data");
        assert_eq!(block.previous_hash, "previous_hash_123");
        assert_eq!(block.hash, String::new()); // Hash vide à la création
        assert_eq!(block.nonce, 0); // Nonce initialisé à 0
    }

    #[test]
    fn test_calculate_hash() {
        // Créer un bloc
        let block = Block::new(1, "Test data".to_string(), "prev_hash".to_string());

        // Calculer le hash
        let hash = block.calculate_hash();

        // Vérifier que le hash fait 64 caractères (SHA-256 en hexadécimal)
        assert_eq!(hash.len(), 64);

        // Vérifier que le hash est déterministe (même bloc = même hash)
        let hash2 = block.calculate_hash();
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_genesis_block() {
        // Créer le bloc genesis
        let genesis = Block::genesis();

        // Vérifier les propriétés du bloc genesis
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert_eq!(genesis.data, "Genesis Block");
        assert_eq!(genesis.hash.len(), 64); // Hash doit être calculé
        assert_eq!(genesis.nonce, 0);

        // Vérifier que le hash est valide
        assert_eq!(genesis.hash, genesis.calculate_hash());
    }
}
