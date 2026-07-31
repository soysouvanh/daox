DROP MATERIALIZED VIEW IF EXISTS comp_types_mat_view CASCADE;
DROP VIEW IF EXISTS comp_types_active_view CASCADE;
DROP VIEW IF EXISTS comp_types_view CASCADE;
DROP VIEW IF EXISTS active_users CASCADE;
DROP TABLE IF EXISTS comp_types_metadata CASCADE;
DROP TABLE IF EXISTS comp_types_table CASCADE;
DROP TABLE IF EXISTS currencies CASCADE;
DROP TABLE IF EXISTS product_metadata CASCADE;
DROP TABLE IF EXISTS configurations CASCADE;
DROP TABLE IF EXISTS order_items CASCADE;
DROP TABLE IF EXISTS user_roles CASCADE;
DROP TABLE IF EXISTS users CASCADE;
DROP TYPE IF EXISTS category_enum CASCADE;

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

-- Table exhaustive pour tester tous les types courants
CREATE TABLE comp_types_table (
    id BIGSERIAL PRIMARY KEY,
    f_bool BOOLEAN,
    f_int INT,
    f_float REAL,
    f_double DOUBLE PRECISION,
    f_decimal DECIMAL(10, 2),
    f_varchar VARCHAR(255),
    f_text TEXT
);

CREATE TABLE comp_types_metadata (
    id BIGSERIAL PRIMARY KEY,
    comp_types_id BIGINT NOT NULL,
    f_date DATE,
    f_datetime TIMESTAMP,
    f_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    f_blob BYTEA,
    f_json JSON
);

CREATE VIEW comp_types_view AS 
SELECT t.id, t.f_bool, t.f_int, t.f_float, t.f_double, t.f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;

CREATE VIEW comp_types_active_view AS
SELECT t.id, t.f_varchar, t.f_int, m.f_date
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id
WHERE t.f_int > 0;

CREATE MATERIALIZED VIEW comp_types_mat_view AS
SELECT t.id, t.f_bool, t.f_int, t.f_float, t.f_double, t.f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;