CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_email ON users (email);
CREATE INDEX idx_name ON users (last_name, first_name);

CREATE VIEW active_users AS 
SELECT id, email, first_name, last_name 
FROM users 
WHERE status = 'active';

CREATE TABLE user_roles (
    user_id BIGINT NOT NULL,
    role_name VARCHAR(50) NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX idx_user_id ON user_roles (user_id);
CREATE UNIQUE INDEX idx_user_role ON user_roles (user_id, role_name);

CREATE TABLE order_items (
    order_id BIGINT NOT NULL,
    product_id BIGINT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    PRIMARY KEY (order_id, product_id)
);

CREATE TABLE configurations (
    id SERIAL PRIMARY KEY,
    "type" VARCHAR(50) NOT NULL,
    "match" VARCHAR(255),
    value TEXT
);

CREATE TYPE category_enum AS ENUM ('tech', 'food', 'books');
CREATE TABLE product_metadata (
    id UUID PRIMARY KEY,
    category category_enum NOT NULL,
    attributes JSON,
    raw_data BYTEA
);

CREATE TABLE currencies (
    code VARCHAR(3) PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);