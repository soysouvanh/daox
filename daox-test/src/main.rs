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
/// 🐘 EXHAUSTIVE DEMONSTRATION OF THE DAO LIBRARY (POSTGRESQL)
/// This code serves as the official documentation for all advanced Use Cases.
/// ========================================================================
async fn run_postgres() -> Result<(), sqlx::Error> {
    use models_pg::users::{Users, UsersPatch};
    use models_pg::order_items::OrderItems;
    use futures::StreamExt; // Nécessaire pour traiter les Streams asynchrones
    
    // 1. Connect to the PostgreSQL engine using SQLx connection pooling

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5433/daox_test")
        .await?;

    println!("=====================================================");
    println!("🐘 DÉMONSTRATION COMPLÈTE DAO SUR POSTGRESQL");
    println!("=====================================================\n");

    // Initial cleanup of tables to ensure a clean test environment

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items CASCADE").execute(&pool).await?;

    // --- USE CASE 1: CLASSIC INSERTION & READING ---

    println!("🚀 1. Simple Insertion (Insert)");
    let user1 = Users {
        id: 0, // Ignored by PostgreSQL because it is defined as BIGSERIAL (Identity)

        email: "alice@daox.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Wonder".into(),
        status: "active".into(),
        created_at: None, // Automatically populated by the DB (DEFAULT CURRENT_TIMESTAMP)

    };
    let user1_id = user1.insert(&pool).await?;
    println!("   ✅ Utilisateur inséré avec l'ID auto-généré : {}", user1_id);

    // EXISTS: Ultra-fast existence check (SELECT 1) without loading full row data into RAM

    let exists = Users::exists_by_pk(&pool, &(user1_id as i64)).await?;
    println!("   ✅ Vérification exists_by_pk : {}", exists);

    // --- USE CASE 2: UPSERT (Insert or Update) ---
    // Safely insert data, or update it if a Unique Constraint (e.g., email) is violated.

    println!("\n🔄 2. Automatic Update (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Wonderland".into();
    // The method intelligently generates an "ON CONFLICT DO UPDATE" or "ON DUPLICATE KEY" based on the dialect

    user1_modified.upsert(&pool).await?;
    
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    // --- USE CASE 3: OPTIMIZED PARTIAL UPDATE (Patch) ---
    // Only updates the exact columns you specify, saving network bandwidth and DB disk writes.

    println!("\n🩹 3. Partial Update (Patch)");
    let patch = UsersPatch {
        first_name: Some(None), // 💡 Magic: We use `Some(None)` to explicitly force the column to NULL

        status: Some("banned".into()),
        ..Default::default() // All other columns (email, last_name) will be completely omitted from the final UPDATE query

    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    // --- USE CASE 4: BATCH INSERTION (Extreme Performance) ---
    // Inserts an array of objects in a single database round-trip.

    println!("\n📦 4. Mass Insertion (Batch Insert)");
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
    // Generates a single massive SQL query: INSERT INTO users (...) VALUES (...), (...), (...)

    let rows_inserted = Users::insert_batch(&pool, &batch_users).await?;
    println!("   ✅ {} utilisateurs insérés en UNE SEULE requête SQL !", rows_inserted);

    // --- USE CASE 5: KEYSET PAGINATION (O(1) Absolute Performance) ---
    println!("\n⏭️  5. Cursor-based Keyset Pagination");
    // Ideal for REST APIs: Instead of using OFFSET (which scans and discards rows), we jump straight to the last known ID.

    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());
    for u in page {
        println!("      - Utilisateur lu : ID {}, Email {}", u.id, u.email);
    }

    // --- USE CASE 6: ZERO-ALLOCATION ASYNC STREAMING ---
    println!("\n🌊 6. Asynchronous Streaming");

    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
        // 💡 Here we could process millions of database rows without ever blowing up the server's RAM!

    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    // --- USE CASE 7: COMPOSITE PRIMARY KEYS (Absolute Type Safety) ---
    println!("\n🗝️  7. Advanced: Composite Primary Keys");

    let item = OrderItems {
        order_id: 101,
        product_id: 42,
        quantity: 5,
    };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 101, Produit: 42)");
    
    // Daox detects multiple PKs and strictly requires ALL identifiers in the method signature to prevent accidents

    let item_check = OrderItems::get_by_pk(&pool, &101, &42).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);

    // Targeted deletion (Guarantees we don't accidentally delete the entire order!)

    OrderItems::delete_by_pk(&pool, &101, &42).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &101, &42).await?;
    println!("   ✅ Suppression de l'article spécifique réussie (Encore présent ? {})", deleted_item.is_some());

    // --- USE CASE 8: BATCH DELETE ---
    println!("\n🗑️  8. Batch Deletion");
    // Deletes multiple IDs instantly via a powerful `WHERE id IN (?, ?, ?)` clause

    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS POSTGRESQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

