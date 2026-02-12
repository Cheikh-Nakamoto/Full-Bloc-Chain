# 📂 Structure Finale du Projet

## 🌳 Arborescence Complète

```
01-mini-blockchain/
├── Cargo.toml                          # Dépendances (secp256k1, rustls, etc.)
├── README.md                           # Documentation générale
├── README_IMPLEMENTATION.md            # Guide d'implémentation (NOUVEAU)
├── PROJECT_STRUCTURE.md                # Ce fichier (NOUVEAU)
│
├── config/                             # 🆕 Configurations nodes
│   ├── node1.toml                      # Config pour node local 1
│   ├── node2.toml                      # Config pour node local 2
│   ├── node3.toml                      # Config pour node local 3
│   └── vps.toml                        # Config pour le VPS
│
├── certs/                              # 🆕 Certificats TLS (NE PAS COMMITER .key)
│   ├── ca/
│   │   ├── ca.crt                      # Certificat autorité
│   │   └── ca.key                      # Clé privée CA (SECRET)
│   ├── node1/
│   │   ├── node1.crt
│   │   └── node1.key                   # (SECRET)
│   ├── node2/
│   │   ├── node2.crt
│   │   └── node2.key                   # (SECRET)
│   ├── node3/
│   │   ├── node3.crt
│   │   └── node3.key                   # (SECRET)
│   └── vps/
│       ├── vps.crt
│       └── vps.key                     # (SECRET)
│
├── scripts/                            # 🆕 Scripts utilitaires
│   ├── generate_certs.sh               # Génération certificats TLS
│   ├── start-node1.sh                  # Démarrer node 1
│   ├── start-node2.sh                  # Démarrer node 2
│   ├── start-node3.sh                  # Démarrer node 3
│   └── test-network.sh                 # Tests réseau P2P
│
├── deploy/                             # 🆕 Déploiement VPS
│   ├── setup_vps.sh                    # Setup complet VPS
│   └── blockchain.service              # Systemd service
│
├── wallets/                            # 🆕 Wallets sauvegardés
│   ├── alice.json                      # Wallet Alice
│   ├── bob.json                        # Wallet Bob
│   └── miner.json                      # Wallet mineur
│
├── data/                               # 🆕 Données persistantes
│   ├── node1/
│   │   └── blockchain.json             # Blockchain node 1
│   ├── node2/
│   │   └── blockchain.json             # Blockchain node 2
│   ├── node3/
│   │   └── blockchain.json             # Blockchain node 3
│   └── vps/
│       └── blockchain.json             # Blockchain VPS
│
├── docs/                               # 🆕 Documentation
│   ├── TLS_SETUP.md                    # Guide setup TLS
│   ├── API.md                          # Documentation API
│   └── CONCEPTS.md                     # Concepts Bitcoin/UTXO
│
├── src/
│   ├── main.rs                         # ✏️ Entry point (à modifier)
│   ├── lib.rs                          # ✏️ Exports modules (à modifier)
│   │
│   ├── block.rs                        # ✏️ MODIFIÉ - Vec<Transaction>
│   ├── blockchain.rs                   # ✏️ MODIFIÉ - UTXO set intégré
│   ├── proof_of_work.rs                # ✅ INCHANGÉ
│   │
│   ├── crypto.rs                       # 🆕 NOUVEAU - ITINE
│   ├── transaction.rs                  # 🆕 NOUVEAU - ITINE
│   ├── utxo_set.rs                     # 🆕 NOUVEAU - MBAYE
│   ├── wallet.rs                       # 🆕 NOUVEAU - MBAYE
│   │
│   ├── api/
│   │   ├── mod.rs                      # ✅ Existant
│   │   ├── handlers.rs                 # ✏️ ÉTENDU - 6 nouveaux endpoints
│   │   └── models.rs                   # ✏️ ÉTENDU - Nouveaux modèles
│   │
│   ├── network/                        # 🆕 NOUVEAU MODULE - MOUNIROU
│   │   ├── mod.rs
│   │   ├── peer.rs                     # Structure Peer
│   │   ├── peer_manager.rs             # Gestion des peers
│   │   ├── p2p_server.rs               # Serveur P2P TCP
│   │   ├── p2p_client.rs               # Client P2P
│   │   ├── messages.rs                 # Messages P2P
│   │   ├── message_format.rs           # Format avec checksums
│   │   ├── heartbeat.rs                # Heartbeat & reconnexion
│   │   ├── rate_limiter.rs             # Rate limiting
│   │   └── metrics.rs                  # Métriques réseau
│   │
│   └── crypto/                         # 🆕 NOUVEAU MODULE
│       └── tls.rs                      # Gestion TLS
│
├── tests/
│   ├── blockchain_tests.rs             # ✏️ Tests existants (à adapter)
│   ├── api_tests.rs                    # ✏️ Tests API (à adapter)
│   ├── integration_tests.rs            # 🆕 Tests end-to-end
│   ├── performance_tests.rs            # 🆕 Tests performance
│   └── p2p_tests.rs                    # 🆕 Tests réseau P2P
│
├── examples/                           # 🆕 Exemples d'utilisation
│   ├── create_wallet.rs                # Créer un wallet
│   ├── send_transaction.rs             # Envoyer des coins
│   └── mine_block.rs                   # Miner un bloc
│
└── .gitignore                          # ✏️ MODIFIÉ - Ignorer .key et wallets

```

