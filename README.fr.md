[English](README.md) | [Français](README.fr.md)

# Daox Framework

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** est un générateur d'objets d'accès aux données (DAO) "Database-first" multi-base, hautement optimisé et sans surcoût (zero-overhead) pour Rust.

Ce dépôt héberge l'ensemble de l'espace de travail (workspace) du code source pour l'écosystème Daox.

---

## Structure du workspace

Voici comment s'organise l'écosystème Daox au travers de ce Cargo workspace :

```text
daox-workspace/
├── docker-compose.yml      # Automatisation des BDD (PostgreSQL, MySQL)
├── init_*.sql              # Schémas de base pré-chargés par Docker
├── daox/                   # Le framework principal (publié sur crates.io)
│   ├── src/                # Builders de dialecte, parser AOP, logique macro
│   └── assets/             # Diagrammes SVG explicatifs de l'architecture
│
└── daox-test/              # Tests d'intégration & Documentation vivante
    ├── src/models/         # DAOs auto-générés (lors du build.rs)
    └── src/main.rs         # Workflows de démo (Streams, Pagination, Patching)
```

-> **[Lire la documentation officielle et le guide de démarrage ici](./daox/README.fr.md)**

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
DAOX_TEST_PG=1 cargo run

# Tester le dialecte MySQL / MariaDB (Défaut actif silencieusement)
cargo run

# Tester le dialecte SQLite (s'exécute dans target/ via OUT_DIR)
DAOX_TEST_SQLITE=1 cargo run
```

---

## Auteur

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## Licence

Ce projet est sous licence MIT.
