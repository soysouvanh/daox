# Daox Framework

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** est un générateur d'objets d'accès aux données (DAO) "Database-first" hautement optimisé et sans surcoût (zero-overhead) pour Rust.

Ce dépôt héberge l'ensemble de l'espace de travail (workspace) du code source pour l'écosystème Daox.

---

## Structure du workspace

Ce dépôt est organisé en tant que Cargo workspace et contient les répertoires suivants :

### 1. `daox/` (Le coeur du framework)

C'est ici que réside le code principal du framework. Ce répertoire contient la logique d'introspection de schéma et le générateur de code à la compilation.  
-> **[Lire la documentation officielle et le guide de démarrage ici](./daox/README.fr.md)**

### 2. `daox-test/` (Tests d'intégration et démo)

Ce dossier contient la suite de tests complète et l'application de démonstration permettant de prouver la portabilité absolue du framework. Il déploie automatiquement des bases de données via Docker Compose et exécute les classes DAO générées sur les trois dialectes SQL supportés.

---

## Comment exécuter les tests d'intégration localement

Si vous avez cloné ce dépôt et souhaitez lancer la suite de tests exhaustive pour apprendre ou contribuer, suivez ces étapes accessibles :

### Étape 1 : Démarrer les bases de données

Daox a besoin de bases de données en direct pour effectuer son introspection. Nous fournissons un fichier `docker-compose.yml` propre pour les lancer instantanément sans polluer votre système local.

```bash
docker compose up -d
```

_Note : Cela déploiera une instance PostgreSQL sur le port 5433 et une instance MySQL sur le port 3307 afin d'éviter tout conflit avec vos bases de données locales pré-existantes._

### Étape 2 : Lancer les tests

Naviguez dans le répertoire de test et exécutez le moteur de votre choix. Le script `build.rs` se connectera automatiquement aux instances Docker, générera les modèles Rust à la volée, et l'application `main.rs` exécutera tous les cas d'usage réels (Upsert, Patching, Streaming, Pagination Keyset).

```bash
cd daox-test

# Tester le dialecte PostgreSQL
cargo run -- postgres

# Tester le dialecte MySQL / MariaDB
cargo run -- mysql

# Tester le dialecte SQLite (crée automatiquement un fichier local .sqlite)
cargo run -- sqlite
```

---

## Auteur

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## Licence

Ce projet est sous licence MIT.
