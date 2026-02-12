# 🚀 Guide d'Implémentation - Blockchain UTXO avec Réseau P2P

## 📋 Vue d'Ensemble

Ce projet transforme votre mini-blockchain en une **vraie blockchain décentralisée** similaire à Bitcoin avec:
- ✅ Transactions avec modèle UTXO
- ✅ Signatures cryptographiques ECDSA
- ✅ Wallets pour gérer les clés
- ✅ Réseau P2P mesh complet
- ✅ Communications TLS sécurisées
- ✅ Déploiement sur Internet (VPS + nodes locaux)

---

## 👥 Répartition des Tâches

### 🟦 ITINE - Cryptographie & Transactions (15 tâches)

**Votre plan détaillé:** [`~/.claude/plans/plan-itine.md`](~/.claude/plans/plan-itine.md)

**Responsabilités:**
1. Module `crypto.rs` - Signatures ECDSA
2. Module `transaction.rs` - Transactions Bitcoin-like
3. Certificats TLS pour le réseau

**Fichiers à créer:**
- `src/crypto.rs`
- `src/transaction.rs`
- `scripts/generate_certs.sh`
- `docs/TLS_SETUP.md`

**Durée estimée:** 8-9 jours

**Dépendances:**
- ✅ Peut commencer immédiatement (aucune dépendance)

---

### 🟩 MBAYE - UTXO Set & Wallets (12 tâches)

**Votre plan détaillé:** [`~/.claude/plans/plan-mbaye.md`](~/.claude/plans/plan-mbaye.md)

**Responsabilités:**
1. Module `utxo_set.rs` - Gestion des sorties non dépensées
2. Module `wallet.rs` - Portefeuilles utilisateurs
3. Tests de performance

**Fichiers à créer:**
- `src/utxo_set.rs`
- `src/wallet.rs`
- `tests/performance_tests.rs`

**Durée estimée:** 7-8 jours

**Dépendances:**
- ⚠️ Wallet dépend de `crypto.rs` et `transaction.rs` (ITINE)
- ✅ UTXO Set peut commencer immédiatement

---

### 🟨 MOUNIROU - Blockchain, Réseau & API (27 tâches)

**Votre plan détaillé:** [`~/.claude/plans/plan-mounirou.md`](~/.claude/plans/plan-mounirou.md)

**Responsabilités:**
1. Modifications `block.rs` et `blockchain.rs`
2. Réseau P2P avec TLS
3. API complète (6 nouveaux endpoints)
4. Déploiement VPS

**Fichiers à créer:**
- `src/network/*.rs` (modules réseau)
- `src/crypto/tls.rs`
- Nouveaux handlers API
- Scripts de déploiement VPS

**Durée estimée:** 14-16 jours

**Dépendances:**
- ⚠️ Block/Blockchain dépendent de `transaction.rs` (ITINE) et `utxo_set.rs` (MBAYE)
- ✅ Réseau P2P peut commencer en parallèle

---

## 🔄 Timeline Recommandé

### Semaine 1 (Jours 1-7)

**ITINE:**
- ✅ Tâches 1-6: crypto.rs complet
- ✅ Tâches 7-11: transaction.rs complet

**MBAYE:**
- ✅ Tâches 16-21: utxo_set.rs complet
- 🔄 Attendre crypto.rs pour wallet.rs

**MOUNIROU:**
- ✅ Étudier le code existant
- ✅ Préparer architecture réseau P2P
- ✅ Commencer TLS (tâche 44)

### Semaine 2 (Jours 8-14)

**ITINE:**
- ✅ Tâches 12: Tests crypto/transaction
- ✅ Tâches 13-15: Certificats TLS

**MBAYE:**
- ✅ Tâches 22-25: wallet.rs complet
- ✅ Tâche 26: Tests UTXO/Wallet

**MOUNIROU:**
- ✅ Tâches 28-35: Modifications Block/Blockchain
- ✅ Tâches 36-43: API complète

### Semaine 3 (Jours 15-21)

**ITINE:**
- ✅ Aide aux tests d'intégration
- ✅ Distribution des certificats

**MBAYE:**
- ✅ Tâche 27: Tests de performance
- ✅ Aide aux tests d'intégration

**MOUNIROU:**
- ✅ Tâches 44-53: Réseau P2P complet
- ✅ Tâche 54: Tests end-to-end
- ✅ Déploiement VPS

