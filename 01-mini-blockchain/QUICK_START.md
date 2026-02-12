# 🚀 Quick Start - Par où commencer ?

## 📋 Résumé Ultra-Rapide

Vous allez créer une **blockchain décentralisée complète** avec :
- Transactions UTXO (comme Bitcoin)
- Signatures cryptographiques
- Wallets
- Réseau P2P mesh avec TLS
- Déploiement sur Internet (VPS)

**Durée totale :** 14-21 jours (3 semaines)

---

## 👤 Pour ITINE (Cryptographie & Transactions)

### Votre Mission
Implémenter la **sécurité** de la blockchain : signatures ECDSA, transactions, certificats TLS.

### Vos Fichiers
1. **Plan détaillé :** `~/.claude/plans/plan-itine.md`
2. **Tâches :** 1 à 15 (15 tâches au total)

### Par Où Commencer ?

#### Jour 1 - Setup
```bash
# 1. Lire votre plan complet
cd ~/Full-Bloc-Chain/01-mini-blockchain
cat ~/.claude/plans/plan-itine.md | less

# 2. Ouvrir Cargo.toml
code Cargo.toml

# 3. Ajouter les dépendances (Tâche 1)
# Copier depuis le plan :
#   secp256k1 = { version = "0.29", features = ["rand", "serde"] }
#   rand = "0.8"
#   tokio-rustls = "0.26"
#   rustls = "0.23"
#   rustls-pemfile = "2.0"
#   governor = "0.6"

# 4. Vérifier que ça compile
cargo check
```

#### Jour 2-3 - Module crypto.rs
```bash
# 1. Créer le fichier
touch src/crypto.rs

# 2. Ouvrir dans votre éditeur
code src/crypto.rs

# 3. Suivre le plan - Tâche 2 à 6
# Implémenter :
#   - Structure KeyPair
#   - generate() - Générer une paire de clés
#   - sign() - Signer un message
#   - verify() - Vérifier une signature
#   - get_address() - Générer une adresse

# 4. Tester chaque fonction
cargo test crypto::tests
```

#### Jour 4-6 - Module transaction.rs
```bash
# Suivre le plan - Tâche 7 à 11
# Implémenter Transaction, TxInput, TxOutput

cargo test transaction::tests
```

#### Jour 7-8 - Certificats TLS
```bash
# Tâche 13-15
./scripts/generate_certs.sh
```

### Synchronisation avec l'équipe
- **Jour 6 :** Informer MBAYE que crypto.rs est prêt ✅
- **Jour 7 :** Informer MOUNIROU que transaction.rs est prêt ✅
- **Jour 8 :** Distribuer les certificats à l'équipe ✅

---

## 👤 Pour MBAYE (UTXO Set & Wallets)

### Votre Mission
Gérer **l'état** de la blockchain : UTXO set, wallets, création de transactions.

### Vos Fichiers
1. **Plan détaillé :** `~/.claude/plans/plan-mbaye.md`
2. **Tâches :** 16 à 27 (12 tâches au total)

### Par Où Commencer ?

#### Jour 1-5 - Module utxo_set.rs
```bash
# 1. Lire votre plan
cat ~/.claude/plans/plan-mbaye.md | less

# 2. Créer le fichier
touch src/utxo_set.rs

# 3. Implémenter (Tâches 16-21)
#   - Structure UTXO et UTXOSet
#   - add_utxo() - Ajouter des UTXO
#   - spend_utxo() - Dépenser des UTXO
#   - find_utxos_for_address() - Chercher par adresse
#   - get_balance() - Calculer le solde
#   - update_with_transaction() - Mettre à jour avec une TX

cargo test utxo_set::tests
```

#### Jour 6-9 - Module wallet.rs
```bash
# ⚠️ ATTENDRE qu'ITINE finisse crypto.rs et transaction.rs

# Implémenter (Tâches 22-25)
#   - Structure Wallet
#   - new() et from_keys() - Créer un wallet
#   - create_transaction() - Créer une TX avec sélection UTXO
#   - save_to_file() et load_from_file() - Persister

cargo test wallet::tests
```

#### Jour 10-11 - Tests
```bash
# Tâches 26-27
cargo test utxo_set::tests
cargo test wallet::tests
cargo test --release -- --ignored performance
```

### Synchronisation avec l'équipe
- **Jour 5 :** Informer MOUNIROU que utxo_set.rs est prêt ✅
- **Jour 9 :** Informer MOUNIROU que wallet.rs est prêt ✅

---

## 👤 Pour MOUNIROU (Blockchain, Réseau & API)

### Votre Mission
**Intégrer** tout le système : blockchain, réseau P2P, API, déploiement VPS.

### Vos Fichiers
1. **Plan détaillé :** `~/.claude/plans/plan-mounirou.md`
2. **Tâches :** 28 à 54 (27 tâches au total)

### Par Où Commencer ?

#### Jour 1-3 - Étude du code existant
```bash
# 1. Lire votre plan
cat ~/.claude/plans/plan-mounirou.md | less

# 2. Étudier le code actuel
cat src/block.rs
cat src/blockchain.rs
cat src/api/handlers.rs

# 3. Comprendre la structure
# Vous allez modifier ces fichiers après qu'ITINE et MBAYE terminent
```

#### Jour 4-10 - Modifications Core
```bash
# ⚠️ ATTENDRE qu'ITINE finisse transaction.rs
# ⚠️ ATTENDRE que MBAYE finisse utxo_set.rs

# Tâches 28-35
# Modifier :
#   - block.rs (data → transactions)
#   - blockchain.rs (ajouter UTXO set)
#   - Implémenter validate_transaction()
#   - Implémenter create_block_with_transactions()

cargo test blockchain::tests
```

