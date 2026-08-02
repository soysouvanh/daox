use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Tell Cargo to re-run this build script ONLY if `build.rs` itself changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Re-run if the core generator (`daox`) source code changes.
    println!("cargo:rerun-if-changed=../daox/src");

    // Re-run if schema files change
    println!("cargo:rerun-if-changed=../.daox_schema");
    println!("cargo:rerun-if-changed=../overrides");

    // Database connection URLs
    let mysql_url = std::env::var("DATABASE_URL_MYSQL")
        .expect("DATABASE_URL_MYSQL is required for Daox code generation");
    let pg_url = std::env::var("DATABASE_URL_PG")
        .expect("DATABASE_URL_PG is required for Daox code generation");

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
    let generator_mysql = daox::DaoxGenerator::new(&mysql_url, mysql_out);
    generator_mysql
        .generate()
        .await
        .expect("Failed to generate MySQL DAOs");

    // Generate PostgreSQL DAOs.
    let generator_pg = daox::DaoxGenerator::new(&pg_url, pg_out);
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
    let mut queries = Vec::new();
    let mut in_string = false;
    let mut in_line_comment = false;
    let mut start = 0;
    for (i, c) in sqlite_schema.char_indices() {
        if in_line_comment {
            if c == '\n' {
                in_line_comment = false;
            }
            continue;
        }

        if !in_string && c == '-' {
            if sqlite_schema[i..].starts_with("--") {
                in_line_comment = true;
                continue;
            }
        }

        if c == '\'' {
            in_string = !in_string;
        }

        if c == ';' && !in_string {
            let q = sqlite_schema[start..i].trim();
            if !q.is_empty() {
                queries.push(q);
            }
            start = i + 1;
        }
    }
    let q = sqlite_schema[start..].trim();
    if !q.is_empty() {
        queries.push(q);
    }

    for (i, q) in queries.into_iter().enumerate() {
        sqlx::Executor::execute(&sqlite_pool, q)
            .await
            .unwrap_or_else(|e| panic!("Query {} failed: {}\n{}", i, e, q));
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
