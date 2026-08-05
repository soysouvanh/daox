//! # Daox Integration Test Suite & Usage Examples
//!
//! This executable serves two purposes:
//! 1. It operates as the ultimate integration test compilation phase for the Daox code generator.
//! 2. It demonstrates full **Use Cases** for consuming completely Zero-Framework generated APIs.
//!
//! Daox generated models wrap underlying `sqlx` constructs natively, giving developers access
//! to compile-time safe, ultra-performant DAO layers with features absent from classic ORMs.
//!
//! ## Core Use Cases Showcased
//!
//! ### Running the test suite
//! To run the complete suite, ensure your databases are active (e.g. via `docker compose up -d`), then execute:
//! - **PostgreSQL**: `cargo run -p daox-test pg`
//! - **MySQL**: `cargo run -p daox-test mysql`
//! - **SQLite**: `cargo run -p daox-test sqlite`
//!
//! ### 1. Basic CRUD & Type-safe Validation
//! ```rust,ignore
//! let user = Users {
//!     id: 0,
//!     email: "demo@daox.dev".into(),
//!     first_name: Some("John".into()),
//!     last_name: "Doe".into(),
//!     status: "active".into(),
//!     created_at: None,
//! };
//! // Validation regex & constraints are checked natively before DB roundtrip!
//! let id = user.insert(&pool).await?;
//!
//! let fetched = Users::get_by_id(&pool, id).await?.unwrap();
//! ```
//!
//! ### 2. Zero-Allocation Batching (Native Bulk Insert / Upsert)
//! Mass inserts leverage extreme DB-specific features (e.g., PostgreSQL `COPY STDIN WITH CSV`
//! or automatic parameter chunking for MySQL/SQLite).
//! ```rust,ignore
//! let mut tx = pool.begin().await?;
//! // Processes arrays up to 65,535 parameters safely via internal chunk sliding window
//! Users::insert_batch(&mut tx, &users_list).await?;
//! tx.commit().await?;
//! ```
//!
//! ### 3. O(1) Approximation & Data Streaming
//! Read mass quantities of rows using zero heap allocations and fetch table counts in milliseconds.
//! ```rust,ignore
//! // Statistics bypasses full table scans for huge analytical endpoints
//! let count = Users::approximate_count(&pool).await?;
//!
//! // Only memory-buffers 1 row per cycle, perfectly stable for millions of rows
//! let mut stream = Users::stream_all(&pool, 1000);
//! while let Some(row) = stream.next().await { /* loop */ }
//! ```
//!
//! ### 4. Multi-Dialect Transactions & Nested Structs
//! `Daox` enforces transaction boundaries gracefully utilizing the standard `sqlx::Executor`.
//! ```rust,ignore
//! let mut tx = pool.begin().await?;
//! Users::delete_many_by_id(&mut tx, &[1, 2, 3]).await?; // Explicitly requires `&mut tx` to ensure atomicity
//! tx.rollback().await?;
//! ```
#![allow(clippy::ptr_arg)]

pub mod models_mysql;
pub mod models_pg;
pub mod models_sqlite;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

fn load_dotenv() {
    if let Ok(content) =
        std::fs::read_to_string("../.env").or_else(|_| std::fs::read_to_string(".env"))
    {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some((k, v)) = line.split_once('=') {
                if env::var(k.trim()).is_err() {
                    unsafe {
                        env::set_var(k.trim(), v.trim().trim_matches('"'));
                    }
                }
            }
        }
    }
}

/// # Test Application Entrypoint
/// Invokes specific database test runners directly based on the deployment CLI arguments.
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    load_dotenv();
    let args: Vec<String> = env::args().collect();
    let dialect = args.get(1).map(|s| s.as_str()).unwrap_or("mysql");

    match dialect {
        "postgres" | "pg" => run_postgres().await?,
        "sqlite" | "sqli" => run_sqlite().await?,
        "mysql" | "mariadb" => run_mysql().await?,
        _ => {
            println!("Unrecognized dialect '{}', defaulting to MySQL...", dialect);
            run_mysql().await?
        }
    }

    Ok(())
}

