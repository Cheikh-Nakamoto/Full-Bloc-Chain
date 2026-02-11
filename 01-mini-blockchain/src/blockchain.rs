use crate::block::Block;
use crate::proof_of_work;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use thiserror::Error;

/// Erreurs possibles lors de l'utilisation de la blockchain
#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Invalid block hash")]
    InvalidHash,

    #[error("Invalid previous hash")]
    InvalidPreviousHash,

    #[error("Invalid block index")]
    InvalidIndex,

    #[error("Mining failed")]
    MiningFailed,

    #[error("Empty data")]
    EmptyData,
}

/// Structure représentant la blockchain complète
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

/// Type pour partager la blockchain entre threads (pour l'API)
pub type SharedBlockchain = Arc<RwLock<Blockchain>>;

impl Blockchain {
    /// Créer une nouvelle blockchain avec le bloc genesis
    ///
    /// # Arguments
    /// * `difficulty` - Niveau de difficulté du mining (nombre de zéros requis)
    ///
    /// # Returns
    /// Une nouvelle blockchain initialisée avec le bloc genesis
    pub fn new(difficulty: usize) -> Self {
        let mut chain = Vec::new();
        let genesis = Block::genesis();
        chain.push(genesis);
        Self { chain, difficulty }
    }

    /// Ajouter un nouveau bloc à la chaîne
    ///
    /// # Arguments
    /// * `data` - Données à stocker dans le bloc
    ///
    /// # Returns
    /// Result contenant une référence au bloc ajouté ou une erreur
    pub fn add_block(&mut self, data: String) -> Result<&Block, BlockchainError> {
        // Vérifier que les données ne sont pas vides
        if data.is_empty() {
            return Err(BlockchainError::EmptyData);
        }

        // Obtenir le dernier bloc de la chaîne
        let last_block = self.latest_block();

        // Calculer le nouvel index
        let new_index = last_block.index + 1;

        // Créer un nouveau bloc
        let mut new_block = Block::new(new_index, data, last_block.hash.clone());

        // Miner le bloc avec la difficulté configurée
        proof_of_work::mine_block(&mut new_block, self.difficulty);

        // Ajouter le bloc à la chaîne
        self.chain.push(new_block);

        // Retourner une référence au dernier bloc
        Ok(self.latest_block())
    }

    /// Valider l'intégrité de toute la chaîne
    ///
    /// # Returns
    /// true si la chaîne est valide, false sinon
    pub fn is_valid(&self) -> bool {
        // Vérifier le bloc genesis par ses propriétés et son hash
        let genesis = &self.chain[0];
        if genesis.index != 0 || genesis.previous_hash != "0" || genesis.data != "Genesis Block" {
            println!("Genesis block properties are invalid");
            return false;
        }

        // Vérifier que le hash du genesis est correct
        if genesis.hash != genesis.calculate_hash() {
            println!("Genesis block hash is invalid");
            return false;
        }

        // Vérifier tous les autres blocs
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // Vérifier que le hash est correct
            if current.hash != current.calculate_hash() {
                println!("Invalid hash at block index {}", current.index);
                return false;
            }

            // Vérifier le lien avec le bloc précédent
            if current.previous_hash != previous.hash {
                println!("Invalid previous hash at block index {}", current.index);
                return false;
            }

            // Vérifier que l'index est séquentiel
            if current.index != previous.index + 1 {
                println!("Invalid index at block index {}", current.index);
                return false;
            }
        }

        true
    }

    /// Obtenir le dernier bloc de la chaîne
    ///
    /// # Returns
    /// Référence au dernier bloc
    pub fn latest_block(&self) -> &Block {
        self.chain
            .last()
            .expect("La chaîne doit toujours avoir au moins un bloc")
    }

    /// Obtenir un bloc par son index
    ///
    /// # Arguments
    /// * `index` - Index du bloc recherché
    ///
    /// # Returns
    /// Option contenant une référence au bloc si trouvé
    pub fn get_block(&self, index: u64) -> Option<&Block> {
        self.chain.get(index as usize)
    }

    /// Obtenir la taille de la chaîne
    ///
    /// # Returns
    /// Nombre de blocs dans la chaîne
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Vérifier si la chaîne est vide
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        // Créer une blockchain avec difficulté 2
        let blockchain = Blockchain::new(2);

        // Vérifier qu'elle a un bloc genesis
        assert_eq!(blockchain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert_eq!(blockchain.chain[0].data, "Genesis Block");
        assert_eq!(blockchain.difficulty, 2);
    }

    #[test]
    fn test_add_block() {
        // Créer une blockchain avec difficulté faible pour un test rapide
        let mut blockchain = Blockchain::new(1);

        // Ajouter plusieurs blocs
        let result1 = blockchain.add_block("First block".to_string());
        assert!(result1.is_ok());
        assert_eq!(blockchain.len(), 2);

        let result2 = blockchain.add_block("Second block".to_string());
        assert!(result2.is_ok());
        assert_eq!(blockchain.len(), 3);

        // Vérifier les propriétés des blocs
        assert_eq!(blockchain.chain[1].index, 1);
        assert_eq!(blockchain.chain[1].data, "First block");
        assert_eq!(blockchain.chain[2].index, 2);
        assert_eq!(blockchain.chain[2].data, "Second block");

        // Vérifier que les hash commencent par "0" (difficulté 1)
        assert!(blockchain.chain[1].hash.starts_with("0"));
        assert!(blockchain.chain[2].hash.starts_with("0"));

        // Vérifier l'erreur avec données vides
        let result_empty = blockchain.add_block("".to_string());
        assert!(result_empty.is_err());
    }

    #[test]
    fn test_chain_validation() {
        // Créer une blockchain avec difficulté 1
        let mut blockchain = Blockchain::new(1);

        // Ajouter des blocs
        blockchain.add_block("Block 1".to_string()).unwrap();
        blockchain.add_block("Block 2".to_string()).unwrap();
        blockchain.add_block("Block 3".to_string()).unwrap();

        // Vérifier que la chaîne est valide
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_invalid_chain_detection() {
        // Créer une blockchain valide
        let mut blockchain = Blockchain::new(1);
        blockchain.add_block("Block 1".to_string()).unwrap();
        blockchain.add_block("Block 2".to_string()).unwrap();

        // Vérifier que la chaîne est initialement valide
        assert!(blockchain.is_valid());

        // Altérer un bloc (modifier les données sans recalculer le hash)
        blockchain.chain[1].data = "Modified data".to_string();

        // Vérifier que la chaîne est maintenant invalide
        assert!(!blockchain.is_valid());
    }
}
