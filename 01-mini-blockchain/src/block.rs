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
        // TODO: Initialiser tous les champs
        // - index: utiliser le paramètre
        // - timestamp: Utc::now()
        // - data: utiliser le paramètre
        // - previous_hash: utiliser le paramètre
        // - hash: String vide au début
        // - nonce: 0
        todo!("Implémenter Block::new()")
    }

    /// Calculer le hash SHA-256 du bloc
    ///
    /// # Returns
    /// String hexadécimal de 64 caractères représentant le hash
    pub fn calculate_hash(&self) -> String {
        // TODO: Calculer le hash du bloc
        // 1. Créer une chaîne en concaténant : index + timestamp + data + previous_hash + nonce
        //    Utiliser format!() ou format!("{}{}{}{}{}", ...)
        //    Pour timestamp, utiliser self.timestamp.to_rfc3339()
        // 2. Créer un hasher SHA-256 : let mut hasher = Sha256::new();
        // 3. Hasher la chaîne : hasher.update(input.as_bytes());
        // 4. Finaliser et encoder en hex : hex::encode(hasher.finalize())
        todo!("Implémenter calculate_hash()")
    }

    /// Miner le bloc avec Proof of Work
    ///
    /// # Arguments
    /// * `difficulty` - Nombre de zéros requis au début du hash
    pub fn mine_block(&mut self, difficulty: usize) {
        // TODO: Implémenter l'algorithme de mining
        // 1. Créer la cible : let target = "0".repeat(difficulty);
        // 2. Boucle infinie :
        //    - Calculer le hash avec self.calculate_hash()
        //    - Si hash commence par la cible (starts_with), assigner à self.hash et sortir
        //    - Sinon, incrémenter self.nonce et continuer
        todo!("Implémenter mine_block()")
    }

    /// Créer le bloc genesis (premier bloc de la chaîne)
    ///
    /// # Returns
    /// Le bloc genesis avec index 0 et previous_hash "0"
    pub fn genesis() -> Self {
        // TODO: Créer le bloc genesis
        // 1. Appeler Block::new(0, "Genesis Block".to_string(), "0".to_string())
        // 2. Calculer son hash avec calculate_hash()
        // 3. Assigner le hash au bloc
        // 4. Retourner le bloc
        todo!("Implémenter genesis()")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        // TODO: Tester la création d'un bloc
        // Créer un bloc et vérifier ses champs
    }

    #[test]
    fn test_calculate_hash() {
        // TODO: Tester le calcul de hash
        // Vérifier que le hash fait 64 caractères
    }

    #[test]
    fn test_genesis_block() {
        // TODO: Tester le bloc genesis
        // Vérifier que l'index est 0 et previous_hash est "0"
    }
}
