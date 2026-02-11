use crate::block::Block;

/// Miner un bloc avec l'algorithme Proof of Work
///
/// # Arguments
/// * `block` - Le bloc à miner (mutable)
/// * `difficulty` - Nombre de zéros requis au début du hash
///
/// # Description
/// Cette fonction modifie le bloc en trouvant un nonce qui produit
/// un hash commençant par N zéros (N = difficulty). Elle incrémente
/// le nonce jusqu'à trouver un hash valide.
pub fn mine_block(block: &mut Block, difficulty: usize) {
    let target = "0".repeat(difficulty);

    loop {
        let hash = block.calculate_hash();
        if hash.starts_with(&target) {
            block.hash = hash;
            break;
        } else {
            block.nonce += 1;
        }
    }
}

/// Vérifier qu'un hash respecte la difficulté du Proof of Work
///
/// # Arguments
/// * `hash` - Le hash à vérifier
/// * `difficulty` - Le nombre de zéros requis au début du hash
///
/// # Returns
/// true si le hash commence par N zéros, false sinon
///
/// # Exemple
/// ```
/// let hash = "0000abc123...";
/// assert!(verify_proof_of_work(hash, 4)); // true car commence par "0000"
/// ```
#[allow(dead_code)]
pub fn verify_proof_of_work(hash: &str, difficulty: usize) -> bool {
    let target = "0".repeat(difficulty);
    hash.starts_with(&target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_difficulty_1() {
        // Créer un bloc de test
        let mut block = Block::new(1, "Test mining".to_string(), "prev_hash".to_string());

        // Miner le bloc avec difficulté 1
        mine_block(&mut block, 1);

        // Vérifier que le hash commence par "0"
        assert!(block.hash.starts_with("0"));

        // Vérifier que le hash est valide
        assert_eq!(block.hash, block.calculate_hash());

        // Vérifier avec la fonction de vérification
        assert!(verify_proof_of_work(&block.hash, 1));
    }

    #[test]
    fn test_mining_difficulty_2() {
        // Créer un bloc de test
        let mut block = Block::new(2, "Mining test 2".to_string(), "previous".to_string());

        // Miner le bloc avec difficulté 2
        mine_block(&mut block, 2);

        // Vérifier que le hash commence par "00"
        assert!(block.hash.starts_with("00"));

        // Vérifier que le hash est valide
        assert_eq!(block.hash, block.calculate_hash());

        // Vérifier avec la fonction de vérification
        assert!(verify_proof_of_work(&block.hash, 2));

        // Vérifier qu'il ne satisfait pas une difficulté inférieure incorrectement
        // (un hash commençant par "00" satisfait aussi difficulté 1)
        assert!(verify_proof_of_work(&block.hash, 1));
    }
}
