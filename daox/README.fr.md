[English](README.md) | [Français](README.fr.md)

# Daox : L'outil de base de données ultime pour Rust

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Bienvenue sur **Daox** ! Si vous débutez en programmation ou avec Rust, vous êtes exactement au bon endroit.

**Daox** est un outil hautement optimisé qui relie automatiquement votre application Rust à votre base de données.

Au lieu que vous écriviez du code répétitif et complexe pour communiquer avec votre base de données, Daox s'y connecte **au moment de la compilation**, analyse vos tables et **écrit le code exact dont vous avez besoin à votre place**. Le résultat est une application incroyablement rapide et sans risque d'erreur humaine.

> **Bases de données supportées :** PostgreSQL, MySQL/MariaDB et SQLite.

---

## 🎯 Le guide pas-à-pas pour les débutants absolus

Ce guide est conçu avec une "précision militaire" pour une clarté absolue. Même si vous avez une expérience technique minimale, suivre ces étapes rigoureusement garantira votre succès.

### Étape 0 : Prérequis

Avant de commencer, assurez-vous d'avoir :

1. **Rust installé :** Allez sur [rustup.rs](https://rustup.rs/) et suivez les instructions pour installer Rust sur votre ordinateur.
2. **Une base de données active :** Nous utiliserons PostgreSQL dans cet exemple. Si Docker est installé, vous pouvez en démarrer une via :
   `docker run --name my-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres`

---

### Étape 1 : Créer votre nouveau projet

Ouvrez votre terminal (Invite de commandes, PowerShell ou bash) et exécutez strictement ces commandes pour créer un nouveau projet Rust :

```bash
# Ceci crée un nouveau dossier nommé 'mon_app' contenant un projet Rust vierge
cargo new mon_app

# Entrez dans le nouveau dossier
cd mon_app
```

---

### Étape 2 : Ajouter les dépendances requises

Rust utilise un fichier appelé `Cargo.toml` pour gérer les outils et bibliothèques (les dépendances).
Ouvrez le fichier `Cargo.toml` dans votre dossier `mon_app` avec n'importe quel éditeur de texte.

Modifiez-le pour qu'il ressemble **exactement** à ceci :

```toml
[package]
name = "mon_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# sqlx est l'outil qui permet de se connecter à la bdd quand l'app tourne
sqlx = { version = "0.9", features = ["runtime-tokio", "tls-rustls", "postgres"] }
# futures gère efficacement les flux de données (streams)
futures = "0.3"
# tokio fournit l'environnement asynchrone nécessaire à l'application
tokio = { version = "1", features = ["full"] }

[build-dependencies]
# Daox est notre outil magique qui génère le code avant que l'app ne s'exécute
daox = "0.2.5"
# tokio est aussi nécessaire pour le script de génération de code
tokio = { version = "1", features = ["full"] }
```

---

### Étape 3 : Préparer votre base de données

Daox est "Database-first" (Base de données en premier). Cela signifie que vos tables doivent exister dans votre base de données _avant_ que Daox ne puisse générer le code.

Connectez-vous à votre base PostgreSQL (à l'aide de votre client préféré comme DBeaver, pgAdmin ou psql) et exécutez cette commande SQL précise pour créer une table `utilisateurs` :

```sql
CREATE TABLE utilisateurs (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    statut VARCHAR(50) DEFAULT 'actif'
);
```

---

### Étape 4 : Configurer le générateur de code (`build.rs`)

Nous avons besoin d'un script qui demandera à Daox d'inspecter la base de données et d'écrire le code Rust correspondant.

Créez un nouveau fichier nommé très exactement `build.rs` à la racine de votre projet (dans le dossier `mon_app`, au même niveau que `Cargo.toml`).

Copiez et collez ce code précis dans `build.rs` :

```rust
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // 1. Demander à Rust de ne rejouer ce script que si le script lui-même est modifié
    println!("cargo:rerun-if-changed=build.rs");

    // 2. Définissez votre URL de base de données exacte.
    // Format : postgres://[utilisateur]:[mot_de_passe]@[hote]:[port]/[nom_bdd]
    // CHANGEZ CETTE URL pour correspondre à vos identifiants si besoin !
    let db_url = "postgres://postgres:password@localhost:5432/postgres";

    // 3. Définissez le dossier où Daox sauvegardera le code généré
    let output_dir = "src";

    // 4. Lancer Daox ! Il lira votre base de données et générera les fichiers Rust automatiquement.
    let generator = daox::DaoxGenerator::new(db_url, output_dir);
    generator.generate().await.expect("ERREUR CRITIQUE : Échec de la génération des modèles. Vérifiez l'URL de votre base et sa connexion.");
}
```

---

### Étape 5 : Écrire le code de votre application (`src/main.rs`)

Daox générera parfaitement votre code dans le fichier `src/daox_generated.rs`. Utilisons-le !
Ouvrez le fichier `src/main.rs`, supprimez tout son contenu, et remplacez-le par ce code exact :

```rust
// 1. Indique à Rust d'inclure le fichier que Daox vient de générer
pub mod daox_generated;

use sqlx::postgres::PgPoolOptions;
use futures::StreamExt; // Requis pour lire efficacement de nombreux utilisateurs
use daox_generated::Utilisateurs; // Importer l'objet 'Utilisateurs' généré

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 2. Se connecter à la base. Assurez-vous que l'URL est la même que dans build.rs !
    let db_url = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new().connect(db_url).await?;

    println!("Connecté à la base de données avec succès !");

    // 3. CRÉER UN NOUVEL UTILISATEUR
    let nouvel_utilisateur = Utilisateurs {
        id: 0, // '0' est ignoré car PostgreSQL auto-génère l'ID seul
        email: "hello@daox.dev".into(),
        statut: Some("actif".to_string()),
    };

    // Sauvegarder l'utilisateur dans la base
    let utilisateur_id = nouvel_utilisateur.insert(&pool).await?;
    println!("SUCCÈS : Nouvel utilisateur inséré avec l'ID {}", utilisateur_id);

    // 4. METTRE À JOUR L'UTILISATEUR (Smart Patching)
    // Nous mettons à jour UNIQUEMENT la colonne statut de manière chirurgicale
    Utilisateurs::update_partial_by_id(&pool, utilisateur_id as i64, &daox_generated::UtilisateursPatch {
        statut: Some(Some("inactif".to_string())),
        ..Default::default()
    }).await?;
    println!("SUCCÈS : Statut de l'utilisateur mis à jour à 'inactif'");

    // 5. LIRE TOUS LES UTILISATEURS (Streaming)
    // Daox diffuse la donnée ligne par ligne, la RAM de votre PC ne saturera jamais
    let mut stream = Utilisateurs::stream_all(&pool);
    println!("--- Liste de tous les utilisateurs ---");
    while let Some(user_result) = stream.next().await {
        let user = user_result?;
        println!("Utilisateur : ID {}, Email {}, Statut {:?}", user.id, user.email, user.statut);
    }

    Ok(())
}
```

---

### Étape 6 : Compiler et Lancer !

Vous êtes prêt. Retournez dans votre terminal (assurez-vous d'être bien dans le dossier `mon_app`) et lancez votre programme :

```bash
cargo run
```

**Que se passe-t-il exactement ?**

1. Le script `build.rs` s'exécute en premier. Daox se connecte, lit la structure de la table `utilisateurs`, et rédige automatiquement un code Rust parfait dans le fichier `src/daox_generated.rs`.
2. `src/main.rs` utilise directement ce code tout fraichement créé.
3. Le programme sauvegarde un utilisateur en base, met à jour son statut, et imprime tous les utilisateurs à l'écran.

**Félicitations ! Vous maîtrisez désormais l'implantation de Daox.**

---

## 🚀 Architecture avancée & Capacités de Daox

Si vous êtes un utilisateur technique, voici pourquoi Daox établit l'État de l'Art (State Of The Art - SOTA) parmi les ORM Rust.

### Le Paradigme Database-First (Base de données en premier)

La plupart des ORM traditionnels (comme Diesel ou SeaORM) vous obligent à définir manuellement des macros ou structures Rust que vous devez rigoureusement aligner avec votre base. **Daox inverse totalement cette approche.**

Voici le processus modélisé :

![Architecture et cycle de vie des données](./assets/architecture_fr.svg)

### Fonctionnalités de Qualité Supérieure (SOTA)

- **Zéro surcoût :** Propulsé par `sqlx`. Aucune abstraction ORM lourde en exécution.
- **Pagination O(1) par clé (Keyset) :** Pagination par curseur native (`list_by_cursor`) détruisant les goulots d'étranglement de l'inefficace commande `OFFSET`.

![Pagination O(1) par clé vs OFFSET](./assets/pagination_fr.svg)

- **Flux d'exécution sans allocation (Streams) :** Traitez des millions de lignes via `stream_all()` sans les charger entièrement dans votre RAM.

![Flux d'exécution sans allocation](./assets/streams_fr.svg)

- **Opérations par lots et Upserts SOTA :** Intégration stricte native avec gestion parfaite multi-bases (`ON CONFLICT` et `ON DUPLICATE KEY`). ACID atomicity 100% garanti de manière ultra performante.
- **Patching intelligent :** Mises à jour partielles garanties (`update_partial_by_pk`) pour réduire la bande passante et l'usure de vos disques (WAL).
- **Conscient du dialecte et sécurisé contre les injections :** Échappement méticuleux.
- **Clés composites et Index Secondaires :** Génération native multi-clés primaires.

### Modèles d'Interaction Avancés

#### Robustesse par Transactions

Chaque méthode s'attend à un environnement transactionnel `executor` naturel :

```rust
let mut tx = pool.begin().await?;

// Exécute proprement sous la même transaction
let utilisateur_id = nouvel_utilisateur.insert(&mut *tx).await?;
Utilisateurs::delete_by_id(&mut *tx, utilisateur_id as i64).await?;

tx.commit().await?; // Base de données validée
```

#### Typage immédiat par l'IDE (Sync)

Si vous renommez une colonne dans votre base de données, relancez simplement `cargo build`. Daox met à jour `src/daox_generated.rs` instantanément. Le code qui utilise l'ancien nom de colonne **échouera immédiatement à la compilation**, sécurisant parfaitement la structure de votre application.

---

## Le projet officiel de Test & Démo

Pour vérifier ces allégations par le code, clonez le dépôt et explorez le répertoire **`daox-test`**. C'est une suite de test exhaustive (Docker inclus pour MySQL et Postgres) qui valide l'ensemble des limites de Daox.

---

## Auteur

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## Licence

Ce projet est sous licence MIT.
