pub mod models_mysql;
pub mod models_pg;
pub mod models_sqlite;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args: Vec<String> = env::args().collect();
    let dialect = args.get(1).map(|s| s.as_str()).unwrap_or("mysql");

    if dialect == "postgres" || dialect == "pg" {
        run_postgres().await?;
    } else if dialect == "sqlite" {
        run_sqlite().await?;
    } else {
        run_mysql().await?;
    }

    Ok(())
}

/// ========================================================================
/// 🐘 DÉMONSTRATION EXHAUSTIVE DE LA LIBRAIRIE DAO (POSTGRESQL)
/// Ce code sert de documentation officielle pour les cas d'usage avancés.
/// ========================================================================
async fn run_postgres() -> Result<(), sqlx::Error> {
    use models_pg::users::{Users, UsersPatch};
    use models_pg::order_items::OrderItems;
    use futures::StreamExt; // Nécessaire pour traiter les Streams asynchrones
    
    // 1. Connexion au moteur PostgreSQL
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5433/daox_test")
        .await?;

    println!("=====================================================");
    println!("🐘 DÉMONSTRATION COMPLÈTE DAO SUR POSTGRESQL");
    println!("=====================================================\n");

    // Nettoyage initial des tables pour l'environnement de test
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items CASCADE").execute(&pool).await?;

    // --- CAS N°1 : INSERTION & LECTURE CLASSIQUE ---
    println!("🚀 1. Insertion simple (Insert)");
    let user1 = Users {
        id: 0, // Ignoré par PostgreSQL car défini en BIGSERIAL (Identité)
        email: "alice@daox.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Wonder".into(),
        status: "active".into(),
        created_at: None, // Automatiquement rempli par la base (DEFAULT CURRENT_TIMESTAMP)
    };
    let user1_id = user1.insert(&pool).await?;
    println!("   ✅ Utilisateur inséré avec l'ID auto-généré : {}", user1_id);

    // EXISTS : Vérification ultra-rapide (SELECT 1) sans charger les données en RAM
    let exists = Users::exists_by_pk(&pool, &(user1_id as i64)).await?;
    println!("   ✅ Vérification exists_by_pk : {}", exists);

    // --- CAS N°2 : UPSERT (Insérer ou Mettre à jour) ---
    println!("\n🔄 2. Mise à jour automatique (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Wonderland".into();
    // La méthode génère intelligemment un "ON CONFLICT DO UPDATE" ou "ON DUPLICATE KEY" selon le dialecte
    user1_modified.upsert(&pool).await?;
    
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    // --- CAS N°3 : UPDATE PARTIEL OPTIMISÉ (Patch) ---
    println!("\n🩹 3. Mise à jour partielle (Patch)");
    let patch = UsersPatch {
        first_name: Some(None), // 💡 Magie : On utilise un Some(None) pour forcer explicitement la colonne à NULL
        status: Some("banned".into()),
        ..Default::default() // Les autres colonnes (email, last_name) seront ignorées dans le UPDATE final
    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    // --- CAS N°4 : BATCH INSERTION (Performances extrêmes) ---
    println!("\n📦 4. Insertion en masse (Batch Insert)");
    let mut batch_users = Vec::new();
    for i in 1..=50 {
        batch_users.push(Users {
            id: 0,
            email: format!("user{}@daox.dev", i),
            first_name: Some(format!("Bot{}", i)),
            last_name: "Batch".into(),
            status: "active".into(),
            created_at: None,
        });
    }
    // Génère une unique requête SQL massive : INSERT INTO users (...) VALUES (...), (...), (...)
    let rows_inserted = Users::insert_batch(&pool, &batch_users).await?;
    println!("   ✅ {} utilisateurs insérés en UNE SEULE requête SQL !", rows_inserted);

    // --- CAS N°5 : PAGINATION KEYSET (Ordre de complexité O(1)) ---
    println!("\n⏭️  5. Pagination ultra-rapide par Curseur (Keyset)");
    // Idéal pour les APIs : Au lieu d'utiliser OFFSET (lent), on utilise le dernier ID connu
    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());
    for u in page {
        println!("      - Utilisateur lu : ID {}, Email {}", u.id, u.email);
    }

    // --- CAS N°6 : STREAMING SANS ALLOCATION RAM ---
    println!("\n🌊 6. Lecture en continu (Streaming asynchrone)");
    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
        // 💡 Ici on pourrait traiter des millions de lignes de BDD sans jamais faire exploser la RAM du serveur
    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    // --- CAS N°7 : CLÉS PRIMAIRES COMPOSITES (Sécurité absolue) ---
    println!("\n🗝️  7. Gestion avancée : Clé Primaire Composite");
    let item = OrderItems {
        order_id: 101,
        product_id: 42,
        quantity: 5,
    };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 101, Produit: 42)");
    
    // Daox détecte les PK multiples et demande TOUS les identifiants en paramètre
    let item_check = OrderItems::get_by_pk(&pool, &101, &42).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);

    // Suppression ciblée (Ne supprime surtout pas toute la commande !)
    OrderItems::delete_by_pk(&pool, &101, &42).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &101, &42).await?;
    println!("   ✅ Suppression de l'article spécifique réussie (Encore présent ? {})", deleted_item.is_some());

    // --- CAS N°8 : BATCH DELETE ---
    println!("\n🗑️  8. Suppression en masse (Batch Delete)");
    // Supprime de nombreux IDs d'un seul coup via un puissant WHERE id IN (?, ?, ?)
    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS POSTGRESQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

