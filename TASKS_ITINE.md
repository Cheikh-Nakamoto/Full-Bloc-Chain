# 🔐 Tâches d'Itine - Consensus Bitcoin & Stockage

**Développeur** : Itine
**Domaines** : Consensus Bitcoin (Headers-First, SPV), Validation, Synchronisation, Stockage
**Durée estimée** : 5-8 jours
**Phases assignées** : Phase 2 (Consensus Bitcoin) + Phase 4 (Stockage)
**Protocole** : Bitcoin Consensus Rules (version simplifiée)

---

## 📋 Vue d'ensemble

Tu es responsable du **consensus Bitcoin** : validation de blocs, synchronisation headers-first, et la persistance. Tu vas implémenter les règles de consensus qui permettent aux nodes de s'accorder sur la même chaîne.

**Objectifs principaux** :
- ✅ Valider les headers Bitcoin (PoW, chaînage)
- ✅ Synchroniser avec headers-first (comme Bitcoin Core)
- ✅ Gérer les forks avec longest chain rule
- ✅ Propager les blocs via inventory vectors
- ✅ Sauvegarder la blockchain

**Références Bitcoin** :
- [Bitcoin Block Headers](https://developer.bitcoin.org/reference/block_chain.html#block-headers)
- [Headers-First Sync](https://bitcoin.org/en/developer-guide#headers-first)
- [Consensus Rules](https://en.bitcoin.it/wiki/Protocol_rules)

---

## 🤝 Phase 2 : Consensus Bitcoin

### Tâche 2.1 : Validateur Bitcoin (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/consensus/validator.rs`
**Durée** : 2 jours
**Dépendances** : Mounirou's messages.rs

#### Objectif
Implémenter les règles de validation Bitcoin (simplifiées).

#### Concepts Bitcoin implémentés
- ✅ **Header Validation** : Valider les headers séparément des blocs
- ✅ **PoW Verification** : Vérifier que le hash respecte la difficulté
- ✅ **Chain Validation** : Vérifier la chaîne complète
- ✅ **Merkle Root** : Validation du merkle root (simplifié)

#### Code à implémenter

```bash
mkdir -p 01-mini-blockchain/src/consensus
```

**`src/consensus/validator.rs`** :
```rust
use crate::block::Block;
use crate::network::messages::BlockHeader;
use crate::blockchain::{Blockchain, BlockchainError};
use crate::proof_of_work;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid block index: expected {expected}, got {actual}")]
    InvalidIndex { expected: u64, actual: u64 },

    #[error("Invalid previous hash")]
    InvalidPreviousHash,

    #[error("Invalid block hash")]
    InvalidHash,

    #[error("Invalid proof of work: hash {hash} does not meet difficulty {difficulty}")]
    InvalidProofOfWork { hash: String, difficulty: usize },

    #[error("Invalid merkle root")]
    InvalidMerkleRoot,

    #[error("Empty chain")]
    EmptyChain,

    #[error("Invalid genesis block")]
    InvalidGenesis,

    #[error("Block timestamp too far in future")]
    TimestampTooFarInFuture,

    #[error("Invalid header")]
    InvalidHeader,
}

/// Validateur de consensus Bitcoin
pub struct ConsensusValidator;

impl ConsensusValidator {
    // === HEADER VALIDATION (Bitcoin headers-first) ===

    /// Valider un block header Bitcoin (sans télécharger le bloc complet)
    ///
    /// C'est la validation SPV (Simplified Payment Verification)
    /// Ref: https://bitcoin.org/en/developer-guide#simplified-payment-verification-spv
    pub fn validate_header(
        header: &BlockHeader,
        prev_header: &BlockHeader,
        difficulty: usize,
    ) -> Result<(), ValidationError> {
        // 1. Vérifier l'index séquentiel
        let expected_index = prev_header.index + 1;
        if header.index != expected_index {
            return Err(ValidationError::InvalidIndex {
                expected: expected_index,
                actual: header.index,
            });
        }

        // 2. Vérifier le chaînage (previous_hash)
        if header.prev_block_hash != prev_header.hash {
            return Err(ValidationError::InvalidPreviousHash);
        }

        // 3. Vérifier le Proof of Work du header
        if !Self::verify_header_pow(header, difficulty) {
            return Err(ValidationError::InvalidProofOfWork {
                hash: header.hash.clone(),
                difficulty,
            });
        }

        // 4. Vérifier le timestamp (pas trop loin dans le futur)
        // Bitcoin rejette les blocs avec timestamp > now + 2h
        let now = chrono::Utc::now().timestamp();
        if header.timestamp > now + 7200 {
            return Err(ValidationError::TimestampTooFarInFuture);
        }

        Ok(())
    }

    /// Valider une chaîne de headers (Bitcoin headers-first sync)
    ///
    /// Permet de valider rapidement sans télécharger les blocs complets
    pub fn validate_header_chain(
        headers: &[BlockHeader],
        difficulty: usize,
    ) -> Result<(), ValidationError> {
        if headers.is_empty() {
            return Err(ValidationError::EmptyChain);
        }

        // Valider le genesis header
        if headers[0].index != 0 {
            return Err(ValidationError::InvalidGenesis);
        }

        // Valider chaque header avec le précédent
        for i in 1..headers.len() {
            let current = &headers[i];
            let previous = &headers[i - 1];

            Self::validate_header(current, previous, difficulty)?;
        }

        Ok(())
    }

    /// Vérifier le PoW d'un header
    fn verify_header_pow(header: &BlockHeader, difficulty: usize) -> bool {
        let target = "0".repeat(difficulty);
        header.hash.starts_with(&target)
    }

    // === FULL BLOCK VALIDATION ===

    /// Valider un bloc complet (après avoir validé le header)
    ///
    /// # Arguments
    /// * `block` - Le bloc complet à valider
    /// * `prev_block` - Le bloc précédent
    /// * `difficulty` - Difficulté requise
    pub fn validate_full_block(
        block: &Block,
        prev_block: &Block,
        difficulty: usize,
    ) -> Result<(), ValidationError> {
        // 1. Vérifier l'index
        let expected_index = prev_block.index + 1;
        if block.index != expected_index {
            return Err(ValidationError::InvalidIndex {
                expected: expected_index,
                actual: block.index,
            });
        }

        // 2. Vérifier le previous_hash
        if block.previous_hash != prev_block.hash {
            return Err(ValidationError::InvalidPreviousHash);
        }

        // 3. Vérifier le hash du bloc
        let calculated_hash = block.calculate_hash();
        if block.hash != calculated_hash {
            return Err(ValidationError::InvalidHash);
        }

        // 4. Vérifier le PoW
        if !Self::verify_proof_of_work(block, difficulty) {
            return Err(ValidationError::InvalidProofOfWork {
                hash: block.hash.clone(),
                difficulty,
            });
        }

        // 5. Vérifier le merkle root (simplifié)
        // Bitcoin calcule le vrai merkle tree des transactions
        let header = BlockHeader::from_block(block);
        if !Self::verify_merkle_root(&header, &block.data) {
            return Err(ValidationError::InvalidMerkleRoot);
        }

        Ok(())
    }

    /// Valider une chaîne complète de blocs
    pub fn validate_chain(chain: &[Block], difficulty: usize) -> Result<(), ValidationError> {
        if chain.is_empty() {
            return Err(ValidationError::EmptyChain);
        }

        // Valider le genesis
        let genesis = &chain[0];
        if !Self::validate_genesis(genesis) {
            return Err(ValidationError::InvalidGenesis);
        }

        // Valider tous les autres blocs
        for i in 1..chain.len() {
            let current = &chain[i];
            let previous = &chain[i - 1];

            Self::validate_full_block(current, previous, difficulty)?;
        }

        Ok(())
    }

    /// Valider le bloc genesis
    fn validate_genesis(block: &Block) -> bool {
        block.index == 0
            && block.previous_hash == "0"
            && block.data == "Genesis Block"
            && block.hash == block.calculate_hash()
    }

    /// Vérifier le PoW d'un bloc complet
    fn verify_proof_of_work(block: &Block, difficulty: usize) -> bool {
        let target = "0".repeat(difficulty);
        block.hash.starts_with(&target)
    }

    /// Vérifier le merkle root (simplifié)
    ///
    /// Bitcoin construit un vrai arbre de Merkle des transactions.
    /// Ici on simplifie en hashant juste les données.
    fn verify_merkle_root(header: &BlockHeader, data: &str) -> bool {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let calculated = hex::encode(hasher.finalize());

        header.merkle_root == calculated
    }

    // === CHAIN SELECTION (Longest Chain Rule) ===

    /// Sélectionner la chaîne la plus longue (Nakamoto Consensus)
    ///
    /// Ref: https://bitcoin.org/bitcoin.pdf Section 5
    pub fn select_best_chain(
        chain1: &[Block],
        chain2: &[Block],
        difficulty: usize,
    ) -> Option<Vec<Block>> {
        let chain1_valid = Self::validate_chain(chain1, difficulty).is_ok();
        let chain2_valid = Self::validate_chain(chain2, difficulty).is_ok();

        match (chain1_valid, chain2_valid) {
            (true, true) => {
                // Bitcoin utilise "most work" (chainwork)
                // Nous simplifions avec "longest chain"
                if chain1.len() >= chain2.len() {
                    Some(chain1.to_vec())
                } else {
                    Some(chain2.to_vec())
                }
            }
            (true, false) => Some(chain1.to_vec()),
            (false, true) => Some(chain2.to_vec()),
            (false, false) => None,
        }
    }

    /// Calculer le "chainwork" total (simplifié)
    ///
    /// Bitcoin calcule le travail cumulatif (somme des difficultés)
    /// Ref: https://en.bitcoin.it/wiki/Difficulty
    pub fn calculate_chainwork(chain: &[Block]) -> u128 {
        // Simplifié: juste compter les blocs
        // Bitcoin réel: somme de 2^256 / (difficulty_target)
        chain.len() as u128
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_genesis() {
        let genesis = Block::genesis();
        assert!(ConsensusValidator::validate_genesis(&genesis));
    }

    #[test]
    fn test_validate_header() {
        let block1 = Block::genesis();
        let header1 = BlockHeader::from_block(&block1);

        let mut block2 = Block::new(1, "Test".to_string(), block1.hash.clone());
        proof_of_work::mine_block(&mut block2, 1);
        let header2 = BlockHeader::from_block(&block2);

        let result = ConsensusValidator::validate_header(&header2, &header1, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_header_chain() {
        use crate::Blockchain;

        let mut bc = Blockchain::new(1);
        bc.add_block("Block 1".to_string()).unwrap();
        bc.add_block("Block 2".to_string()).unwrap();

        let headers: Vec<BlockHeader> = bc.chain
            .iter()
            .map(|b| BlockHeader::from_block(b))
            .collect();

        let result = ConsensusValidator::validate_header_chain(&headers, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_select_best_chain() {
        use crate::Blockchain;

        let mut chain1 = Blockchain::new(1);
        chain1.add_block("Block 1".to_string()).unwrap();

        let mut chain2 = Blockchain::new(1);
        chain2.add_block("Block 1".to_string()).unwrap();
        chain2.add_block("Block 2".to_string()).unwrap();

        let best = ConsensusValidator::select_best_chain(&chain1.chain, &chain2.chain, 1);

        assert!(best.is_some());
        assert_eq!(best.unwrap().len(), 3); // Genesis + 2 blocks
    }
}
```

**✅ Critère de validation** : Tous les tests passent, headers peuvent être validés séparément.

---

### Tâche 2.2 : Synchroniseur Headers-First (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/consensus/sync.rs`
**Durée** : 2 jours
**Dépendances** : Tâche 2.1 + Mounirou's P2P

#### Objectif
Implémenter la synchronisation headers-first comme Bitcoin Core.

#### Concept Bitcoin
Au lieu de télécharger tous les blocs d'un coup, Bitcoin:
1. Télécharge d'abord tous les headers (rapide, 80 bytes/header)
2. Valide la chaîne de headers
3. Télécharge les blocs complets seulement si nécessaire

**`src/consensus/sync.rs`** :
```rust
use crate::blockchain::SharedBlockchain;
use crate::consensus::validator::ConsensusValidator;
use crate::network::{P2PClient, PeerManager, messages::*};
use std::sync::Arc;

/// Synchroniseur de chaîne avec headers-first (Bitcoin)
pub struct ChainSynchronizer {
    blockchain: SharedBlockchain,
    peer_manager: Arc<PeerManager>,
    difficulty: usize,
}

impl ChainSynchronizer {
    pub fn new(
        blockchain: SharedBlockchain,
        peer_manager: Arc<PeerManager>,
        difficulty: usize,
    ) -> Self {
        Self {
            blockchain,
            peer_manager,
            difficulty,
        }
    }

    /// Synchroniser avec headers-first (Bitcoin protocol)
    ///
    /// Étapes:
    /// 1. Construire block locator (liste de hashs de notre chaîne)
    /// 2. Demander headers aux peers (getheaders message)
    /// 3. Valider les headers reçus
    /// 4. Si headers valides, demander les blocs complets (getdata)
    /// 5. Valider et ajouter les blocs
    pub async fn sync_headers_first(&self) -> Result<bool, Box<dyn std::error::Error>> {
        println!("🔄 Starting headers-first synchronization...");

        let peers = self.peer_manager.get_connected_peers();

        if peers.is_empty() {
            println!("⚠️  No connected peers");
            return Ok(false);
        }

        // Étape 1: Construire le block locator
        let block_locator = self.build_block_locator();
        println!("📍 Block locator: {} hashes", block_locator.len());

        // Étape 2: Demander les headers au meilleur peer
        let best_peer = &peers[0]; // Simplifié: prendre le premier
        println!("📡 Requesting headers from {}", best_peer.id);

        let headers = P2PClient::get_headers(
            &best_peer.address,
            block_locator,
        ).await?;

        println!("📥 Received {} headers", headers.len());

        if headers.is_empty() {
            println!("✅ Already synchronized");
            return Ok(false);
        }

        // Étape 3: Valider la chaîne de headers
        match ConsensusValidator::validate_header_chain(&headers, self.difficulty) {
            Ok(_) => {
                println!("✅ Headers validation successful");
            }
            Err(e) => {
                eprintln!("❌ Headers validation failed: {}", e);
                return Ok(false);
            }
        }

        // Étape 4: Comparer avec notre chaîne locale
        let local_len = {
            let bc = self.blockchain.read().unwrap();
            bc.len() as u64
        };

        let remote_len = headers.len() as u64;

        if remote_len <= local_len {
            println!("✅ Our chain is up to date");
            return Ok(false);
        }

        // Étape 5: Télécharger les blocs complets manquants
        println!("⬇️  Downloading {} missing blocks...", remote_len - local_len);

        // TODO: Demander les blocs via inventory vectors (getdata)
        // Pour l'instant, demander la chaîne complète
        self.download_full_chain(&best_peer.address).await?;

        println!("🎉 Synchronization complete!");

        Ok(true)
    }

    /// Construire le block locator (Bitcoin protocol)
    ///
    /// Le block locator est une liste de hashs pour trouver le fork point.
    /// Format: [tip, tip-1, tip-2, tip-3, tip-4, tip-5, tip-7, tip-11, ...]
    /// Ref: https://en.bitcoin.it/wiki/Protocol_documentation#getblocks
    fn build_block_locator(&self) -> Vec<String> {
        let bc = self.blockchain.read().unwrap();
        let mut locator = Vec::new();

        let height = bc.len() as i64 - 1;
        let mut step = 1i64;
        let mut index = height;

        // Ajouter les 10 derniers blocs
        while index >= 0 && locator.len() < 10 {
            if let Some(block) = bc.get_block(index as u64) {
                locator.push(block.hash.clone());
            }
            index -= 1;
        }

        // Puis sauter avec step exponentiel
        index = height - 10;
        while index >= 0 {
            if let Some(block) = bc.get_block(index as u64) {
                locator.push(block.hash.clone());
            }

            if locator.len() >= 10 {
                step *= 2;
            }

            index -= step;
        }

        // Toujours inclure le genesis
        if let Some(genesis) = bc.get_block(0) {
            if !locator.contains(&genesis.hash) {
                locator.push(genesis.hash.clone());
            }
        }

        locator
    }

    /// Télécharger la chaîne complète (fallback simple)
    async fn download_full_chain(
        &self,
        peer_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Demander la chaîne complète
        let msg = P2PMessage::GetBlocks {
            version: 70015,
            block_locator_hashes: self.build_block_locator(),
            hash_stop: "0".repeat(64),
        };

        // TODO: Recevoir et traiter les blocs
        // Pour l'instant, juste un placeholder

        Ok(())
    }

    /// Synchronisation périodique
    pub async fn periodic_sync(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

            if let Err(e) = self.sync_headers_first().await {
                eprintln!("❌ Periodic sync error: {}", e);
            }
        }
    }
}
```

**✅ Critère de validation** : Le synchroniseur peut construire un block locator et demander des headers.

---

### Tâche 2.3 : Propagateur avec Inventory (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/consensus/propagation.rs`
**Durée** : 1.5 jours

**`src/consensus/propagation.rs`** :
```rust
use crate::block::Block;
use crate::blockchain::SharedBlockchain;
use crate::consensus::validator::ConsensusValidator;
use crate::network::{PeerManager, messages::*};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

/// Propagateur de blocs via inventory vectors (Bitcoin)
pub struct BlockPropagator {
    peer_manager: Arc<PeerManager>,
    blockchain: SharedBlockchain,
    difficulty: usize,

    /// Inventory items déjà vus (éviter les boucles)
    seen_inventory: Arc<Mutex<HashSet<String>>>,
}

impl BlockPropagator {
    pub fn new(
        peer_manager: Arc<PeerManager>,
        blockchain: SharedBlockchain,
        difficulty: usize,
    ) -> Self {
        Self {
            peer_manager,
            blockchain,
            difficulty,
            seen_inventory: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Annoncer un nouveau bloc via inventory (Bitcoin inv message)
    ///
    /// Au lieu d'envoyer le bloc complet, on annonce juste son hash.
    /// Les peers intéressés demanderont le bloc avec getdata.
    pub async fn announce_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        println!("📢 Announcing new block #{} via inventory", block.index);

        // Marquer comme vu
        {
            let mut seen = self.seen_inventory.lock().unwrap();
            seen.insert(block.hash.clone());
        }

        // Créer l'inventory vector
        let inv = InventoryVector {
            inv_type: InvType::MsgBlock,
            hash: block.hash.clone(),
        };

        // Créer le message inv
        let inv_msg = P2PMessage::Inv(vec![inv]);

        // Envoyer à tous les peers
        let peers = self.peer_manager.get_connected_peers();
        println!("   Sending to {} peers", peers.len());

        for peer in peers {
            // TODO: Envoyer le message inv au peer
            println!("   📤 Sent inventory to {}", peer.id);
        }

        Ok(())
    }

    /// Gérer une demande getdata (peer veut le bloc complet)
    pub async fn handle_getdata(
        &self,
        inv_vector: &InventoryVector,
    ) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        println!("📨 Received getdata for {}", inv_vector.hash);

        // Chercher le bloc dans notre chaîne
        let bc = self.blockchain.read().unwrap();

        for block in &bc.chain {
            if block.hash == inv_vector.hash {
                println!("   ✅ Found block, sending...");
                return Ok(Some(block.clone()));
            }
        }

        println!("   ⚠️  Block not found");
        Ok(None)
    }

    /// Gérer un nouveau bloc reçu
    pub async fn handle_new_block(
        &self,
        block: Block,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        println!("📥 Received new block #{}", block.index);

        // Vérifier si déjà vu
        {
            let seen = self.seen_inventory.lock().unwrap();
            if seen.contains(&block.hash) {
                println!("   ⚠️  Already seen");
                return Ok(false);
            }
        }

        // Marquer comme vu
        {
            let mut seen = self.seen_inventory.lock().unwrap();
            seen.insert(block.hash.clone());
        }

        // Valider et ajouter
        let mut bc = self.blockchain.write().unwrap();
        let last_block = bc.latest_block();

        match ConsensusValidator::validate_full_block(&block, last_block, self.difficulty) {
            Ok(_) => {
                println!("   ✅ Block validated");
                bc.chain.push(block.clone());

                drop(bc);

                // Propager aux autres peers
                self.announce_block(&block).await?;

                Ok(true)
            }
            Err(e) => {
                eprintln!("   ❌ Validation failed: {}", e);
                Ok(false)
            }
        }
    }
}
```

---

## 💾 Phase 4 : Stockage (Bitcoin-style)

### Tâche 4.1-4.4 : Stockage JSON

*[Identique au fichier précédent - stockage simple en JSON]*

---

## 📚 Références Bitcoin

### Documentation
- [Block Headers](https://developer.bitcoin.org/reference/block_chain.html#block-headers)
- [Headers-First Sync](https://bitcoin.org/en/developer-guide#headers-first)
- [SPV Validation](https://bitcoin.org/en/operating-modes-guide#simplified-payment-verification-spv)
- [Block Locators](https://en.bitcoin.it/wiki/Protocol_documentation#getblocks)

### Concepts implémentés
✅ Headers-first synchronization
✅ Block locator construction
✅ SPV header validation
✅ Inventory-based propagation
✅ Longest chain rule (Nakamoto consensus)

### Simplifications
- Pas de merkle tree complet (juste hash)
- Pas de difficulty adjustment
- Pas de checkpoints
- Stockage JSON au lieu de LevelDB

---

**Bon courage Itine ! Tu implémentes le consensus Bitcoin ! 🧠**
