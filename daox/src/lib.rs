use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, Row};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use heck::{AsPascalCase, AsSnakeCase};

// --- STRUCTURES DE MÉTADONNÉES ---

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary: bool,
    pub is_auto_increment: bool, // <-- NOUVEAU
}

#[derive(Debug, Clone)]
pub struct Index {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub is_view: bool,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
}

// --- GÉNÉRATEUR ---

pub struct DaoxGenerator {
    database_url: String,
    output_dir: String,
}

impl DaoxGenerator {
    pub fn new(database_url: &str, output_dir: &str) -> Self {
        Self {
            database_url: database_url.to_string(),
            output_dir: output_dir.to_string(),
        }
    }

    pub async fn generate(&self) -> Result<()> {
        println!("cargo:warning=🚀 Démarrage de la génération daox...");
        
        let pool = MySqlPoolOptions::new()
            .max_connections(2)
            .connect(&self.database_url)
            .await
            .context("Impossible de se connecter à la BDD. Docker tourne-t-il ?")?;

        let db_name = self.database_url.rsplit('/').next()
            .unwrap_or("")
            .split('?')
            .next()
            .unwrap_or("");

        let tables = self.introspect_mysql(&pool, db_name).await?;

        // S'assurer que le dossier de sortie existe
        fs::create_dir_all(&self.output_dir)?;

        let mut mod_imports = Vec::new();

        for table in &tables {
            let struct_name = format!("{}", AsPascalCase(&table.name));
            let file_name = format!("{}.rs", AsSnakeCase(&table.name));
            let file_path = Path::new(&self.output_dir).join(&file_name);

            // On garde en mémoire pour générer le `mod.rs` à la fin
            mod_imports.push(AsSnakeCase(&table.name).to_string());

            println!("cargo:warning=✍️ Génération du modèle : {} -> {}", table.name, file_name);

            // Construction du code Rust pour la structure (Model)
            let mut code = String::new();
            code.push_str("// Code généré automatiquement par daox. NE PAS MODIFIER.\n\n");
            code.push_str("#[derive(Debug, Clone, sqlx::FromRow)]\n");
            code.push_str(&format!("pub struct {} {{\n", struct_name));

            for col in &table.columns {
                let rust_type = map_mysql_type(&col.data_type, col.is_nullable);
                let snake_col_name = format!("{}", AsSnakeCase(&col.name));
                let rust_field = escape_rust_keyword(&snake_col_name);
                code.push_str(&format!("    pub {}: {},\n", rust_field, rust_type));
            }
            code.push_str("}\n\n");

            // --- NOUVEAU : AJOUT DES LECTURES ---
            // S'applique aux TABLES et aux VUES de manière égale !
            code.push_str(&generate_read_methods(table));

            // Les méthodes d'écriture ne s'appliquent QUE pour les Tables
            if !table.is_view {
                code.push_str(&generate_write_methods(table));
                
                // --- NOUVEAU : STRUCTURE ET FONCTION PATCH ---
                code.push_str(&generate_patch_struct(table));
                code.push_str(&generate_partial_update_method(table));
            }

            // Écriture physique du fichier
            let mut file = File::create(file_path)?;
            file.write_all(code.as_bytes())?;
        }

        // Générer le fichier central `mod.rs` pour lier tous les fichiers générés
        let mut mod_code = String::new();
        mod_code.push_str("// Code généré automatiquement par daox. NE PAS MODIFIER.\n\n");
        for import in mod_imports {
            mod_code.push_str(&format!("pub mod {};\n", import));
        }
        
        let mod_path = Path::new(&self.output_dir).join("mod.rs");
        let mut mod_file = File::create(mod_path)?;
        mod_file.write_all(mod_code.as_bytes())?;

        println!("cargo:warning=✅ Génération terminée avec succès !");
        Ok(())
    }

    // --- LOGIQUE D'INTROSPECTION MYSQL / MARIADB ---

