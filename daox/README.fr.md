[English](README.md) | [Français](README.fr.md)

# Daox

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** est un générateur d'objets d'accès aux données (DAO) "Database-first" hautement optimisé et sans surcoût (zero-overhead) pour Rust.

En se connectant à votre base de données à la **compilation** (via `build.rs`), Daox introspecte votre base de données existante et génère des structures Rust fortement typées, accompagnées de méthodes CRUD asynchrones extrêmement efficaces.

> **Bases de données supportées :** PostgreSQL, MySQL/MariaDB et SQLite.

---

## Architecture et cycle de vie des données

La plupart des ORM traditionnels (comme Diesel ou SeaORM) vous obligent à définir manuellement des macros ou des structures Rust, que vous devez soigneusement maintenir pour correspondre à votre base de données. **Daox inverse totalement ce paradigme.**

Voici l'approche **Database-first** (base de données d'abord) visualisée :

![Architecture et cycle de vie des données](./assets/architecture_fr.svg)

### Pourquoi Daox ? (Fonctionnalités SOTA)

- **Zéro surcoût :** Propulsé directement par `sqlx`. Aucune abstraction ORM lourde n'est chargée à l'exécution.
- **Pagination O(1) par clé (Keyset) :** Pagination native par curseur (`list_by_cursor`) qui détruit les goulots d'étranglement de performance du traditionnel `OFFSET`.

![Pagination O(1) par clé vs OFFSET](./assets/pagination_fr.svg)

- **Flux d'exécution sans allocation (Streams) :** Traitez des millions de lignes efficacement via `stream_all()` sans les charger entièrement dans la RAM.

