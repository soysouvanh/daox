# Daox Framework

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** is a highly optimized, zero-overhead, database-first Data Access Object (DAO) generator for Rust.

This repository hosts the entire source code workspace for the Daox ecosystem.

---

## Workspace structure

This repository is organized as a Cargo workspace containing the following directories:

### 1. `daox/` (The core library)

This is the heart of the framework. It contains the schema introspection logic and the compile-time code generator.  
-> **[Read the official Documentation & Quickstart here](./daox/README.md)**

### 2. `daox-test/` (Integration tests & demo)

This folder contains the complete test suite and demonstration application to prove the framework's absolute portability. It automatically spins up databases via Docker Compose and runs the generated DAOs against all three supported SQL dialects.

---

## How to run the integration tests locally

If you cloned this repository and want to run the exhaustive test suite to learn or contribute, follow these accessible steps:

### Step 1: Start the databases

Daox needs live databases to perform its introspection. We provide a clean `docker-compose.yml` to launch them instantly without polluting your local system.

```bash
docker compose up -d
```

_Note: This will spin up a PostgreSQL instance on port 5433 and a MySQL instance on port 3307 to avoid conflicting with your existing local databases._

### Step 2: Run the tests

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

## Author

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## License

This project is licensed under the MIT License.
