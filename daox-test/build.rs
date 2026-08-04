use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tell Cargo to re-run this build script ONLY if `build.rs` itself changes.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../.env");

    if let Ok(content) = std::fs::read_to_string("../.env") {
        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                if let Some((k, v)) = line.split_once('=') {
                    unsafe {
                        std::env::set_var(k.trim(), v.trim().trim_matches('"'));
                    }
                }
            }
        }
    }

    // Re-run if the core generator (`daox`) source code changes.
    println!("cargo:rerun-if-changed=../daox/src");

    // Re-run if schema files change
    println!("cargo:rerun-if-changed=src/models_mysql/.daox_schema");
    println!("cargo:rerun-if-changed=src/models_mysql/overrides");
    println!("cargo:rerun-if-changed=src/models_pg/.daox_schema");
    println!("cargo:rerun-if-changed=src/models_pg/overrides");
    println!("cargo:rerun-if-changed=src/models_sqlite/.daox_schema");
    println!("cargo:rerun-if-changed=src/models_sqlite/overrides");

    // Database connection URLs
    let mysql_url = std::env::var("DATABASE_URL_MYSQL").map_err(|e| {
        format!(
            "DATABASE_URL_MYSQL is required for Daox code generation: {}",
            e
        )
    })?;
    let pg_url = std::env::var("DATABASE_URL_PG").map_err(|e| {
        format!(
            "DATABASE_URL_PG is required for Daox code generation: {}",
            e
        )
    })?;

    let mysql_out = "src/models_mysql";
    let pg_out = "src/models_pg";

    // Ensure output directories exist.
    if !Path::new(mysql_out).exists() {
        fs::create_dir_all(mysql_out)?;
    }
    if !Path::new(pg_out).exists() {
        fs::create_dir_all(pg_out)?;
    }

    // Generate MySQL DAOs (pure sqlx, zero framework dependency).
    let generator_mysql = daox::DaoxGenerator::new(&mysql_url, mysql_out);
    generator_mysql
        .generate()
        .await
        .map_err(|e| format!("Failed to generate MySQL DAOs: {}", e))?;

    // Generate PostgreSQL DAOs.
    let generator_pg = daox::DaoxGenerator::new(&pg_url, pg_out);
    generator_pg
        .generate()
        .await
        .map_err(|e| format!("Failed to generate Postgres DAOs: {}", e))?;

    // --- SPECIAL HANDLING FOR SQLITE ---
    let out_dir = std::env::var("OUT_DIR")?;
    let sqlite_db_path = format!("{}/daox_test.sqlite", out_dir);
    if !Path::new(&sqlite_db_path).exists() {
        fs::File::create(&sqlite_db_path)?;
    }

    let sqlite_url = format!("sqlite://{}", sqlite_db_path);
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&sqlite_url)
        .await?;

    // Initialize SQLite schema.
    let sqlite_schema: &'static str = include_str!("../init_sqlite.sql");
    sqlx::Executor::execute(&sqlite_pool, sqlite_schema)
        .await
        .map_err(|e| format!("SQLite init failed: {}", e))?;

    let sqlite_out = "src/models_sqlite";
    if !Path::new(sqlite_out).exists() {
        fs::create_dir_all(sqlite_out)?;
    }

    // Generate SQLite DAOs.
    let generator_sqlite = daox::DaoxGenerator::new(&sqlite_url, sqlite_out);
    generator_sqlite
        .generate()
        .await
        .map_err(|e| format!("Failed to generate SQLite DAOs: {}", e))?;

    Ok(())
}