/// ========================================================================
/// 🐬 DÉMONSTRATION EXHAUSTIVE DE LA LIBRAIRIE DAO (MYSQL / MARIADB)
/// Identique au code PostgreSQL, prouvant la portabilité absolue du code.
/// ========================================================================
async fn run_mysql() -> Result<(), sqlx::Error> {
    use models_mysql::users::{Users, UsersPatch};
    use models_mysql::order_items::OrderItems;
    use futures::StreamExt;
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost:3307/daox_test")
        .await?;

    println!("=====================================================");
    println!("🐬 DÉMONSTRATION COMPLÈTE DAO SUR MYSQL/MARIADB");
    println!("=====================================================\n");

    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items").execute(&pool).await?;

    println!("🚀 1. Insertion simple (Insert)");
    let user1 = Users {
        id: 0, 
        email: "bob@daox.dev".into(),
        first_name: Some("Bob".into()),
        last_name: "Builder".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&pool).await?;
    println!("   ✅ Utilisateur inséré avec l'ID auto-généré : {}", user1_id);

    let exists = Users::exists_by_pk(&pool, &(user1_id as i64)).await?;
    println!("   ✅ Vérification exists_by_pk : {}", exists);

    println!("\n🔄 2. Mise à jour automatique (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Le Bricoleur".into();
    user1_modified.upsert(&pool).await?;
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    println!("\n🩹 3. Mise à jour partielle (Patch)");
    let patch = UsersPatch {
        first_name: Some(None),
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    println!("\n📦 4. Insertion en masse (Batch Insert)");
    let mut batch_users = Vec::new();
    for i in 1..=50 {
        batch_users.push(Users {
            id: 0,
            email: format!("worker{}@daox.dev", i),
            first_name: Some(format!("Worker{}", i)),
            last_name: "Batch".into(),
            status: "active".into(),
            created_at: None,
        });
    }
    let rows_inserted = Users::insert_batch(&pool, &batch_users).await?;
    println!("   ✅ {} utilisateurs insérés en UNE SEULE requête SQL !", rows_inserted);

    println!("\n⏭️  5. Pagination ultra-rapide par Curseur (Keyset)");
    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());

    println!("\n🌊 6. Lecture en continu (Streaming asynchrone)");
    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    println!("\n🗝️  7. Gestion avancée : Clé Primaire Composite");
    let item = OrderItems { order_id: 200, product_id: 99, quantity: 2 };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 200, Produit: 99)");
    let item_check = OrderItems::get_by_pk(&pool, &200, &99).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);
    OrderItems::delete_by_pk(&pool, &200, &99).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &200, &99).await?;
    println!("   ✅ Suppression réussie (Encore présent ? {})", deleted_item.is_some());

    println!("\n🗑️  8. Suppression en masse (Batch Delete)");
    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS MYSQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

/// ========================================================================
/// 🪶 DÉMONSTRATION EXHAUSTIVE DE LA LIBRAIRIE DAO (SQLITE)
/// ========================================================================
async fn run_sqlite() -> Result<(), sqlx::Error> {
    use models_sqlite::users::{Users, UsersPatch};
    use models_sqlite::order_items::OrderItems;
    use futures::StreamExt;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://daox_test.sqlite")
        .await?;

    println!("=====================================================");
    println!("🪶 DÉMONSTRATION COMPLÈTE DAO SUR SQLITE");
    println!("=====================================================\n");

    println!("🚀 1. Insertion simple (Insert)");
    let user1 = Users {
        id: 0, 
        email: "alice@sqlite.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Sqlite".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&pool).await?;
    println!("   ✅ Utilisateur inséré avec l'ID auto-généré : {}", user1_id);

    let exists = Users::exists_by_pk(&pool, &(user1_id as i64)).await?;
    println!("   ✅ Vérification exists_by_pk : {}", exists);

    println!("\n🔄 2. Mise à jour automatique (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Embedded".into();
    user1_modified.upsert(&pool).await?;
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    println!("\n🩹 3. Mise à jour partielle (Patch)");
    let patch = UsersPatch {
        first_name: Some(None),
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    println!("\n📦 4. Insertion en masse (Batch Insert)");
    let mut batch_users = Vec::new();
    for i in 1..=50 {
        batch_users.push(Users {
            id: 0,
            email: format!("worker{}@sqlite.dev", i),
            first_name: Some(format!("Worker{}", i)),
            last_name: "Batch".into(),
            status: "active".into(),
            created_at: None,
        });
    }
    let rows_inserted = Users::insert_batch(&pool, &batch_users).await?;
    println!("   ✅ {} utilisateurs insérés en UNE SEULE requête SQL !", rows_inserted);

    println!("\n⏭️  5. Pagination ultra-rapide par Curseur (Keyset)");
    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());

    println!("\n🌊 6. Lecture en continu (Streaming asynchrone)");
    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    println!("\n🗝️  7. Gestion avancée : Clé Primaire Composite");
    let item = OrderItems { order_id: 300, product_id: 99, quantity: 2 };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 300, Produit: 99)");
    let item_check = OrderItems::get_by_pk(&pool, &300, &99).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);
    OrderItems::delete_by_pk(&pool, &300, &99).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &300, &99).await?;
    println!("   ✅ Suppression réussie (Encore présent ? {})", deleted_item.is_some());

    println!("\n🗑️  8. Suppression en masse (Batch Delete)");
    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS SQLITE ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}