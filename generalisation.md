# Plan : Transformer Mini-Blockchain en Blockchain P2P Décentralisée (3 Nodes)

## 📋 Contexte

### Situation actuelle
Vous avez une **mini-blockchain éducative centralisée** avec :
- ✅ Blocs chaînés avec hashing SHA-256
- ✅ Proof of Work fonctionnel
- ✅ Validation d'intégrité de chaîne
- ✅ API REST HTTP (Axum)
- ✅ Stockage en mémoire (Arc<RwLock<Blockchain>>)
- ❌ **1 serveur centralisé** (localhost:8090)
- ❌ Aucun réseau P2P
- ❌ Aucun consensus distribué

### Objectif
Créer une **blockchain décentralisée P2P comme Bitcoin** avec :
- 🎯 **3 nodes indépendants** qui communiquent entre eux
- 🎯 Réseau **Peer-to-Peer** (pas de serveur central)
- 🎯 **Consensus décentralisé** (longest chain rule)
- 🎯 **Synchronisation automatique** entre nodes
- 🎯 **Propagation de blocs** à tous les peers
- 🎯 Chaque personne peut lancer un node

### Pourquoi ce changement ?
Pour comprendre les **vrais mécanismes de Bitcoin** :
- Décentralisation (pas de point de défaillance unique)
- Consensus distribué (tous les nodes s'accordent)
- Résilience (nodes peuvent partir/rejoindre)
- Architecture P2P (communication directe entre peers)

---

## 🏗️ Architecture Cible

### Vue d'ensemble : 3 Nodes P2P

```
                    RÉSEAU P2P LOCAL

    ┌─────────────────────────────────────────────────┐
    │                                                 │
    │   ┌──────────┐         ┌──────────┐            │
    │   │  Node 1  │◄───────►│  Node 2  │            │
    │   │ :8091    │         │ :8092    │            │
    │   └─────┬────┘         └────┬─────┘            │
    │         │                   │                   │
    │         │    ┌──────────┐   │                   │
    │         └───►│  Node 3  │◄──┘                   │
    │              │ :8093    │                       │
    │              └──────────┘                       │
    │                                                 │
    │  Chaque node a :                                │
    │  - Sa propre blockchain                         │
    │  - Sa liste de peers                            │
    │  - Son API HTTP (lecture)                       │
    │  - Son serveur P2P (communication)              │
    └─────────────────────────────────────────────────┘
```

### Architecture d'un Node

```
┌─────────────────────────────────────────────────────┐
│                    NODE COMPLET                      │
├─────────────────────────────────────────────────────┤
│                                                      │
│  ┌────────────┐    ┌──────────────┐                │
│  │  HTTP API  │    │  P2P Server  │                │
│  │  :809x     │    │  :909x       │                │
│  └─────┬──────┘    └──────┬───────┘                │
│        │                   │                        │
│        ▼                   ▼                        │
│  ┌─────────────────────────────────┐               │
│  │      BLOCKCHAIN ENGINE          │               │
│  │  ┌──────────────────────────┐   │               │
│  │  │  Blockchain              │   │               │
│  │  │  (Arc<RwLock>)           │   │               │
│  │  └──────────────────────────┘   │               │
│  │  ┌──────────────────────────┐   │               │
│  │  │  Mempool                 │   │               │
│  │  │  (pending transactions)  │   │               │
│  │  └──────────────────────────┘   │               │
│  │  ┌──────────────────────────┐   │               │
│  │  │  Peers Manager           │   │               │
│  │  │  (connected nodes)       │   │               │
│  │  └──────────────────────────┘   │               │
│  └─────────────────────────────────┘               │
│                                                      │
│  ┌─────────────────────────────────┐               │
│  │      CONSENSUS LAYER            │               │
│  │  - Block validation             │               │
│  │  - Chain synchronization        │               │
│  │  - Longest chain selection      │               │
│  └─────────────────────────────────┘               │
│                                                      │
│  ┌─────────────────────────────────┐               │
│  │      NETWORK LAYER              │               │
│  │  - Peer discovery               │               │
│  │  - Block propagation            │               │
│  │  - Message serialization        │               │
│  └─────────────────────────────────┘               │
└─────────────────────────────────────────────────────┘
```

---

## 🔧 Composants à Ajouter

### 1. **Réseau P2P** (Nouveau module `src/network/`)

#### 1.1 Structure de Peer
```rust
// src/network/peer.rs
pub struct Peer {
    pub id: String,          // Identifiant unique
    pub address: SocketAddr, // IP:Port
    pub connected: bool,     // Statut de connexion
    pub last_seen: DateTime<Utc>,
}
```

#### 1.2 Gestionnaire de Peers
```rust
// src/network/peer_manager.rs
pub struct PeerManager {
    peers: Arc<RwLock<Vec<Peer>>>,
    node_id: String,
}

impl PeerManager {
    pub fn add_peer(&mut self, peer: Peer) -> Result<()>
    pub fn remove_peer(&mut self, id: &str) -> Result<()>
    pub fn get_connected_peers(&self) -> Vec<Peer>
    pub fn broadcast_to_all<T>(&self, msg: T) -> Result<()>
}
```

#### 1.3 Messages P2P
```rust
// src/network/messages.rs
#[derive(Serialize, Deserialize)]
pub enum P2PMessage {
    // Discovery
    Ping,
    Pong,
    GetPeers,
    Peers(Vec<SocketAddr>),

    // Blockchain
    GetBlocks { start_index: u64 },
    Blocks(Vec<Block>),
    NewBlock(Block),

    // Consensus
    GetChainLength,
    ChainLength(usize),
    RequestChain,
    FullChain(Vec<Block>),
}
```

#### 1.4 Serveur P2P
```rust
// src/network/p2p_server.rs
pub struct P2PServer {
    listener: TcpListener,
    peer_manager: Arc<PeerManager>,
    blockchain: SharedBlockchain,
}

impl P2PServer {
    pub async fn start(&mut self) -> Result<()>
    pub async fn handle_connection(&self, stream: TcpStream) -> Result<()>
    pub async fn send_message(&self, peer: &Peer, msg: P2PMessage) -> Result<()>
}
```

### 2. **Consensus & Synchronisation** (Nouveau module `src/consensus/`)

#### 2.1 Synchroniseur de Chaîne
```rust
// src/consensus/sync.rs
pub struct ChainSynchronizer {
    blockchain: SharedBlockchain,
    peer_manager: Arc<PeerManager>,
}

impl ChainSynchronizer {
    // Demande les chaînes à tous les peers
    pub async fn sync_chain(&self) -> Result<()>

    // Sélectionne la chaîne la plus longue valide
    pub fn select_longest_chain(&self, chains: Vec<Vec<Block>>) -> Vec<Block>

    // Remplace la chaîne locale si une plus longue est trouvée
    pub async fn replace_chain_if_longer(&self, new_chain: Vec<Block>) -> Result<bool>
}
```

#### 2.2 Validateur de Consensus
```rust
// src/consensus/validator.rs
pub struct ConsensusValidator;

impl ConsensusValidator {
    // Valide un nouveau bloc reçu du réseau
    pub fn validate_new_block(block: &Block, prev_block: &Block) -> Result<()>

    // Valide une chaîne complète
    pub fn validate_chain(chain: &[Block]) -> Result<()>

    // Vérifie que le PoW est correct
    pub fn verify_proof_of_work(block: &Block, difficulty: usize) -> Result<()>
}
```

#### 2.3 Propagateur de Blocs
```rust
// src/consensus/propagation.rs
pub struct BlockPropagator {
    peer_manager: Arc<PeerManager>,
}

impl BlockPropagator {
    // Diffuse un nouveau bloc à tous les peers
    pub async fn broadcast_block(&self, block: &Block) -> Result<()>

    // Gère un nouveau bloc reçu
    pub async fn handle_new_block(&self, block: Block, blockchain: &mut Blockchain) -> Result<()>
}
```

### 3. **Configuration Node** (Nouveau module `src/config.rs`)

```rust
// src/config.rs
#[derive(Debug, Clone, Deserialize)]
pub struct NodeConfig {
    // Identité
    pub node_id: String,

    // Ports
    pub http_port: u16,        // Ex: 8091
    pub p2p_port: u16,         // Ex: 9091

    // Réseau
    pub bootstrap_nodes: Vec<SocketAddr>, // Peers initiaux
    pub max_peers: usize,

    // Blockchain
    pub difficulty: usize,
    pub auto_mine: bool,       // Mine automatiquement ou non
}
```

### 4. **Mempool** (Nouveau module `src/mempool.rs`)

```rust
// src/mempool.rs
pub struct Mempool {
    pending_data: Arc<RwLock<Vec<String>>>, // Simplifié (pas de vraies tx)
}

impl Mempool {
    pub fn add(&mut self, data: String) -> Result<()>
    pub fn get_pending(&self) -> Vec<String>
    pub fn remove(&mut self, data: &str)
    pub fn clear(&mut self)
}
```

### 5. **Stockage Persistant** (Nouveau module `src/storage/`)

```rust
// src/storage/db.rs
pub trait BlockchainStorage {
    fn save_block(&self, block: &Block) -> Result<()>;
    fn load_blocks(&self) -> Result<Vec<Block>>;
    fn get_block(&self, index: u64) -> Result<Option<Block>>;
}

// Implémentation simple avec JSON
pub struct JsonStorage {
    file_path: PathBuf,
}
```

---

## 📝 Plan d'Implémentation Détaillé

### Phase 1 : Infrastructure Réseau P2P (3-5 jours)

#### Étape 1.1 : Messages P2P
**Fichier** : `src/network/messages.rs`
- Définir enum `P2PMessage` avec sérialisation
- Implémenter encodage/décodage binaire ou JSON
- Tests unitaires pour sérialisation

#### Étape 1.2 : Peer Management
**Fichier** : `src/network/peer.rs`, `src/network/peer_manager.rs`
- Structure `Peer` avec métadonnées
- `PeerManager` pour gérer la liste
- Méthodes : add, remove, get_connected, broadcast

#### Étape 1.3 : Serveur P2P
**Fichier** : `src/network/p2p_server.rs`
- TcpListener pour accepter connexions
- Handler async pour chaque peer
- Envoi/réception de messages P2P
- Heartbeat (ping/pong) pour détecter peers déconnectés

#### Étape 1.4 : Client P2P
**Fichier** : `src/network/p2p_client.rs`
- Connexion aux peers bootstrap
- Envoi de requêtes aux peers
- Gestion de reconnexion

### Phase 2 : Consensus & Synchronisation (3-5 jours)

#### Étape 2.1 : Validateur
**Fichier** : `src/consensus/validator.rs`
- Validation de blocs individuels
- Validation de chaîne complète
- Vérification PoW stricte

#### Étape 2.2 : Synchroniseur
**Fichier** : `src/consensus/sync.rs`
- Demander longueur de chaîne aux peers
- Télécharger chaîne complète du peer le plus long
- Remplacer chaîne locale si plus courte
- Gérer forks (règle : longest chain wins)

#### Étape 2.3 : Propagation
**Fichier** : `src/consensus/propagation.rs`
- Broadcast nouveau bloc à tous peers
- Recevoir bloc et l'ajouter si valide
- Éviter boucles infinies (tracking des blocs déjà vus)

### Phase 3 : Configuration & Démarrage Multi-Nodes (2-3 jours)

#### Étape 3.1 : Système de Configuration
**Fichier** : `src/config.rs`
- Charger config depuis fichier TOML
- Variables d'environnement
- Configs pour Node 1, 2, 3

#### Étape 3.2 : Refactoring Main
**Fichier** : `src/main.rs`
- Initialiser node avec config
- Démarrer HTTP API ET serveur P2P en parallèle
- Connexion aux bootstrap nodes
- Sync initial avec le réseau

#### Étape 3.3 : Fichiers de Configuration
**Fichiers** : `config/node1.toml`, `config/node2.toml`, `config/node3.toml`
```toml
# config/node1.toml
[node]
node_id = "node-1"
http_port = 8091
p2p_port = 9091

[network]
bootstrap_nodes = [
    "127.0.0.1:9092",
    "127.0.0.1:9093"
]
max_peers = 10

[blockchain]
difficulty = 2
auto_mine = false
```

### Phase 4 : Stockage Persistant (1-2 jours)

#### Étape 4.1 : Interface de Stockage
**Fichier** : `src/storage/mod.rs`, `src/storage/json_storage.rs`
- Trait `BlockchainStorage`
- Implémentation JSON simple
- Save/Load blockchain vers fichier

#### Étape 4.2 : Intégration
**Fichiers** : `src/blockchain.rs`, `src/main.rs`
- Charger blockchain au démarrage
- Sauvegarder après chaque nouveau bloc
- Path : `data/node{X}/blockchain.json`

### Phase 5 : Mempool & Mining Distribué (2-3 jours)

#### Étape 5.1 : Mempool
**Fichier** : `src/mempool.rs`
- Pool de données en attente
- API pour ajouter/retirer
- Propagation aux peers

#### Étape 5.2 : Mining Loop
**Fichier** : `src/consensus/miner.rs`
- Boucle infinie qui mine des blocs
- Prend données du mempool
- Broadcast bloc miné à tous peers
- Compétition entre nodes (premier qui mine gagne)

### Phase 6 : API Updates & Testing (2-3 jours)

#### Étape 6.1 : Mettre à jour API
**Fichier** : `src/api/handlers.rs`
- `POST /blocks` → Ajoute au mempool (ne mine pas directement)
- `GET /peers` → Liste des peers connectés
- `GET /sync` → Force synchronisation
- `GET /status` → État du node (hauteur, peers, mining)

#### Étape 6.2 : Tests d'Intégration
**Fichier** : `tests/p2p_tests.rs`
- Démarrer 3 nodes en parallèle
- Vérifier découverte de peers
- Ajouter bloc sur Node 1 → vérifier propagation Node 2, 3
- Tester sync quand node rejoint tard

#### Étape 6.3 : Scripts de Démarrage
**Fichiers** : `scripts/start-node1.sh`, `scripts/start-node2.sh`, `scripts/start-node3.sh`
```bash
#!/bin/bash
# scripts/start-node1.sh
cargo run -- --config config/node1.toml
```

---

## 🔄 Flow d'Exécution Complet

### Démarrage d'un Node

```
1. Charger configuration (node1.toml)
2. Initialiser blockchain (charger depuis storage ou créer genesis)
3. Initialiser mempool vide
4. Initialiser PeerManager
5. Démarrer HTTP API (port 8091)
6. Démarrer P2P Server (port 9091)
7. Se connecter aux bootstrap nodes
8. Synchroniser la blockchain avec peers
9. Démarrer mining loop (si auto_mine=true)
10. Prêt à recevoir requêtes
```

### Ajout d'un Bloc (Flow Complet)

```
User → POST /blocks {"data": "Transaction"}
  ↓
Node 1 API Handler
  ↓
Ajouter au Mempool
  ↓
Propagation mempool → Peers (optionnel)
  ↓
Mining Loop prend données du mempool
  ↓
Mine le bloc (PoW)
  ↓
Ajoute à la blockchain locale
  ↓
Broadcast NewBlock(block) à tous peers
  ↓
Node 2, 3 reçoivent le bloc
  ↓
Valident le bloc
  ↓
Ajoutent à leur blockchain
  ↓
Tous les nodes synchronisés ✓
```

### Synchronisation (Node rejoint tard)

```
Node 3 démarre (blockchain vide ou courte)
  ↓
Se connecte à Node 1, 2
  ↓
Envoie GetChainLength à tous peers
  ↓
Reçoit ChainLength(5) de Node 1
  ↓
Reçoit ChainLength(5) de Node 2
  ↓
Chaîne locale = 1 bloc < 5 blocs
  ↓
Envoie RequestChain à Node 1
  ↓
Reçoit FullChain([block0...block4])
  ↓
Valide la chaîne reçue
  ↓
Remplace chaîne locale
  ↓
Maintenant synchronisé ✓
```

### Gestion de Fork (2 blocs minés en même temps)

```
Node 1 mine Bloc A (index 5) → Broadcast
Node 2 mine Bloc B (index 5) → Broadcast
  ↓
Node 3 reçoit Bloc A en premier
  ↓
Node 3 ajoute Bloc A à sa chaîne
  ↓
Node 3 reçoit Bloc B ensuite
  ↓
Node 3 voit conflit (même index)
  ↓
Stocke Bloc B comme "orphan"
  ↓
Continue avec chaîne A (premier reçu)
  ↓
Plus tard, si chaîne B devient plus longue → switch
  ↓
Sinon, chaîne A reste la principale
```

---

## 📂 Structure Finale du Projet

```
01-mini-blockchain/
├── Cargo.toml
├── config/
│   ├── node1.toml
│   ├── node2.toml
│   └── node3.toml
├── scripts/
│   ├── start-node1.sh
│   ├── start-node2.sh
│   ├── start-node3.sh
│   └── test-network.sh
├── data/
│   ├── node1/
│   │   └── blockchain.json
│   ├── node2/
│   │   └── blockchain.json
│   └── node3/
│       └── blockchain.json
├── src/
│   ├── main.rs                  # Entry point (multi-node support)
│   ├── lib.rs                   # Module exports
│   ├── config.rs                # Configuration loading
│   ├── block.rs                 # ✅ Déjà implémenté
│   ├── blockchain.rs            # ✅ Déjà implémenté (à étendre)
│   ├── proof_of_work.rs         # ✅ Déjà implémenté
│   ├── mempool.rs               # 🆕 NOUVEAU
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers.rs          # Update pour mempool + peers
│   │   └── models.rs            # ✅ Déjà implémenté
│   ├── network/                 # 🆕 NOUVEAU MODULE
│   │   ├── mod.rs
│   │   ├── messages.rs          # P2P message types
│   │   ├── peer.rs              # Peer struct
│   │   ├── peer_manager.rs      # Peer management
│   │   ├── p2p_server.rs        # P2P TCP server
│   │   └── p2p_client.rs        # P2P TCP client
│   ├── consensus/               # 🆕 NOUVEAU MODULE
│   │   ├── mod.rs
│   │   ├── validator.rs         # Block/chain validation
│   │   ├── sync.rs              # Chain synchronization
│   │   ├── propagation.rs       # Block broadcasting
│   │   └── miner.rs             # Mining loop
│   └── storage/                 # 🆕 NOUVEAU MODULE
│       ├── mod.rs
│       └── json_storage.rs      # JSON file storage
├── tests/
│   ├── blockchain_tests.rs      # ✅ Déjà implémenté
│   ├── api_tests.rs             # ✅ Déjà implémenté
│   └── p2p_tests.rs             # 🆕 NOUVEAU
└── examples/
    └── three_nodes.rs           # 🆕 Example de 3 nodes locaux
```

---

## 🧪 Plan de Test

### Test 1 : Démarrage Basique
```bash
# Terminal 1
./scripts/start-node1.sh

# Terminal 2
./scripts/start-node2.sh

# Terminal 3
./scripts/start-node3.sh

# Vérifier logs : "Connected to peer node-X"
```

### Test 2 : Propagation de Bloc
```bash
# Ajouter un bloc sur Node 1
curl -X POST http://localhost:8091/blocks \
  -H "Content-Type: application/json" \
  -d '{"data":"Test from Node 1"}'

# Attendre quelques secondes

# Vérifier sur Node 2
curl http://localhost:8092/chain

# Vérifier sur Node 3
curl http://localhost:8093/chain

# Résultat attendu : Tous ont le même bloc
```

### Test 3 : Synchronisation
```bash
# Démarrer seulement Node 1 et 2
./scripts/start-node1.sh
./scripts/start-node2.sh

# Ajouter 5 blocs
for i in {1..5}; do
  curl -X POST http://localhost:8091/blocks -d "{\"data\":\"Block $i\"}"
  sleep 1
done

# Vérifier Node 1 et 2 ont 5 blocs
curl http://localhost:8091/chain | jq '.length'  # 6 (5 + genesis)
curl http://localhost:8092/chain | jq '.length'  # 6

# Démarrer Node 3 (late joiner)
./scripts/start-node3.sh

# Attendre sync

# Vérifier Node 3 a rattrapé
curl http://localhost:8093/chain | jq '.length'  # 6
```

### Test 4 : Compétition de Mining
```bash
# Config : auto_mine = true pour les 3 nodes

# Démarrer tous
./scripts/start-node1.sh &
./scripts/start-node2.sh &
./scripts/start-node3.sh &

# Ajouter données au mempool de chaque node
curl -X POST http://localhost:8091/blocks -d '{"data":"Tx1"}'
curl -X POST http://localhost:8092/blocks -d '{"data":"Tx2"}'
curl -X POST http://localhost:8093/blocks -d '{"data":"Tx3"}'

# Observer logs : quel node mine en premier ?
# Vérifier consensus : tous convergent vers même chaîne
```

---

## 🎯 Critères de Succès

### Fonctionnalités Minimum (MVP)
- [ ] 3 nodes peuvent démarrer indépendamment
- [ ] Nodes se découvrent mutuellement (peer discovery)
- [ ] Nouveau bloc miné sur Node 1 → propagé à Node 2, 3
- [ ] Node qui rejoint tard synchronise automatiquement
- [ ] Validation stricte : blocs invalides sont rejetés
- [ ] API HTTP fonctionne sur chaque node
- [ ] Chaîne persiste sur disque (redémarrage OK)

### Fonctionnalités Avancées (Nice-to-have)
- [ ] Mempool partagé entre nodes
- [ ] Gestion de forks (longest chain rule)
- [ ] Métriques : nodes/sec, blocs/min
- [ ] Interface CLI pour interagir avec node
- [ ] Tests automatisés pour scenarios P2P

---

## 📚 Dépendances Supplémentaires

Ajouter à `Cargo.toml` :

```toml
[dependencies]
# Existantes (garder toutes)
# ...

# Nouvelles pour P2P
tokio = { version = "1.0", features = ["full", "net", "sync"] }
bincode = "1.3"          # Sérialisation binaire efficace
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"             # Parsing config files

# Optionnel mais recommandé
uuid = { version = "1.0", features = ["v4"] }  # Node IDs
```

---

## ⏱️ Timeline Estimé

| Phase | Durée | Complexité |
|-------|-------|-----------|
| Phase 1 : Réseau P2P | 3-5 jours | ⭐⭐⭐ |
| Phase 2 : Consensus | 3-5 jours | ⭐⭐⭐⭐ |
| Phase 3 : Config Multi-nodes | 2-3 jours | ⭐⭐ |
| Phase 4 : Stockage | 1-2 jours | ⭐ |
| Phase 5 : Mempool & Mining | 2-3 jours | ⭐⭐⭐ |
| Phase 6 : Testing | 2-3 jours | ⭐⭐ |

**Total : 13-21 jours** (environ 3 semaines)

---

## 🚀 Démarrage Rapide (après implémentation)

```bash
# 1. Build le projet
cargo build --release

# 2. Créer dossiers de données
mkdir -p data/{node1,node2,node3}

# 3. Démarrer les 3 nodes (3 terminaux)
cargo run --release -- --config config/node1.toml
cargo run --release -- --config config/node2.toml
cargo run --release -- --config config/node3.toml

# 4. Vérifier connectivité
curl http://localhost:8091/peers
curl http://localhost:8092/peers
curl http://localhost:8093/peers

# 5. Ajouter un bloc
curl -X POST http://localhost:8091/blocks \
  -H "Content-Type: application/json" \
  -d '{"data":"Premier bloc distribué!"}'

# 6. Vérifier propagation
curl http://localhost:8092/chain
curl http://localhost:8093/chain
```

---

## 📖 Ressources & Références

### Protocols P2P
- Bitcoin P2P Protocol : https://en.bitcoin.it/wiki/Protocol_documentation
- Ethereum Devp2p : https://github.com/ethereum/devp2p

### Consensus
- Nakamoto Consensus : https://bitcoin.org/bitcoin.pdf
- Longest Chain Rule : https://en.bitcoin.it/wiki/Block_chain

### Implémentations de Référence
- Bitcoin Core (C++) : https://github.com/bitcoin/bitcoin
- Parity Ethereum (Rust) : https://github.com/paritytech/parity-ethereum

---

## 💡 Simplifications vs Bitcoin Réel

Ce plan crée une blockchain fonctionnelle mais **simplifiée** :

| Aspect | Notre Implémentation | Bitcoin Réel |
|--------|---------------------|--------------|
| **Transactions** | String simple | UTXO, inputs/outputs, signatures |
| **Consensus** | Longest chain | Longest chain + checkpoints |
| **Difficulté** | Fixe | Ajustement tous les 2016 blocs |
| **Récompenses** | Aucune | Block reward + fees |
| **Mempool** | Simple liste | Priority queue par fees |
| **Réseau** | 3 nodes locaux | 15,000+ nodes globaux |
| **Storage** | JSON | LevelDB avec merkle trees |
| **Cryptographie** | Hash seulement | ECDSA signatures + merkle proofs |

Mais vous aurez les **concepts fondamentaux** :
- ✅ Décentralisation
- ✅ Consensus distribué
- ✅ Propagation P2P
- ✅ Synchronisation
- ✅ Résilience

---

## 🎓 Compétences Acquises

Après ce projet, vous maîtriserez :
- Architecture P2P en Rust
- Programmation réseau async (Tokio)
- Consensus décentralisé (Nakamoto)
- Sérialisation/désérialisation efficace
- Gestion d'état distribué
- Threading et synchronisation (Arc, RwLock)
- Tests d'intégration multi-processus

**Parfait pour comprendre Bitcoin, Ethereum, et autres cryptomonnaies !** 🚀
