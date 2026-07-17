# Daox 🚀

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** is a highly optimized, zero-overhead, database-first Data Access Object (DAO) generator for Rust.

It connects to your database at **compile time** (via `build.rs`), introspects your existing schema, and generates strongly-typed Rust structs and highly efficient asynchronous CRUD methods.

Supports **PostgreSQL**, **MySQL/MariaDB**, and **SQLite**.

---

## 🔥 Why Daox?

Most Rust ORMs (like Diesel or SeaORM) force you to maintain complex Rust macros or schema definitions that must perfectly match your database.

**Daox flips this on its head:**

1. You design your database using pure SQL (Database-first).
2. Daox reads your live database at compile time.
3. Daox generates the Rust boilerplate for you.

### State of the Art (SOTA) Features

- 🚀 **Zero-Overhead:** Powered directly by `sqlx`. No heavy ORM abstractions.
- ⚡ **O(1) Keyset Pagination:** Native Cursor-based pagination (`list_by_cursor`) that destroys `OFFSET` performance bottlenecks.
- 🌊 **Zero-Allocation Streams:** Process millions of rows efficiently via `stream_all()` without loading them into RAM.
- 🩹 **Smart Patching:** Send partial network updates (`update_partial_by_pk`) to save bandwidth and DB disk writes (WAL).
- 🛡️ **Dialect-Aware & Injection Safe:** Fully escapes reserved SQL keywords (`type`, `order`) and uses prepared statements to prevent SQL injections.
- 🗝️ **Composite Keys:** Full support for tables with multiple primary keys.

---

## 🎓 Step-by-Step Guide (For Rust Novices)

> **💡 Note:** If you downloaded the entire workspace source code, you can test the library immediately by navigating to the `daox-test/` directory and typing `cargo run`. 
> 
> The guide below explains how to integrate Daox into **your own** new project.

### Step 1: Create a Project & Configuration (`Cargo.toml`)

First, create a brand new Rust project in your terminal:
```bash
cargo new my_app
cd my_app
```

In Rust, `Cargo.toml` is the configuration file where you declare your project's dependencies.
Daox is unique because it needs to generate code **before** your actual application compiles. Therefore, it is added as a `build-dependency`.

Open your `Cargo.toml` and add the following to pull the library directly from `crates.io`:

```toml
[dependencies]
# SQLx handles the actual database connection at runtime in your app
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "mysql", "postgres", "sqlite"] }
# Futures is required to use Daox's asynchronous streams
futures = "0.3" 

[build-dependencies]
# Daox runs at compile-time to generate your DAO code
daox = "0.1.0"
# Tokio provides the asynchronous runtime needed by Daox to connect to the DB
tokio = { version = "1", features = ["full"] }
```

### Step 2: The Code Generator (`build.rs`)

In Rust, if you create a file named `build.rs` at the root of your project (next to `Cargo.toml`), Cargo will automatically execute it **before** compiling your main code.
This is where Daox shines: it connects to your live database, reads the schema, and creates the Rust files.

Create a `build.rs` file at the root of your project:

```rust
use std::fs;
use std::path::Path;

#[tokio::main] // Required because database connections are asynchronous
async fn main() {
    // 1. Tell Cargo to re-run this script ONLY if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // 2. Define your database URL (make sure your DB is running or exists!)
    // 👉 PostgreSQL: "postgres://user:pass@localhost:5432/my_database"
    // 👉 MySQL:      "mysql://user:pass@localhost:3306/my_database"
    // 👉 SQLite:     "sqlite://my_database.sqlite"
    let db_url = "postgres://user:pass@localhost:5432/my_database";
    
    // 3. Define where Daox should save the generated Rust files
    let output_dir = "src/models";
    if !Path::new(output_dir).exists() { 
        fs::create_dir_all(output_dir).unwrap(); 
    }

    // 4. Connect to the DB, read the schema, and generate the files!
    let generator = daox::DaoxGenerator::new(db_url, output_dir);
    generator.generate().await.expect("Failed to generate DAOs");
}
```

