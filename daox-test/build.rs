use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Tell Cargo to re-run this build script ONLY if `build.rs` itself changes.
    // This prevents unnecessary infinite loops during compilation.
    println!("cargo:rerun-if-changed=build.rs");

    // Tell Cargo to also re-run this script if the core generator (`daox`) source code changes.
    // This ensures that our generated models are always up to date with the latest generator logic.
    println!("cargo:rerun-if-changed=../daox/src");

    // Database connection URLs (these databases are currently running via Docker Compose).
    // The generator needs active connections to introspect the schemas.
    let mysql_url = "mysql://root:root@localhost:3307/daox_test";
    let pg_url = "postgres://root:root@localhost:5433/daox_test";

    let mysql_out = "src/models_mysql";
    let pg_out = "src/models_pg";

    // Ensure the output directories exist before we attempt to write files into them.
    if !Path::new(mysql_out).exists() {
        fs::create_dir_all(mysql_out).unwrap();
    }
    if !Path::new(pg_out).exists() {
        fs::create_dir_all(pg_out).unwrap();
    }

    // Instantiate and run the Daox Generator for MySQL.
    // This connects to the DB, parses the schema, and creates `users.rs`, `order_items.rs`, etc.
    let generator_mysql = daox::DaoxGenerator::new(mysql_url, mysql_out);
    generator_mysql
        .generate()
        .await
        .expect("Failed to generate MySQL DAOs");

    // Repeat the process for PostgreSQL to generate the Postgres-specific syntax models.
    let generator_pg = daox::DaoxGenerator::new(pg_url, pg_out);
    generator_pg
        .generate()
        .await
        .expect("Failed to generate Postgres DAOs");

    // --- SPECIAL HANDLING FOR SQLITE ---
    // Unlike MySQL/PG which run in Docker, SQLite is a local file.
    // We must physically create the file and initialize its schema right here in the build script.

    let sqlite_db_path = "daox_test.sqlite";
    if !Path::new(sqlite_db_path).exists() {
        fs::File::create(sqlite_db_path).unwrap();
    }

    let sqlite_url = format!("sqlite://{}", sqlite_db_path);
    let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect(&sqlite_url)
        .await
        .unwrap();

    // Read the init SQL file and execute each query separated by `;` to build the tables.
    let sqlite_schema = fs::read_to_string("../init_sqlite.sql").unwrap();
    for query in sqlite_schema.split(";") {
        let q = query.trim();
        if !q.is_empty() {
            sqlx::query(q).execute(&sqlite_pool).await.unwrap();
        }
    }

    let sqlite_out = "src/models_sqlite";
    if !Path::new(sqlite_out).exists() {
        fs::create_dir_all(sqlite_out).unwrap();
    }

    // Now that the SQLite database is ready and hydrated with tables, generate its DAOs.
    let generator_sqlite = daox::DaoxGenerator::new(&sqlite_url, sqlite_out);
    generator_sqlite
        .generate()
        .await
        .expect("Failed to generate SQLite DAOs");
}