---

## 📊 Statistiques du Projet

### Fichiers par Responsable

| Responsable | Nouveaux Fichiers | Fichiers Modifiés | Total |
|-------------|------------------|-------------------|-------|
| **ITINE**   | 4                | 1                 | 5     |
| **MBAYE**   | 3                | 0                 | 3     |
| **MOUNIROU**| 15               | 5                 | 20    |
| **TOTAL**   | **22**           | **6**             | **28**|

### Lignes de Code Estimées

| Module             | Lignes de Code | Complexité |
|--------------------|----------------|------------|
| crypto.rs          | ~300           | ⭐⭐⭐     |
| transaction.rs     | ~250           | ⭐⭐⭐     |
| utxo_set.rs        | ~200           | ⭐⭐       |
| wallet.rs          | ~300           | ⭐⭐⭐⭐   |
| block.rs (modifs)  | +50            | ⭐⭐       |
| blockchain.rs      | +200           | ⭐⭐⭐⭐   |
| network/*          | ~800           | ⭐⭐⭐⭐⭐ |
| api/handlers.rs    | +300           | ⭐⭐⭐     |
| **TOTAL**          | **~2400**      |            |

---

## 🔗 Dépendances entre Modules

```
crypto.rs (ITINE)
    ↓
transaction.rs (ITINE)
    ↓
    ├─→ utxo_set.rs (MBAYE)
    │       ↓
    │   wallet.rs (MBAYE)
    │       ↓
    └─→ block.rs (MOUNIROU)
            ↓
        blockchain.rs (MOUNIROU)
            ↓
        api/handlers.rs (MOUNIROU)
            ↓
        network/* (MOUNIROU)
            ↓
        🎉 Projet Complet
```

---

## 🎯 Modules par Phase

### Phase 1: Fondations (Semaine 1)
```
✅ crypto.rs           (ITINE)
✅ transaction.rs      (ITINE)
✅ utxo_set.rs         (MBAYE)
```

### Phase 2: Intégration (Semaine 2)
```
✅ wallet.rs           (MBAYE)
✅ block.rs            (MOUNIROU)
✅ blockchain.rs       (MOUNIROU)
✅ api/handlers.rs     (MOUNIROU)
✅ api/models.rs       (MOUNIROU)
```

### Phase 3: Réseau & Déploiement (Semaine 3)
```
✅ crypto/tls.rs           (MOUNIROU)
✅ network/peer.rs         (MOUNIROU)
✅ network/p2p_server.rs   (MOUNIROU)
✅ network/p2p_client.rs   (MOUNIROU)
✅ network/heartbeat.rs    (MOUNIROU)
✅ network/rate_limiter.rs (MOUNIROU)
✅ deploy/setup_vps.sh     (MOUNIROU)
```

---

## 📝 Fichiers de Configuration

### node1.toml
```toml
[node]
node_id = "node-1"
http_port = 8091
p2p_port = 9091

[network]
bootstrap_nodes = [
    "127.0.0.1:9092",
    "127.0.0.1:9093",
    "vps.example.com:9094"
]

[tls]
cert_path = "certs/node1/node1.crt"
key_path = "certs/node1/node1.key"
ca_cert_path = "certs/ca/ca.crt"

[blockchain]
difficulty = 2
genesis_address = "miner_address"
```

---

## 🔐 Fichiers Secrets (Ne JAMAIS Commiter)

```
certs/**/*.key          # Clés privées TLS
wallets/*.json          # Wallets utilisateurs
data/                   # Blockchains locales
.env                    # Variables d'environnement
```

### .gitignore Mis à Jour
```gitignore
# Clés privées TLS
certs/**/*.key
certs/**/*.srl

