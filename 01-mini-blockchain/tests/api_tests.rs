// Tests d'intégration API
// Note: Ces tests nécessitent que le serveur soit en cours d'exécution

#[tokio::test]
#[ignore] // Ignorer par défaut car nécessite un serveur en cours
async fn test_api_get_chain() {
    // TODO: Tester GET /chain
    // 1. Créer un client reqwest
    // 2. Faire une requête GET à http://127.0.0.1:8080/chain
    // 3. Vérifier le status code
    // 4. Parser la réponse JSON
}

#[tokio::test]
#[ignore]
async fn test_api_add_block() {
    // TODO: Tester POST /blocks
    // 1. Créer un client reqwest
    // 2. Créer le payload JSON : {"data": "Test transaction"}
    // 3. Faire une requête POST
    // 4. Vérifier la réponse
}

#[tokio::test]
#[ignore]
async fn test_api_get_block() {
    // TODO: Tester GET /blocks/:index
    // Tester avec un index valide et invalide
}

#[tokio::test]
#[ignore]
async fn test_api_validate() {
    // TODO: Tester GET /validate
}