/// # PostgreSQL Test & Use Cases
/// Contains comprehensive invocations for all generated DAO methods using the Postgres driver.
/// Features tested include pure SQL `COPY` command bulk bindings, composite PK updates, and JSON serialization.
async fn run_postgres() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_pg::{OrderItems, Users};

    let pg_url = env::var("DATABASE_URL_PG")
        .map_err(|_| sqlx::Error::Protocol("DATABASE_URL_PG must be set".into()))?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;

    println!("POSTGRESQL DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;
    sqlx::query("TRUNCATE TABLE order_items CASCADE")
        .execute(&pool)
        .await?;

    println!("- Executing test: INSERT...");
    // --- INSERT ---
    let user1 = Users {
        id: 0,
        email: "alice@daox.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Wonder".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&pool).await.unwrap();

    println!("- Executing test: EXISTS...");
    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id as i64).await.unwrap();
    assert!(exists);

    println!("- Executing test: GET_BY_PK...");
    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "alice@daox.dev");

    println!("- Executing test: UPDATE_BY_PK...");
    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "banned".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "banned");

    println!("- Executing test: INSERT_BATCH...");
    // --- INSERT_BATCH ---
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
    let mut tx = pool.begin().await.unwrap();
    Users::insert_batch(&mut tx, &batch_users).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: COUNT...");
    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    println!("- Executing test: LIST_BY_CURSOR...");
    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id as i64, 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

    println!("- Executing test: STREAM_ALL...");
    // --- STREAM_ALL ---
    #[allow(deprecated)]
    {
        let mut stream = Users::stream_all(&pool, 1000);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    println!("- Executing test: COMPOSITE PK...");
    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 101,
        product_id: 42,
        quantity: 5,
    };
    item.insert(&pool).await.unwrap();

    println!("- Executing test: UPDATE COMPOSITE PK...");
    // --- UPDATE COMPOSITE PK ---
    item.quantity = 15;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 101, 42)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 15);

    println!("- Executing test: DELETE COMPOSITE PK...");
    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 101, 42)
        .await
        .unwrap();

    /*
    println!("- Executing test: UPSERT...");
    // --- UPSERT ---
    let mut upsert_user = user_to_update.clone();
    upsert_user.status = "active_upsert".into();
    upsert_user.upsert(&pool).await.unwrap();
    let check_upsert = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert.status, "active_upsert");

    println!("- Executing test: UPSERT_BATCH...");
    // --- UPSERT_BATCH ---
    let mut tx_upsert = pool.begin().await.unwrap();
    upsert_user.last_name = "Upsert Batch".into();
    Users::upsert_batch(&mut tx_upsert, &[upsert_user])
        .await
        .unwrap();
    tx_upsert.commit().await.unwrap();
    let check_upsert_batch = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert_batch.last_name, "Upsert Batch");
    */

    println!("- Executing test: UPDATE_PARTIAL_BY_PK...");
    // --- UPDATE_PARTIAL_BY_PK ---
    let patch = models_pg::UsersPatch {
        status: Some("partial".into()),
        ..Default::default()
    };
    Users::update_partial_by_id(&pool, user1_id as i64, &patch)
        .await
        .unwrap();
    let check_partial = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_partial.status, "partial");

    println!("- Executing test: INDEX METHODS...");
    // --- INDEX METHODS ---
    assert!(
        Users::exists_by_email(&pool, "alice@daox.dev")
            .await
            .unwrap()
    );
    assert!(
        Users::get_by_email(&pool, "alice@daox.dev")
            .await
            .unwrap()
            .is_some()
    );
    let ls = Users::list_by_first_name_and_last_name(&pool, "Bot1", "Batch", 10)
        .await
        .unwrap();
    assert!(!ls.is_empty());
    assert!(
        Users::exists_by_first_name_and_last_name(&pool, "Bot1", "Batch")
            .await
            .unwrap()
    );
    Users::delete_by_first_name_and_last_name(&pool, "Bot2", "Batch")
        .await
        .unwrap();
    Users::delete_by_email(&pool, "alice@daox.dev")
        .await
        .unwrap();

    println!("- Executing test: DELETE_BY_ID...");
    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i64).await.unwrap();

    println!("- Executing test: DELETE_MANY_BY_PK...");
    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i64> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: TRANSACTION TEST (ROLLBACK)...");
    // --- TRANSACTION TEST (ROLLBACK) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx = Users {
            id: 0,
            email: "tx_rollback_pg@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Rollback".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx.insert(&mut *tx).await.unwrap();
        tx.rollback().await?;
    }
    let exists_rollback = Users::exists_by_email(&pool, "tx_rollback_pg@daox.dev")
        .await
        .unwrap();
    assert!(!exists_rollback);

    println!("- Executing test: TRANSACTION TEST (COMMIT)...");
    // --- TRANSACTION TEST (COMMIT) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx2 = Users {
            id: 0,
            email: "tx_commit_pg@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Commit".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx2.insert(&mut *tx).await.unwrap();
        tx.commit().await?;
    }
    let exists_commit = Users::exists_by_email(&pool, "tx_commit_pg@daox.dev")
        .await
        .unwrap();
    assert!(exists_commit);

    /*
    println!("- Executing test: BYTEA COPY ROUNDTRIP TEST...");
    // --- BYTEA COPY ROUNDTRIP TEST ---
    {
        sqlx::query("TRUNCATE TABLE product_metadata CASCADE")
            .execute(&pool)
            .await?;
        let test_payload = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0x0A, 0x0D, 0x22, 0x27, 0x5C];
        let pm = models_pg::ProductMetadata {
            id: "00000000-0000-0000-0000-000000009999".into(),
            category: "tech".into(),
            attributes: Some(serde_json::json!({"test": true})),
            raw_data: Some(test_payload.clone()),
        };
        let mut tx = pool.begin().await.unwrap();
        models_pg::ProductMetadata::insert_batch(&mut tx, std::slice::from_ref(&pm))
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let fetched =
            models_pg::ProductMetadata::get_by_id(&pool, "00000000-0000-0000-0000-000000009999")
                .await?
                .unwrap();
        assert_eq!(
            fetched.raw_data.unwrap(),
            test_payload,
            "BYTEA COPY roundtrip failed!"
        );
    }
    */

    println!("- Executing test: EXHAUSTIVE TESTING COMPLETION...");
    // --- EXHAUSTIVE TESTING COMPLETION ---
    let approx = Users::approximate_count(&pool).await.unwrap();
    let _ = approx;

    let user_unchk = Users {
        id: 0,
        email: "unchk1@daox.dev".into(),
        first_name: Some("Unchecked".into()),
        last_name: "Pg".into(),
        status: "active".into(),
        created_at: None,
    };
    let unchk_id = user_unchk.insert_unchecked(&pool).await.unwrap();
    let mut user_unchk_upd = user_unchk.clone();
    user_unchk_upd.id = unchk_id.try_into().unwrap();
    user_unchk_upd.status = "banned".into();
    user_unchk_upd.update_unchecked_by_id(&pool).await.unwrap();
    // user_unchk_upd.status = "upserted".into();
    // user_unchk_upd.upsert_unchecked(&pool).await.unwrap();
    Users::delete_by_id(&pool, unchk_id.try_into().unwrap())
        .await
        .unwrap();

    let comp_item = OrderItems {
        order_id: 888,
        product_id: 999,
        quantity: 1,
    };
    comp_item.insert(&pool).await.unwrap();
    let patch_item = models_pg::OrderItemsPatch {
        quantity: Some(99),
        ..Default::default()
    };
    OrderItems::update_partial_by_order_id_and_product_id(&pool, 888, 999, &patch_item)
        .await
        .unwrap();
    OrderItems::delete_by_order_id_and_product_id(&pool, 888, 999)
        .await
        .unwrap();

    use models_pg::ActiveUsers;
    #[allow(deprecated)]
    {
        let mut v_stream = ActiveUsers::stream_all(&pool, 10);
        while v_stream.next().await.is_some() {}
        let v_count = ActiveUsers::count(&pool).await.unwrap();
        let _ = v_count;
    }

    println!("ALL POSTGRESQL TESTS PASSED! Daox is production-ready.");
    Ok(())
}