    async fn introspect_mysql(&self, pool: &sqlx::MySqlPool, db_name: &str) -> Result<Vec<Table>> {
        let mut tables = Vec::new();

        // A. Récupérer les tables et vues
        let rows = sqlx::query(
            "SELECT TABLE_NAME, TABLE_TYPE 
             FROM information_schema.TABLES 
             WHERE TABLE_SCHEMA = ?"
        )
        .bind(db_name)
        .fetch_all(pool)
        .await?;

        for row in rows {
            let table_name: String = row.get("TABLE_NAME");
            let table_type: String = row.get("TABLE_TYPE");
            let is_view = table_type == "VIEW";

            // B. Récupérer les colonnes pour cette table
            let col_rows = sqlx::query(
                "SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY, EXTRA 
                 FROM information_schema.COLUMNS 
                 WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? 
                 ORDER BY ORDINAL_POSITION"
            )
            .bind(db_name)
            .bind(&table_name)
            .fetch_all(pool)
            .await?;

            let mut columns = Vec::new();
            for col in col_rows {
                let extra: String = col.get("EXTRA"); // <-- NOUVEAU
                columns.push(Column {
                    name: col.get("COLUMN_NAME"),
                    data_type: col.get("DATA_TYPE"),
                    is_nullable: col.get::<String, _>("IS_NULLABLE") == "YES",
                    is_primary: col.get::<String, _>("COLUMN_KEY") == "PRI",
                    is_auto_increment: extra.to_lowercase().contains("auto_increment"), // <-- NOUVEAU
                });
            }

            // C. Récupérer les index (uniquement pour les tables, les vues n'en ont pas)
            let mut indexes: Vec<Index> = Vec::new();
            if !is_view {
                let idx_rows = sqlx::query(
                    "SELECT INDEX_NAME, COLUMN_NAME, NON_UNIQUE 
                     FROM information_schema.STATISTICS 
                     WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? 
                     ORDER BY INDEX_NAME, SEQ_IN_INDEX"
                )
                .bind(db_name)
                .bind(&table_name)
                .fetch_all(pool)
                .await?;

                let mut index_map: HashMap<String, Index> = HashMap::new();
                for idx in idx_rows {
                    let index_name: String = idx.get("INDEX_NAME");
                    // On ignore la clé primaire classique (déjà gérée dans la colonne)
                    if index_name == "PRIMARY" { continue; }

                    let column_name: String = idx.get("COLUMN_NAME");
                    
                    // Astuce pour gérer les types de retour selon la version de MySQL/MariaDB
                    let non_unique: i64 = idx.try_get::<i64, _>("NON_UNIQUE")
                        .or_else(|_| idx.try_get::<i32, _>("NON_UNIQUE").map(|v| v as i64))
                        .unwrap_or(1);

                    index_map.entry(index_name.clone())
                        .and_modify(|e| e.columns.push(column_name.clone()))
                        .or_insert(Index {
                            name: index_name,
                            columns: vec![column_name],
                            is_unique: non_unique == 0,
                        });
                }
                indexes = index_map.into_values().collect();
            }

            tables.push(Table {
                name: table_name,
                is_view,
                columns,
                indexes,
            });
        }

        Ok(tables)
    }
}

// --- TRADUCTION DES TYPES SQL EN TYPES RUST ---

fn map_mysql_type(sql_type: &str, is_nullable: bool) -> String {
    let rust_type = match sql_type.to_lowercase().as_str() {
        "bigint" => "i64",
        "int" | "integer" => "i32",
        "mediumint" => "i32",
        "smallint" => "i16",
        "tinyint" => "i8", // ou bool, i8 est plus générique
        "double" | "real" => "f64",
        "float" => "f32",
        "varchar" | "char" | "text" | "longtext" | "mediumtext" | "tinytext" => "String",
        "date" => "chrono::NaiveDate",
        "time" => "chrono::NaiveTime",
        "datetime" | "timestamp" => "chrono::NaiveDateTime",
        "blob" | "binary" | "varbinary" | "longblob" => "Vec<u8>",
        _ => "String", // Repli sécurisé par défaut
    };

    if is_nullable {
        format!("Option<{}>", rust_type)
    } else {
        rust_type.to_string()
    }
}

