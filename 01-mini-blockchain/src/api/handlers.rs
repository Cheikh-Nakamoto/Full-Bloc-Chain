use crate::api::models::*;
use crate::blockchain::SharedBlockchain;
use crate::Block;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

/// Handler pour GET /chain
/// Retourne toute la blockchain
pub async fn get_chain(
    State(blockchain): State<SharedBlockchain>,
) -> Result<Json<ChainResponse>, StatusCode> {
    // Acquérir le verrou en lecture
    let bc = blockchain.read().unwrap();

    // Créer la réponse avec la chaîne complète
    let response = ChainResponse {
        chain: bc.chain.clone(),
        length: bc.chain.len(),
        is_valid: bc.is_valid(),
    };
    Ok(Json(response))
}

/// Handler pour POST /blocks
/// Ajoute un nouveau bloc à la chaîne
pub async fn add_block(
    State(blockchain): State<SharedBlockchain>,
    Json(payload): Json<AddBlockRequest>,
) -> Result<Json<AddBlockResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Vérifier que les données ne sont pas vides
    if payload.data.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Data cannot be empty".to_string(),
            }),
        ));
    }

    // Acquérir le verrou en écriture pour modifier la blockchain
    let mut bc = blockchain.write().unwrap();

    // Ajouter le bloc et gérer le résultat
    match bc.add_block(payload.data) {
        Ok(block) => Ok(Json(AddBlockResponse {
            block: block.clone(),
            message: "Block added successfully".to_string(),
        })),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )),
    }
}

/// Handler pour GET /blocks/:index
/// Retourne un bloc spécifique par son index
pub async fn get_block(
    State(blockchain): State<SharedBlockchain>,
    Path(index): Path<u64>,
) -> Result<Json<Block>, StatusCode> {
    // Acquérir le verrou en lecture
    let bc = blockchain.read().unwrap();

    // Rechercher et retourner le bloc, ou 404 si non trouvé
    bc.get_block(index)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

/// Handler pour GET /validate
/// Valide l'intégrité de la chaîne
pub async fn validate_chain(State(blockchain): State<SharedBlockchain>) -> Json<serde_json::Value> {
    // Acquérir le verrou en lecture
    let bc = blockchain.read().unwrap();

    // Créer la réponse avec le statut de validation et la longueur
    let response = serde_json::json!({
        "is_valid": bc.is_valid(),
        "chain_length": bc.len()
    });

    Json(response)
}