---

## 🚀 Commencer l'Implémentation

### Pour ITINE

```bash
# 1. Aller dans le répertoire
cd 01-mini-blockchain

# 2. Lire votre plan
cat ~/.claude/plans/plan-itine.md

# 3. Commencer par la Tâche 1
# Ouvrir Cargo.toml et ajouter les dépendances

# 4. Créer le fichier crypto.rs
touch src/crypto.rs

# 5. Suivre le plan étape par étape
```

### Pour MBAYE

```bash
# 1. Lire votre plan
cat ~/.claude/plans/plan-mbaye.md

# 2. Créer le fichier utxo_set.rs
cd 01-mini-blockchain
touch src/utxo_set.rs

# 3. Attendre qu'ITINE termine transaction.rs avant de commencer wallet.rs

# 4. Suivre le plan étape par étape
```

### Pour MOUNIROU

```bash
# 1. Lire votre plan
cat ~/.claude/plans/plan-mounirou.md

# 2. Étudier le code existant
cat src/block.rs
cat src/blockchain.rs

# 3. Préparer l'architecture réseau pendant que ITINE/MBAYE font le core

# 4. Suivre le plan étape par étape
```

---

## 📚 Documentation Disponible

### Plans Détaillés
- **Plan Principal:** `~/.claude/plans/cached-singing-oasis.md`
- **Plan ITINE:** `~/.claude/plans/plan-itine.md`
- **Plan MBAYE:** `~/.claude/plans/plan-mbaye.md`
- **Plan MOUNIROU:** `~/.claude/plans/plan-mounirou.md`
- **Plan Réseau:** `~/.claude/plans/network-internet-addon.md`

### Concepts Expliqués
Tous les plans contiennent des explications détaillées sur:
- UTXO (Unspent Transaction Output)
- ECDSA (Signatures cryptographiques)
- TXID (Transaction ID)
- Wallets
- Réseau P2P mesh
- TLS/SSL

---

## 🤝 Points de Synchronisation

### Sync Point 1 (Jour 6-7)
**ITINE termine crypto.rs → MBAYE peut commencer wallet.rs**

Actions:
- ITINE: Confirmer que crypto.rs compile et tous les tests passent
- MBAYE: Vérifier que vous pouvez importer `use crate::crypto::KeyPair;`

### Sync Point 2 (Jour 11-12)
**ITINE et MBAYE terminent → MOUNIROU peut modifier block.rs**

Actions:
- ITINE: Confirmer transaction.rs terminé
- MBAYE: Confirmer utxo_set.rs terminé
- MOUNIROU: Commencer les modifications Block/Blockchain

### Sync Point 3 (Jour 15)
**Intégration complète**

Actions:
- Tous: Exécuter `cargo test` ensemble
- Résoudre les erreurs de compilation
- Vérifier que tous les modules s'intègrent

### Sync Point 4 (Jour 20)
**Déploiement VPS**

Actions:
- ITINE: Distribuer les certificats TLS
- MOUNIROU: Déployer sur VPS
- Tous: Tester le réseau mesh complet

---

## ✅ Checklist Quotidienne

### Pour ITINE
- [ ] Lire la tâche du jour dans plan-itine.md
- [ ] Implémenter la fonctionnalité
- [ ] Écrire les tests
- [ ] Vérifier que `cargo test` passe
- [ ] Commit le code
- [ ] Informer l'équipe de la progression

### Pour MBAYE
- [ ] Lire la tâche du jour dans plan-mbaye.md
- [ ] Implémenter la fonctionnalité
- [ ] Écrire les tests
- [ ] Vérifier que `cargo test` passe
- [ ] Commit le code
- [ ] Informer l'équipe de la progression

### Pour MOUNIROU
- [ ] Lire la tâche du jour dans plan-mounirou.md
- [ ] Implémenter la fonctionnalité
- [ ] Écrire les tests (si applicable)
- [ ] Vérifier que `cargo test` passe
- [ ] Commit le code
- [ ] Informer l'équipe de la progression

---

## 🧪 Tests Critiques

### Test 1: Crypto (ITINE)
```bash
cargo test crypto::tests
```

### Test 2: Transaction (ITINE)
```bash
cargo test transaction::tests
```

### Test 3: UTXO (MBAYE)
```bash
cargo test utxo_set::tests
```

