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
    use models_pg::OrderItems;
    use models_pg::RequestContext;
    use models_pg::Users;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5433/daox_test")
        .await?;
    let mut ctx = RequestContext {
        raw_body: lightx::ext::bytes::Bytes::new(),
        global_state: std::sync::Arc::new(lightx::ext::tokio::sync::broadcast::channel(1).0),
        rate_limiter: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        response_cache: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        client_ip: "127.0.0.1".parse().unwrap(),
        user_id: None,
        headers: lightx::ext::hyper::HeaderMap::new(),
        raw_req: None,
        default_pool: pool.clone(),
        default_tx: None,
    };

    println!("🐘 DÉMONSTRATION POSTGRESQL");

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await?;
    sqlx::query("TRUNCATE TABLE order_items CASCADE")
        .execute(&pool)
        .await?;

    let user1 = Users {
        id: 0,
        email: "alice@daox.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Wonder".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&mut ctx).await.unwrap();
    let exists = Users::exists_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap();
    assert!(exists);

    // TEST UPSERT
    let mut user_to_upsert = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    user_to_upsert.status = "banned".into();
    user_to_upsert.upsert(&mut ctx).await.unwrap();
    let check_patch = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_patch.status, "banned");

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
    Users::insert_many(&batch_users, &mut ctx).await.unwrap();

    let count = Users::count(&mut ctx).await.unwrap();
    assert!(count >= 51);

    let all_users = Users::find_all(&mut ctx, 10, 0).await.unwrap();
    assert_eq!(all_users.len(), 10);
    let page = Users::list_by_id_cursor(&mut ctx, Some(user1_id as i64), 5)
        .await
        .unwrap();
    assert!(page.len() <= 5);

    {
        let mut stream = Users::stream_all(&mut ctx);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    let mut item = OrderItems {
        order_id: 101,
        product_id: 42,
        quantity: 5,
    };
    item.insert(&mut ctx).await.unwrap();

    let deleted_users = Users::delete_all(&mut ctx).await.unwrap();
    assert!(deleted_users >= 51);

    // TEST UPSERT
    item.quantity = 15;
    item.upsert(&mut ctx).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&mut ctx, 101, 42)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 15);

    OrderItems::delete_by_order_id_and_product_id(&mut ctx, 101, 42)
        .await
        .unwrap();

    // TEST TRANSACTIONS (ROLLBACK)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx = Users {
        id: 0,
        email: "tx_rollback_pg@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Rollback".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx.insert(&mut ctx).await.unwrap();
    ctx.rollback_default_tx().await.unwrap();
    let all_users_rollback = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        !all_users_rollback
            .iter()
            .any(|u| u.email == "tx_rollback_pg@daox.dev")
    );

    // TEST TRANSACTIONS (COMMIT)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx2 = Users {
        id: 0,
        email: "tx_commit_pg@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Commit".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx2.insert(&mut ctx).await.unwrap();
    ctx.commit_default_tx().await.unwrap();
    let all_users_commit = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        all_users_commit
            .iter()
            .any(|u| u.email == "tx_commit_pg@daox.dev")
    );

    println!("🎉 TOUS LES TESTS POSTGRESQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

async fn run_mysql() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_mysql::OrderItems;
    use models_mysql::RequestContext;
    use models_mysql::Users;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost:3307/daox_test")
        .await?;
    let mut ctx = RequestContext {
        raw_body: lightx::ext::bytes::Bytes::new(),
        global_state: std::sync::Arc::new(lightx::ext::tokio::sync::broadcast::channel(1).0),
        rate_limiter: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        response_cache: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        client_ip: "127.0.0.1".parse().unwrap(),
        user_id: None,
        headers: lightx::ext::hyper::HeaderMap::new(),
        raw_req: None,
        default_pool: pool.clone(),
        default_tx: None,
    };

    println!("🐬 DEMONSTRATION MYSQL/MARIADB");

    sqlx::query("TRUNCATE TABLE users").execute(&pool).await?;
    sqlx::query("TRUNCATE TABLE order_items")
        .execute(&pool)
        .await?;

    let user1 = Users {
        id: 0,
        email: "bob@daox.dev".into(),
        first_name: Some("Bob".into()),
        last_name: "Builder".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&mut ctx).await.unwrap();
    let exists = Users::exists_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap();
    assert!(exists);

    // TEST UPSERT
    let mut user_to_upsert = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    user_to_upsert.status = "banned".into();
    user_to_upsert.upsert(&mut ctx).await.unwrap();
    let check_patch = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_patch.status, "banned");

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
    Users::insert_many(&batch_users, &mut ctx).await.unwrap();

    let count = Users::count(&mut ctx).await.unwrap();
    assert!(count >= 51);

    let all_users = Users::find_all(&mut ctx, 10, 0).await.unwrap();
    assert_eq!(all_users.len(), 10);

    let page = Users::list_by_id_cursor(&mut ctx, Some(user1_id as i64), 5)
        .await
        .unwrap();
    assert!(page.len() <= 5);

    {
        let mut stream = Users::stream_all(&mut ctx);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    let mut item = OrderItems {
        order_id: 200,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&mut ctx).await.unwrap();

    let deleted_users = Users::delete_all(&mut ctx).await.unwrap();
    assert!(deleted_users >= 51);

    // TEST UPSERT
    item.quantity = 25;
    item.upsert(&mut ctx).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&mut ctx, 200, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 25);

    OrderItems::delete_by_order_id_and_product_id(&mut ctx, 200, 99)
        .await
        .unwrap();

    // TEST TRANSACTIONS (ROLLBACK)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx = Users {
        id: 0,
        email: "tx_rollback_mysql@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Rollback".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx.insert(&mut ctx).await.unwrap();
    ctx.rollback_default_tx().await.unwrap();
    let all_users_rollback = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        !all_users_rollback
            .iter()
            .any(|u| u.email == "tx_rollback_mysql@daox.dev")
    );

    // TEST TRANSACTIONS (COMMIT)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx2 = Users {
        id: 0,
        email: "tx_commit_mysql@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Commit".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx2.insert(&mut ctx).await.unwrap();
    ctx.commit_default_tx().await.unwrap();
    let all_users_commit = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        all_users_commit
            .iter()
            .any(|u| u.email == "tx_commit_mysql@daox.dev")
    );

    println!("🎉 TOUS LES TESTS MYSQL ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}

async fn run_sqlite() -> Result<(), sqlx::Error> {
    use futures::StreamExt;
    use models_sqlite::OrderItems;
    use models_sqlite::Users;
    // Note: RequestContext for sqlite needs a SqlitePool!
    use models_sqlite::RequestContext;

    let sqlite_db_path = format!("{}/daox_test.sqlite", env!("OUT_DIR"));
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", sqlite_db_path))
        .await?;
    let mut ctx = RequestContext {
        raw_body: lightx::ext::bytes::Bytes::new(),
        global_state: std::sync::Arc::new(lightx::ext::tokio::sync::broadcast::channel(1).0),
        rate_limiter: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        response_cache: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),
        client_ip: "127.0.0.1".parse().unwrap(),
        user_id: None,
        headers: lightx::ext::hyper::HeaderMap::new(),
        raw_req: None,
        default_pool: pool.clone(),
        default_tx: None,
    };

    println!("🪶 DEMONSTRATION SQLITE");

    sqlx::query("DELETE FROM users").execute(&pool).await?;
    sqlx::query("DELETE FROM order_items")
        .execute(&pool)
        .await?;

    let user1 = Users {
        id: 0,
        email: "alice@sqlite.dev".into(),
        first_name: Some("Alice".into()),
        last_name: "Sqlite".into(),
        status: "active".into(),
        created_at: None,
    };
    let user1_id = user1.insert(&mut ctx).await.unwrap();
    let exists = Users::exists_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap();
    assert!(exists);

    // TEST UPSERT
    let mut user_to_upsert = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    user_to_upsert.status = "inactive".into();
    user_to_upsert.upsert(&mut ctx).await.unwrap();
    let check_patch = Users::get_by_id(&mut ctx, user1_id as i64)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(check_patch.status, "inactive");

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
    Users::insert_many(&batch_users, &mut ctx).await.unwrap();

    let count = Users::count(&mut ctx).await.unwrap();
    assert!(count >= 51);

    let all_users = Users::find_all(&mut ctx, 10, 0).await.unwrap();
    assert_eq!(all_users.len(), 10);

    let page = Users::list_by_id_cursor(&mut ctx, Some(user1_id as i64), 5)
        .await
        .unwrap();
    assert!(page.len() <= 5);

    {
        let mut stream = Users::stream_all(&mut ctx);
        let mut stream_count = 0;
        while stream.next().await.is_some() {
            stream_count += 1;
        }
        assert!(stream_count > 0);
    }

    let mut item = OrderItems {
        order_id: 300,
        product_id: 99,
        quantity: 2,
    };
    item.insert(&mut ctx).await.unwrap();

    let deleted_users = Users::delete_all(&mut ctx).await.unwrap();
    assert!(deleted_users >= 51);

    // TEST UPSERT
    item.quantity = 35;
    item.upsert(&mut ctx).await.unwrap();
    let item_check = OrderItems::get_by_order_id_and_product_id(&mut ctx, 300, 99)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(item_check.quantity, 35);

    OrderItems::delete_by_order_id_and_product_id(&mut ctx, 300, 99)
        .await
        .unwrap();

    // TEST TRANSACTIONS (ROLLBACK)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx = Users {
        id: 0,
        email: "tx_rollback_sqlite@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Rollback".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx.insert(&mut ctx).await.unwrap();
    ctx.rollback_default_tx().await.unwrap();
    let all_users_rollback = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        !all_users_rollback
            .iter()
            .any(|u| u.email == "tx_rollback_sqlite@daox.dev")
    );

    // TEST TRANSACTIONS (COMMIT)
    ctx.get_or_create_default_tx().await.unwrap();
    let user_tx2 = Users {
        id: 0,
        email: "tx_commit_sqlite@daox.dev".into(),
        first_name: Some("Tx".into()),
        last_name: "Commit".into(),
        status: "active".into(),
        created_at: None,
    };
    user_tx2.insert(&mut ctx).await.unwrap();
    ctx.commit_default_tx().await.unwrap();
    let all_users_commit = Users::find_all(&mut ctx, 100, 0).await.unwrap();
    assert!(
        all_users_commit
            .iter()
            .any(|u| u.email == "tx_commit_sqlite@daox.dev")
    );

    println!("🎉 TOUS LES TESTS SQLITE ONT RÉUSSI ! Daox est prêt pour la production.");
    Ok(())
}