### Step 3: Trigger the Generation (`cargo build`)

Before writing your application code, you need Daox to generate the models.
Open your terminal and run:

```bash
cargo build
```

**What happens here?** 
Cargo runs your `build.rs`. Daox connects to your database, analyzes your tables, and magically creates perfectly typed `.rs` files inside your `src/models/` folder (e.g., `users.rs`).

### Step 4: Your Application (`src/main.rs`)

Now that the DAOs are generated, you can use them in your main application logic. 
Open `src/main.rs` and write your logic using the newly generated files.

*(Note: Don't forget to declare the generated module using `pub mod models;` at the top of your `main.rs` or `lib.rs`)*

```rust
pub mod models; // Tells Rust to include the folder generated by Daox

// Choose the right pool for your engine:
use sqlx::postgres::PgPoolOptions; 
// use sqlx::mysql::MySqlPoolOptions;
// use sqlx::sqlite::SqlitePoolOptions;

use futures::StreamExt; // Required to read data continuously (Streaming)
use models::users::{Users, UsersPatch}; // Import the generated structures

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 1. Open a connection pool to your database
    // 👉 For PostgreSQL:
    let pool = PgPoolOptions::new().connect("postgres://user:pass@localhost:5432/my_database").await?;
    
    // 👉 For MySQL:
    // let pool = MySqlPoolOptions::new().connect("mysql://user:pass@localhost:3306/my_database").await?;
    
    // 👉 For SQLite:
    // let pool = SqlitePoolOptions::new().connect("sqlite://my_database.sqlite").await?;

    // --- CASE 1: CLASSIC INSERTION ---
    let new_user = Users {
        id: 0, // Ignored by Daox if the column is auto-incremented
        email: "alice@daox.dev".into(),
        status: "active".into(),
    };
    let user_id = new_user.insert(&pool).await?;
    println!("User successfully inserted with ID: {}", user_id);

    // --- CASE 2: UPSERT (Insert or Update) ---
    let mut modified_user = new_user.clone();
    modified_user.status = "banned".into();
    // Automatically generates an 'ON CONFLICT DO UPDATE'
    modified_user.upsert(&pool).await?; 

    // --- CASE 3: PARTIAL UPDATE (Patching) ---
    // Update ONLY the status, ignoring the email to save network bandwidth
    let patch = UsersPatch {
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user_id as i64), &patch).await?;

    // --- CASE 4: ZERO-ALLOCATION STREAMING ---
    println!("Reading all users without overloading the RAM...");
    let mut stream = Users::stream_all(&pool);
    while let Some(user_result) = stream.next().await {
        let user = user_result?;
        println!("Found user: {}", user.email);
    }

    Ok(())
}
```

### Step 5: Run Your Code (`cargo run`)

You're done! Execute your program by typing:

```bash
cargo run
```

Your Rust application will compile and interact seamlessly with your database. You should see the following output in your terminal:

```text
User successfully inserted with ID: 1
Reading all users without overloading the RAM...
Found user: alice@daox.dev
```

Congratulations! You have successfully built a highly optimized, type-safe Rust application using Daox.

---

## 🛠️ Generated Methods Overview

For every table, Daox automatically generates a comprehensive set of strongly-typed methods. Below is the detailed API reference.

### 🌍 Global Methods (Table-wide)

- **`count(executor) -> Result<u64>`**  
  Counts the total number of rows in the table. *(Use with caution on very large tables).*
- **`stream_all(executor) -> BoxStream<Result<Self>>`**  
  Creates a zero-allocation asynchronous stream to iterate over the entire table without loading everything into memory. Ideal for massive data exports.
- **`list_paginated(executor, order_by, page, page_size) -> Result<Vec<Self>>`**  
  Classic `OFFSET/LIMIT` pagination. Useful for admin dashboards. Note: `order_by` must be strictly whitelisted by your application to prevent SQL injection.

### ✍️ Write Methods

- **`insert(&self, executor) -> Result<ID>`**  
  Inserts the current struct instance into the database. Automatically returns the newly generated Primary Key (e.g., `i64` for auto-increment columns).
- **`insert_batch(executor, &[Self]) -> Result<u64>`**  
  High-performance mass insertion. Groups all structs into a single massive SQL query (`INSERT INTO ... VALUES (...), (...)`), returning the number of affected rows.
- **`upsert(&self, executor) -> Result<()>`**  
  Inserts the record, or **updates it** if a unique constraint (like a Primary Key or a UNIQUE index) is violated. Highly recommended for data synchronization.

### 🗝️ Primary Key Methods

These methods are strictly bound to your table's Primary Key(s). If your table has composite primary keys, Daox intelligently requires all of them in the method signature (e.g., `&id1, &id2`).

- **`get_by_pk(executor, id) -> Result<Option<Self>>`**  
  Retrieves a single record by its Primary Key. Returns `None` if the record doesn't exist.
- **`exists_by_pk(executor, id) -> Result<bool>`**  
  Ultra-fast verification using `SELECT 1`. Does not load the actual row data.
- **`update_by_pk(&self, executor) -> Result<()>`**  
  Fully replaces the database row with the current struct's data.
- **`update_partial_by_pk(executor, id, &Patch) -> Result<()>`**  
  Optimized partial update. Daox generates a companion `Patch` struct where every field is an `Option<Option<T>>`. Only the exact fields you specify are updated in the SQL query, saving network and I/O.
- **`delete_by_pk(executor, id) -> Result<u64>`**  
  Deletes the specific record and returns the number of affected rows (usually 1).
- **`delete_many_by_pk(executor, &[id]) -> Result<u64>`**  
  Bulk deletion using a powerful `WHERE id IN (?, ?)` clause.
- **`list_by_cursor(executor, last_id, limit) -> Result<Vec<Self>>`**  
  The SOTA standard for API pagination (Keyset Pagination). Scans the B-Tree index directly from `last_id`, offering absolute `O(1)` performance regardless of table size.

### 🔍 Index Methods (Dynamically Generated)

Daox introspects your database indexes and automatically creates specific methods for them. Replace `<index>` with the actual name of your index or column.

- **`exists_by_<index>(executor, cols...) -> Result<bool>`**  
  Fast existence check using the indexed columns.
- **`get_by_<index>(executor, cols...) -> Result<Option<Self>>`** *(Generated only for UNIQUE indexes)*  
  Retrieves a single record since the index guarantees uniqueness (e.g., `get_by_email`).
- **`list_by_<index>(executor, cols...) -> Result<Vec<Self>>`** *(Generated for non-unique indexes)*  
  Retrieves all matching records.
- **`stream_by_<index>(executor, cols...) -> BoxStream<Result<Self>>`** *(Generated for non-unique indexes)*  
  Streams all matching records without allocating RAM.
- **`update_by_<index>(&self, executor) -> Result<()>`**  
  Updates rows that match the index criteria.
- **`delete_by_<index>(executor, cols...) -> Result<u64>`**  
  Deletes all rows matching the index criteria.

---

## 📁 Repository Structure

If you are cloning this repository from GitHub to contribute or run the tests, you will notice a Cargo workspace containing two main folders:

- **`daox/`**: The core generator library. This is the actual package that gets published to crates.io.
- **`daox-test/`**: The integration test suite and demonstration application. It includes a `docker-compose.yml` to spin up PostgreSQL and MySQL instances, and executes the generated DAOs across all three SQL dialects to ensure absolute parity and prevent regressions.

---

## 👤 Author

**Vincent SOYSOUVANH**  
_Skillwaker_

- 🌐 [https://app.skillwaker.com](https://app.skillwaker.com)
  <!-- - 🐙 [GitHub Profile](https://github.com/your-username) -->
- 𝕏 [Twitter / X](https://x.com/skillwaker)

---

## 📝 License

This project is licensed under the MIT License.
