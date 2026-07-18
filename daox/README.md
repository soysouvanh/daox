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

## 🧪 The Official Test & Demo Project

If you prefer learning by example, we highly recommend cloning the GitHub repository and exploring the **`daox-test`** directory. 

It is a complete, ready-to-use demonstration project that includes a `docker-compose.yml` (for PostgreSQL and MySQL) and exhaustively tests every Daox feature across all 3 SQL dialects. It's the perfect playground to learn the framework!

---

## 🎓 Step-by-Step Guide (For Rust Novices)

If you are starting from scratch, this step-by-step guide will walk you through exactly how Daox works, from installation to execution.

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

### Step 2: Prepare Your Database Schema

Because Daox is "Database-first", your database and its tables must exist **before** you compile your code. Daox needs to read them to generate the Rust models.

For this guide, ensure you have a database running and create this simple table (e.g., in PostgreSQL):
```sql
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) DEFAULT 'active'
);
```

### Step 3: The Code Generator (`build.rs`)

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
    
    // 2. Define your database URL (make sure your DB is running and the table exists!)
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

### Step 4: Trigger the Generation (`cargo build`)

Before writing your application code, you need Daox to generate the models.
Open your terminal and run:

```bash
cargo build
```

**What happens here?** 
Cargo runs your `build.rs`. Daox connects to your database, analyzes your `users` table, and magically creates perfectly typed `.rs` files inside your `src/models/` folder (e.g., `users.rs`).

### Step 5: Your Application (`src/main.rs`)

Now that the DAOs are generated, you can use them in your main application logic. 
Open `src/main.rs` and write your logic using the newly generated files.