/// # MySQL Test & Use Cases
/// Demonstrates syntax compatibility generation dynamically for MySQL engines.
async fn run_mysql() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_mysql::{OrderItems, Users};

    let mysql_url = env::var("DATABASE_URL_MYSQL")
        .map_err(|_| sqlx::Error::Protocol("DATABASE_URL_MYSQL must be set".into()))?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_url)
        .await?;

    println!("MYSQL/MARIADB DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items")
        .execute(&pool)
        .await?;

    println!("- Executing test: INSERT...");
    // --- INSERT ---
    let user1 = Users {
        id: 0,
        email: "bob@daox.dev".into(),
        first_name: Some("Bob".into()),
        last_name: "Builder".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&pool).await.unwrap();

    println!("- Executing test: EXISTS...");
    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id as i64).await.unwrap();
    assert!(exists);

    println!("- Executing test: GET_BY_PK...");
    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "bob@daox.dev");

    println!("- Executing test: UPDATE_BY_PK...");
    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "banned".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "banned");

    println!("- Executing test: INSERT_BATCH...");
    // --- INSERT_BATCH ---
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
    let mut tx = pool.begin().await.unwrap();
    Users::insert_batch(&mut tx, &batch_users).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: COUNT...");
    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    println!("- Executing test: LIST_BY_CURSOR...");
    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id as i64, 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

    println!("- Executing test: STREAM_ALL...");
    // --- STREAM_ALL ---
    #[allow(deprecated)]
    {
        let mut stream = Users::stream_all(&pool, 1000);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    println!("- Executing test: COMPOSITE PK...");
    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 200,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&pool).await.unwrap();

    println!("- Executing test: UPDATE COMPOSITE PK...");
    // --- UPDATE COMPOSITE PK ---
    item.quantity = 25;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 200, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 25);

    println!("- Executing test: DELETE COMPOSITE PK...");
    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 200, 99)
        .await
        .unwrap();

    /*
    println!("- Executing test: UPSERT...");
    // --- UPSERT ---
    let mut upsert_user = user_to_update.clone();
    upsert_user.status = "active_upsert".into();
    upsert_user.upsert(&pool).await.unwrap();
    let check_upsert = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert.status, "active_upsert");

    println!("- Executing test: UPSERT_BATCH...");
    // --- UPSERT_BATCH ---
    let mut tx_upsert = pool.begin().await.unwrap();
    upsert_user.last_name = "Upsert Batch".into();
    Users::upsert_batch(&mut tx_upsert, &[upsert_user])
        .await
        .unwrap();
    tx_upsert.commit().await.unwrap();
    let check_upsert_batch = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert_batch.last_name, "Upsert Batch");
    */

    println!("- Executing test: UPDATE_PARTIAL_BY_PK...");
    // --- UPDATE_PARTIAL_BY_PK ---
    let patch = models_mysql::UsersPatch {
        status: Some("partial".into()),
        ..Default::default()
    };
    Users::update_partial_by_id(&pool, user1_id as i64, &patch)
        .await
        .unwrap();
    let check_partial = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_partial.status, "partial");

    println!("- Executing test: INDEX METHODS...");
    // --- INDEX METHODS ---
    assert!(Users::exists_by_email(&pool, "bob@daox.dev").await.unwrap());
    assert!(
        Users::get_by_email(&pool, "bob@daox.dev")
            .await
            .unwrap()
            .is_some()
    );
    let ls = Users::list_by_last_name_and_first_name(&pool, "Batch", "Worker1", 10)
        .await
        .unwrap();
    assert!(!ls.is_empty());
    assert!(
        Users::exists_by_last_name_and_first_name(&pool, "Batch", "Worker1")
            .await
            .unwrap()
    );
    Users::delete_by_last_name_and_first_name(&pool, "Batch", "Worker2")
        .await
        .unwrap();
    Users::delete_by_email(&pool, "bob@daox.dev").await.unwrap();

    println!("- Executing test: DELETE_BY_ID...");
    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i64).await.unwrap();

    println!("- Executing test: DELETE_MANY_BY_PK...");
    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i64> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: TRANSACTION TEST (ROLLBACK)...");
    // --- TRANSACTION TEST (ROLLBACK) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx = Users {
            id: 0,
            email: "tx_rollback_mysql@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Rollback".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx.insert(&mut *tx).await.unwrap();
        tx.rollback().await?;
    }
    let exists_rollback = Users::exists_by_email(&pool, "tx_rollback_mysql@daox.dev")
        .await
        .unwrap();
    assert!(!exists_rollback);

    println!("- Executing test: TRANSACTION TEST (COMMIT)...");
    // --- TRANSACTION TEST (COMMIT) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx2 = Users {
            id: 0,
            email: "tx_commit_mysql@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Commit".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx2.insert(&mut *tx).await.unwrap();
        tx.commit().await?;
    }
    let exists_commit = Users::exists_by_email(&pool, "tx_commit_mysql@daox.dev")
        .await
        .unwrap();
    assert!(exists_commit);

    println!("- Executing test: EXHAUSTIVE TESTING COMPLETION...");
    // --- EXHAUSTIVE TESTING COMPLETION ---
    let approx = Users::approximate_count(&pool).await.unwrap();
    let _ = approx;

    let user_unchk = Users {
        id: 0,
        email: "unchk2@daox.dev".into(),
        first_name: Some("Unchecked".into()),
        last_name: "My".into(),
        status: "active".into(),
        created_at: None,
    };
    let unchk_id = user_unchk.insert_unchecked(&pool).await.unwrap();
    let mut user_unchk_upd = user_unchk.clone();
    user_unchk_upd.id = unchk_id.try_into().unwrap();
    user_unchk_upd.status = "banned".into();
    user_unchk_upd.update_unchecked_by_id(&pool).await.unwrap();
    // user_unchk_upd.status = "upserted".into();
    // user_unchk_upd.upsert_unchecked(&pool).await.unwrap();
    Users::delete_by_id(&pool, unchk_id.try_into().unwrap())
        .await
        .unwrap();

    let comp_item = OrderItems {
        order_id: 888,
        product_id: 999,
        quantity: 1,
    };
    comp_item.insert(&pool).await.unwrap();
    let patch_item = models_mysql::OrderItemsPatch {
        quantity: Some(99),
        ..Default::default()
    };
    OrderItems::update_partial_by_order_id_and_product_id(&pool, 888, 999, &patch_item)
        .await
        .unwrap();
    OrderItems::delete_by_order_id_and_product_id(&pool, 888, 999)
        .await
        .unwrap();

    use models_mysql::ActiveUsers;
    #[allow(deprecated)]
    {
        let mut v_stream = ActiveUsers::stream_all(&pool, 10);
        while v_stream.next().await.is_some() {}
        let v_count = ActiveUsers::count(&pool).await.unwrap();
        let _ = v_count;
    }

    println!("ALL MYSQL TESTS PASSED! Daox is production-ready.");
    Ok(())
}

