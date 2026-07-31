use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Tell Cargo to re-run this build script ONLY if `build.rs` itself changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Re-run if the core generator (`daox`) source code changes.
    println!("cargo:rerun-if-changed=../daox/src");

    // Database connection URLs (running via Docker Compose).
    let mysql_url = "mysql://root:root@localhost:3307/daox_test";
    let pg_url = "postgres://root:root@localhost:5433/daox_test";

    let mysql_out = "src/models_mysql";
    let pg_out = "src/models_pg";

    // Ensure output directories exist.
    if !Path::new(mysql_out).exists() {
        fs::create_dir_all(mysql_out).unwrap();
    }
    if !Path::new(pg_out).exists() {
        fs::create_dir_all(pg_out).unwrap();
    }

    // Generate MySQL DAOs (pure sqlx, zero framework dependency).
    let generator_mysql = daox::DaoxGenerator::new(mysql_url, mysql_out);
    generator_mysql
        .generate()
        .await
        .expect("Failed to generate MySQL DAOs");

    // Generate PostgreSQL DAOs.
    let generator_pg = daox::DaoxGenerator::new(pg_url, pg_out);
    generator_pg
        .generate()
        .await
        .expect("Failed to generate Postgres DAOs");

    // --- SPECIAL HANDLING FOR SQLITE ---
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let sqlite_db_path = format!("{}/daox_test.sqlite", out_dir);
    if !Path::new(&sqlite_db_path).exists() {
        fs::File::create(&sqlite_db_path).unwrap();
    }

    let sqlite_url = format!("sqlite://{}", sqlite_db_path);
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&sqlite_url)
        .await
        .unwrap();

    // Initialize SQLite schema.
    let sqlite_schema: &'static str = include_str!("../init_sqlite.sql");
    for query in sqlite_schema.split(";") {
        let q = query.trim();
        if !q.is_empty() {
            sqlx::Executor::execute(&sqlite_pool, q).await.unwrap();
        }
    }

    let sqlite_out = "src/models_sqlite";
    if !Path::new(sqlite_out).exists() {
        fs::create_dir_all(sqlite_out).unwrap();
    }

    // Generate SQLite DAOs.
    let generator_sqlite = daox::DaoxGenerator::new(&sqlite_url, sqlite_out);
    generator_sqlite
        .generate()
        .await
        .expect("Failed to generate SQLite DAOs");
}
