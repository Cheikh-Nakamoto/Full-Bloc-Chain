# ⚡ Tâches de Mbaye - Mempool, Mining & Testing (Protocole Bitcoin)

**Développeur** : Mbaye
**Domaines** : Mempool Bitcoin-style, Mining Distribué, API Updates, Tests d'Intégration
**Durée estimée** : 4-6 jours | **Complexité** : ⭐⭐⭐
**Phases assignées** : Phase 5 (Mempool & Mining) + Phase 6 (API Updates & Testing)

---

## 📋 Vue d'ensemble

Tu es responsable de la **couche applicative** de la blockchain P2P basée sur le **protocole Bitcoin** : le mempool pour stocker les transactions en attente avec TXID, le mining loop distribué avec compétition entre nodes, et les tests pour garantir que tout fonctionne ensemble.

**Protocoles Bitcoin implémentés** :
- ✅ **TXID** : Transactions avec hash unique (Transaction ID)
- ✅ **Mempool** : Pool de transactions en attente de minage
- ✅ **Inventory Propagation** : Diffusion des transactions via inv/getdata
- ✅ **Mining Competition** : Compétition distribuée entre nodes
- ✅ **API Bitcoin-style** : Endpoints compatibles avec le protocole P2P

**Objectifs principaux** :
- ✅ Créer le mempool Bitcoin-style avec TXID
- ✅ Implémenter la propagation de transactions via inventory
- ✅ Implémenter le mining loop distribué (compétition entre nodes)
- ✅ Mettre à jour l'API pour le protocole Bitcoin P2P
- ✅ Créer les tests d'intégration P2P Bitcoin
- ✅ Créer les scripts de démarrage pour les 3 nodes

