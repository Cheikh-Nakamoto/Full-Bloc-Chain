pub mod api;
pub mod block;
pub mod blockchain;
pub mod proof_of_work;

// Réexporter les types principaux pour faciliter l'utilisation
pub use block::Block;
pub use blockchain::{Blockchain, BlockchainError, SharedBlockchain};
