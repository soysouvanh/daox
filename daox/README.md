# Daox

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** is a highly optimized, zero-overhead, database-first Data Access Object (DAO) generator for Rust.

By connecting to your database at **compile time** (via `build.rs`), Daox introspects your existing schema and generates strongly-typed Rust structures alongside highly efficient, asynchronous CRUD methods.

> **Supported Databases:** PostgreSQL, MySQL/MariaDB, and SQLite.

---

## Architecture & data lifecycle

Most traditional ORMs (like Diesel or SeaORM) force you to manually define Rust macros or structs, which you must carefully maintain to match your database. **Daox flips this paradigm.**

Here is the **Database-first** approach visualized:

![Architecture & data lifecycle](./assets/architecture.svg)

### Why Daox? (State of the art features)

- **Zero-overhead:** Powered directly by `sqlx`. No heavy ORM abstractions are loaded at runtime.
- **O(1) keyset pagination:** Native Cursor-based pagination (`list_by_cursor`) that destroys `OFFSET` performance bottlenecks.

![O(1) keyset pagination vs OFFSET](./assets/pagination.svg)

- **Zero-allocation streams:** Process millions of rows efficiently via `stream_all()` without loading them into RAM.

![Zero-allocation streams](./assets/streams.svg)

- **Smart patching:** Send partial network updates (`update_partial_by_pk`) to save bandwidth and reduce database disk writes (WAL).
- **Dialect-aware & injection safe:** Fully escapes reserved SQL keywords and uses prepared statements to strictly prevent SQL injections.
- **Composite keys:** Full native support for tables with multiple primary keys.

---

## The official test & demo project

If you prefer learning by reading code, we highly recommend cloning the GitHub repository and exploring the **`daox-test`** directory.

It is a complete, ready-to-use demonstration project that includes a `docker-compose.yml` (for PostgreSQL and MySQL) and exhaustively tests every Daox feature across all 3 SQL dialects. It is the perfect playground to safely learn the framework!

---

## Step-by-step guide (for Rust novices)

This step-by-step guide walks you through the entire Daox workflow, from installation to execution.

### Step 1: Create a project & configuration

First, create a brand new Rust project in your terminal:

```bash
cargo new my_app
cd my_app
```

In Rust, `Cargo.toml` is the configuration file where you declare dependencies. Since Daox generates code **before** your application compiles, it is added as a `build-dependency`.

Open your `Cargo.toml` and add the following:

```toml
[dependencies]
# sqlx handles the actual database connection at runtime
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "mysql", "postgres", "sqlite"] }
# futures is required to handle Daox's asynchronous data streams
futures = "0.3"

[build-dependencies]
# Daox runs at compile-time to generate your DAO code
daox = "0.1.0"
# Tokio provides the asynchronous runtime needed by Daox during generation
tokio = { version = "1", features = ["full"] }
```

### Step 2: Prepare your database schema

Because Daox is "Database-first", your database and tables must exist **before** compilation. Ensure you have a running database and create this simple table (example for PostgreSQL):

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) DEFAULT 'active'
);
```

### Step 3: The code generator (`build.rs`)

If you create a file named `build.rs` at the root of your project, Cargo automatically executes it before compiling the rest of your code. We use this file to trigger Daox.

Create `build.rs` at the root:

```rust
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // 1. Tell Cargo to trigger recompilation only if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // 2. Define your database URL
    let db_url = "postgres://user:pass@localhost:5432/my_database";

    // 3. Define where Daox should output the generated Rust files
    let output_dir = "src/models";
    if !Path::new(output_dir).exists() {
        fs::create_dir_all(output_dir).unwrap();
    }

    // 4. Connect to the DB, read the schema, and generate!
    let generator = daox::DaoxGenerator::new(db_url, output_dir);
    generator.generate().await.expect("Failed to generate DAOs");
}
```

### Step 4: Trigger the generation

Generate the models by compiling your project in the terminal:

```bash
cargo build
```

> **What happens here?** Cargo runs `build.rs`. Daox connects to your database, deeply analyzes your `users` table, and cleanly generates perfectly typed files inside your `src/models/` folder.

### Step 5: Your application (`src/main.rs`)

Everything is ready! Import the generated modules and use them in your main logic.

```rust
pub mod models; // Explicitly include the generated module