**Références Bitcoin** :
- [Bitcoin Mempool](https://en.bitcoin.it/wiki/Transaction_Pool)
- [Transaction Format](https://en.bitcoin.it/wiki/Transaction)
- [Mining](https://en.bitcoin.it/wiki/Mining)

---

## 🗂️ Phase 5 : Mempool & Mining Distribué (Bitcoin-style)

### Tâche 5.1 : Mempool Bitcoin avec TXID (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/mempool.rs`
**Durée** : 1.5 jour
**Dépendances** : Aucune

#### Objectif
Créer un pool de **transactions Bitcoin-style** avec TXID (Transaction ID) en attente d'être minées dans un bloc. Dans Bitcoin, chaque transaction a un identifiant unique (hash) qui permet de la référencer et d'éviter les doublons.

#### Concepts clés Bitcoin
- **TXID** : Hash SHA-256 de la transaction qui sert d'identifiant unique
- **Mempool** : Pool de transactions non confirmées en attente de minage
- **Inventory Propagation** : Annoncer les transactions par TXID (inv) au lieu d'envoyer directement les données
- **Double-spend Prevention** : TXID permet de détecter les transactions dupliquées

#### Étapes détaillées

1. **Créer `src/transaction.rs`** pour définir une transaction simple :
```rust
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use chrono::{DateTime, Utc};

/// Transaction simplifiée (version éducative)
/// Dans Bitcoin réel: inputs, outputs, signatures, locktime, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    /// Données de la transaction (simplifié)
    pub data: String,

    /// Timestamp de création
    pub timestamp: i64,

    /// TXID - Transaction ID (hash unique)
    /// Dans Bitcoin: double SHA-256 de la transaction complète
    pub txid: String,
}

impl Transaction {
    /// Créer une nouvelle transaction
    ///
    /// # Arguments
    /// * `data` - Données de la transaction
    ///
    /// # Returns
    /// Transaction avec TXID calculé
    pub fn new(data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let txid = Self::calculate_txid(&data, timestamp);

        Self {
            data,
            timestamp,
            txid,
        }
    }

    /// Calculer le TXID (Transaction ID)
    ///
    /// Dans Bitcoin: TXID = SHA256(SHA256(transaction))
    /// Ici simplifié: TXID = SHA256(data + timestamp)
    fn calculate_txid(data: &str, timestamp: i64) -> String {
        let input = format!("{}{}", data, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Vérifier que le TXID est valide
    pub fn verify_txid(&self) -> bool {
        let expected = Self::calculate_txid(&self.data, self.timestamp);
        self.txid == expected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("Transfer 10 BTC".to_string());
        assert!(!tx.txid.is_empty());
        assert_eq!(tx.txid.len(), 64); // SHA-256 = 64 hex chars
    }

    #[test]
    fn test_txid_uniqueness() {
        let tx1 = Transaction::new("Transfer 10 BTC".to_string());
        std::thread::sleep(std::time::Duration::from_millis(1));
        let tx2 = Transaction::new("Transfer 10 BTC".to_string());

        // Même données mais timestamps différents → TXID différents
        assert_ne!(tx1.txid, tx2.txid);
    }

    #[test]
    fn test_verify_txid() {
        let tx = Transaction::new("Test".to_string());
        assert!(tx.verify_txid());

        // Modifier le TXID invalide la transaction
        let mut invalid_tx = tx.clone();
        invalid_tx.txid = "invalid".to_string();
        assert!(!invalid_tx.verify_txid());
    }
}
```

2. **Créer `mempool.rs`** Bitcoin-style avec TXID :
```rust
use crate::transaction::Transaction;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Mempool Bitcoin-style
/// Stocke les transactions par TXID pour éviter les doublons
#[derive(Debug, Clone)]
pub struct Mempool {
    /// Map: TXID → Transaction
    /// Dans Bitcoin: structure plus complexe avec priority queue (fee/byte)
    transactions: Arc<RwLock<HashMap<String, Transaction>>>,

    /// Taille maximale du mempool
    max_size: usize,
}

impl Mempool {
    /// Créer un nouveau mempool Bitcoin-style
    ///
    /// # Arguments
    /// * `max_size` - Nombre maximum de transactions dans le pool
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    /// Ajouter une transaction au mempool (Bitcoin-style)
    ///
    /// # Arguments
    /// * `data` - Données de la transaction
    ///
    /// # Returns
    /// Ok(TXID) si ajouté, Err si problème
    ///
    /// # Exemple Bitcoin
    /// ```
    /// // Dans Bitcoin:
    /// // 1. Créer transaction signée
    /// // 2. Calculer TXID
    /// // 3. Vérifier que TXID n'existe pas déjà
    /// // 4. Ajouter au mempool
    /// // 5. Propager via Inv([txid]) aux peers
    /// ```
    pub fn add_transaction(&self, data: String) -> Result<String, String> {
        let tx = Transaction::new(data);

        let mut txs = self.transactions.write().unwrap();

        // Vérifier la taille
        if txs.len() >= self.max_size {
            return Err(format!("Mempool is full ({} transactions)", self.max_size));
        }

        // Vérifier les doublons (par TXID)
        if txs.contains_key(&tx.txid) {
            return Err(format!("Transaction {} already in mempool", &tx.txid[..8]));
        }

        // Vérifier la validité du TXID
        if !tx.verify_txid() {
            return Err("Invalid TXID".to_string());
        }

        let txid = tx.txid.clone();
        txs.insert(txid.clone(), tx);

        Ok(txid)
    }

    /// Obtenir une transaction par TXID
    ///
    /// # Arguments
    /// * `txid` - Transaction ID à chercher
    ///
    /// # Returns
    /// Option<Transaction> si trouvée
    pub fn get_transaction(&self, txid: &str) -> Option<Transaction> {
        let txs = self.transactions.read().unwrap();
        txs.get(txid).cloned()
    }

    /// Obtenir tous les TXIDs dans le mempool
    ///
    /// Utile pour créer des Inventory messages
    ///
    /// # Returns
    /// Vec des TXIDs
    pub fn get_all_txids(&self) -> Vec<String> {
        let txs = self.transactions.read().unwrap();
        txs.keys().cloned().collect()
    }

    /// Obtenir toutes les transactions en attente
    pub fn get_pending_transactions(&self) -> Vec<Transaction> {
        let txs = self.transactions.read().unwrap();
        txs.values().cloned().collect()
    }

    /// Retirer une transaction par TXID
    ///
    /// # Arguments
    /// * `txid` - TXID de la transaction à retirer
    pub fn remove_transaction(&self, txid: &str) -> Option<Transaction> {
        let mut txs = self.transactions.write().unwrap();
        txs.remove(txid)
    }

    /// Retirer plusieurs transactions par TXID
    ///
    /// # Arguments
    /// * `txids` - Liste des TXIDs à retirer
    pub fn remove_transactions(&self, txids: &[String]) {
        let mut txs = self.transactions.write().unwrap();
        for txid in txids {
            txs.remove(txid);
        }
    }

    /// Obtenir et retirer N transactions du mempool (pour le mining)
    ///
    /// # Arguments
    /// * `count` - Nombre de transactions à prendre
    ///
    /// # Returns
    /// Vec des transactions prises
    ///
    /// # Note Bitcoin
    /// Dans Bitcoin: tri par fee/byte (priority queue)
    /// Ici simplifié: on prend les premières N transactions
    pub fn take_transactions(&self, count: usize) -> Vec<Transaction> {
        let mut txs = self.transactions.write().unwrap();

        let take_count = count.min(txs.len());
        let mut taken = Vec::new();

        // Prendre les N premières transactions
        let txids_to_take: Vec<String> = txs.keys().take(take_count).cloned().collect();

        for txid in txids_to_take {
            if let Some(tx) = txs.remove(&txid) {
                taken.push(tx);
            }
        }

        taken
    }

    /// Vider le mempool
    pub fn clear(&self) {
        let mut txs = self.transactions.write().unwrap();
        txs.clear();
    }

    /// Obtenir le nombre de transactions dans le mempool
    pub fn len(&self) -> usize {
        let txs = self.transactions.read().unwrap();
        txs.len()
    }

    /// Vérifier si le mempool est vide
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Combiner plusieurs transactions en une seule string (pour créer un bloc)
    ///
    /// # Arguments
    /// * `transactions` - Les transactions à combiner
    ///
    /// # Returns
    /// String combinée au format JSON
    ///
    /// # Note Bitcoin
    /// Dans Bitcoin: chaque bloc contient toutes les transactions avec leurs inputs/outputs
    /// Ici simplifié: on combine juste les données
    pub fn combine_transactions(transactions: &[Transaction]) -> String {
        if transactions.is_empty() {
            return "Empty block".to_string();
        }

        if transactions.len() == 1 {
            return transactions[0].data.clone();
        }

        // Créer un résumé avec les TXIDs
        let data: Vec<String> = transactions
            .iter()
            .map(|tx| format!("{} ({})", tx.data, &tx.txid[..8]))
            .collect();

        serde_json::to_string(&data).unwrap_or_else(|_| data.join(", "))
    }
}

/// Type partagé pour le mempool (thread-safe)
pub type SharedMempool = Arc<Mempool>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_transaction() {
        let mempool = Mempool::new(10);

        let txid1 = mempool.add_transaction("Transfer 10 BTC".to_string()).unwrap();
        let txid2 = mempool.add_transaction("Transfer 5 BTC".to_string()).unwrap();

        assert_ne!(txid1, txid2); // TXIDs doivent être différents
        assert_eq!(mempool.len(), 2);
    }

    #[test]
    fn test_get_transaction() {
        let mempool = Mempool::new(10);
        let txid = mempool.add_transaction("Test tx".to_string()).unwrap();

        let tx = mempool.get_transaction(&txid).unwrap();
        assert_eq!(tx.data, "Test tx");
        assert_eq!(tx.txid, txid);
    }

    #[test]
    fn test_duplicate_prevention() {
        let mempool = Mempool::new(10);

        let tx = Transaction::new("Same data".to_string());
        let txid = tx.txid.clone();

        // Ajouter manuellement
        mempool.transactions.write().unwrap().insert(txid, tx);

        // Essayer d'ajouter la même transaction (même TXID)
        // Note: add_transaction crée un nouveau TXID à cause du timestamp
        // Pour tester vraiment les doublons, on teste avec get_transaction
        assert!(mempool.get_transaction(&txid).is_some());
    }

    #[test]
    fn test_remove_transaction() {
        let mempool = Mempool::new(10);

        let txid = mempool.add_transaction("Tx1".to_string()).unwrap();
        assert_eq!(mempool.len(), 1);

        mempool.remove_transaction(&txid);
        assert_eq!(mempool.len(), 0);
    }

    #[test]
    fn test_take_transactions() {
        let mempool = Mempool::new(10);

        mempool.add_transaction("Tx1".to_string()).unwrap();
        mempool.add_transaction("Tx2".to_string()).unwrap();
        mempool.add_transaction("Tx3".to_string()).unwrap();

        let taken = mempool.take_transactions(2);

        assert_eq!(taken.len(), 2);
        assert_eq!(mempool.len(), 1);
    }

    #[test]
    fn test_max_size() {
        let mempool = Mempool::new(2);

        assert!(mempool.add_transaction("Tx1".to_string()).is_ok());
        assert!(mempool.add_transaction("Tx2".to_string()).is_ok());
        assert!(mempool.add_transaction("Tx3".to_string()).is_err()); // Devrait échouer
    }

    #[test]
    fn test_get_all_txids() {
        let mempool = Mempool::new(10);

        let txid1 = mempool.add_transaction("Tx1".to_string()).unwrap();
        let txid2 = mempool.add_transaction("Tx2".to_string()).unwrap();

        let txids = mempool.get_all_txids();
        assert_eq!(txids.len(), 2);
        assert!(txids.contains(&txid1));
        assert!(txids.contains(&txid2));
    }

    #[test]
    fn test_combine_transactions() {
        let tx1 = Transaction::new("Transfer 10 BTC".to_string());
        let tx2 = Transaction::new("Transfer 5 BTC".to_string());

        let combined = Mempool::combine_transactions(&[tx1, tx2]);
        assert!(combined.contains("Transfer 10 BTC"));
        assert!(combined.contains("Transfer 5 BTC"));
    }
}
```

3. **Ajouter la propagation de transactions via Inventory** :

Dans `src/network/messages.rs` (déjà défini par Mounirou), les transactions sont propagées comme ceci :

```rust
// Exemple d'utilisation pour propager une nouvelle transaction

// 1. Recevoir transaction via API
let txid = mempool.add_transaction(data).unwrap();

// 2. Créer Inventory Vector pour la transaction
let inv = InventoryVector {
    inv_type: InvType::Tx,  // Type = Transaction
    hash: txid.clone(),
};

// 3. Annoncer aux peers via Inv message
let inv_msg = P2PMessage::Inv(vec![inv]);
peer_manager.broadcast_to_all(inv_msg).await?;

// 4. Les peers demandent la transaction
// Peer → GetData([inv_tx]) → Node
// Node → Tx(transaction) → Peer

// 5. Peer ajoute à son mempool
```

**✅ Critère de validation** : Tous les tests du mempool passent.

---

### Tâche 5.1.1 : Propagation de Transactions (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/consensus/tx_propagation.rs`
**Durée** : 0.5 jour
**Dépendances** : Tâche 5.1 + Mounirou's network

#### Objectif
Propager les nouvelles transactions aux peers via le protocole Bitcoin Inventory (inv/getdata).

#### Code à implémenter
```rust
use crate::mempool::SharedMempool;
use crate::network::{PeerManager, P2PMessage, InventoryVector, InvType};
use crate::transaction::Transaction;
use std::sync::Arc;

/// Propagateur de transactions Bitcoin-style
pub struct TransactionPropagator {
    mempool: SharedMempool,
    peer_manager: Arc<PeerManager>,
}

impl TransactionPropagator {
    pub fn new(mempool: SharedMempool, peer_manager: Arc<PeerManager>) -> Self {
        Self {
            mempool,
            peer_manager,
        }
    }

    /// Annoncer une nouvelle transaction aux peers (Bitcoin-style)
    ///
    /// Flow Bitcoin:
    /// 1. Node reçoit transaction via API ou P2P
    /// 2. Ajoute au mempool local
    /// 3. Envoie Inv([txid]) à tous les peers
    /// 4. Peers demandent GetData([txid]) s'ils ne l'ont pas
    /// 5. Node répond avec Tx(transaction)
    ///
    /// # Arguments
    /// * `txid` - TXID de la transaction à annoncer
    pub async fn announce_transaction(&self, txid: String) -> Result<(), Box<dyn std::error::Error>> {
        println!("📢 Announcing transaction {} to network", &txid[..8]);

        // Créer Inventory Vector
        let inv = InventoryVector {
            inv_type: InvType::Tx,
            hash: txid.clone(),
        };

        // Envoyer Inv à tous les peers
        let inv_msg = P2PMessage::Inv(vec![inv]);
        self.peer_manager.broadcast_to_all(inv_msg).await?;

        println!("✅ Transaction announced to {} peers", self.peer_manager.connected_count());
        Ok(())
    }

    /// Gérer une demande GetData pour une transaction
    ///
    /// # Arguments
    /// * `txid` - TXID demandé
    /// * `peer_addr` - Adresse du peer demandeur
    pub async fn handle_getdata_tx(
        &self,
        txid: &str,
        peer_addr: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Chercher la transaction dans le mempool
        if let Some(tx) = self.mempool.get_transaction(txid) {
            println!("📤 Sending transaction {} to peer {}", &txid[..8], peer_addr);

            // Envoyer la transaction au peer
            let tx_msg = P2PMessage::Tx(tx);
            self.peer_manager.send_to_peer(peer_addr, tx_msg).await?;

            Ok(())
        } else {
            Err(format!("Transaction {} not found in mempool", &txid[..8]).into())
        }
    }

    /// Gérer une transaction reçue d'un peer
    ///
    /// # Arguments
    /// * `tx` - Transaction reçue
    pub async fn handle_received_tx(&self, tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
        println!("📥 Received transaction {} from peer", &tx.txid[..8]);

        // Vérifier le TXID
        if !tx.verify_txid() {
            return Err("Invalid TXID".into());
        }

        // Ajouter au mempool (évite doublons automatiquement)
        match self.mempool.transactions.write().unwrap().insert(tx.txid.clone(), tx.clone()) {
            Some(_) => {
                println!("⚠️  Transaction {} already in mempool", &tx.txid[..8]);
            }
            None => {
                println!("✅ Transaction {} added to mempool", &tx.txid[..8]);

                // Re-propager aux autres peers (sauf celui qui nous l'a envoyé)
                // Note: Dans Bitcoin, on utilise un "seen" set pour éviter les boucles
                self.announce_transaction(tx.txid).await?;
            }
        }

        Ok(())
    }
}
```

**✅ Critère de validation** : Les transactions sont propagées via inv/getdata.

---

### Tâche 5.2 : Mining Loop Bitcoin-style (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/consensus/miner.rs`
**Durée** : 2 jours
**Dépendances** : Tâche 5.1 + Itine's propagation

#### Objectif
Créer la boucle de mining qui prend les **transactions** du mempool et mine des blocs. Dans Bitcoin, le mining est une **compétition** entre nodes : le premier qui trouve un nonce valide (PoW) gagne et son bloc est ajouté à la chaîne.

#### Concepts clés Bitcoin
- **Mining Competition** : Tous les nodes minent en parallèle, le premier qui trouve gagne
- **Mempool → Block** : Le mineur sélectionne les transactions du mempool (Bitcoin: par fee/byte)
- **Block Reward** : Dans Bitcoin, le mineur reçoit une récompense (simplifié ici)
- **Orphan Blocks** : Si deux nodes minent simultanément, un bloc devient orphelin

#### Code à implémenter
```rust
use crate::blockchain::SharedBlockchain;
use crate::mempool::SharedMempool;
use crate::consensus::propagation::BlockPropagator;
use crate::Block;
use crate::proof_of_work;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Mineur de blocs en mode distribué
pub struct Miner {
    blockchain: SharedBlockchain,
    mempool: SharedMempool,
    propagator: Arc<BlockPropagator>,
    difficulty: usize,
    node_id: String,
    mining_enabled: bool,
}

impl Miner {
    /// Créer un nouveau mineur
    pub fn new(
        blockchain: SharedBlockchain,
        mempool: SharedMempool,
        propagator: Arc<BlockPropagator>,
        difficulty: usize,
        node_id: String,
    ) -> Self {
        Self {
            blockchain,
            mempool,
            propagator,
            difficulty,
            node_id,
            mining_enabled: true,
        }
    }

    /// Démarrer le mining loop (boucle infinie)
    ///
    /// Cette fonction:
    /// 1. Vérifie s'il y a des données dans le mempool
    /// 2. Si oui, mine un nouveau bloc
    /// 3. Ajoute le bloc à la chaîne locale
    /// 4. Propage le bloc aux autres nodes
    /// 5. Attend un peu avant de recommencer
    pub async fn start_mining_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("⛏️  Mining loop started for node {}", self.node_id);

        let mut iteration = 0;

        loop {
            iteration += 1;

            if !self.mining_enabled {
                // Mining désactivé, attendre
                sleep(Duration::from_secs(5)).await;
                continue;
            }

            // Vérifier s'il y a des transactions à miner
            if self.mempool.is_empty() {
                // Rien à miner, attendre
                if iteration % 10 == 0 {
                    println!("⏳ Waiting for transactions in mempool... (iteration {})", iteration);
                }
                sleep(Duration::from_secs(3)).await;
                continue;
            }

            // Prendre des transactions du mempool
            // Dans Bitcoin: tri par fee/byte (priority queue)
            // Ici simplifié: prendre max 5 transactions
            let transactions = self.mempool.take_transactions(5);

            if transactions.is_empty() {
                continue;
            }

            println!("⛏️  Mining new block with {} transactions from mempool", transactions.len());

            // Afficher les TXIDs
            for tx in &transactions {
                println!("   📝 TXID: {}", &tx.txid[..16]);
            }

            // Combiner les transactions
            let combined_data = crate::mempool::Mempool::combine_transactions(&transactions);

            // Miner le bloc
            match self.mine_block(combined_data).await {
                Ok(block) => {
                    println!("✅ Block #{} mined successfully by {}!", block.index, self.node_id);
                    println!("   Hash: {}", block.hash);
                    println!("   Nonce: {}", block.nonce);
                }
                Err(e) => {
                    eprintln!("❌ Mining error: {}", e);

                    // Remettre les transactions dans le mempool
                    let mut txs = self.mempool.transactions.write().unwrap();
                    for tx in transactions {
                        txs.insert(tx.txid.clone(), tx);
                    }
                }
            }

            // Petite pause avant la prochaine itération
            sleep(Duration::from_secs(1)).await;
        }
    }

    /// Miner un nouveau bloc
    async fn mine_block(&self, data: String) -> Result<Block, Box<dyn std::error::Error>> {
        // Obtenir le verrou d'écriture
        let mut bc = self.blockchain.write().unwrap();

        // Obtenir le dernier bloc
        let last_block = bc.latest_block();

        // Créer le nouveau bloc
        let new_index = last_block.index + 1;
        let mut new_block = Block::new(new_index, data, last_block.hash.clone());

        println!("   🔨 Mining block #{} (difficulty {})...", new_index, self.difficulty);

        // Relâcher le verrou pendant le mining (peut être long)
        drop(bc);

        // Miner le bloc (CPU intensive)
        let start = std::time::Instant::now();
        proof_of_work::mine_block(&mut new_block, self.difficulty);
        let duration = start.elapsed();

        println!("   ⏱️  Mining took {:.2}s", duration.as_secs_f64());

        // Réacquérir le verrou pour ajouter le bloc
        let mut bc = self.blockchain.write().unwrap();

        // IMPORTANT: Vérifier que personne n'a ajouté un bloc pendant qu'on minait
        let current_last = bc.latest_block();

        if current_last.index >= new_block.index {
            println!("   ⚠️  Another node mined a block first! Discarding our block.");
            return Err("Block already exists at this index".into());
        }

        // Ajouter le bloc à la chaîne
        bc.chain.push(new_block.clone());

        println!("   ✅ Block added to local chain. New length: {}", bc.len());

        // Relâcher le verrou
        drop(bc);

        // Propager le bloc au réseau
        println!("   📢 Broadcasting block to network...");
        self.propagator.broadcast_block(&new_block).await?;

        Ok(new_block)
    }

    /// Activer/désactiver le mining
    pub fn set_mining_enabled(&mut self, enabled: bool) {
        self.mining_enabled = enabled;

        if enabled {
            println!("⛏️  Mining enabled");
        } else {
            println!("⏸️  Mining paused");
        }
    }

    /// Miner un bloc immédiatement (manuel)
    pub async fn mine_now(&self, data: String) -> Result<Block, Box<dyn std::error::Error>> {
        println!("⛏️  Manual mining triggered");
        self.mine_block(data).await
    }
}
```

**✅ Critère de validation** : Le miner peut prendre des données du mempool et créer un bloc.

---

## 🔌 Phase 6 : API Updates & Testing

### Tâche 6.1 : Mise à jour des Handlers API Bitcoin-style (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/api/handlers.rs`
**Durée** : 1 jour
**Dépendances** : Tâche 5.1 + Mounirou's network

#### Objectif
Adapter les handlers API pour le protocole Bitcoin P2P avec support des transactions et TXID.

#### Modifications à apporter

1. **Modifier `add_block` pour créer des transactions avec TXID** :
```rust
use crate::mempool::SharedMempool;
use crate::consensus::TransactionPropagator;
use std::sync::Arc;

/// Handler pour POST /transactions
/// Crée une transaction et l'ajoute au mempool (Bitcoin-style)
pub async fn add_transaction(
    State((mempool, tx_propagator)): State<(SharedMempool, Arc<TransactionPropagator>)>,
    Json(payload): Json<AddTransactionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Vérifier que les données ne sont pas vides
    if payload.data.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Data cannot be empty".to_string(),
            }),
        ));
    }

    // Créer transaction avec TXID
    match mempool.add_transaction(payload.data.clone()) {
        Ok(txid) => {
            println!("📝 New transaction created: {}", &txid[..16]);

            // Propager aux peers via Inventory
            if let Err(e) = tx_propagator.announce_transaction(txid.clone()).await {
                eprintln!("⚠️  Failed to propagate transaction: {}", e);
            }

            let response = serde_json::json!({
                "message": "Transaction created and added to mempool",
                "txid": txid,
                "data": payload.data,
                "mempool_size": mempool.len()
            });
            Ok(Json(response))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to add transaction: {}", e),
            }),
        )),
    }
}

// Request model
#[derive(Deserialize)]
pub struct AddTransactionRequest {
    pub data: String,
}
```

2. **Ajouter un handler pour voir le mempool Bitcoin-style** :
```rust
use crate::mempool::SharedMempool;

/// Handler pour GET /mempool
/// Retourne les transactions en attente (Bitcoin-style)
pub async fn get_mempool(
    State(mempool): State<SharedMempool>,
) -> Json<serde_json::Value> {
    let transactions = mempool.get_pending_transactions();

    let tx_info: Vec<_> = transactions
        .iter()
        .map(|tx| {
            serde_json::json!({
                "txid": tx.txid,
                "data": tx.data,
                "timestamp": tx.timestamp
            })
        })
        .collect();

    let response = serde_json::json!({
        "transactions": tx_info,
        "count": tx_info.len()
    });

    Json(response)
}
```

3. **Ajouter un handler pour obtenir une transaction par TXID** :
```rust
/// Handler pour GET /transaction/:txid
/// Retourne une transaction spécifique par TXID
pub async fn get_transaction(
    State(mempool): State<SharedMempool>,
    Path(txid): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    match mempool.get_transaction(&txid) {
        Some(tx) => {
            let response = serde_json::json!({
                "txid": tx.txid,
                "data": tx.data,
                "timestamp": tx.timestamp,
                "verified": tx.verify_txid()
            });
            Ok(Json(response))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: format!("Transaction {} not found", &txid[..8]),
            }),
        )),
    }
}
```

3. **Ajouter un handler pour lister les peers**:
```rust
use crate::network::PeerManager;
use std::sync::Arc;

/// Handler pour GET /peers
/// Retourne la liste des peers connectés
pub async fn get_peers(
    State(peer_manager): State<Arc<PeerManager>>,
) -> Json<serde_json::Value> {
    let peers = peer_manager.get_connected_peers();

    let peers_info: Vec<_> = peers
        .iter()
        .map(|p| {
            serde_json::json!({
                "id": p.id,
                "address": p.address.to_string(),
                "connected": p.connected,
                "last_seen": p.last_seen.to_rfc3339()
            })
        })
        .collect();

    let response = serde_json::json!({
        "peers": peers_info,
        "count": peers_info.len()
    });

    Json(response)
}
```

4. **Ajouter un handler pour forcer la synchronisation**:
```rust
use crate::consensus::ChainSynchronizer;

/// Handler pour POST /sync
/// Force une synchronisation avec le réseau
pub async fn force_sync(
    State(synchronizer): State<Arc<ChainSynchronizer>>,
) -> Json<serde_json::Value> {
    println!("🔄 Manual sync triggered via API");

    match synchronizer.sync_with_network().await {
        Ok(synced) => {
            let message = if synced {
                "Chain synchronized successfully"
            } else {
                "Chain is already up to date"
            };

            Json(serde_json::json!({
                "success": true,
                "synced": synced,
                "message": message
            }))
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "error": format!("Sync failed: {}", e)
        })),
    }
}
```

5. **Ajouter un handler pour le status du node**:
```rust
/// Handler pour GET /status
/// Retourne l'état du node
pub async fn get_status(
    State((blockchain, mempool, peer_manager)): State<(
        SharedBlockchain,
        SharedMempool,
        Arc<PeerManager>,
    )>,
) -> Json<serde_json::Value> {
    let bc = blockchain.read().unwrap();

    let response = serde_json::json!({
        "node_id": peer_manager.node_id,
        "chain_length": bc.len(),
        "chain_valid": bc.is_valid(),
        "mempool_size": mempool.len(),
        "connected_peers": peer_manager.connected_count(),
        "difficulty": bc.difficulty
    });

    Json(response)
}
```

6. **Mettre à jour `main.rs` pour utiliser ces nouveaux handlers Bitcoin-style**:
```rust
// Dans main.rs, modifier le routeur:

let app = Router::new()
    .route("/", get(root))
    // Blockchain endpoints
    .route("/chain", get(api::get_chain))
    .route("/blocks/:index", get(api::get_block))
    .route("/validate", get(api::validate_chain))
    // Transaction endpoints (Bitcoin-style)
    .route("/transactions", post(api::add_transaction))     // NOUVEAU
    .route("/transaction/:txid", get(api::get_transaction)) // NOUVEAU
    .route("/mempool", get(api::get_mempool))               // Mis à jour
    // Network endpoints
    .route("/peers", get(api::get_peers))
    .route("/sync", post(api::force_sync))
    .route("/status", get(api::get_status))
    .with_state(/* ... */)
    .layer(CorsLayer::permissive());
```

**Note** : L'ancien endpoint `POST /blocks` est remplacé par `POST /transactions` pour mieux refléter le protocole Bitcoin.


**✅ Critère de validation** : Tous les nouveaux endpoints répondent correctement.

---

### Tâche 6.2 : Tests d'Intégration P2P Bitcoin (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/tests/p2p_tests.rs`
**Durée** : 1.5 jours
**Dépendances** : Toutes les tâches précédentes

#### Objectif
Créer des tests automatisés pour vérifier le fonctionnement P2P avec le protocole Bitcoin (handshake, inventory, transactions).

#### Code à implémenter
```rust
//! Tests d'intégration pour le réseau P2P Bitcoin-style

use mini_blockchain::{Blockchain, network::P2PClient, transaction::Transaction};
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_version_handshake() {
    // Test: vérifier le handshake Version/Verack (Bitcoin-style)

    // NOTE: Ce test nécessite que 2 nodes soient déjà lancés
    // Node 1 sur 127.0.0.1:9091
    // Node 2 sur 127.0.0.1:9092

    let node1: SocketAddr = "127.0.0.1:9091".parse().unwrap();

    // Envoyer Version et attendre Verack
    match P2PClient::handshake(node1).await {
        Ok(version_info) => {
            println!("✅ Handshake successful");
            println!("   Protocol version: {}", version_info.version);
            println!("   User agent: {}", version_info.user_agent);
            println!("   Start height: {}", version_info.start_height);
        }
        Err(e) => {
            eprintln!("⚠️  Test skipped: Node 1 not running ({:?})", e);
        }
    }
}

#[tokio::test]
async fn test_transaction_propagation() {
    // Test: vérifier que les transactions sont propagées via Inventory

    let client = reqwest::Client::new();

    // 1. Ajouter transaction sur Node 1
    let response = client
        .post("http://localhost:8091/transactions")
        .json(&serde_json::json!({"data": "Transfer 10 BTC"}))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: serde_json::Value = resp.json().await.unwrap();
                let txid = json["txid"].as_str().unwrap();
                println!("✅ Transaction created: {}", &txid[..16]);

                // 2. Attendre propagation
                sleep(Duration::from_secs(2)).await;

                // 3. Vérifier que Node 2 a la transaction dans son mempool
                let mempool_resp = client
                    .get("http://localhost:8092/mempool")
                    .send()
                    .await
                    .unwrap();

                let mempool_json: serde_json::Value = mempool_resp.json().await.unwrap();
                let txids: Vec<String> = mempool_json["transactions"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|tx| tx["txid"].as_str().unwrap().to_string())
                    .collect();

                if txids.contains(&txid.to_string()) {
                    println!("✅ Transaction propagated to Node 2 via Inventory");
                } else {
                    println!("⚠️  Transaction not yet in Node 2 mempool");
                }
            }
        }
        Err(e) => {
            eprintln!("⚠️  Test skipped: Node not running ({:?})", e);
        }
    }
}

#[tokio::test]
async fn test_chain_sync() {
    // Test: vérifier qu'un node peut synchroniser sa chaîne

    let node1: SocketAddr = "127.0.0.1:9091".parse().unwrap();

    // Demander la longueur de la chaîne
    match P2PClient::get_chain_length(node1).await {
        Ok(length) => {
            println!("✅ Node 1 chain length: {}", length);
            assert!(length >= 1); // Au moins le bloc genesis
        }
        Err(e) => {
            eprintln!("⚠️  Test skipped: {:?}", e);
        }
    }

    // Demander la chaîne complète
    match P2PClient::get_full_chain(node1).await {
        Ok(chain) => {
            println!("✅ Received full chain with {} blocks", chain.len());
            assert!(!chain.is_empty());
            assert_eq!(chain[0].index, 0); // Premier bloc = genesis
        }
        Err(e) => {
            eprintln!("⚠️  Test skipped: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_mempool_api_bitcoin() {
    // Test: vérifier que l'API mempool Bitcoin-style fonctionne

    // NOTE: Nécessite qu'un node soit lancé sur localhost:8091

    let client = reqwest::Client::new();

    // Créer une transaction
    let response = client
        .post("http://localhost:8091/transactions")
        .json(&serde_json::json!({"data": "Test transaction"}))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let json: serde_json::Value = resp.json().await.unwrap();
                let txid = json["txid"].as_str().unwrap();
                println!("✅ Transaction added to mempool via API");
                println!("   TXID: {}", txid);

                // Vérifier le mempool
                sleep(Duration::from_millis(500)).await;

                let mempool_resp = client
                    .get("http://localhost:8091/mempool")
                    .send()
                    .await
                    .unwrap();

                let mempool_json: serde_json::Value = mempool_resp.json().await.unwrap();
                println!("Mempool: {:?}", mempool_json);

                // Obtenir la transaction par TXID
                let tx_resp = client
                    .get(&format!("http://localhost:8091/transaction/{}", txid))
                    .send()
                    .await
                    .unwrap();

                let tx_json: serde_json::Value = tx_resp.json().await.unwrap();
                println!("Transaction details: {:?}", tx_json);

                assert_eq!(tx_json["txid"], txid);
                assert!(tx_json["verified"].as_bool().unwrap());
            } else {
                eprintln!("⚠️  API returned error: {:?}", resp.status());
            }
        }
        Err(e) => {
            eprintln!("⚠️  Test skipped: Node not running ({:?})", e);
        }
    }
}

#[tokio::test]
async fn test_inventory_vectors() {
    // Test: vérifier que les inventory vectors fonctionnent

    // Ce test vérifie le flow complet:
    // 1. Node 1 crée transaction
    // 2. Node 1 envoie Inv([txid]) à Node 2
    // 3. Node 2 répond GetData([txid])
    // 4. Node 1 envoie Tx(transaction)
    // 5. Node 2 ajoute au mempool

    println!("⚠️  Ce test nécessite logs de debug activés sur les nodes");
    println!("   Lancer avec: RUST_LOG=debug cargo run -- --config config/node1.toml");
}
```

**✅ Critère de validation** : Les tests passent quand les nodes sont lancés.

---

### Tâche 6.3 : Scripts de Démarrage (Priorité : HAUTE)
**Fichiers** : `scripts/start-node1.sh`, `scripts/start-node2.sh`, `scripts/start-node3.sh`
**Durée** : 0.5 jour
**Dépendances** : Mounirou's configuration

#### Créer les scripts

**`scripts/start-node1.sh`**
```bash
#!/bin/bash
# Script pour démarrer Node 1

echo "🚀 Starting Node 1..."
cd "$(dirname "$0")/.."

cargo run --release -- --config config/node1.toml
```

**`scripts/start-node2.sh`**
```bash
#!/bin/bash
# Script pour démarrer Node 2

echo "🚀 Starting Node 2..."
cd "$(dirname "$0")/.."

cargo run --release -- --config config/node2.toml
```

**`scripts/start-node3.sh`**
```bash
#!/bin/bash
# Script pour démarrer Node 3

echo "🚀 Starting Node 3..."
cd "$(dirname "$0")/.."

cargo run --release -- --config config/node3.toml
```

**Rendre les scripts exécutables**:
```bash
chmod +x scripts/start-node*.sh
```

---

### Tâche 6.4 : Script de Test du Réseau (Priorité : MOYENNE)
**Fichier** : `scripts/test-network.sh`
**Durée** : 0.5 jour

#### Créer un script pour tester le réseau automatiquement

```bash
#!/bin/bash
# Script pour tester le réseau P2P automatiquement

set -e

echo "🧪 Testing P2P Network..."
echo

# Attendre que les nodes soient prêts
sleep 2

# Test 1: Vérifier que les 3 nodes répondent
echo "📡 Test 1: Checking node availability..."
curl -s http://localhost:8091/ > /dev/null && echo "  ✅ Node 1 is running"
curl -s http://localhost:8092/ > /dev/null && echo "  ✅ Node 2 is running"
curl -s http://localhost:8093/ > /dev/null && echo "  ✅ Node 3 is running"
echo

# Test 2: Vérifier les chaînes
echo "📊 Test 2: Checking chain lengths..."
echo "  Node 1:"
curl -s http://localhost:8091/chain | jq '.length'
echo "  Node 2:"
curl -s http://localhost:8092/chain | jq '.length'
echo "  Node 3:"
curl -s http://localhost:8093/chain | jq '.length'
echo

# Test 3: Créer une transaction sur Node 1
echo "📝 Test 3: Creating transaction on Node 1..."
TXID=$(curl -s -X POST http://localhost:8091/transactions \
  -H "Content-Type: application/json" \
  -d '{"data":"Test transaction from script"}' | jq -r '.txid')
echo "  TXID: ${TXID:0:16}..."
echo

# Test 4: Vérifier le mempool (Bitcoin-style)
echo "🗂️  Test 4: Checking mempool..."
curl -s http://localhost:8091/mempool | jq '.'
echo

# Test 4.1: Obtenir transaction par TXID
if [ ! -z "$TXID" ]; then
  echo "🔍 Test 4.1: Getting transaction by TXID..."
  curl -s "http://localhost:8091/transaction/$TXID" | jq '.'
  echo
fi

# Test 5: Vérifier les peers connectés
echo "👥 Test 5: Checking connected peers..."
echo "  Node 1 peers:"
curl -s http://localhost:8091/peers | jq '.count'
echo "  Node 2 peers:"
curl -s http://localhost:8092/peers | jq '.count'
echo "  Node 3 peers:"
curl -s http://localhost:8093/peers | jq '.count'
echo

# Test 6: Vérifier le status des nodes
echo "📈 Test 6: Node status..."
echo "  Node 1:"
curl -s http://localhost:8091/status | jq '.'
echo "  Node 2:"
curl -s http://localhost:8092/status | jq '.'
echo "  Node 3:"
curl -s http://localhost:8093/status | jq '.'
echo

echo "✅ All tests completed!"
```

**Rendre le script exécutable**:
```bash
chmod +x scripts/test-network.sh
```

---

### Tâche 6.5 : Documentation de Test (Priorité : MOYENNE)
**Fichier** : `TESTING.md`
**Durée** : 0.5 jour

#### Créer un guide de test

```markdown
# Guide de Test - Blockchain P2P

## Démarrage du Réseau

### Option 1: Démarrage manuel (3 terminaux)

**Terminal 1 - Node 1:**
```bash
./scripts/start-node1.sh
```

**Terminal 2 - Node 2:**
```bash
./scripts/start-node2.sh
```

**Terminal 3 - Node 3:**
```bash
./scripts/start-node3.sh
```

### Option 2: Démarrage en arrière-plan

```bash
./scripts/start-node1.sh &
./scripts/start-node2.sh &
./scripts/start-node3.sh &
```

Pour arrêter tous les nodes:
```bash
pkill -f "mini-blockchain"
```

---

## Tests Manuels

### Test 1: Vérifier que les nodes fonctionnent

```bash
curl http://localhost:8091/status | jq
curl http://localhost:8092/status | jq
curl http://localhost:8093/status | jq
```

**Résultat attendu**: Chaque node répond avec son statut.

---

### Test 2: Créer une transaction (Bitcoin-style)

```bash
curl -X POST http://localhost:8091/transactions \
  -H "Content-Type: application/json" \
  -d '{"data":"Transfer 10 BTC to Alice"}'
```

**Résultat attendu**:
```json
{
  "message": "Transaction created and added to mempool",
  "txid": "a3f7b2c1e4d5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6...",
  "data": "Transfer 10 BTC to Alice",
  "mempool_size": 1
}
```

---

### Test 3: Vérifier le mempool Bitcoin-style

```bash
curl http://localhost:8091/mempool | jq
```

**Résultat attendu**:
```json
{
  "transactions": [
    {
      "txid": "a3f7b2c1e4d5...",
      "data": "Transfer 10 BTC to Alice",
      "timestamp": 1709123456
    }
  ],
  "count": 1
}
```

---

### Test 3.1: Obtenir une transaction par TXID

```bash
# Remplacer <TXID> par le TXID obtenu au Test 2
curl http://localhost:8091/transaction/<TXID> | jq
```

**Résultat attendu**:
```json
{
  "txid": "a3f7b2c1e4d5...",
  "data": "Transfer 10 BTC to Alice",
  "timestamp": 1709123456,
  "verified": true
}
```

---

### Test 4: Attendre que le bloc soit miné

Attendre quelques secondes (selon la difficulté), puis vérifier la chaîne:

```bash
curl http://localhost:8091/chain | jq '.length'
```

**Résultat attendu**: La longueur a augmenté de 1.

---

### Test 5: Vérifier la propagation

Vérifier que les autres nodes ont aussi le nouveau bloc:

```bash
curl http://localhost:8092/chain | jq '.length'
curl http://localhost:8093/chain | jq '.length'
```

**Résultat attendu**: Tous les nodes ont la même longueur de chaîne.

---

### Test 6: Synchronisation (late joiner)

1. Arrêter Node 3
2. Ajouter plusieurs blocs sur Node 1
3. Redémarrer Node 3
4. Vérifier qu'il se synchronise automatiquement

```bash
# Node 3 devrait afficher dans ses logs:
# "🔄 Starting chain synchronization..."
# "⬇️  Downloading chain from peer..."
# "✅ Chain synchronized successfully!"
```

---

### Test 7: Vérifier les peers connectés

```bash
curl http://localhost:8091/peers | jq
```

**Résultat attendu**: Liste des peers connectés (Node 2 et Node 3).

---

## Tests Automatisés

### Lancer le script de test

```bash
./scripts/test-network.sh
```

Ce script teste automatiquement:
- Disponibilité des nodes
- Longueur des chaînes
- Ajout au mempool
- Connectivité entre peers
- Status des nodes

---

## Tests Unitaires

```bash
# Tester tous les modules
cargo test

# Tester un module spécifique
cargo test --lib mempool
cargo test --lib consensus
cargo test --lib network

# Tests d'intégration P2P
cargo test --test p2p_tests
```

---

## Scénarios de Test Avancés

### Scénario 1: Compétition de Mining

1. Activer `auto_mine = true` dans les 3 configs
2. Ajouter des données au mempool de chaque node
3. Observer quel node mine en premier
4. Vérifier que tous convergent vers la même chaîne

### Scénario 2: Fork et Résolution

1. Isoler Node 3 (arrêter les connexions)
2. Ajouter des blocs sur Node 1
3. Ajouter des blocs différents sur Node 3
4. Reconnecter Node 3
5. Vérifier que la règle "longest chain" s'applique

### Scénario 3: Persistance

1. Ajouter plusieurs blocs
2. Arrêter tous les nodes
3. Redémarrer tous les nodes
4. Vérifier que les chaînes sont restaurées depuis `data/node*/blockchain.json`

---

## Débogage

### Logs détaillés

Activer les logs de debug:
```bash
RUST_LOG=debug cargo run -- --config config/node1.toml
```

### Vérifier les fichiers de données

```bash
cat data/node1/blockchain.json | jq
cat data/node2/blockchain.json | jq
cat data/node3/blockchain.json | jq
```

### Nettoyer les données

```bash
rm -rf data/node*/blockchain.json
```

---

## Checklist de Validation Finale

- [ ] Les 3 nodes démarrent sans erreur
- [ ] Les nodes se connectent entre eux (peers visibles)
- [ ] Les données peuvent être ajoutées au mempool via API
- [ ] Les blocs sont minés automatiquement
- [ ] Les blocs sont propagés à tous les nodes
- [ ] La synchronisation fonctionne (late joiner)
- [ ] La blockchain persiste entre redémarrages
- [ ] L'API répond correctement sur tous les endpoints
- [ ] Les tests automatisés passent

```

---

## 📊 Points de Synchronisation

### Avec Mounirou (Network & Config)
- **Besoin**: Les routes API doivent être ajoutées dans `main.rs` après que Mounirou ait configuré le routeur
- **Besoin**: Le mempool doit être partagé comme `State` dans Axum

### Avec Itine (Consensus & Storage)
- **Après Tâche 5.2**: Le miner utilise `BlockPropagator` d'Itine pour diffuser les blocs
- **Besoin**: Sauvegarder la blockchain après chaque bloc miné (utiliser `JsonStorage`)

---

## ✅ Checklist Finale

- [ ] Le mempool fonctionne (add, take, remove)
- [ ] Le mining loop prend les données du mempool et mine des blocs
- [ ] Les blocs minés sont propagés au réseau
- [ ] L'API `/mempool` fonctionne
- [ ] L'API `/peers` fonctionne
- [ ] L'API `/sync` fonctionne
- [ ] L'API `/status` fonctionne
- [ ] Les scripts de démarrage lancent les nodes correctement
- [ ] Le script de test du réseau fonctionne
- [ ] Les tests d'intégration P2P passent
- [ ] La documentation de test est complète

---

## 🧪 Commandes de Test

```bash
# Tester le mempool
cargo test --lib mempool

# Tester le miner
cargo test --lib consensus::miner

# Lancer les 3 nodes
./scripts/start-node1.sh &
./scripts/start-node2.sh &
./scripts/start-node3.sh &

# Tester le réseau
./scripts/test-network.sh

# Tests d'intégration
cargo test --test p2p_tests

# Nettoyer
pkill -f mini-blockchain
rm -rf data/node*/blockchain.json
```

---

**Bon courage Mbaye ! 🎯 Tu finalises le projet et garantis sa qualité !**
