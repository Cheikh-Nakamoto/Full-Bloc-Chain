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
        // TODO: Créer une nouvelle blockchain
        // 1. Créer un vecteur vide : let mut chain = Vec::new();
        // 2. Créer le bloc genesis : let genesis = Block::genesis();
        // 3. Ajouter le genesis au vecteur : chain.push(genesis);
        // 4. Retourner Self { chain, difficulty }
        todo!("Implémenter Blockchain::new()")
    }

    /// Ajouter un nouveau bloc à la chaîne
    ///
    /// # Arguments
    /// * `data` - Données à stocker dans le bloc
    ///
    /// # Returns
    /// Result contenant une référence au bloc ajouté ou une erreur
    pub fn add_block(&mut self, data: String) -> Result<&Block, BlockchainError> {
        // TODO: Ajouter un nouveau bloc
        // 1. Vérifier que data n'est pas vide
        //    if data.is_empty() { return Err(BlockchainError::EmptyData); }
        // 2. Obtenir le dernier bloc : let last_block = self.latest_block();
        // 3. Calculer le nouvel index : let new_index = last_block.index + 1;
        // 4. Créer un nouveau bloc : let mut new_block = Block::new(new_index, data, last_block.hash.clone());
        // 5. Miner le bloc : proof_of_work::mine_block(&mut new_block, self.difficulty);
        // 6. Ajouter le bloc à la chaîne : self.chain.push(new_block);
        // 7. Retourner une référence au dernier bloc
        todo!("Implémenter add_block()")
    }

    /// Valider l'intégrité de toute la chaîne
    ///
    /// # Returns
    /// true si la chaîne est valide, false sinon
    pub fn is_valid(&self) -> bool {
        // TODO: Valider la blockchain
        // 1. Vérifier le bloc genesis :
        //    if self.chain[0] != Block::genesis() { return false; }
        //
        // 2. Pour chaque bloc (à partir de l'index 1) :
        //    for i in 1..self.chain.len() {
        //        let current = &self.chain[i];
        //        let previous = &self.chain[i - 1];
        //
        //        // Vérifier que le hash est correct
        //        if current.hash != current.calculate_hash() { return false; }
        //
        //        // Vérifier le lien avec le bloc précédent
        //        if current.previous_hash != previous.hash { return false; }
        //
        //        // Vérifier que l'index est séquentiel
        //        if current.index != previous.index + 1 { return false; }
        //    }
        //
        // 3. Si tout est OK, retourner true
        todo!("Implémenter is_valid()")
    }

    /// Obtenir le dernier bloc de la chaîne
    ///
    /// # Returns
    /// Référence au dernier bloc
    pub fn latest_block(&self) -> &Block {
        // TODO: Retourner le dernier bloc
        // Utiliser self.chain.last().unwrap()
        // (unwrap est OK ici car la chaîne a toujours au moins le genesis)
        todo!("Implémenter latest_block()")
    }

    /// Obtenir un bloc par son index
    ///
    /// # Arguments
    /// * `index` - Index du bloc recherché
    ///
    /// # Returns
    /// Option contenant une référence au bloc si trouvé
    pub fn get_block(&self, index: u64) -> Option<&Block> {
        // TODO: Retourner un bloc spécifique
        // 1. Chercher dans le vecteur : self.chain.iter().find(|b| b.index == index)
        // Ou utiliser : self.chain.get(index as usize)
        todo!("Implémenter get_block()")
    }

    /// Obtenir la taille de la chaîne
    ///
    /// # Returns
    /// Nombre de blocs dans la chaîne
    pub fn len(&self) -> usize {
        // TODO: Retourner la longueur
        // self.chain.len()
        todo!("Implémenter len()")
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
        // TODO: Tester la création d'une blockchain
        // Vérifier qu'elle a un bloc genesis
    }

    #[test]
    fn test_add_block() {
        // TODO: Tester l'ajout de blocs
        // Ajouter plusieurs blocs et vérifier la longueur
    }

    #[test]
    fn test_chain_validation() {
        // TODO: Tester la validation de chaîne valide
    }

    #[test]
    fn test_invalid_chain_detection() {
        // TODO: Tester la détection de chaîne altérée
        // Modifier un bloc et vérifier que is_valid() retourne false
    }
}
