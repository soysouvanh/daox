[English](README.md) | [Français](README.fr.md)

# Daox: The Ultimate Database Tool for Rust

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Welcome to **Daox**! If you are new to programming or Rust, you are in the exact right place.

**Daox** is a highly optimized tool that connects your Rust application to your database automatically.

Instead of writing repetitive and complex code to communicate with your database, Daox connects to it **at compile time**, analyzes your tables, and **writes the exact code you need for you**. This results in an incredibly fast application with zero guesswork.

> **Supported Databases:** PostgreSQL, MySQL/MariaDB, and SQLite.

---

## 🎯 The absolute beginner's step-by-step guide

This guide is designed with "military precision" for extreme clarity. Even if you have minimal technical experience, following these steps strictly will guarantee success.

### Step 0: Prerequisites

Before starting, ensure you have:

1. **Rust installed:** Go to [rustup.rs](https://rustup.rs/) and follow the instructions to install Rust on your computer.
2. **A Database running:** We will use PostgreSQL in this example. If you have Docker installed, you can start one by running:
   `docker run --name my-postgres -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres`

---

### Step 1: Create your new project

Open your terminal (Command Prompt, PowerShell, or bash) and run these exact commands to create a new Rust project:

```bash
# This creates a new folder called 'my_app' containing a blank Rust project
cargo new my_app

# Enter the new folder
cd my_app
```

---

### Step 2: Add Required Dependencies

Rust uses a file called `Cargo.toml` to manage tools and libraries (dependencies).
Open the `Cargo.toml` file in your `my_app` folder using any text editor.

Update it to look exactly like this:

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
# sqlx is the tool that allows connecting to the database when your app is running
sqlx = { version = "0.9", features = ["runtime-tokio", "tls-rustls", "postgres"] }
# futures handles streams of data efficiently
futures = "0.3"
# tokio provides the asynchronous environment needed for the app to run
tokio = { version = "1", features = ["full"] }

[build-dependencies]
# Daox is our magical tool that generates code for you before the app starts
daox = "0.2.5"
# tokio is also needed for the code generation script
tokio = { version = "1", features = ["full"] }
```

---

### Step 3: Prepare your Database

Daox is "Database-first". This means your tables must exist in your database _before_ Daox can generate code for them.

Connect to your PostgreSQL database (using your preferred database client like DBeaver, pgAdmin, or psql) and run this exact SQL command to create a `users` table:

```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) DEFAULT 'active'
);
```

---

### Step 4: Set up the Code Generator (`build.rs`)

We need a script that tells Daox to inspect the database and write the Rust code for it.

Create a new file named `build.rs` at the exact root of your project (in the `my_app` folder, exactly next to `Cargo.toml`).

Copy and paste this exact code into `build.rs`:

```rust
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // 1. Tell Rust to only re-run this script if the script itself changes
    println!("cargo:rerun-if-changed=build.rs");

    // 2. Define your exact Database URL.
    // Format: postgres://[username]:[password]@[host]:[port]/[database_name]
    // CHANGE THIS URL to match your database settings if they are different!
    let db_url = "postgres://postgres:password@localhost:5432/postgres";

    // 3. Define where Daox will save the generated code
    let output_dir = "src";

    // 4. Start Daox! It will read your database and generate the Rust files automatically.
    let generator = daox::DaoxGenerator::new(db_url, output_dir);
    generator.generate().await.expect("CRITICAL ERROR: Failed to generate models. Check your Database URL and connection.");
}
```

---

### Step 5: Write your Application Code (`src/main.rs`)

Daox will perfectly generate the code inside the `src/daox_generated.rs` file. Let's use it!
Open the `src/main.rs` file, delete everything inside, and replace it with this exact code:

```rust
// 1. Tell Rust to include the file Daox just generated for us
pub mod daox_generated;