### Test 4: Wallet (MBAYE)
```bash
cargo test wallet::tests
```

### Test 5: Blockchain (MOUNIROU)
```bash
cargo test blockchain::tests
```

### Test 6: End-to-End (Tous)
```bash
cargo test integration_tests
```

---

## 🐛 Debugging

### Problème: Erreur de compilation
```bash
# Vérifier toutes les dépendances
cargo check

# Voir les erreurs détaillées
cargo build --verbose
```

### Problème: Tests échouent
```bash
# Exécuter un test spécifique
cargo test nom_du_test -- --nocapture

# Voir la sortie complète
cargo test -- --nocapture
```

### Problème: Conflits Git
```bash
# Voir l'état
git status

# Résoudre les conflits
git pull origin main
# Éditer les fichiers en conflit
git add .
git commit -m "Resolve conflicts"
```

---

## 📞 Communication

### Daily Standup (Recommandé)
Chaque jour, chaque membre partage:
1. Ce que j'ai fait hier
2. Ce que je fais aujourd'hui
3. Blockers/difficultés

### Canaux de communication
- **Questions techniques:** Discord/Slack
- **Code reviews:** Pull Requests GitHub
- **Décisions importantes:** Réunion d'équipe

---

## 🎯 Objectifs par Semaine

### Semaine 1: Core Fonctionnel
- ✅ crypto.rs et transaction.rs terminés
- ✅ utxo_set.rs terminé
- ✅ Tests unitaires passent

### Semaine 2: Intégration
- ✅ wallet.rs terminé
- ✅ block.rs et blockchain.rs modifiés
- ✅ API complète fonctionnelle
- ✅ Tests locaux passent

### Semaine 3: Réseau & Déploiement
- ✅ Réseau P2P avec TLS
- ✅ VPS déployé
- ✅ Mesh complet fonctionnel
- ✅ Tests end-to-end via Internet

---

## 🚨 Points d'Attention Critiques

### Sécurité
- ⚠️ **JAMAIS commiter les clés privées** (.key files)
- ⚠️ Toujours valider les transactions avant de les ajouter
- ⚠️ Vérifier TOUTES les signatures

### Performance
- 💡 UTXO set doit être rapide (index par adresse si besoin)
- 💡 Rate limiting sur l'API
- 💡 TLS ajoute de la latence (acceptable)

### Code Quality
- ✅ Écrire des tests pour CHAQUE fonction
- ✅ Documenter avec des /// comments
- ✅ Pas de `unwrap()` dans le code de production (utiliser `?`)

---

## 🎓 Ressources d'Apprentissage

### Concepts Bitcoin
- Whitepaper: https://bitcoin.org/bitcoin.pdf
- Mastering Bitcoin: https://github.com/bitcoinbook/bitcoinbook

### Rust
- The Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/

### Cryptographie
- secp256k1: https://docs.rs/secp256k1/
- ECDSA: https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm

### Réseau
- TLS: https://docs.rs/rustls/
- P2P patterns: https://en.bitcoin.it/wiki/Network

---

## ✨ Critères de Succès Final

Le projet est **terminé** quand:

1. ✅ Alice peut créer un wallet
2. ✅ Bob peut créer un wallet
3. ✅ Un bloc genesis est miné avec coinbase pour Alice
4. ✅ Alice a 50 coins (vérifiable via API)
5. ✅ Alice crée une transaction pour envoyer 30 coins à Bob
6. ✅ La transaction est signée avec ECDSA
7. ✅ Un nouveau bloc est miné avec cette transaction
8. ✅ L'UTXO set est mis à jour correctement
9. ✅ Alice a 20 coins de change + 50 de coinbase = 70 coins
10. ✅ Bob a 30 coins
11. ✅ La blockchain est valide (signatures, hashes, liens)
12. ✅ Tout fonctionne en réseau P2P avec TLS
13. ✅ Le VPS participe au réseau mesh
14. ✅ Les nodes locaux communiquent avec le VPS via Internet

---

## 🎉 Bon Courage !

Vous allez créer une **vraie blockchain décentralisée** de A à Z. C'est un projet ambitieux mais vous avez tous les plans détaillés pour réussir.

**Rappelez-vous:**
- 📖 Lisez les plans attentivement
- 🤝 Communiquez régulièrement
- 🧪 Testez tout
- 💪 N'hésitez pas à demander de l'aide

**Let's build something amazing! 🚀**