![Flux d'exécution sans allocation](./assets/streams_fr.svg)

- **Patching intelligent :** Envoyez des mises à jour partielles sur le réseau (`update_partial_by_pk`) pour économiser de la bande passante et réduire les écritures disque de la base (WAL).
- **Conscient du dialecte et sécurisé contre les injections :** Échappe automatiquement les mots-clés SQL réservés et utilise des requêtes préparées pour prévenir de manière stricte les injections SQL.
- **Clés composites :** Support natif complet pour les tables avec des clés primaires multiples.

---

## Le projet officiel de test & démo

Si vous préférez apprendre en lisant du code, nous vous recommandons vivement de cloner le dépôt GitHub et d'explorer le répertoire **`daox-test`**.

C'est un projet de démonstration complet et prêt à l'emploi. Il inclut un fichier `docker-compose.yml` (pour PostgreSQL et MySQL) et teste de manière exhaustive chaque fonctionnalité de Daox sur les 3 dialectes SQL. C'est le terrain de jeu idéal pour apprendre le framework en toute sécurité !

---

## Guide pas à pas (pour les novices en Rust)

Ce guide pas à pas vous accompagne à travers l'ensemble du workflow Daox, de l'installation à l'exécution.

### Étape 1 : Créer un projet & configuration

Tout d'abord, créez un tout nouveau projet Rust dans votre terminal :

```bash
cargo new mon_app
cd mon_app
```

En Rust, `Cargo.toml` est le fichier de configuration où vous déclarez vos dépendances. Puisque Daox génère du code **avant** que votre application ne soit compilée, il est ajouté en tant que "dépendance de compilation" (`build-dependency`).

Ouvrez votre `Cargo.toml` et ajoutez les lignes suivantes :

```toml
[dependencies]
# sqlx gère la connexion réelle à la base de données à l'exécution
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "mysql", "postgres", "sqlite"] }
# futures est requis pour gérer les flux de données asynchrones de Daox
futures = "0.3"

[build-dependencies]
# Daox s'exécute à la compilation pour générer votre code DAO
daox = "0.2.0"
# Tokio fournit l'environnement d'exécution asynchrone dont Daox a besoin pendant la génération
tokio = { version = "1", features = ["full"] }
```

### Étape 2 : Préparer le schéma de votre base de données

Parce que Daox est "Database-first", votre base de données et vos tables doivent exister **avant** la compilation. Assurez-vous d'avoir une base de données en cours de fonctionnement et créez cette table simple (exemple pour PostgreSQL) :

```sql
CREATE TABLE utilisateurs (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    statut VARCHAR(50) DEFAULT 'actif'
);
```

### Étape 3 : Le générateur de code (`build.rs`)

En créant un fichier nommé `build.rs` à la racine de votre projet, Cargo l'exécutera automatiquement avant de compiler le reste du code. Nous utilisons ce fichier pour déclencher Daox.

Créez un fichier `build.rs` à la racine de votre projet :

```rust
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // 1. Demander à Cargo de recompiler seulement si build.rs est modifié
    println!("cargo:rerun-if-changed=build.rs");

    // 2. Définir l'URL de votre base de données
    let db_url = "postgres://utilisateur:mdp@localhost:5432/ma_bdd";

    // 3. Définir l'emplacement où Daox doit sauvegarder les fichiers Rust générés
    let output_dir = "src/models";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    // 4. Se connecter à la base, lire le schéma, et générer !
    let generator = daox::DaoxGenerator::new(db_url, output_dir);
    generator.generate().await.expect("Échec de la génération des DAO");
}
```

### Étape 4 : Déclencher la génération

Générez les modèles en compilant votre projet dans le terminal :

```bash
cargo build
```

> **Que se passe-t-il ici ?** Cargo exécute `build.rs`. Daox se connecte à votre base de données, analyse en profondeur votre table `utilisateurs`, et génère proprement des fichiers parfaitement typés dans votre dossier `src/models/`.

### Étape 5 : Votre application (`src/main.rs`)

Tout est prêt ! Importez les modules générés et utilisez-les dans votre logique principale.

```rust
pub mod models; // Inclure explicitement le module généré

use sqlx::postgres::PgPoolOptions;
use futures::StreamExt; // Requis pour les flux de données continus
use models::utilisateurs::{Utilisateurs, UtilisateursPatch}; // Importer les structures générées

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 1. Ouvrir un pool de connexions vers votre base de données
    let pool = PgPoolOptions::new().connect("postgres://utilisateur:mdp@localhost:5432/ma_bdd").await?;

    // --- CAS 1 : INSERTION CLASSIQUE ---
    let nouvel_utilisateur = Utilisateurs {
        id: 0, // Ignoré automatiquement par Daox pour les colonnes auto-incrémentées
        email: "alice@daox.dev".into(),
        statut: "actif".into(),
    };
    let utilisateur_id = nouvel_utilisateur.insert(&pool).await?;
    println!("ID de l'utilisateur inséré : {}", utilisateur_id);

    // --- CAS 2 : UPSERT (Insérer, ou mettre à jour en cas de conflit) ---
    let mut utilisateur_modifie = nouvel_utilisateur.clone();
    utilisateur_modifie.statut = "banni".into();
    utilisateur_modifie.upsert(&pool).await?;

    // --- CAS 3 : PATCHING INTELLIGENT (Mise à jour partielle) ---
    // Mettre à jour *uniquement* le statut. Daox ignorera le champ email.
    let patch = UtilisateursPatch {
        statut: Some("inactif".into()),
        ..Default::default()
    };
    Utilisateurs::update_partial_by_pk(&pool, &(utilisateur_id as i64), &patch).await?;

    // --- CAS 4 : FLUX SANS ALLOCATION (Streaming) ---
    // Itérer proprement, ligne par ligne, sans déborder la mémoire du système
    let mut stream = Utilisateurs::stream_all(&pool);
    while let Some(user_result) = stream.next().await {
        let user = user_result?;
        println!("Utilisateur trouvé : {}", user.email);
    }

    Ok(())
}
```

### Étape 6 : Exécuter votre code

Lancez votre application :

```bash
cargo run
```

---

## Aperçu des méthodes générées et cas d'usage

Daox comprend précisément votre schéma. En fonction de vos colonnes et de vos index, il génère les méthodes adéquates et structurellement saines.

### Méthodes globales à la table

- **`count(executor)`** ➔ Compte total des lignes de la table. _(Idéal pour les indicateurs (KPIs) d'un tableau de bord)_
- **`stream_all(executor)`** ➔ Streaming des lignes sans allouer la mémoire. _(Crucial pour l'exportation de Big Data ou les migrations en arrière-plan)_
- **`list_paginated(executor, order, page, size)`** ➔ Pagination traditionnelle. _(Pour les grilles de données internes)_

### Méthodes d'écriture

- **`insert(&self, executor)`** ➔ Insère la structure et retourne l'ID généré.
- **`insert_batch(executor, &[Self])`** ➔ Insertion de masse à haute performance.
- **`upsert(&self, executor)`** ➔ Insère, ou met à jour de manière puissante si un conflit sur unicité se produit.

### Méthodes pilotées par la clé primaire (PK)

- **`get_by_pk(executor, pk)`** ➔ Récupère un enregistrement précis.
- **`exists_by_pk(executor, pk)`** ➔ Vérification ultra-rapide optimisée par cache sans charger la ligne complète.
- **`update_by_pk(&self, executor)`** ➔ Écrase complètement l'enregistrement dans la base de données.
- **`update_partial_by_pk(executor, pk, &Patch)`** ➔ Modification partielle sur les champs définis (économise l'espace de stockage WAL !).
- **`delete_by_pk(executor, pk)`** ➔ Suppression d'un enregistrement unique.
- **`delete_many_by_pk(executor, &[pk])`** ➔ Suppression partielle en masse (scalable).
- **`list_by_cursor(executor, last_id, limit)`** ➔ La sainte **Pagination Keyset en O(1)** (état de l'art) pour les fils d'actualité en défilement infini ("infinite scrolling").

### Méthodes d'index auto-générées

Daox scrute les index de votre BDD et cartographie des méthodes de requête parfaitement optimisées.
_(Exemple, pour un index sur `email`)_

- **`exists_by_<index>`** ➔ Exemple : **`exists_by_email`**
- **`get_by_<index>`** ➔ _(Pour les index UNIQUE)_ Exemple : **`get_by_email`**
- **`list_by_<index>`** ➔ _(Pour les index standard)_ Récupère de multiples correspondances.
- **`stream_by_<index>`** ➔ Streaming sécurisé des correspondances.
- **`delete_by_<index>`** ➔ Suppression ciblée et efficace selon un index.

---

## Modèles d'interaction avancés

### 1. Robustesse par transactions

Chaque méthode Daox auto-générée accepte de manière inhérente un objet (`executor`). Vous n'êtes pas forcé de passer votre pool global `&pool` ; vous pouvez facilement grouper et exécuter des opérations de manière atomique au coeur d'une transaction :

```rust
let mut tx = pool.begin().await?;

// Exécute proprement sous la même enveloppe de transaction
let utilisateur_id = nouvel_utilisateur.insert(&mut *tx).await?;
Utilisateurs::delete_by_pk(&mut *tx, &(utilisateur_id as i64)).await?;

tx.commit().await?; // Validation des modifications BDD
```

### 2. Typage immédiat par l'IDE (Sync)

En s'intégrant au coeur de `cargo build`, Daox garantit que vos modèles Rust reflètent votre schéma de base de données en 1 pour 1.

Si vous renommez une colonne, le prochain `cargo build` mettra instantanément à jour `src/models/*.rs`. Votre code appelant l'ancien nom de cette colonne provoquera immédiatement une **erreur de compilation**. Ceci garantit une robustesse maximale - les propriétés typées dont vous bénéficiez demeurent toujours l'unique source de vérité.

---

## Structure du dépôt

- **`daox/`** : Le code coeur du framework visant le registre `crates.io`.
- **`daox-test/`** : La très complète suite d'intégration servant de terrain de jeu d'apprentissage pour vérifier simultanément tout le comportement du code sur MySQL, Postgres et SQLite via Docker.

---

## Auteur

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## Licence

Ce projet est sous licence MIT.
