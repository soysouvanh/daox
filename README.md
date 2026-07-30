[English](README.md) | [Français](README.fr.md)

# Daox Framework

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** is a highly optimized, zero-overhead, multi-database Data Access Object (DAO) generator for Rust.

This repository hosts the entire source code workspace for the Daox ecosystem.

---

## Workspace structure

Here is how the Daox ecosystem is organized as a Cargo workspace:

```text
daox-workspace/
├── docker-compose.yml      # DB automation config (PostgreSQL, MySQL)
├── init_*.sql              # Base schemas auto-loaded by Docker
├── daox/                   # The core framework (published on crates.io)
│   ├── src/                # Dialect builders, AOP parsers, macro logic
│   └── assets/             # Explanatory SVG architecture diagrams
│
└── daox-test/              # Integration tests & Living documentation
    ├── src/models/         # Auto-generated DAOs (by build.rs)
    └── src/main.rs         # Demonstration workflows (Streams, Pagination, Patching)
```

-> **[Read the official Documentation & Quickstart here](./daox/README.md)**

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
DAOX_TEST_PG=1 cargo run

# Test MySQL / MariaDB dialect (Silently deployed by default)
cargo run

# Test SQLite dialect (runs securely isolated within target/ OUT_DIR)
DAOX_TEST_SQLITE=1 cargo run
```

---

## Author

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## License

This project is licensed under the MIT License.