# Wallets
wallets/*.json

# Données
data/

# Rust
target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
```

---

## 🚀 Scripts de Démarrage

### start-node1.sh
```bash
#!/bin/bash
cargo run --release -- --config config/node1.toml
```

### start-all-nodes.sh
```bash
#!/bin/bash
./scripts/start-node1.sh &
./scripts/start-node2.sh &
./scripts/start-node3.sh &
echo "✅ Tous les nodes démarrés"
```

---

## 📊 Endpoints API Complets

### Endpoints Existants
```
GET  /                    # Root
GET  /chain               # Voir la blockchain
POST /blocks              # ❌ DEPRECATED - Utiliser /mine
GET  /blocks/:index       # Voir un bloc
GET  /validate            # Valider la chaîne
```

### Nouveaux Endpoints
```
POST /wallets                      # Créer un wallet
GET  /wallets/:address/balance     # Voir le solde
GET  /utxos/:address               # Voir les UTXO

POST /transactions                 # Créer une transaction
GET  /transactions/:txid           # Voir une transaction

POST /mine                         # Miner un bloc

GET  /peers                        # 🆕 Voir les peers connectés
GET  /health                       # 🆕 Health check
GET  /metrics                      # 🆕 Métriques réseau
```

---

## ✅ Commandes Utiles

### Développement
```bash
# Compiler
cargo build

# Compiler en release
cargo build --release

# Tester
cargo test

# Tester avec output
cargo test -- --nocapture

# Vérifier
cargo check

# Formater
cargo fmt

# Linter
cargo clippy
```

### Réseau
```bash
# Générer certificats
./scripts/generate_certs.sh

# Démarrer node 1
./scripts/start-node1.sh

# Tester le réseau
./scripts/test-network.sh

# Déployer sur VPS
./deploy/setup_vps.sh
```

### API Tests
```bash
# Créer un wallet
curl -X POST http://localhost:8091/wallets

# Voir le solde
curl http://localhost:8091/wallets/<address>/balance

# Créer une transaction
curl -X POST http://localhost:8091/transactions \
  -H "Content-Type: application/json" \
  -d '{"from_wallet_path":"wallets/alice.json","to_address":"bob_addr","amount":1000000000}'

# Miner un bloc
curl -X POST http://localhost:8091/mine \
  -H "Content-Type: application/json" \
  -d '{"miner_address":"miner_addr","transactions":[]}'
```

---

## 🎓 Rappel des Concepts

### UTXO
- Unspent Transaction Output
- "Billet de banque numérique"
- Peut être dépensé UNE SEULE fois

### Transaction
- Inputs: UTXO à dépenser
- Outputs: Nouveaux UTXO créés
- Signatures: Preuve de propriété

### Wallet
- Paire de clés (privée/publique)
- Adresse dérivée de la clé publique
- Gère les UTXO et crée les TX

### Blockchain
- Chaîne de blocs liés
- Chaque bloc contient des transactions
- UTXO set = état global

---

## 📈 Progression Visuelle

```
Semaine 1: [████████░░░░░░░░░░░░] 40%  - Fondations
Semaine 2: [████████████████░░░░] 80%  - Intégration
Semaine 3: [████████████████████] 100% - Réseau & VPS
```

---

**Projet créé par : ITINE, MBAYE, MOUNIROU**
**Date : 2025**
**Licence : MIT**
