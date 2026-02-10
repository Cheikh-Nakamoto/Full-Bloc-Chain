use crate::api::models::*;
use crate::blockchain::SharedBlockchain;
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
    // TODO: Implémenter get_chain()
    // 1. Acquérir le verrou en lecture : let bc = blockchain.read().unwrap();
    // 2. Créer ChainResponse :
    //    - chain: cloner la chaîne bc.chain.clone()
    //    - length: bc.len()
    //    - is_valid: bc.is_valid()
    // 3. Retourner Ok(Json(response))
    todo!("Implémenter get_chain()")
}

/// Handler pour POST /blocks
/// Ajoute un nouveau bloc à la chaîne
pub async fn add_block(
    State(blockchain): State<SharedBlockchain>,
    Json(payload): Json<AddBlockRequest>,
) -> Result<Json<AddBlockResponse>, (StatusCode, Json<ErrorResponse>)> {
    // TODO: Implémenter add_block()
    // 1. Vérifier que data n'est pas vide :
    //    if payload.data.is_empty() {
    //        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse { error: "...".to_string() })));
    //    }
    //
    // 2. Acquérir le verrou en écriture : let mut bc = blockchain.write().unwrap();
    //
    // 3. Ajouter le bloc :
    //    match bc.add_block(payload.data) {
    //        Ok(block) => Ok(Json(AddBlockResponse {
    //            block: block.clone(),
    //            message: "Block added successfully".to_string(),
    //        })),
    //        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e.to_string() }))),
    //    }
    todo!("Implémenter add_block()")
}

/// Handler pour GET /blocks/:index
/// Retourne un bloc spécifique par son index
pub async fn get_block(
    State(blockchain): State<SharedBlockchain>,
    Path(index): Path<u64>,
) -> Result<Json<Block>, StatusCode> {
    // TODO: Implémenter get_block()
    // 1. Acquérir le verrou en lecture : let bc = blockchain.read().unwrap();
    // 2. Chercher le bloc : bc.get_block(index)
    // 3. Si trouvé, cloner et retourner : .cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
    todo!("Implémenter get_block()")
}

/// Handler pour GET /validate
/// Valide l'intégrité de la chaîne
pub async fn validate_chain(State(blockchain): State<SharedBlockchain>) -> Json<serde_json::Value> {
    // TODO: Implémenter validate_chain()
    // 1. Acquérir le verrou en lecture : let bc = blockchain.read().unwrap();
    // 2. Créer un JSON avec :
    //    - is_valid: bc.is_valid()
    //    - chain_length: bc.len()
    // 3. Utiliser serde_json::json!({ "is_valid": ..., "chain_length": ... })
    todo!("Implémenter validate_chain()")
}
