DROP VIEW IF EXISTS comp_types_active_view;
DROP VIEW IF EXISTS comp_types_view;
DROP VIEW IF EXISTS active_users;
DROP TABLE IF EXISTS comp_types_mat_view;
DROP TABLE IF EXISTS comp_types_metadata;
DROP TABLE IF EXISTS comp_types_table;
DROP TABLE IF EXISTS currencies;
DROP TABLE IF EXISTS product_metadata;
DROP TABLE IF EXISTS configurations;
DROP TABLE IF EXISTS order_items;
DROP TABLE IF EXISTS user_roles;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email VARCHAR(255) NOT NULL UNIQUE,
    first_name VARCHAR(100),
    last_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_name ON users (last_name, first_name);

CREATE VIEW IF NOT EXISTS active_users AS 
SELECT id, email, first_name, last_name 
FROM users 
WHERE status = 'active';

CREATE TABLE IF NOT EXISTS user_roles (
    user_id BIGINT NOT NULL,
    role_name VARCHAR(50) NOT NULL,
    assigned_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_user_id ON user_roles (user_id);
CREATE UNIQUE INDEX IF NOT EXISTS idx_user_role ON user_roles (user_id, role_name);

CREATE TABLE IF NOT EXISTS order_items (
    order_id BIGINT NOT NULL,
    product_id BIGINT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    PRIMARY KEY (order_id, product_id)
);

CREATE TABLE IF NOT EXISTS configurations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    "type" VARCHAR(50) NOT NULL,
    "match" VARCHAR(255),
    value TEXT
);

CREATE TABLE IF NOT EXISTS product_metadata (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    attributes TEXT,
    raw_data BLOB
);

CREATE TABLE IF NOT EXISTS currencies (
    code VARCHAR(3) PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- Table exhaustive pour tester tous les types courants
CREATE TABLE IF NOT EXISTS comp_types_table (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    f_bool BOOLEAN,
    f_int INTEGER,
    f_float REAL,
    f_double REAL,
    f_decimal DECIMAL(10, 2),
    f_varchar VARCHAR(255),
    f_text TEXT
);

CREATE TABLE IF NOT EXISTS comp_types_metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    comp_types_id INTEGER NOT NULL,
    f_date DATE,
    f_datetime DATETIME,
    f_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    f_blob BLOB,
    f_json TEXT
);

CREATE VIEW IF NOT EXISTS comp_types_view AS 
SELECT t.id, t.f_bool, t.f_int, t.f_float, t.f_double, t.f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;

CREATE VIEW IF NOT EXISTS comp_types_active_view AS
SELECT t.id, t.f_varchar, t.f_int, m.f_date
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id
WHERE t.f_int > 0;

-- SQLite ne supporte pas nativement CREATE MATERIALIZED VIEW
-- On simule donc une vue matérialisée avec une table
CREATE TABLE IF NOT EXISTS comp_types_mat_view AS
SELECT t.id, t.f_bool, t.f_int, t.f_float, t.f_double, t.f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;
