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

-- Une table classique avec une PK
CREATE TABLE users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    first_name VARCHAR(100),
    last_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Index simple (unique) : pour générer get_by_email, delete_by_email
    UNIQUE INDEX idx_email (email),
    
    -- Index multiple : pour générer update_by_last_name_and_first_name
    INDEX idx_name (last_name, first_name)
);

-- Une vue : ne générera que les fonctions de lecture (get, stream, list...)
CREATE VIEW active_users AS 
SELECT id, email, first_name, last_name 
FROM users 
WHERE status = 'active';

-- Une table de relation (sans PK explicite, mais avec des index)
CREATE TABLE user_roles (
    user_id BIGINT NOT NULL,
    role_name VARCHAR(50) NOT NULL,
    assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    -- Index pour la recherche rapide
    INDEX idx_user_id (user_id),
    
    -- Index unique sur le couple (souvent utilisé comme pseudo-PK dans une table de liaison)
    UNIQUE INDEX idx_user_role (user_id, role_name)
);

-- 1. Clé primaire composite
CREATE TABLE order_items (
    order_id BIGINT NOT NULL,
    product_id BIGINT NOT NULL,
    quantity INT NOT NULL DEFAULT 1,
    PRIMARY KEY (order_id, product_id)
);

-- 2. Mots-clés réservés en Rust
CREATE TABLE configurations (
    id INT AUTO_INCREMENT PRIMARY KEY,
    `type` VARCHAR(50) NOT NULL,
    `match` VARCHAR(255),
    value TEXT
);

-- 3. Types de données complexes
CREATE TABLE product_metadata (
    id BINARY(16) PRIMARY KEY,
    category ENUM('tech', 'food', 'books') NOT NULL,
    attributes JSON,
    raw_data BLOB
);

-- 4. Clé primaire SANS auto-increment
CREATE TABLE currencies (
    code VARCHAR(3) PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- 5. Table exhaustive pour tester tous les types courants
CREATE TABLE comp_types_table (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    f_bool BOOLEAN,
    f_int INT,
    f_float FLOAT,
    f_double DOUBLE,
    f_decimal DECIMAL(10, 2),
    f_varchar VARCHAR(255),
    f_text TEXT
);

CREATE TABLE comp_types_metadata (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    comp_types_id BIGINT NOT NULL,
    f_date DATE,
    f_datetime DATETIME,
    f_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    f_blob BLOB,
    f_json JSON
);

CREATE OR REPLACE VIEW comp_types_view AS 
SELECT t.id, t.f_bool, t.f_int, CAST(t.f_float AS CHAR) AS f_float, CAST(t.f_double AS CHAR) AS f_double, CAST(t.f_decimal AS CHAR) AS f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;

CREATE OR REPLACE VIEW comp_types_active_view AS
SELECT t.id, t.f_varchar, t.f_int, m.f_date
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id
WHERE t.f_int > 0;

DROP TABLE IF EXISTS comp_types_mat_view;
CREATE TABLE comp_types_mat_view AS
SELECT t.id, t.f_bool, t.f_int, CAST(t.f_float AS CHAR) AS f_float, CAST(t.f_double AS CHAR) AS f_double, CAST(t.f_decimal AS CHAR) AS f_decimal, t.f_varchar, t.f_text,
       m.f_date, m.f_datetime, m.f_timestamp, m.f_blob, m.f_json
FROM comp_types_table t
JOIN comp_types_metadata m ON t.id = m.comp_types_id;
