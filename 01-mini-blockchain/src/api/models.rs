use crate::block::Block;
use serde::{Deserialize, Serialize};

/// Requête pour ajouter un nouveau bloc
#[derive(Debug, Deserialize)]
pub struct AddBlockRequest {
    pub data: String,
}

/// Réponse après l'ajout d'un bloc
#[derive(Debug, Serialize)]
pub struct AddBlockResponse {
    pub block: Block,
    pub message: String,
}

/// Réponse pour obtenir toute la chaîne
#[derive(Debug, Serialize)]
pub struct ChainResponse {
    pub chain: Vec<Block>,
    pub length: usize,
    pub is_valid: bool,
}

/// Réponse en cas d'erreur
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