use sqlx::postgres::PgPoolOptions;
use futures::StreamExt; // Required to read many users efficiently
use daox_generated::Users; // Import the generated 'Users' object

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // 2. Connect to the database. Make sure this URL matches the one in build.rs!
    let db_url = "postgres://postgres:password@localhost:5432/postgres";
    let pool = PgPoolOptions::new().connect(db_url).await?;

    println!("Successfully connected to the database!");

    // 3. CREATE A NEW USER
    let new_user = Users {
        id: 0, // '0' is ignored because PostgreSQL auto-generates the ID
        email: "hello@daox.dev".into(),
        status: Some("active".to_string()),
    };

    // Save the user in the database
    let user_id = new_user.insert(&pool).await?;
    println!("SUCCESS: Inserted New User with ID: {}", user_id);

    // 4. UPDATE THE USER (Smart Patching)
    // We update ONLY the status column without overriding the rest of the data
    Users::update_partial_by_id(&pool, user_id as i64, &daox_generated::UsersPatch {
        status: Some(Some("inactive".to_string())),
        ..Default::default()
    }).await?;
    println!("SUCCESS: User status updated to 'inactive'");

    // 5. READ ALL USERS (Streaming)
    // Daox streams data row-by-row, so it never overloads your computer's memory
    let mut stream = Users::stream_all(&pool);
    println!("--- Listing all users in database ---");
    while let Some(user_result) = stream.next().await {
        let user = user_result?;
        println!("User: ID {}, Email {}, Status {:?}", user.id, user.email, user.status);
    }

    Ok(())
}
```

---

### Step 6: Compile and Run!

You are ready. Go back to your terminal (make sure you are inside the `my_app` folder) and run your program:

```bash
cargo run
```

**What happens exactly?**

1. The `build.rs` script runs first. Daox connects to your database, reads the `users` table layout, and automatically writes perfect Rust code into the `src/daox_generated.rs` file.
2. `src/main.rs` uses that freshly generated code.
3. The program saves a user to the database, updates their status, and prints all users to the screen.

**Congratulations! You have successfully mastered Daox.**

---

## 🚀 Advanced Architecture & Daox Capabilities

If you are a technical user, here is why Daox establishes the State OF The Art (SOTA) in Rust ORMs.

### The Database-First Paradigm

Most traditional ORMs (like Diesel or SeaORM) force you to manually define Rust macros or structs, which you must carefully maintain to match your database. **Daox flips this paradigm.**

Here is the Database-first approach visualized:

![Architecture & data lifecycle](./assets/architecture.svg)

### State of the Art (SOTA) Features

- **Zero-overhead:** Powered directly by `sqlx`. No heavy ORM abstractions are loaded at runtime.
- **O(1) keyset pagination:** Native Cursor-based pagination (`list_by_cursor`) that destroys `OFFSET` performance bottlenecks.

![O(1) keyset pagination vs OFFSET](./assets/pagination.svg)

- **Zero-allocation streams:** Process millions of rows efficiently via `stream_all()` without loading them into RAM.

![Zero-allocation streams](./assets/streams.svg)

- **SOTA Batch Operations & Upserts:** Deeply integrated, cross-dialect native `upserts` (using `ON CONFLICT` for PG/SQLite and `ON DUPLICATE KEY` for MySQL) ensuring scalable ACID atomicity effortlessly via `insert_batch`.
- **Smart patching:** Send partial network updates (`update_partial_by_pk`) to save bandwidth and reduce database disk writes (WAL).
- **Dialect-aware & injection safe:** Fully escapes reserved SQL keywords dynamically parsing context via `databases.toml`.
- **Runtime Formats Validation:** Enforces data integrity in bulk inserts and updates by analyzing table schemas (using constraints like `min_length`, `enum`, regex, NaN/Infinity rejection). Call `.validate()` manually.
- **Composite keys and Secondary Indexes:** Native support with highly scalable batch generation bindings for multiple primary keys, automatic `get_by_{index}`.
- **O(1) Table counting:** Call `estimated_count_upper_bound()` to return an approximate table count using blazing fast internal database statistics (`MAX(rowid)`, `pg_class`, `information_schema.tables`).

### Advanced Interaction Models

#### Robust Transactions

Every generated Daox method inherently accepts an `executor` trait. You can easily perform transaction-grouped atomic operations:

```rust
let mut tx = pool.begin().await?;

// Executes cleanly under the same transaction envelope
let user_id = new_user.insert(&mut *tx).await?;
Users::delete_by_id(&mut *tx, user_id as i64).await?;

tx.commit().await?; // Commit database changes
```

#### Immediate IDE Typings / Sync

If you rename a column in your database, simply re-run `cargo build`. Daox updates `src/daox_generated.rs` instantly. Code that uses the old column name will immediately **fail to compile**, securing your application structure flawlessly.

---

## The Official Test & Demo Project

To learn more by reading code, clone the GitHub repository and explore the **`daox-test`** directory. It exhaustively tests every Daox feature across all 3 SQL dialects using Docker, serving as the ultimate playground to learn advanced Daox configurations.

---

## Author

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## License

This project is licensed under the MIT License.