fn escape_rust_keyword(name: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mut", "pub", "ref", "return",
        "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
        "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ];
    if KEYWORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

// --- GÉNÉRATEUR DES MÉTHODES D'ÉCRITURE ---

fn generate_write_methods(table: &Table) -> String {
    use heck::{AsSnakeCase, AsPascalCase};
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    
    code.push_str(&format!("impl {} {{\n", struct_name));

    let pk_col = table.columns.iter().find(|c| c.is_primary);
    
    // On exclut la colonne auto-incrémentée de l'insertion
    let insert_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_auto_increment).collect();
    let col_names = insert_cols.iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", ");
    let placeholders = insert_cols.iter().map(|_| "?").collect::<Vec<_>>().join(", ");

    // 1. --- INSERT ---
    code.push_str("    /// Insère la ligne en base de données. Retourne l'ID généré (ou 0).\n");
    code.push_str("    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {\n");
    code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({})\";\n", table.name, col_names, placeholders));
    code.push_str("        let result = sqlx::query(query)\n");
    for col in &insert_cols {
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("            .bind(&self.{})\n", field));
    }
    code.push_str("            .execute(executor).await?;\n");
    code.push_str("        Ok(result.last_insert_id())\n");
    code.push_str("    }\n\n");

    // 2. --- INSERT BATCH (Performance Extrême) ---
    code.push_str("    /// Insère de multiples lignes en une seule requête réseau (Batch).\n");
    code.push_str("    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {\n");
    code.push_str("        if items.is_empty() { return Ok(0); }\n");
    code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(\"INSERT INTO {} ({}) \");\n", table.name, col_names));
    code.push_str("        query_builder.push_values(items, |mut b, item| {\n");
    for col in &insert_cols {
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("            b.push_bind(&item.{});\n", field));
    }
    code.push_str("        });\n");
    code.push_str("        let result = query_builder.build().execute(executor).await?;\n");
    code.push_str("        Ok(result.rows_affected())\n");
    code.push_str("    }\n\n");

    // 3. --- UPSERT ---
    let update_clauses = insert_cols.iter().map(|c| format!("{0} = VALUES({0})", c.name)).collect::<Vec<_>>().join(", ");
    code.push_str("    /// Insère ou met à jour la ligne si une contrainte d'unicité est violée (Upsert).\n");
    code.push_str("    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {\n");
    code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE {}\";\n", table.name, col_names, placeholders, update_clauses));
    code.push_str("        let result = sqlx::query(query)\n");
    for col in &insert_cols {
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("            .bind(&self.{})\n", field));
    }
    code.push_str("            .execute(executor).await?;\n");
    code.push_str("        Ok(result.rows_affected())\n");
    code.push_str("    }\n\n");

    // OPÉRATIONS LIÉES À LA CLÉ PRIMAIRE
    if let Some(pk) = pk_col {
        let pk_field = format!("{}", AsSnakeCase(&pk.name));
        let pk_rust_type = map_mysql_type(&pk.data_type, pk.is_nullable);
        let update_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_primary).collect();
        let set_clauses = update_cols.iter().map(|c| format!("{} = ?", c.name)).collect::<Vec<_>>().join(", ");
        
        // 4. --- UPDATE BY PK ---
        code.push_str("    /// Met à jour la ligne entière via sa clé primaire.\n");
        code.push_str("    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {\n");
        code.push_str(&format!("        let query = \"UPDATE {} SET {} WHERE {} = ?\";\n", table.name, set_clauses, pk.name));
        code.push_str("        let result = sqlx::query(query)\n");
        for col in &update_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind(&self.{})\n", field));
        }
        code.push_str(&format!("            .bind(&self.{})\n", pk_field));
        code.push_str("            .execute(executor).await?;\n");
        code.push_str("        Ok(result.rows_affected())\n");
        code.push_str("    }\n\n");

        // 5. --- DELETE BY PK ---
        code.push_str("    /// Supprime la ligne via sa clé primaire.\n");
        // Note: On demande une simple référence `&T` pour la PK, garantissant la Zéro-Allocation sur des Strings.
        code.push_str(&format!("    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}: &{}) -> sqlx::Result<u64> {{\n", pk_field, pk_rust_type));
        code.push_str(&format!("        let query = \"DELETE FROM {} WHERE {} = ?\";\n", table.name, pk.name));
        code.push_str(&format!("        let result = sqlx::query(query).bind({}).execute(executor).await?;\n", pk_field));
        code.push_str("        Ok(result.rows_affected())\n");
        code.push_str("    }\n\n");

        // 6. --- DELETE MANY BY PK ---
        code.push_str("    /// Supprime de multiples lignes via leurs clés primaires (Batch).\n");
        code.push_str(&format!("    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, ids: &[{k_rust_type}]) -> sqlx::Result<u64> {{\n", k_rust_type = pk_rust_type));
        code.push_str("        if ids.is_empty() { return Ok(0); }\n");
        code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(\"DELETE FROM {} WHERE {} IN \");\n", table.name, pk.name));
        code.push_str("        query_builder.push(\"(\");\n");
        code.push_str("        let mut separated = query_builder.separated(\", \");\n");
        code.push_str("        for id in ids {\n");
        code.push_str("            separated.push_bind(id);\n");
        code.push_str("        }\n");
        code.push_str("        separated.push_unseparated(\")\");\n");
        code.push_str("        let result = query_builder.build().execute(executor).await?;\n");
        code.push_str("        Ok(result.rows_affected())\n");
        code.push_str("    }\n\n");
    }

    // 7. --- OPÉRATIONS PAR INDEX ---
    for idx in &table.indexes {
        let func_suffix = idx.columns.iter().map(|c| format!("{}", AsSnakeCase(c))).collect::<Vec<_>>().join("_and_");
        let where_clauses = idx.columns.iter().map(|c| format!("{} = ?", c)).collect::<Vec<_>>().join(" AND ");
        
        let mut idx_params = Vec::new();
        for c in &idx.columns {
            if let Some(col) = table.columns.iter().find(|col| &col.name == c) {
                idx_params.push(format!("{}: &{}", AsSnakeCase(c), map_mysql_type(&col.data_type, col.is_nullable)));
            }
        }
        let params_str = idx_params.join(", ");

        // UPDATE BY INDEX
        let set_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_primary && !idx.columns.contains(&c.name)).collect();
        if !set_cols.is_empty() {
            let set_clauses = set_cols.iter().map(|c| format!("{} = ?", c.name)).collect::<Vec<_>>().join(", ");
            code.push_str(&format!("    /// Met à jour la ligne via l'index `{}`.\n", idx.name));
            code.push_str(&format!("    pub async fn update_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {{\n", func_suffix));
            code.push_str(&format!("        let query = \"UPDATE {} SET {} WHERE {}\";\n", table.name, set_clauses, where_clauses));
            code.push_str("        let result = sqlx::query(query)\n");
            for col in &set_cols {
                let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
                code.push_str(&format!("            .bind(&self.{})\n", field));
            }
            // On utilise les valeurs courantes de `self` pour la clause WHERE
            for c in &idx.columns {
                let field = escape_rust_keyword(&format!("{}", AsSnakeCase(c)));
                code.push_str(&format!("            .bind(&self.{})\n", field));
            }
            code.push_str("            .execute(executor).await?;\n");
            code.push_str("        Ok(result.rows_affected())\n");
            code.push_str("    }\n\n");
        }

        // DELETE BY INDEX (Méthode statique)
        code.push_str(&format!("    /// Supprime des lignes via l'index `{}`.\n", idx.name));
        code.push_str(&format!("    pub async fn delete_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}) -> sqlx::Result<u64> {{\n", func_suffix, params_str));
        code.push_str(&format!("        let query = \"DELETE FROM {} WHERE {}\";\n", table.name, where_clauses));
        code.push_str("        let result = sqlx::query(query)\n");
        for c in &idx.columns {
            code.push_str(&format!("            .bind({})\n", AsSnakeCase(c)));
        }
        code.push_str("            .execute(executor).await?;\n");
        code.push_str("        Ok(result.rows_affected())\n");
        code.push_str("    }\n\n");
    }

    code.push_str("}\n\n");
    code
}