use sqlx::postgres::PgPoolOptions;
use futures::StreamExt; // Required for continuous data streams
use models::users::{Users, UsersPatch}; // Import the generated structures

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 1. Open a connection pool to your database
    let pool = PgPoolOptions::new().connect("postgres://user:pass@localhost:5432/my_database").await?;

    // --- CASE 1: CLASSIC INSERTION ---
    let new_user = Users {
        id: 0, // Automatically ignored by Daox for auto-incremented columns
        email: "alice@daox.dev".into(),
        status: "active".into(),
    };
    let user_id = new_user.insert(&pool).await?;
    println!("Inserted User ID: {}", user_id);

    // --- CASE 2: UPSERT (Insert or Update on conflict) ---
    let mut modified_user = new_user.clone();
    modified_user.status = "banned".into();
    modified_user.upsert(&pool).await?;

    // --- CASE 3: SMART PATCHING (Partial Update) ---
    // Update *only* the status. Daox will ignore the email field.
    let patch = UsersPatch {
        status: Some("inactive".into()),
        ..Default::default()
    };
    Users::update_partial_by_pk(&pool, &(user_id as i64), &patch).await?;

    // --- CASE 4: ZERO-ALLOCATION STREAMING ---
    // Iterates cleanly without overflowing the system's memory
    let mut stream = Users::stream_all(&pool);
    while let Some(user_result) = stream.next().await {
        let user = user_result?;
        println!("Found user: {}", user.email);
    }

    Ok(())
}
```

### Step 6: Run your code

Run your application:

```bash
cargo run
```

---

## Generated methods overview & use cases

Daox understands your schema intimately. Based on your columns and indexes, it generates dedicated, structurally sound methods.

### Global table methods

- **`count(executor)`** ➔ Total table row count. _(Ideal for admin KPIs)_
- **`stream_all(executor)`** ➔ Zero-allocation row streaming. _(Crucial for exporting Big Data or background migrations)_
- **`list_paginated(executor, order, page, size)`** ➔ Traditional pagination. _(For internal data-grids)_

### Write methods

- **`insert(&self, executor)`** ➔ Insert the struct and get the generated ID.
- **`insert_batch(executor, &[Self])`** ➔ High-performance mass ingestion.
- **`upsert(&self, executor)`** ➔ Insert, or powerfully update if a unique conflict occurs.

### Primary key (PK) driven methods

- **`get_by_pk(executor, pk)`** ➔ Fetch one precise record.
- **`exists_by_pk(executor, pk)`** ➔ Ultra-fast cache-friendly verify without pulling the full row.
- **`update_by_pk(&self, executor)`** ➔ Fully overwrite the database record.
- **`update_partial_by_pk(executor, pk, &Patch)`** ➔ Efficient partial field patching (saves WAL storage!).
- **`delete_by_pk(executor, pk)`** ➔ Single record deletion.
- **`delete_many_by_pk(executor, &[pk])`** ➔ Scalable mass deletion.
- **`list_by_cursor(executor, last_id, limit)`** ➔ The SOTA **O(1) keyset pagination** for infinite scrolling feeds.

### Auto-generated index methods

Daox scans your DB indexes and maps perfectly optimized query methods.
_(For example, an index on `email`)_

- **`exists_by_<index>`** ➔ Example: **`exists_by_email`**
- **`get_by_<index>`** ➔ _(For UNIQUE indexes)_ Example: **`get_by_email`**
- **`list_by_<index>`** ➔ _(For standard indexes)_ Retrieve multiple matches.
- **`stream_by_<index>`** ➔ Stream matches safely.
- **`delete_by_<index>`** ➔ Efficient targeted deletion based on an index.

---

## Advanced interaction models

### 1. Robust transactions

Every generated Daox method inherently accepts an `executor` trait. You are not forced to pass the standard `&pool`; you can easily perform transaction-grouped atomic operations:

```rust
let mut tx = pool.begin().await?;

// Executes cleanly under the same transaction envelope
let user_id = new_user.insert(&mut *tx).await?;
Users::delete_by_pk(&mut *tx, &(user_id as i64)).await?;

tx.commit().await?; // Commit database changes
```

### 2. Immediate IDE typings / sync

By integrating with `cargo build`, Daox ensures your Rust models match your database schema 1:1.

If you rename a column, the next `cargo build` updates `src/models/*.rs` instantly. Code that uses the old column name will immediately **fail to compile**. This guarantees maximum robustness—your typed properties are always the single source of truth for the database layout.

---

## Repository structure

- **`daox/`**: The core framework code targeting `crates.io`.
- **`daox-test/`**: The deep integration suite and learning playground testing identical behaviors across MySQL, Postgres, and SQLite simultaneously via Docker.

---

## Author

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## License

This project is licensed under the MIT License.
