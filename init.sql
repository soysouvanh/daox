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