// --- GÉNÉRATEUR DES MÉTHODES DE LECTURE (TABLES & VUES) ---

fn generate_read_methods(table: &Table) -> String {
    use heck::{AsSnakeCase, AsPascalCase};
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    
    code.push_str(&format!("impl {} {{\n", struct_name));

    // 1. --- MÉTHODES GLOBALES ---
    code.push_str("    /// Compte le nombre total de lignes.\n");
    code.push_str("    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {\n");
    code.push_str(&format!("        let query = \"SELECT COUNT(*) FROM {}\";\n", table.name));
    code.push_str("        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;\n");
    code.push_str("        Ok(count as u64)\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.\n");
    code.push_str("    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {\n");
    code.push_str(&format!("        let query = \"SELECT * FROM {}\";\n", table.name));
    code.push_str("        sqlx::query_as::<_, Self>(query).fetch(executor)\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Pagination par numéro de page et tri dynamique (Offset/Limit).\n");
    code.push_str("    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.\n");
    code.push_str("    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {\n");
    code.push_str("        let offset = page.saturating_sub(1) * page_size;\n");
    code.push_str(&format!("        let query = format!(\"SELECT * FROM {} ORDER BY {{}} LIMIT ? OFFSET ?\", order_by);\n", table.name));
    code.push_str("        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await\n");
    code.push_str("    }\n\n");

    let pk_col = table.columns.iter().find(|c| c.is_primary);

    // 2. --- OPÉRATIONS LIÉES À LA CLÉ PRIMAIRE ---
    if let Some(pk) = pk_col {
        let pk_field = format!("{}", AsSnakeCase(&pk.name));
        let pk_rust_type = map_mysql_type(&pk.data_type, pk.is_nullable);

        code.push_str("    /// Récupère une ligne via sa clé primaire.\n");
        code.push_str(&format!("    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}: &{}) -> sqlx::Result<Option<Self>> {{\n", pk_field, pk_rust_type));
        code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {} = ?\";\n", table.name, pk.name));
        code.push_str(&format!("        sqlx::query_as::<_, Self>(query).bind({}).fetch_optional(executor).await\n", pk_field));
        code.push_str("    }\n\n");

        code.push_str("    /// Vérifie si une ligne existe (Très léger, évite la RAM).\n");
        code.push_str(&format!("    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}: &{}) -> sqlx::Result<bool> {{\n", pk_field, pk_rust_type));
        code.push_str(&format!("        let query = \"SELECT 1 FROM {} WHERE {} = ? LIMIT 1\";\n", table.name, pk.name));
        code.push_str(&format!("        let exists: Option<(i32,)> = sqlx::query_as(query).bind({}).fetch_optional(executor).await?;\n", pk_field));
        code.push_str("        Ok(exists.is_some())\n");
        code.push_str("    }\n\n");

        code.push_str("    /// Pagination par curseur (Performance absolue O(1) sur le B-Tree).\n");
        code.push_str(&format!("    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_id: &{}, limit: u32) -> sqlx::Result<Vec<Self>> {{\n", pk_rust_type));
        code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {} > ? ORDER BY {} ASC LIMIT ?\";\n", table.name, pk.name, pk.name));
        code.push_str("        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit).fetch_all(executor).await\n");
        code.push_str("    }\n\n");
    }

    // 3. --- OPÉRATIONS LIÉES AUX INDEX ---
    for idx in &table.indexes {
        let func_suffix = idx.columns.iter().map(|c| format!("{}", AsSnakeCase(c))).collect::<Vec<_>>().join("_and_");
        let where_clauses = idx.columns.iter().map(|c| format!("{} = ?", c)).collect::<Vec<_>>().join(" AND ");
        
        let mut idx_params = Vec::new();
        let mut bind_calls = String::new();
        let mut stream_binds = String::new();
        
        for c in &idx.columns {
            if let Some(col) = table.columns.iter().find(|col| &col.name == c) {
                let snake = format!("{}", AsSnakeCase(c));
                idx_params.push(format!("{}: &{}", snake, map_mysql_type(&col.data_type, col.is_nullable)));
                bind_calls.push_str(&format!(".bind({})", snake));
                // Le stream retourne un flux qui vit plus longtemps que l'appel de fonction. 
                // On utilise .clone() pour transférer la propriété de la variable à la requête.
                stream_binds.push_str(&format!(".bind({}.clone())", snake));
            }
        }
        let params_str = idx_params.join(", ");

        code.push_str(&format!("    /// Vérifie l'existence via l'index `{}`.\n", idx.name));
        code.push_str(&format!("    pub async fn exists_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}) -> sqlx::Result<bool> {{\n", func_suffix, params_str));
        code.push_str(&format!("        let query = \"SELECT 1 FROM {} WHERE {} LIMIT 1\";\n", table.name, where_clauses));
        code.push_str(&format!("        let exists: Option<(i32,)> = sqlx::query_as(query){}.fetch_optional(executor).await?;\n", bind_calls));
        code.push_str("        Ok(exists.is_some())\n");
        code.push_str("    }\n\n");

        if idx.is_unique {
            code.push_str(&format!("    /// Récupère une ligne (unique) via l'index `{}`.\n", idx.name));
            code.push_str(&format!("    pub async fn get_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}) -> sqlx::Result<Option<Self>> {{\n", func_suffix, params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch_optional(executor).await\n", bind_calls));
            code.push_str("    }\n\n");
        } else {
            code.push_str(&format!("    /// Liste des lignes via l'index `{}`.\n", idx.name));
            code.push_str(&format!("    pub async fn list_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}) -> sqlx::Result<Vec<Self>> {{\n", func_suffix, params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch_all(executor).await\n", bind_calls));
            code.push_str("    }\n\n");

            code.push_str(&format!("    /// Flux asynchrone (Stream) zéro-allocation sur l'index `{}`.\n", idx.name));
            code.push_str(&format!("    pub fn stream_by_{}<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, {}) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n", func_suffix, params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch(executor)\n", stream_binds));
            code.push_str("    }\n\n");
        }
    }

    code.push_str("}\n\n");
    code
}

// --- GÉNÉRATEUR DE LA STRUCTURE PATCH (UPDATE PARTIEL) ---

fn generate_patch_struct(table: &Table) -> String {
    use heck::{AsPascalCase, AsSnakeCase};
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    let patch_struct_name = format!("{}Patch", struct_name);

    code.push_str(&format!("/// Structure pour la mise à jour partielle (Patch) de `{}`.\n", table.name));
    code.push_str("#[derive(Debug, Clone, Default)]\n");
    code.push_str(&format!("pub struct {} {{\n", patch_struct_name));

    for col in &table.columns {
        // La clé primaire n'est pas modifiable, on l'utilise uniquement pour le WHERE
        if col.is_primary { continue; } 
        
        let rust_type = map_mysql_type(&col.data_type, col.is_nullable);
        let field_name = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        
        // Tous les champs sont encapsulés dans un Option
        // Si le champ de base était déjà Option<T>, il deviendra Option<Option<T>>
        code.push_str(&format!("    pub {}: Option<{}>,\n", field_name, rust_type));
    }
    code.push_str("}\n\n");
    
    code
}

// --- GÉNÉRATEUR DE LA FONCTION UPDATE_PARTIAL_BY_PK ---

fn generate_partial_update_method(table: &Table) -> String {
    use heck::{AsPascalCase, AsSnakeCase};
    
    let pk_col = match table.columns.iter().find(|c| c.is_primary) {
        Some(col) => col,
        None => return String::new(), // Pas de clé primaire, pas de mise à jour ciblée possible
    };
    
    let struct_name = format!("{}", AsPascalCase(&table.name));
    let patch_struct_name = format!("{}Patch", struct_name);
    let pk_field = format!("{}", AsSnakeCase(&pk_col.name));
    let pk_rust_type = map_mysql_type(&pk_col.data_type, pk_col.is_nullable);

    let mut code = String::new();
    code.push_str(&format!("impl {} {{\n", struct_name));
    code.push_str("    /// Met à jour uniquement les colonnes renseignées (Patch).\n");
    code.push_str("    /// Économise le réseau et les écritures disque de la base de données.\n");
    
    code.push_str(&format!("    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, {}: &{}, patch: &{}) -> sqlx::Result<u64> {{\n", pk_field, pk_rust_type, patch_struct_name));
    code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(\"UPDATE {} SET \");\n", table.name));
    code.push_str("        let mut has_fields = false;\n");
    
    // Le constructeur `separated` permet d'insérer des virgules automatiquement !
    code.push_str("        let mut separated = query_builder.separated(\", \");\n\n");

    for col in &table.columns {
        if col.is_primary { continue; }
        
        let field_name = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        
        code.push_str(&format!("        if let Some(val) = &patch.{} {{\n", field_name));
        code.push_str("            has_fields = true;\n");
        code.push_str(&format!("            separated.push(\"{} = \");\n", col.name));
        // `.clone()` permet de transférer l'ownership au QueryBuilder proprement
        code.push_str("            separated.push_bind_unseparated(val.clone());\n");
        code.push_str("        }\n");
    }

    code.push_str("\n        if !has_fields {\n");
    code.push_str("            // Si le patch est vide, on économise un aller-retour réseau\n");
    code.push_str("            return Ok(0);\n");
    code.push_str("        }\n\n");

    code.push_str(&format!("        query_builder.push(\" WHERE {} = \");\n", pk_col.name));
    code.push_str(&format!("        query_builder.push_bind({}.clone());\n\n", pk_field));

    code.push_str("        let result = query_builder.build().execute(executor).await?;\n");
    code.push_str("        Ok(result.rows_affected())\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code
}