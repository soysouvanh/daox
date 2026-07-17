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
