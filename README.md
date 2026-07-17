# Daox Framework 🚀

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** is a highly optimized, zero-overhead, database-first Data Access Object (DAO) generator for Rust.

This repository hosts the entire source code workspace for the Daox ecosystem.

---

## 📁 Workspace Structure

This repository is organized as a Cargo Workspace containing the following directories:

### 1. `daox/` (The Core Library)
This is the heart of the framework. It contains the schema introspection logic and the compile-time code generator.  
👉 **[Read the official Documentation & Quickstart here](./daox/README.md)**

### 2. `daox-test/` (Integration Tests & Demo)
This folder contains the complete test suite and demonstration application to prove the framework's absolute portability. It automatically spins up databases via Docker Compose and runs the generated DAOs against all three supported SQL dialects.

---

## 🛠️ How to run the Integration Tests locally

If you cloned this repository and want to run the test suite, follow these steps:

### 1. Start the Databases
Daox needs live databases to perform its introspection. We provide a `docker-compose.yml` to launch them instantly.

```bash
docker compose up -d
```
*This will spin up a PostgreSQL instance on port 5433 and a MySQL instance on port 3307 to avoid conflicting with your local databases.*

### 2. Run the tests
Navigate into the test directory and run the engine of your choice. The `build.rs` script will automatically connect to the Docker instances, generate the Rust models on the fly, and the `main.rs` application will execute all Use Cases (Upsert, Patch, Streaming, Keyset Pagination).

```bash
cd daox-test

# Test PostgreSQL dialect
cargo run -- postgres

# Test MySQL / MariaDB dialect
cargo run -- mysql

# Test SQLite dialect (creates a local .sqlite file automatically)
cargo run -- sqlite
```

---

## 👤 Author

**Vincent SOYSOUVANH**  
_Skillwaker_

- 🌐 [https://app.skillwaker.com](https://app.skillwaker.com)
- 𝕏 [Twitter / X](https://x.com/skillwaker)

## 📝 License

This project is licensed under the MIT License.
