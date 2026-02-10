use axum::{
    routing::{get, post},
    Router,
};
use mini_blockchain::{api, Blockchain};
use std::sync::{Arc, RwLock};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // TODO: Configurer le serveur API
    //
    // 1. Initialiser le système de logging
    //    tracing_subscriber::fmt::init();
    //
    // 2. Créer une blockchain avec difficulté 4
    //    let blockchain = Arc::new(RwLock::new(Blockchain::new(4)));
    //
    // 3. Créer le routeur Axum avec toutes les routes :
    //    let app = Router::new()
    //        .route("/", get(root))
    //        .route("/chain", get(api::get_chain))
    //        .route("/blocks", post(api::add_block))
    //        .route("/blocks/:index", get(api::get_block))
    //        .route("/validate", get(api::validate_chain))
    //        .with_state(blockchain)
    //        .layer(CorsLayer::permissive());
    //
    // 4. Créer le listener TCP sur 127.0.0.1:8080
    //    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
    //        .await
    //        .unwrap();
    //
    // 5. Afficher un message de démarrage
    //    println!("🚀 Blockchain API server running on http://127.0.0.1:8080");
    //
    // 6. Démarrer le serveur
    //    axum::serve(listener, app).await.unwrap();

    todo!("Implémenter main() pour démarrer le serveur API")
}

/// Handler pour GET /
/// Retourne un message de bienvenue
async fn root() -> &'static str {
    // TODO: Retourner un message de bienvenue
    // "Mini Blockchain API - See /chain for the blockchain"
    todo!("Implémenter root()")
}