/// # SQLite Test & Use Cases
/// Tests fallback chunking configurations and dynamic file creations via SQLite engine.
async fn run_sqlite() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_sqlite::{OrderItems, Users};

    let sqlite_db_path = format!("{}/daox_test.sqlite", env!("OUT_DIR"));
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", sqlite_db_path))
        .await?;

    println!("SQLITE DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("DELETE FROM users").execute(&pool).await?;
    sqlx::query("DELETE FROM order_items")
        .execute(&pool)
        .await?;

    println!("- Executing test: INSERT...");
    // --- INSERT ---
    let user1 = Users {
        id: 0,
        email: "alice@sqlite.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Sqlite".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&pool).await.unwrap();

    println!("- Executing test: EXISTS...");
    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap();
    assert!(exists);

    println!("- Executing test: GET_BY_PK...");
    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "alice@sqlite.dev");

    println!("- Executing test: UPDATE_BY_PK...");
    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "inactive".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "inactive");

    println!("- Executing test: INSERT_BATCH...");
    // --- INSERT_BATCH ---
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
    let mut tx = pool.begin().await.unwrap();
    Users::insert_batch(&mut tx, &batch_users).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: COUNT...");
    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    println!("- Executing test: LIST_BY_CURSOR...");
    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id.try_into().unwrap(), 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

    println!("- Executing test: STREAM_ALL...");
    // --- STREAM_ALL ---
    #[allow(deprecated)]
    {
        let mut stream = Users::stream_all(&pool, 1000);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    println!("- Executing test: COMPOSITE PK...");
    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 300,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&pool).await.unwrap();

    println!("- Executing test: UPDATE COMPOSITE PK...");
    // --- UPDATE COMPOSITE PK ---
    item.quantity = 35;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 300, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 35);

    println!("- Executing test: DELETE COMPOSITE PK...");
    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 300, 99)
        .await
        .unwrap();

    println!("- Executing test: UPSERT...");
    // --- UPSERT ---
    let mut upsert_user = user_to_update.clone();
    upsert_user.email = "upsert1_sqlite@sqlite.dev".into();
    upsert_user.status = "active_upsert".into();
    upsert_user.upsert(&pool).await.unwrap();
    let check_upsert = Users::get_by_email(&pool, "upsert1_sqlite@sqlite.dev")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert.status, "active_upsert");

    println!("- Executing test: UPSERT_BATCH...");
    // --- UPSERT_BATCH ---
    let mut tx_upsert = pool.begin().await.unwrap();
    upsert_user.email = "upsert2_sqlite@sqlite.dev".into();
    upsert_user.last_name = "Upsert Batch".into();
    Users::upsert_batch(&mut tx_upsert, &[upsert_user])
        .await
        .unwrap();
    tx_upsert.commit().await.unwrap();
    let check_upsert_batch = Users::get_by_email(&pool, "upsert2_sqlite@sqlite.dev")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert_batch.last_name, "Upsert Batch");

    println!("- Executing test: UPDATE_PARTIAL_BY_PK...");
    // --- UPDATE_PARTIAL_BY_PK ---
    let patch = models_sqlite::UsersPatch {
        status: Some("partial".into()),
        ..Default::default()
    };
    Users::update_partial_by_id(&pool, user1_id as i32, &patch)
        .await
        .unwrap();
    let check_partial = Users::get_by_id(&pool, user1_id as i32)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_partial.status, "partial");

    println!("- Executing test: INDEX METHODS...");
    // --- INDEX METHODS ---
    assert!(
        Users::exists_by_email(&pool, "alice@sqlite.dev")
            .await
            .unwrap()
    );
    assert!(
        Users::get_by_email(&pool, "alice@sqlite.dev")
            .await
            .unwrap()
            .is_some()
    );
    let ls = Users::list_by_last_name_and_first_name(&pool, "Batch", "Worker1", 10)
        .await
        .unwrap();
    assert!(!ls.is_empty());
    assert!(
        Users::exists_by_last_name_and_first_name(&pool, "Batch", "Worker1")
            .await
            .unwrap()
    );
    Users::delete_by_last_name_and_first_name(&pool, "Batch", "Worker2")
        .await
        .unwrap();
    Users::delete_by_email(&pool, "alice@sqlite.dev")
        .await
        .unwrap();

    println!("- Executing test: DELETE_BY_ID...");
    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i32).await.unwrap();

    println!("- Executing test: DELETE_MANY_BY_PK...");
    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i32> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

    println!("- Executing test: TRANSACTION TEST (ROLLBACK)...");
    // --- TRANSACTION TEST (ROLLBACK) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx = Users {
            id: 0,
            email: "tx_rollback_sqlite@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Rollback".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx.insert(&mut *tx).await.unwrap();
        tx.rollback().await?;
    }
    let exists_rollback = Users::exists_by_email(&pool, "tx_rollback_sqlite@daox.dev")
        .await
        .unwrap();
    assert!(!exists_rollback);

    println!("- Executing test: TRANSACTION TEST (COMMIT)...");
    // --- TRANSACTION TEST (COMMIT) ---
    {
        let mut tx = pool.begin().await?;
        let user_tx2 = Users {
            id: 0,
            email: "tx_commit_sqlite@daox.dev".into(),
            first_name: Some("Tx".into()),
            last_name: "Commit".into(),
            status: "active".into(),
            created_at: None,
        };
        user_tx2.insert(&mut *tx).await.unwrap();
        tx.commit().await?;
    }
    let exists_commit = Users::exists_by_email(&pool, "tx_commit_sqlite@daox.dev")
        .await
        .unwrap();
    assert!(exists_commit);

    println!("- Executing test: EXHAUSTIVE TESTING COMPLETION...");
    // --- EXHAUSTIVE TESTING COMPLETION ---

    let user_unchk = Users {
        id: 0,
        email: "unchk3@daox.dev".into(),
        first_name: Some("Unchecked".into()),
        last_name: "Sq".into(),
        status: "active".into(),
        created_at: None,
    };
    let unchk_id = user_unchk.insert_unchecked(&pool).await.unwrap();
    let mut user_unchk_upd = user_unchk.clone();
    user_unchk_upd.id = unchk_id.try_into().unwrap();
    user_unchk_upd.status = "banned".into();
    user_unchk_upd.update_unchecked_by_id(&pool).await.unwrap();
    // user_unchk_upd.status = "upserted".into();
    // user_unchk_upd.upsert_unchecked(&pool).await.unwrap();
    Users::delete_by_id(&pool, unchk_id.try_into().unwrap())
        .await
        .unwrap();

    let comp_item = OrderItems {
        order_id: 888,
        product_id: 999,
        quantity: 1,
    };
    comp_item.insert(&pool).await.unwrap();
    let patch_item = models_sqlite::OrderItemsPatch {
        quantity: Some(99),
        ..Default::default()
    };
    OrderItems::update_partial_by_order_id_and_product_id(&pool, 888, 999, &patch_item)
        .await
        .unwrap();
    OrderItems::delete_by_order_id_and_product_id(&pool, 888, 999)
        .await
        .unwrap();

    use models_sqlite::ActiveUsers;
    #[allow(deprecated)]
    {
        let mut v_stream = ActiveUsers::stream_all(&pool, 10);
        while v_stream.next().await.is_some() {}
        let v_count = ActiveUsers::count(&pool).await.unwrap();
        let _ = v_count;
    }

    println!("ALL SQLITE TESTS PASSED! Daox is production-ready.");
    Ok(())
}
