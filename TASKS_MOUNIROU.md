# 🚀 Tâches de Mounirou - Infrastructure Réseau P2P (Protocole Bitcoin Simplifié)

**Développeur** : Mounirou
**Domaines** : Infrastructure Réseau P2P basée sur Bitcoin, Configuration Multi-Nodes
**Durée estimée** : 6-9 jours
**Phases assignées** : Phase 1 (Réseau P2P Bitcoin) + Phase 3 (Configuration)
**Protocole** : Bitcoin P2P Protocol (version simplifiée éducative)

---

## 📋 Vue d'ensemble

Tu es responsable de l'infrastructure réseau P2P **basée sur le vrai protocole Bitcoin**, mais dans une version simplifiée éducative. Cela inclut le handshake version/verack, les inventory vectors, et la propagation headers-first.

**Objectifs principaux** :
- ✅ Implémenter le handshake Bitcoin (version/verack)
- ✅ Créer les messages P2P Bitcoin (inv, getdata, headers, block)
- ✅ Gérer l'inventory propagation (comme Bitcoin)
- ✅ Implémenter headers-first synchronization
- ✅ Configuration multi-nodes

**Références Bitcoin** :
- [Bitcoin P2P Protocol](https://en.bitcoin.it/wiki/Protocol_documentation)
- [Bitcoin Developer Guide](https://developer.bitcoin.org/devguide/p2p_network.html)

---

## 🔧 Phase 1 : Infrastructure Réseau P2P (Protocole Bitcoin)

### Tâche 1.1 : Messages P2P Bitcoin (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/network/messages.rs`
**Durée** : 2 jours
**Dépendances** : Aucune

#### Objectif
Implémenter les messages P2P du protocole Bitcoin (version simplifiée).

#### Concepts Bitcoin implémentés
- ✅ **Version Handshake** : Négociation de version au lieu de simple ping
- ✅ **Inventory Vectors** : Annoncer des données disponibles par hash
- ✅ **Headers-First** : Télécharger headers avant blocs complets
- ✅ **Block Locators** : Trouver le point de divergence dans la chaîne

#### Code à implémenter

```bash
mkdir -p 01-mini-blockchain/src/network
```

**`src/network/messages.rs`** :
```rust
use crate::Block;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// Messages P2P basés sur le protocole Bitcoin (simplifié)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum P2PMessage {
    // === HANDSHAKE (Bitcoin Version/Verack) ===
    /// Version handshake - premier message lors de la connexion
    /// Ref: https://en.bitcoin.it/wiki/Protocol_documentation#version
    Version {
        version: u32,           // Version du protocole (ex: 70015 pour Bitcoin)
        services: u64,          // Services supportés (0 = none pour simplifier)
        timestamp: i64,         // Timestamp Unix
        addr_recv: String,      // Adresse du peer destinataire
        addr_from: String,      // Notre adresse
        nonce: u64,            // Nonce unique anti-self-connection
        user_agent: String,     // "/MiniBlockchain:0.1.0/"
        start_height: u64,      // Hauteur de chaîne actuelle
    },

    /// Verack - Acknowledge du handshake
    Verack,

    // === PEER DISCOVERY (Bitcoin 'addr' message) ===
    /// Demander des adresses de peers
    GetAddr,

    /// Liste d'adresses de peers (max 1000 dans Bitcoin, on limite à 100)
    Addr(Vec<String>),

    // === INVENTORY PROPAGATION (Bitcoin 'inv'/'getdata') ===
    /// Inventory - Annoncer qu'on a des données (blocs ou tx)
    /// Ref: https://en.bitcoin.it/wiki/Protocol_documentation#inv
    Inv(Vec<InventoryVector>),

    /// GetData - Demander des données annoncées dans Inv
    GetData(Vec<InventoryVector>),

    // === BLOCK SYNCHRONIZATION (Headers-First) ===
    /// Headers - Envoyer des block headers (Bitcoin headers message)
    /// Ref: https://en.bitcoin.it/wiki/Protocol_documentation#headers
    Headers(Vec<BlockHeader>),

    /// GetHeaders - Demander des headers pour sync
    /// Ref: https://en.bitcoin.it/wiki/Protocol_documentation#getheaders
    GetHeaders {
        version: u32,
        /// Block locator hashes - liste de hashs pour trouver le fork point
        block_locator_hashes: Vec<String>,
        /// Hash où arrêter l'envoi (0000...0000 = pas de limite)
        hash_stop: String,
    },

    /// Block - Envoyer un bloc complet
    Block(Block),

    /// GetBlocks - Demander des blocs (ancien protocole Bitcoin)
    GetBlocks {
        version: u32,
        block_locator_hashes: Vec<String>,
        hash_stop: String,
    },

    // === TRANSACTION PROPAGATION ===
    /// Tx - Diffuser une transaction
    Tx(Transaction),

    // === MEMPOOL ===
    /// MemPool - Demander les tx en attente
    MemPool,

    // === PING/PONG (Keepalive) ===
    /// Ping avec nonce (Bitcoin utilise ça pour mesurer latence)
    Ping { nonce: u64 },

    /// Pong - Réponse au Ping avec même nonce
    Pong { nonce: u64 },

    // === REJECT (Bitcoin error reporting) ===
    /// Reject - Rejeter un message invalide
    Reject {
        message: String,  // Type de message rejeté
        ccode: u8,       // Code d'erreur
        reason: String,  // Raison lisible
    },
}

/// Inventory Vector (Bitcoin inventory vector)
/// Utilisé pour annoncer l'existence de données sans les transmettre
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InventoryVector {
    /// Type d'inventaire
    pub inv_type: InvType,
    /// Hash de l'objet (block hash ou txid)
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InvType {
    Error = 0,
    MsgTx = 1,      // Transaction
    MsgBlock = 2,   // Bloc complet
}

/// Block Header (simplifié du header Bitcoin 80 bytes)
/// Bitcoin header contient: version, prev_hash, merkle_root, time, bits, nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub index: u64,              // Hauteur du bloc
    pub version: u32,            // Version
    pub prev_block_hash: String, // Hash du bloc précédent
    pub merkle_root: String,     // Merkle root (simplifié: hash des données)
    pub timestamp: i64,          // Unix timestamp
    pub difficulty: u32,         // Bits de difficulté
    pub nonce: u64,             // Nonce du PoW
    pub hash: String,           // Hash de ce header
}

impl BlockHeader {
    /// Créer un header depuis un bloc complet
    pub fn from_block(block: &Block) -> Self {
        Self {
            index: block.index,
            version: 1,
            prev_block_hash: block.previous_hash.clone(),
            merkle_root: Self::calculate_merkle_root(&block.data),
            timestamp: block.timestamp.timestamp(),
            difficulty: 2, // Simplifié
            nonce: block.nonce,
            hash: block.hash.clone(),
        }
    }

    /// Calculer le merkle root (simplifié: juste hash des données)
    /// Bitcoin utilise un vrai arbre de Merkle des transactions
    fn calculate_merkle_root(data: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        hex::encode(hasher.finalize())
    }
}

/// Transaction simplifiée (Bitcoin a inputs/outputs UTXO complets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Transaction ID (hash de la transaction)
    pub txid: String,
    /// Données de transaction (simplifié - pas de vraie UTXO)
    pub data: String,
    /// Timestamp
    pub timestamp: i64,
}

impl Transaction {
    /// Créer une nouvelle transaction
    pub fn new(data: String) -> Self {
        use chrono::Utc;
        use sha2::{Sha256, Digest};

        let timestamp = Utc::now().timestamp();
        let txid_input = format!("{}{}", data, timestamp);

        let mut hasher = Sha256::new();
        hasher.update(txid_input.as_bytes());
        let txid = hex::encode(hasher.finalize());

        Self {
            txid,
            data,
            timestamp,
        }
    }
}

impl P2PMessage {
    /// Sérialiser en bytes (Bitcoin utilise un format binaire custom)
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(bincode::serialize(self)?)
    }

    /// Désérialiser depuis bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize(bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_message() {
        let msg = P2PMessage::Version {
            version: 70015,
            services: 0,
            timestamp: 1234567890,
            addr_recv: "127.0.0.1:8333".to_string(),
            addr_from: "127.0.0.1:8334".to_string(),
            nonce: 123456,
            user_agent: "/MiniBlockchain:0.1.0/".to_string(),
            start_height: 42,
        };

        let bytes = msg.to_bytes().unwrap();
        let decoded = P2PMessage::from_bytes(&bytes).unwrap();

        match decoded {
            P2PMessage::Version { start_height, .. } => assert_eq!(start_height, 42),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_inventory_vector() {
        let inv = InventoryVector {
            inv_type: InvType::MsgBlock,
            hash: "00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048".to_string(),
        };

        assert_eq!(inv.inv_type, InvType::MsgBlock);
    }

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction::new("Test transaction".to_string());

        assert!(!tx.txid.is_empty());
        assert_eq!(tx.data, "Test transaction");
    }
}
```

**✅ Critère de validation** : Les tests passent et les messages peuvent être sérialisés.

---

### Tâche 1.2 : Peer avec Handshake Bitcoin (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/network/peer.rs`
**Durée** : 1 jour
**Dépendances** : Tâche 1.1

#### Objectif
Structure Peer avec état de handshake Bitcoin (version/verack).

#### Code à implémenter
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

/// État du handshake Bitcoin
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HandshakeState {
    /// Pas encore connecté
    NotConnected,
    /// Version envoyée, attente de Verack
    VersionSent,
    /// Verack reçu, connexion établie
    Connected,
}

/// Représente un peer du réseau (node distant)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    /// ID unique du peer
    pub id: String,

    /// Adresse réseau
    pub address: String,

    /// État du handshake
    pub handshake_state: HandshakeState,

    /// Version du protocole du peer
    pub protocol_version: u32,

    /// User agent du peer (ex: "/Satoshi:0.21.0/")
    pub user_agent: String,

    /// Hauteur de chaîne du peer
    pub start_height: u64,

    /// Services supportés par le peer
    pub services: u64,

    /// Dernière activité
    pub last_seen: DateTime<Utc>,

    /// Latency en millisecondes (mesurée par ping/pong)
    pub latency_ms: Option<u64>,
}

impl Peer {
    /// Créer un nouveau peer
    pub fn new(id: String, address: String) -> Self {
        Self {
            id,
            address,
            handshake_state: HandshakeState::NotConnected,
            protocol_version: 0,
            user_agent: String::new(),
            start_height: 0,
            services: 0,
            last_seen: Utc::now(),
            latency_ms: None,
        }
    }

    /// Marquer le handshake comme version envoyée
    pub fn mark_version_sent(&mut self) {
        self.handshake_state = HandshakeState::VersionSent;
        self.last_seen = Utc::now();
    }

    /// Marquer le handshake comme complet (verack reçu)
    pub fn mark_connected(
        &mut self,
        protocol_version: u32,
        user_agent: String,
        start_height: u64,
        services: u64,
    ) {
        self.handshake_state = HandshakeState::Connected;
        self.protocol_version = protocol_version;
        self.user_agent = user_agent;
        self.start_height = start_height;
        self.services = services;
        self.last_seen = Utc::now();
    }

    /// Vérifier si le peer est connecté
    pub fn is_connected(&self) -> bool {
        self.handshake_state == HandshakeState::Connected
    }

    /// Mettre à jour la latence
    pub fn update_latency(&mut self, latency_ms: u64) {
        self.latency_ms = Some(latency_ms);
        self.last_seen = Utc::now();
    }

    /// Vérifier si le peer est stale (> 2 minutes sans activité)
    pub fn is_stale(&self) -> bool {
        let now = Utc::now();
        let duration = now.signed_duration_since(self.last_seen);
        duration.num_seconds() > 120
    }

    /// Mettre à jour last_seen
    pub fn touch(&mut self) {
        self.last_seen = Utc::now();
    }
}
```

**✅ Critère de validation** : La structure Peer gère correctement les états de handshake.

---

### Tâche 1.3 : PeerManager Bitcoin-Style (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/network/peer_manager.rs`
**Durée** : 1 jour
**Dépendances** : Tâche 1.2

#### Code à implémenter
```rust
use crate::network::peer::{Peer, HandshakeState};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Gestionnaire de peers avec handshake Bitcoin
#[derive(Debug, Clone)]
pub struct PeerManager {
    /// Map de peers par ID
    peers: Arc<RwLock<HashMap<String, Peer>>>,

    /// Notre node ID
    pub node_id: String,

    /// Notre version de protocole
    pub protocol_version: u32,

    /// Notre user agent
    pub user_agent: String,
}

impl PeerManager {
    /// Créer un nouveau gestionnaire
    pub fn new(node_id: String) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            node_id,
            protocol_version: 70015, // Version Bitcoin actuelle
            user_agent: "/MiniBlockchain:0.1.0/".to_string(),
        }
    }

    /// Ajouter un peer
    pub fn add_peer(&self, peer: Peer) -> Result<(), String> {
        let mut peers = self.peers.write().unwrap();

        if peers.contains_key(&peer.id) {
            return Err(format!("Peer {} already exists", peer.id));
        }

        peers.insert(peer.id.clone(), peer);
        Ok(())
    }

    /// Obtenir tous les peers connectés (handshake complet)
    pub fn get_connected_peers(&self) -> Vec<Peer> {
        let peers = self.peers.read().unwrap();
        peers
            .values()
            .filter(|p| p.is_connected())
            .cloned()
            .collect()
    }

    /// Obtenir un peer par ID
    pub fn get_peer(&self, peer_id: &str) -> Option<Peer> {
        let peers = self.peers.read().unwrap();
        peers.get(peer_id).cloned()
    }

    /// Mettre à jour l'état d'un peer
    pub fn update_peer<F>(&self, peer_id: &str, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut Peer),
    {
        let mut peers = self.peers.write().unwrap();

        if let Some(peer) = peers.get_mut(peer_id) {
            updater(peer);
            Ok(())
        } else {
            Err(format!("Peer {} not found", peer_id))
        }
    }

    /// Compter les peers connectés
    pub fn connected_count(&self) -> usize {
        self.get_connected_peers().len()
    }

    /// Nettoyer les peers stale
    pub fn cleanup_stale_peers(&self) {
        let mut peers = self.peers.write().unwrap();
        peers.retain(|_, peer| !peer.is_stale() || peer.is_connected());
    }

    /// Obtenir les adresses des peers pour broadcast (message 'addr')
    pub fn get_peer_addresses(&self, max: usize) -> Vec<String> {
        let peers = self.peers.read().unwrap();
        peers
            .values()
            .filter(|p| p.is_connected())
            .take(max)
            .map(|p| p.address.clone())
            .collect()
    }
}
```

**✅ Critère de validation** : Le PeerManager gère correctement le cycle de vie des peers.

---

### Tâche 1.4 : Serveur P2P avec Handshake Bitcoin (Priorité : CRITIQUE)
**Fichier** : `01-mini-blockchain/src/network/p2p_server.rs`
**Durée** : 3 jours
**Dépendances** : Tâches 1.1, 1.2, 1.3

#### Objectif
Serveur P2P qui implémente le handshake Bitcoin version/verack.

#### Code à implémenter
```rust
use crate::network::messages::*;
use crate::network::peer::{Peer, HandshakeState};
use crate::network::peer_manager::PeerManager;
use crate::blockchain::SharedBlockchain;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use chrono::Utc;

/// Serveur P2P avec protocole Bitcoin
pub struct P2PServer {
    port: u16,
    peer_manager: Arc<PeerManager>,
    blockchain: SharedBlockchain,
}

impl P2PServer {
    pub fn new(port: u16, peer_manager: Arc<PeerManager>, blockchain: SharedBlockchain) -> Self {
        Self {
            port,
            peer_manager,
            blockchain,
        }
    }

    /// Démarrer le serveur
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).await?;

        println!("🌐 P2P Server (Bitcoin protocol) listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    println!("📥 New connection from {}", addr);

                    let peer_manager = self.peer_manager.clone();
                    let blockchain = self.blockchain.clone();

                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(stream, peer_manager, blockchain, addr.to_string()).await {
                            eprintln!("❌ Connection error from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("❌ Accept error: {}", e);
                }
            }
        }
    }

    /// Gérer une connexion avec handshake Bitcoin
    async fn handle_connection(
        mut stream: TcpStream,
        peer_manager: Arc<PeerManager>,
        blockchain: SharedBlockchain,
        peer_addr: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Créer un peer temporaire
        let peer_id = format!("peer_{}", uuid::Uuid::new_v4());
        let mut peer = Peer::new(peer_id.clone(), peer_addr.clone());

        // Envoyer notre message Version (étape 1 du handshake Bitcoin)
        let version_msg = P2PMessage::Version {
            version: peer_manager.protocol_version,
            services: 0,
            timestamp: Utc::now().timestamp(),
            addr_recv: peer_addr.clone(),
            addr_from: format!("0.0.0.0:{}", 0), // Simplifié
            nonce: rand::random(),
            user_agent: peer_manager.user_agent.clone(),
            start_height: blockchain.read().unwrap().len() as u64,
        };

        let bytes = version_msg.to_bytes()?;
        stream.write_all(&bytes).await?;
        peer.mark_version_sent();

        println!("📤 Sent Version to {}", peer_addr);

        // Ajouter le peer temporairement
        peer_manager.add_peer(peer.clone()).ok();

        let mut buffer = vec![0u8; 65536]; // Bitcoin messages peuvent être grands

        loop {
            let n = stream.read(&mut buffer).await?;

            if n == 0 {
                println!("🔌 Connection closed from {}", peer_addr);
                break;
            }

            let message = P2PMessage::from_bytes(&buffer[..n])?;

            // Traiter le message
            let response = Self::process_message(
                message,
                &peer_manager,
                &blockchain,
                &peer_id,
            ).await?;

            // Envoyer la réponse si nécessaire
            if let Some(resp) = response {
                let bytes = resp.to_bytes()?;
                stream.write_all(&bytes).await?;
            }
        }

        Ok(())
    }

    /// Traiter un message P2P Bitcoin
    async fn process_message(
        message: P2PMessage,
        peer_manager: &Arc<PeerManager>,
        blockchain: &SharedBlockchain,
        peer_id: &str,
    ) -> Result<Option<P2PMessage>, Box<dyn std::error::Error>> {
        match message {
            // === HANDSHAKE ===
            P2PMessage::Version {
                version,
                user_agent,
                start_height,
                services,
                ..
            } => {
                println!("📨 Received Version from {} (height: {})", user_agent, start_height);

                // Mettre à jour le peer
                peer_manager.update_peer(peer_id, |peer| {
                    peer.mark_connected(version, user_agent.clone(), start_height, services);
                }).ok();

                // Répondre avec Verack (étape 2 du handshake)
                Ok(Some(P2PMessage::Verack))
            }

            P2PMessage::Verack => {
                println!("📨 Received Verack - Handshake complete!");

                peer_manager.update_peer(peer_id, |peer| {
                    peer.handshake_state = HandshakeState::Connected;
                }).ok();

                Ok(None)
            }

            // === PING/PONG ===
            P2PMessage::Ping { nonce } => {
                println!("📨 Received Ping (nonce: {})", nonce);
                Ok(Some(P2PMessage::Pong { nonce }))
            }

            P2PMessage::Pong { nonce } => {
                println!("📨 Received Pong (nonce: {})", nonce);
                // TODO: Calculer latence
                Ok(None)
            }

            // === INVENTORY ===
            P2PMessage::Inv(inv_vectors) => {
                println!("📨 Received Inv with {} items", inv_vectors.len());
                // TODO: Demander les données avec GetData si on ne les a pas
                Ok(None)
            }

            P2PMessage::GetData(inv_vectors) => {
                println!("📨 Received GetData for {} items", inv_vectors.len());
                // TODO: Envoyer les blocs/tx demandés
                Ok(None)
            }

            // === HEADERS FIRST ===
            P2PMessage::GetHeaders { .. } => {
                println!("📨 Received GetHeaders");
                let bc = blockchain.read().unwrap();

                // Convertir blocs en headers
                let headers: Vec<BlockHeader> = bc.chain
                    .iter()
                    .map(|block| BlockHeader::from_block(block))
                    .collect();

                Ok(Some(P2PMessage::Headers(headers)))
            }

            P2PMessage::Headers(headers) => {
                println!("📨 Received {} headers", headers.len());
                // TODO: Traiter les headers (Itine s'occupera de ça)
                Ok(None)
            }

            // === PEER DISCOVERY ===
            P2PMessage::GetAddr => {
                println!("📨 Received GetAddr");
                let addrs = peer_manager.get_peer_addresses(100);
                Ok(Some(P2PMessage::Addr(addrs)))
            }

            P2PMessage::Addr(addrs) => {
                println!("📨 Received {} peer addresses", addrs.len());
                // TODO: Ajouter les peers
                Ok(None)
            }

            _ => {
                println!("⚠️ Unhandled message type");
                Ok(None)
            }
        }
    }
}
```

**✅ Critère de validation** : Le serveur effectue le handshake Bitcoin (version/verack).

---

### Tâche 1.5 : Client P2P Bitcoin (Priorité : HAUTE)
**Fichier** : `01-mini-blockchain/src/network/p2p_client.rs`
**Durée** : 1 jour

#### Code à implémenter
```rust
use crate::network::messages::*;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use chrono::Utc;

/// Client P2P Bitcoin
pub struct P2PClient;

impl P2PClient {
    /// Se connecter à un peer avec handshake Bitcoin
    pub async fn connect_with_handshake(
        peer_addr: &str,
        our_height: u64,
    ) -> Result<TcpStream, Box<dyn std::error::Error>> {
        // Se connecter
        let mut stream = TcpStream::connect(peer_addr).await?;

        // Envoyer Version
        let version_msg = P2PMessage::Version {
            version: 70015,
            services: 0,
            timestamp: Utc::now().timestamp(),
            addr_recv: peer_addr.to_string(),
            addr_from: "0.0.0.0:0".to_string(),
            nonce: rand::random(),
            user_agent: "/MiniBlockchain:0.1.0/".to_string(),
            start_height: our_height,
        };

        let bytes = version_msg.to_bytes()?;
        stream.write_all(&bytes).await?;

        // Attendre Verack
        let mut buffer = vec![0u8; 8192];
        let n = stream.read(&mut buffer).await?;

        let response = P2PMessage::from_bytes(&buffer[..n])?;

        match response {
            P2PMessage::Verack => {
                println!("✅ Handshake successful with {}", peer_addr);
                Ok(stream)
            }
            _ => Err("Expected Verack".into()),
        }
    }

    /// Envoyer un message et recevoir réponse
    pub async fn send_message(
        peer_addr: &str,
        message: P2PMessage,
    ) -> Result<Option<P2PMessage>, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(peer_addr).await?;

        let bytes = message.to_bytes()?;
        stream.write_all(&bytes).await?;

        let mut buffer = vec![0u8; 65536];
        let n = stream.read(&mut buffer).await?;

        if n == 0 {
            return Ok(None);
        }

        let response = P2PMessage::from_bytes(&buffer[..n])?;
        Ok(Some(response))
    }

    /// Demander des headers (Bitcoin getheaders)
    pub async fn get_headers(
        peer_addr: &str,
        block_locator: Vec<String>,
    ) -> Result<Vec<BlockHeader>, Box<dyn std::error::Error>> {
        let msg = P2PMessage::GetHeaders {
            version: 70015,
            block_locator_hashes: block_locator,
            hash_stop: "0".repeat(64), // Pas de limite
        };

        match Self::send_message(peer_addr, msg).await? {
            Some(P2PMessage::Headers(headers)) => Ok(headers),
            _ => Err("Invalid response".into()),
        }
    }
}
```

---

## ⚙️ Phase 3 : Configuration (identique mais avec ajustements Bitcoin)

*[Le reste de la configuration reste similaire mais avec les nouveaux types de messages]*

---

## 📚 Références Bitcoin

### Documentation officielle
- [Bitcoin P2P Protocol](https://en.bitcoin.it/wiki/Protocol_documentation)
- [Bitcoin Developer Guide - P2P Network](https://developer.bitcoin.org/devguide/p2p_network.html)
- [BIP 0031 - Pong Message](https://github.com/bitcoin/bips/blob/master/bip-0031.mediawiki)
- [BIP 0130 - sendheaders](https://github.com/bitcoin/bips/blob/master/bip-0130.mediawiki)

### Concepts implémentés (simplifiés)
✅ Version handshake (version/verack)
✅ Inventory vectors (inv/getdata)
✅ Headers-first synchronization
✅ Ping/pong avec nonce
✅ Peer discovery (addr message)
✅ Block locators

### Simplifications vs Bitcoin réel
- Pas de bloom filters
- Pas de compact blocks (BIP 152)
- Pas de DNS seeds
- Format binaire simplifié (bincode au lieu du format Bitcoin)
- Pas de checksum dans les messages

---

**Bon courage Mounirou ! Tu implémentes le vrai protocole Bitcoin ! 🚀**