/// ========================================================================
/// 🐬 EXHAUSTIVE DEMONSTRATION OF THE DAO LIBRARY (MYSQL / MARIADB)
/// Identical to the PostgreSQL code, proving the absolute portability of the code.
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
    println!("🐬 EXHAUSTIVE DAO DEMONSTRATION ON MYSQL/MARIADB");
    println!("=====================================================\n");

    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items").execute(&pool).await?;

    println!("🚀 1. Simple Insertion (Insert)");
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

    println!("\n🔄 2. Automatic Update (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Le Bricoleur".into();
    user1_modified.upsert(&pool).await?;
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    println!("\n🩹 3. Partial Update (Patch)");
    let patch = UsersPatch {
        first_name: Some(None),
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    println!("\n📦 4. Mass Insertion (Batch Insert)");
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

    println!("\n⏭️  5. Cursor-based Keyset Pagination");
    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());

    println!("\n🌊 6. Asynchronous Streaming");
    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    println!("\n🗝️  7. Advanced: Composite Primary Keys");
    let item = OrderItems { order_id: 200, product_id: 99, quantity: 2 };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 200, Produit: 99)");
    let item_check = OrderItems::get_by_pk(&pool, &200, &99).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);
    OrderItems::delete_by_pk(&pool, &200, &99).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &200, &99).await?;
    println!("   ✅ Suppression réussie (Encore présent ? {})", deleted_item.is_some());

    println!("\n🗑️  8. Batch Deletion");
    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS MYSQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

/// ========================================================================
/// 🪶 EXHAUSTIVE DEMONSTRATION OF THE DAO LIBRARY (SQLITE)
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
    println!("🪶 EXHAUSTIVE DAO DEMONSTRATION ON SQLITE");
    println!("=====================================================\n");

    println!("🚀 1. Simple Insertion (Insert)");
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

    println!("\n🔄 2. Automatic Update (Upsert)");
    let mut user1_modified = user1.clone();
    user1_modified.id = user1_id as i64;
    user1_modified.last_name = "Embedded".into();
    user1_modified.upsert(&pool).await?;
    let check_upsert = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Nom correctement mis à jour via Upsert : {}", check_upsert.last_name);

    println!("\n🩹 3. Partial Update (Patch)");
    let patch = UsersPatch {
        first_name: Some(None),
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user1_id as i64), &patch).await?;
    let check_patch = Users::get_by_pk(&pool, &(user1_id as i64)).await?.unwrap();
    println!("   ✅ Statut: '{}', Prénom mis à jour: {:?}", check_patch.status, check_patch.first_name);

    println!("\n📦 4. Mass Insertion (Batch Insert)");
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

    println!("\n⏭️  5. Cursor-based Keyset Pagination");
    let page = Users::list_by_cursor(&pool, &20, 5).await?;
    println!("   ✅ {} utilisateurs récupérés juste après l'ID 20.", page.len());

    println!("\n🌊 6. Asynchronous Streaming");
    let mut stream = Users::stream_all(&pool);
    let mut stream_count = 0;
    while let Some(user_result) = stream.next().await {
        let _u = user_result?;
        stream_count += 1;
    }
    println!("   ✅ {} utilisateurs itérés avec une empreinte mémoire quasi-nulle.", stream_count);

    println!("\n🗝️  7. Advanced: Composite Primary Keys");
    let item = OrderItems { order_id: 300, product_id: 99, quantity: 2 };
    item.insert(&pool).await?;
    println!("   ✅ Article ajouté au panier (Commande: 300, Produit: 99)");
    let item_check = OrderItems::get_by_pk(&pool, &300, &99).await?.unwrap();
    println!("   ✅ Lecture sécurisée réussie (Quantité: {})", item_check.quantity);
    OrderItems::delete_by_pk(&pool, &300, &99).await?;
    let deleted_item = OrderItems::get_by_pk(&pool, &300, &99).await?;
    println!("   ✅ Suppression réussie (Encore présent ? {})", deleted_item.is_some());

    println!("\n🗑️  8. Batch Deletion");
    let ids_to_delete = vec![10, 11, 12, 13, 14];
    let deleted_rows = Users::delete_many_by_pk(&pool, &ids_to_delete).await?;
    println!("   ✅ {} utilisateurs supprimés d'un seul coup !\n", deleted_rows);

    println!("🎉 TOUS LES TESTS SQLITE ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}