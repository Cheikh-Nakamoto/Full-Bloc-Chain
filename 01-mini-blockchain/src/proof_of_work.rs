use crate::block::Block;

/// Miner un bloc avec l'algorithme Proof of Work
///
/// # Arguments
/// * `block` - Le bloc à miner (mutable)
/// * `difficulty` - Nombre de zéros requis au début du hash
///
/// # Description
/// Cette fonction modifie le bloc en trouvant un nonce qui produit
/// un hash commençant par N zéros (N = difficulty)
pub fn mine_block(block: &mut Block, difficulty: usize) {
    // TODO: Implémenter l'algorithme de mining
    // 1. Créer la cible : une chaîne de N zéros
    //    let target = "0".repeat(difficulty);
    // 2. Boucle infinie :
    //    - Calculer le hash du bloc avec block.calculate_hash()
    //    - Si le hash commence par la cible (use starts_with):
    //      * Assigner le hash à block.hash
    //      * Sortir de la boucle (break)
    //    - Sinon :
    //      * Incrémenter block.nonce
    //      * Continuer la boucle
    //
    // Note : Cette fonction est appelée par Blockchain::add_block()
    todo!("Implémenter mine_block()")
}

/// Vérifier qu'un hash respecte la difficulté (optionnel)
///
/// # Arguments
/// * `hash` - Le hash à vérifier
/// * `difficulty` - Le nombre de zéros requis
///
/// # Returns
/// true si le hash commence par N zéros, false sinon
#[allow(dead_code)]
pub fn verify_proof_of_work(hash: &str, difficulty: usize) -> bool {
    // TODO: (Optionnel) Vérifier si un hash est valide
    // 1. Créer la cible : "0".repeat(difficulty)
    // 2. Vérifier si hash commence par la cible
    // 3. Retourner le résultat
    todo!("Implémenter verify_proof_of_work() (optionnel)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_difficulty_1() {
        // TODO: Tester le mining avec difficulté 1
        // Créer un bloc et le miner, vérifier que le hash commence par "0"
    }

    #[test]
    fn test_mining_difficulty_2() {
        // TODO: Tester le mining avec difficulté 2
        // Vérifier que le hash commence par "00"
    }
}
