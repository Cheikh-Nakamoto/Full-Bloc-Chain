# 01 - Mini-Blockchain | Basic Blockchain in Rust 🦀⛓️

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](../LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.93.0+-orange.svg)](https://www.rust-lang.org/)

---

## 🇬🇧 English Version

### About

A simple educational blockchain implementation in Rust featuring Proof of Work consensus mechanism and a RESTful API.

This project demonstrates core blockchain concepts including:
- Block structure and hashing
- Proof of Work mining
- Chain integrity validation
- HTTP API for blockchain interaction

### Features

- ✅ **Block Structure**: Complete block with index, timestamp, data, hashes, and nonce
- ✅ **SHA-256 Hashing**: Cryptographic hashing for block integrity
- ✅ **Proof of Work**: Mining algorithm with adjustable difficulty
- ✅ **Chain Validation**: Comprehensive integrity checking
- ✅ **REST API**: HTTP endpoints for blockchain operations
- ✅ **Thread-Safe**: Concurrent access with Arc<RwLock>

### Quick Start

#### Prerequisites

- Rust 1.93.0 or higher
- Cargo (comes with Rust)

#### Installation & Running

```bash
# Clone the repository
git clone https://github.com/cheikh-nakamoto/Full-Bloc-Chain
cd Full-Bloc-Chain/01-mini-blockchain

# Build the project
cargo build

# Run the server
cargo run

# The API will be available at http://localhost:8080
```

### API Documentation

#### GET /chain
Retrieve the entire blockchain.

```bash
curl http://localhost:8080/chain
```

**Response:**
```json
{
  "chain": [...],
  "length": 5,
  "is_valid": true
}
```

#### POST /blocks
Add a new block to the blockchain.

```bash
curl -X POST http://localhost:8080/blocks \
  -H "Content-Type: application/json" \
  -d '{"data":"My transaction"}'
```

**Response:**
```json
{
  "block": {
    "index": 1,
    "timestamp": "2026-02-10T12:00:00Z",
    "data": "My transaction",
    "previous_hash": "...",
    "hash": "...",
    "nonce": 12345
  },
  "message": "Block added successfully"
}
```

#### GET /blocks/:index
Get a specific block by its index.

```bash
curl http://localhost:8080/blocks/1
```

#### GET /validate
Validate the blockchain integrity.

```bash
curl http://localhost:8080/validate
```

**Response:**
```json
{
  "is_valid": true,
  "chain_length": 5
}
```

### Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run a specific test
cargo test test_blockchain_creation

# Run example
cargo run --example simple_blockchain
```

### Code Structure

```
01-mini-blockchain/
├── src/
│   ├── main.rs              # API server entry point
│   ├── lib.rs               # Library root
│   ├── block.rs             # Block structure and hashing
│   ├── blockchain.rs        # Blockchain logic and validation
│   ├── proof_of_work.rs     # Mining algorithm
│   └── api/
│       ├── handlers.rs      # HTTP request handlers
│       └── models.rs        # Request/response models
├── tests/
│   ├── blockchain_tests.rs  # Integration tests
│   └── api_tests.rs         # API tests
└── examples/
    └── simple_blockchain.rs # Usage example
```

### Architecture

- **Block**: Immutable data structure with cryptographic hash
- **Blockchain**: Vector of blocks with genesis block
- **Proof of Work**: Mining algorithm requiring hash with N leading zeros
- **API**: Axum-based REST API with thread-safe blockchain access

### License

MIT License - See [LICENSE](../LICENSE)

---

## 🇫🇷 Version Française

### À Propos

Une implémentation éducative simple de blockchain en Rust avec mécanisme de consensus Proof of Work et API RESTful.

Ce projet démontre les concepts fondamentaux de la blockchain incluant :
- Structure de bloc et hachage
- Mining Proof of Work
- Validation de l'intégrité de la chaîne
- API HTTP pour l'interaction avec la blockchain

### Fonctionnalités

- ✅ **Structure de Bloc**: Bloc complet avec index, timestamp, données, hashes et nonce
- ✅ **Hachage SHA-256**: Hachage cryptographique pour l'intégrité des blocs
- ✅ **Proof of Work**: Algorithme de mining avec difficulté ajustable
- ✅ **Validation de Chaîne**: Vérification complète de l'intégrité
- ✅ **API REST**: Endpoints HTTP pour les opérations blockchain
- ✅ **Thread-Safe**: Accès concurrent avec Arc<RwLock>

### Démarrage Rapide

#### Prérequis

- Rust 1.93.0 ou supérieur
- Cargo (fourni avec Rust)

#### Installation & Exécution

```bash
# Cloner le dépôt
git clone https://github.com/cheikh-nakamoto/Full-Bloc-Chain
cd Full-Bloc-Chain/01-mini-blockchain

# Construire le projet
cargo build

# Lancer le serveur
cargo run

# L'API sera disponible sur http://localhost:8080
```

### Documentation API

#### GET /chain
Récupérer toute la blockchain.

```bash
curl http://localhost:8080/chain
```

**Réponse:**
```json
{
  "chain": [...],
  "length": 5,
  "is_valid": true
}
```

#### POST /blocks
Ajouter un nouveau bloc à la blockchain.

```bash
curl -X POST http://localhost:8080/blocks \
  -H "Content-Type: application/json" \
  -d '{"data":"Ma transaction"}'
```

**Réponse:**
```json
{
  "block": {
    "index": 1,
    "timestamp": "2026-02-10T12:00:00Z",
    "data": "Ma transaction",
    "previous_hash": "...",
    "hash": "...",
    "nonce": 12345
  },
  "message": "Block added successfully"
}
```

#### GET /blocks/:index
Obtenir un bloc spécifique par son index.

```bash
curl http://localhost:8080/blocks/1
```

#### GET /validate
Valider l'intégrité de la blockchain.

```bash
curl http://localhost:8080/validate
```

**Réponse:**
```json
{
  "is_valid": true,
  "chain_length": 5
}
```

### Tests

```bash
# Exécuter tous les tests
cargo test

# Exécuter avec sortie détaillée
cargo test -- --nocapture

# Exécuter un test spécifique
cargo test test_blockchain_creation

# Exécuter l'exemple
cargo run --example simple_blockchain
```

### Structure du Code

```
01-mini-blockchain/
├── src/
│   ├── main.rs              # Point d'entrée du serveur API
│   ├── lib.rs               # Racine de la bibliothèque
│   ├── block.rs             # Structure de bloc et hachage
│   ├── blockchain.rs        # Logique blockchain et validation
│   ├── proof_of_work.rs     # Algorithme de mining
│   └── api/
│       ├── handlers.rs      # Gestionnaires de requêtes HTTP
│       └── models.rs        # Modèles requête/réponse
├── tests/
│   ├── blockchain_tests.rs  # Tests d'intégration
│   └── api_tests.rs         # Tests API
└── examples/
    └── simple_blockchain.rs # Exemple d'utilisation
```

### Architecture

- **Block**: Structure de données immuable avec hash cryptographique
- **Blockchain**: Vecteur de blocs avec bloc genesis
- **Proof of Work**: Algorithme de mining nécessitant un hash avec N zéros
- **API**: API REST basée sur Axum avec accès thread-safe à la blockchain

### Licence

Licence MIT - Voir [LICENSE](../LICENSE)

---

## 📚 Learning Resources | Ressources d'Apprentissage

### Blockchain Concepts
- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
- [Blockchain Basics](https://www.investopedia.com/terms/b/blockchain.asp)

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

**Author | Auteur**: Cheikh Mounirou Coly Diouf
**Project**: 01 - Mini-Blockchain
**Year | Année**: 2026
