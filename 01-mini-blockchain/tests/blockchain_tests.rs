use mini_blockchain::{Block, Blockchain};

#[test]
fn test_full_blockchain_workflow() {
    // TODO: Tester le workflow complet
    // 1. Créer une blockchain : let mut bc = Blockchain::new(3);
    // 2. Ajouter 5 blocs : bc.add_block("Transaction X".to_string()).unwrap();
    // 3. Vérifier la longueur : assert_eq!(bc.len(), 6); // Genesis + 5
    // 4. Vérifier la validité : assert!(bc.is_valid());
    // 5. Vérifier les liens entre blocs
}

#[test]
fn test_chain_validation() {
    // TODO: Tester la validation
    // Créer une blockchain, ajouter des blocs, et vérifier is_valid()
}

#[test]
fn test_tampered_chain_detection() {
    // TODO: Tester la détection d'altération
    // 1. Créer une blockchain et ajouter des blocs
    // 2. Altérer un bloc : bc.chain[1].data = "Tampered".to_string();
    // 3. Vérifier que is_valid() retourne false
}
