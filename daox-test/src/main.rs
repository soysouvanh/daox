#![allow(clippy::ptr_arg)]

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

    match dialect {
        "postgres" | "pg" => run_postgres().await?,
        "sqlite" => run_sqlite().await?,
        _ => run_mysql().await?,
    }

    Ok(())
}

async fn run_postgres() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_pg::{OrderItems, Users};

    let pg_url = env::var("DATABASE_URL_PG")
        .map_err(|_| sqlx::Error::Protocol("DATABASE_URL_PG must be set".into()))?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await?;

    println!("🐘 POSTGRESQL DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;
    sqlx::query("TRUNCATE TABLE order_items CASCADE")
        .execute(&pool)
        .await?;

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

    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id as i64).await.unwrap();
    assert!(exists);

    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "alice@daox.dev");

    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "banned".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "banned");

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

    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id as i64, 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

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

    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 101,
        product_id: 42,
        quantity: 5,
    };
    item.insert(&pool).await.unwrap();

    // --- UPDATE COMPOSITE PK ---
    item.quantity = 15;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 101, 42)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 15);

    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 101, 42)
        .await
        .unwrap();

    // --- UPSERT ---
    let mut upsert_user = user_to_update.clone();
    upsert_user.status = "active_upsert".into();
    upsert_user.upsert(&pool).await.unwrap();
    let check_upsert = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert.status, "active_upsert");

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

    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i64).await.unwrap();

    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i64> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

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

    // --- BYTEA COPY ROUNDTRIP TEST ---
    {
        sqlx::query("TRUNCATE TABLE product_metadata CASCADE")
            .execute(&pool)
            .await?;
        let test_payload = vec![0x00, 0x01, 0x02, 0xFF, 0xFE, 0x0A, 0x0D, 0x22, 0x27, 0x5C];
        let pm = models_pg::ProductMetadata {
            id: "9999".into(),
            category: "test".into(),
            attributes: Some(serde_json::json!({"test": true})),
            raw_data: Some(test_payload.clone()),
        };
        let mut tx = pool.begin().await?;
        models_pg::ProductMetadata::insert_batch(&mut tx, std::slice::from_ref(&pm))
            .await
            .unwrap();
        tx.commit().await?;

        let fetched = models_pg::ProductMetadata::get_by_id(&pool, "9999")
            .await?
            .unwrap();
        assert_eq!(
            fetched.raw_data.unwrap(),
            test_payload,
            "BYTEA COPY roundtrip failed!"
        );
    }

    println!("🎉 ALL POSTGRESQL TESTS PASSED! Daox is production-ready.");
    Ok(())
}

async fn run_mysql() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_mysql::{OrderItems, Users};

    let mysql_url = env::var("DATABASE_URL_MYSQL")
        .map_err(|_| sqlx::Error::Protocol("DATABASE_URL_MYSQL must be set".into()))?;
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_url)
        .await?;

    println!("🐬 MYSQL/MARIADB DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items")
        .execute(&pool)
        .await?;

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

    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id as i64).await.unwrap();
    assert!(exists);

    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "bob@daox.dev");

    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "banned".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "banned");

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

    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id as i64, 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

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

    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 200,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&pool).await.unwrap();

    // --- UPDATE COMPOSITE PK ---
    item.quantity = 25;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 200, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 25);

    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 200, 99)
        .await
        .unwrap();

    // --- UPSERT ---
    let mut upsert_user = user_to_update.clone();
    upsert_user.status = "active_upsert".into();
    upsert_user.upsert(&pool).await.unwrap();
    let check_upsert = Users::get_by_id(&pool, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_upsert.status, "active_upsert");

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

    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i64).await.unwrap();

    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i64> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

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

    println!("🎉 ALL MYSQL TESTS PASSED! Daox is production-ready.");
    Ok(())
}

async fn run_sqlite() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_sqlite::{OrderItems, Users};

    let sqlite_db_path = format!("{}/daox_test.sqlite", env!("OUT_DIR"));
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", sqlite_db_path))
        .await?;

    println!("🪶 SQLITE DEMONSTRATION (pure sqlx, zero framework)");

    sqlx::query("DELETE FROM users").execute(&pool).await?;
    sqlx::query("DELETE FROM order_items")
        .execute(&pool)
        .await?;

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

    // --- EXISTS ---
    let exists = Users::exists_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap();
    assert!(exists);

    // --- GET_BY_PK ---
    let fetched = Users::get_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched.email, "alice@sqlite.dev");

    // --- UPDATE_BY_PK ---
    let mut user_to_update = fetched.clone();
    user_to_update.status = "inactive".into();
    user_to_update.update_by_id(&pool).await.unwrap();
    let check = Users::get_by_id(&pool, user1_id.try_into().unwrap())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check.status, "inactive");

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

    // --- COUNT ---
    #[allow(deprecated)]
    let count = Users::count(&pool).await.unwrap();
    assert!(count >= 51);

    // --- LIST_BY_CURSOR ---
    let cursor_page = Users::list_by_cursor(&pool, user1_id.try_into().unwrap(), 5)
        .await
        .unwrap();
    assert!(cursor_page.len() <= 5);

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

    // --- COMPOSITE PK ---
    let mut item = OrderItems {
        order_id: 300,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&pool).await.unwrap();

    // --- UPDATE COMPOSITE PK ---
    item.quantity = 35;
    item.update_by_order_id_and_product_id(&pool).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&pool, 300, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 35);

    // --- DELETE COMPOSITE PK ---
    OrderItems::delete_by_order_id_and_product_id(&pool, 300, 99)
        .await
        .unwrap();

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

    // --- DELETE_BY_ID ---
    Users::delete_by_id(&pool, user1_id as i32).await.unwrap();

    // --- DELETE_MANY_BY_PK ---
    let ids: Vec<i32> = (1..=10).collect();
    let mut tx = pool.begin().await.unwrap();
    Users::delete_many_by_id(&mut tx, &ids).await.unwrap();
    tx.commit().await.unwrap();

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

    println!("🎉 ALL SQLITE TESTS PASSED! Daox is production-ready.");
    Ok(())
}