*(Note: Don't forget to declare the generated module using `pub mod models;` at the top of your file)*

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

### Step 6: Run Your Code (`cargo run`)

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

## 🛠️ Generated Methods Overview (API Reference & Use Cases)

For every table, Daox automatically generates a comprehensive set of strongly-typed methods. Below is the detailed API reference with practical use cases.

### 🌍 Global Methods (Table-wide)

- **`count(executor) -> Result<u64>`**  
  Counts the total number of rows in the table.  
  *Use case:* Displaying the total number of registered users on an admin dashboard. *(Note: Use thoughtfully on very large tables).*
- **`stream_all(executor) -> BoxStream<Result<Self>>`**  
  Creates a zero-allocation asynchronous stream to iterate over the entire table without loading everything into memory.  
  *Use case:* Exporting 5 million users to a CSV file or running a heavy background migration without exploding your server's RAM.
- **`list_paginated(executor, order_by, page, page_size) -> Result<Vec<Self>>`**  
  Classic `OFFSET/LIMIT` pagination.  
  *Use case:* Displaying a traditional data grid table in an internal back-office. Note: `order_by` must be strictly whitelisted by your application to prevent SQL injection.

### ✍️ Write Methods

- **`insert(&self, executor) -> Result<ID>`**  
  Inserts the current struct instance into the database. Automatically returns the newly generated Primary Key (e.g., `i64` for auto-increment columns).  
  *Use case:* Registering a new user who just filled out a signup form.
- **`insert_batch(executor, &[Self]) -> Result<u64>`**  
  High-performance mass insertion. Groups all structs into a single massive SQL query (`INSERT INTO ... VALUES (...), (...)`), returning the number of affected rows.  
  *Use case:* Importing thousands of products from an external API or Excel file in a single database round-trip.
- **`upsert(&self, executor) -> Result<()>`**  
  Inserts the record, or **updates it** if a unique constraint (like a Primary Key or a UNIQUE index) is violated.  
  *Use case:* Synchronizing external data where you don't know if the record already exists in your database or not.

### 🗝️ Primary Key Methods

These methods are strictly bound to your table's Primary Key(s). If your table has composite primary keys, Daox intelligently requires all of them in the method signature (e.g., `&id1, &id2`).

- **`get_by_pk(executor, id) -> Result<Option<Self>>`**  
  Retrieves a single record by its Primary Key. Returns `None` if the record doesn't exist.  
  *Use case:* Fetching a specific user's profile data when they log in.
- **`exists_by_pk(executor, id) -> Result<bool>`**  
  Ultra-fast verification using `SELECT 1`. Does not load the actual row data.  
  *Use case:* Checking if an item is still in the database before processing a payment, without wasting bandwidth downloading all its columns.
- **`update_by_pk(&self, executor) -> Result<()>`**  
  Fully replaces the database row with the current struct's data.  
  *Use case:* Saving a user profile when the user has edited the entire form.
- **`update_partial_by_pk(executor, id, &Patch) -> Result<()>`**  
  Optimized partial update. Daox generates a companion `Patch` struct where every field is an `Option<Option<T>>`. Only the exact fields you specify are updated in the SQL query.  
  *Use case:* Updating *only* the user's `status` to "banned", without sending the `email` or `password_hash` back over the network (saves database I/O and WAL disk writes).
- **`delete_by_pk(executor, id) -> Result<u64>`**  
  Deletes the specific record and returns the number of affected rows (usually 1).  
  *Use case:* A user deleting their account permanently.
- **`delete_many_by_pk(executor, &[id]) -> Result<u64>`**  
  Bulk deletion using a powerful `WHERE id IN (?, ?)` clause.  
  *Use case:* An administrator selecting 50 spam accounts via checkboxes and clicking "Delete All".
- **`list_by_cursor(executor, last_id, limit) -> Result<Vec<Self>>`**  
  The SOTA standard for API pagination (Keyset Pagination). Scans the B-Tree index directly from `last_id`, offering absolute `O(1)` performance regardless of table size.  
  *Use case:* Implementing an "Infinite Scroll" timeline (like Twitter or Facebook) that stays blazing fast even with billions of rows, unlike traditional `OFFSET`.

### 🔍 Index Methods (Dynamically Generated)

Daox introspects your database indexes and automatically creates specific methods for them. Replace `<index>` with the actual name of your index or column.

- **`exists_by_<index>(executor, cols...) -> Result<bool>`**  
  Fast existence check using the indexed columns.  
  *Use case:* Checking if an `email` is already taken during user registration.
- **`get_by_<index>(executor, cols...) -> Result<Option<Self>>`** *(Generated only for UNIQUE indexes)*  
  Retrieves a single record since the index guarantees uniqueness.  
  *Use case:* Finding the user `get_by_email` during the login process.
- **`list_by_<index>(executor, cols...) -> Result<Vec<Self>>`** *(Generated for non-unique indexes)*  
  Retrieves all matching records.  
  *Use case:* Retrieving all orders for a specific `user_id` (`list_by_user_id`).
- **`stream_by_<index>(executor, cols...) -> BoxStream<Result<Self>>`** *(Generated for non-unique indexes)*  
  Streams all matching records without allocating RAM.  
  *Use case:* Processing millions of logs tied to a specific `tenant_id`.
- **`update_by_<index>(&self, executor) -> Result<()>`**  
  Updates rows that match the index criteria.  
  *Use case:* Changing the `status` of all sessions tied to a compromised `user_id`.
- **`delete_by_<index>(executor, cols...) -> Result<u64>`**  
  Deletes all rows matching the index criteria.  
  *Use case:* Deleting all shopping cart items (`delete_by_cart_id`) once the checkout is complete.

---

## 🚀 Advanced Features

### 1. Full Transaction Support
Because every Daox method accepts an `executor` (which implements `sqlx::Executor`), you are not restricted to passing a database pool (`&pool`). You can seamlessly pass a transaction to perform atomic operations:

```rust
let mut tx = pool.begin().await?;

// Both methods will execute within the exact same database transaction
let new_user = Users { id: 0, email: "bob@daox.dev".into(), status: "active".into() };
let user_id = new_user.insert(&mut *tx).await?;

Users::delete_by_pk(&mut *tx, &(user_id as i64)).await?;

tx.commit().await?; // Commit the transaction
```

### 2. Immediate Autocompletion Sync
Daox is purely Database-First. If you use a database migration tool (or manually add a column `phone_number` to your `users` table), simply run `cargo build` again. 
Daox will instantly regenerate the models, and your IDE (VSCode, RustRover) will immediately propose `.phone_number` in its autocomplete suggestions. If a column is deleted or renamed in the database, your Rust code will immediately fail to compile, guaranteeing that your application is always perfectly synchronized with your live database schema.

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
- 𝕏 [Twitter / X](https://x.com/skillwaker)

## 📝 License

This project is licensed under the MIT License.