#### Jour 11-14 - API Complète
```bash
# Tâches 36-43
# Créer 6 nouveaux endpoints :
#   POST /wallets
#   GET /wallets/:address/balance
#   POST /transactions
#   POST /mine
#   GET /utxos/:address
#   GET /transactions/:txid

cargo test api::tests
```

#### Jour 15-21 - Réseau P2P & VPS
```bash
# Tâches 44-54
# Créer network/*
# Implémenter TLS
# Déployer sur VPS
```

### Synchronisation avec l'équipe
- **Jour 10 :** Intégration complète avec ITINE et MBAYE ✅
- **Jour 15 :** Demander les certificats TLS à ITINE ✅
- **Jour 21 :** Déploiement final ✅

---

## 🔄 Timeline Visuelle

### Semaine 1
```
ITINE    : [████████] crypto.rs + transaction.rs
MBAYE    : [████████] utxo_set.rs
MOUNIROU : [████    ] Étude + Préparation réseau
```

### Semaine 2
```
ITINE    : [████    ] Tests + Certificats TLS
MBAYE    : [████████] wallet.rs + Tests
MOUNIROU : [████████] Block + Blockchain + API
```

### Semaine 3
```
ITINE    : [████    ] Support équipe
MBAYE    : [████    ] Tests performance
MOUNIROU : [████████] Réseau P2P + VPS
```

---

## ✅ Checklist Quotidienne (Pour Tous)

### Chaque Matin
- [ ] Lire la tâche du jour dans mon plan
- [ ] Vérifier les dépendances (attendre si nécessaire)
- [ ] Communiquer avec l'équipe

### Pendant le Travail
- [ ] Implémenter la fonctionnalité
- [ ] Écrire les tests en même temps
- [ ] Documenter avec /// comments

### Chaque Soir
- [ ] Exécuter `cargo test`
- [ ] Commit le code si les tests passent
- [ ] Informer l'équipe de la progression

---

## 🆘 En Cas de Problème

### Erreur de compilation
```bash
cargo check
cargo build --verbose
```

### Tests qui échouent
```bash
cargo test nom_du_test -- --nocapture
```

### Besoin d'aide
1. Relire le plan
2. Chercher dans la documentation Rust
3. Demander à l'équipe
4. Ne pas rester bloqué > 30 minutes

---

## 📞 Communication Recommandée

### Daily Standup (Optionnel mais utile)
Chaque jour, partager dans le groupe:
```
Hier : J'ai fait X
Aujourd'hui : Je fais Y
Blockers : Aucun / J'attends Z
```

### Points de Sync Critiques

#### Sync Point 1 (Jour 6)
**ITINE termine crypto.rs → MBAYE peut commencer wallet.rs**

#### Sync Point 2 (Jour 11)
**ITINE + MBAYE terminent → MOUNIROU intègre tout**

#### Sync Point 3 (Jour 15)
**Intégration complète → Tests ensemble**

#### Sync Point 4 (Jour 20)
**Déploiement VPS → Test mesh complet**

---

## 🎯 Objectif Final

Le projet est **terminé** quand :

1. ✅ Alice et Bob ont des wallets
2. ✅ Bloc genesis avec coinbase pour Alice (50 coins)
3. ✅ Alice envoie 30 coins à Bob (transaction signée)
4. ✅ Bloc miné avec cette transaction
5. ✅ UTXO set mis à jour correctement
6. ✅ Alice a 70 coins (20 change + 50 coinbase bloc 2)
7. ✅ Bob a 30 coins
8. ✅ Blockchain valide (signatures + hashes + liens)
9. ✅ Réseau P2P mesh fonctionne
10. ✅ VPS connecté au mesh
11. ✅ Tout communique via TLS

---

## 🔗 Liens Rapides

| Document | Chemin |
|----------|--------|
| **Votre Plan ITINE** | `~/.claude/plans/plan-itine.md` |
| **Votre Plan MBAYE** | `~/.claude/plans/plan-mbaye.md` |
| **Votre Plan MOUNIROU** | `~/.claude/plans/plan-mounirou.md` |
| **Guide Implémentation** | `README_IMPLEMENTATION.md` |
| **Structure Projet** | `PROJECT_STRUCTURE.md` |
| **Plan Réseau** | `~/.claude/plans/network-internet-addon.md` |

---

## 🚀 Commandes de Démarrage Rapide

### ITINE - Premier Jour
```bash
cd ~/Full-Bloc-Chain/01-mini-blockchain
cat ~/.claude/plans/plan-itine.md
code Cargo.toml    # Ajouter dépendances
touch src/crypto.rs
code src/crypto.rs  # Commencer Tâche 2
```

### MBAYE - Premier Jour
```bash
cd ~/Full-Bloc-Chain/01-mini-blockchain
cat ~/.claude/plans/plan-mbaye.md
touch src/utxo_set.rs
code src/utxo_set.rs  # Commencer Tâche 16
```

### MOUNIROU - Premier Jour
```bash
cd ~/Full-Bloc-Chain/01-mini-blockchain
cat ~/.claude/plans/plan-mounirou.md
cat src/block.rs        # Étudier le code
cat src/blockchain.rs   # Comprendre la structure
```

---

**Prêts ? C'est parti ! 🚀**

Chaque membre a son plan détaillé. Lisez-le attentivement et **commencez par la Tâche 1** de votre plan.

**Ensemble, vous allez créer quelque chose d'incroyable ! 💪**
