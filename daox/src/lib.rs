use anyhow::{Context, Result};
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions, Row};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;
use heck::{AsPascalCase, AsSnakeCase};

// --- DÉTECTION DU MOTEUR (DIALECTE) ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DbDialect {
    MySql,
    Postgres,
    Sqlite,
}

impl DbDialect {
    fn db_type(&self) -> &'static str {
        match self {
            DbDialect::MySql => "sqlx::MySql",
            DbDialect::Postgres => "sqlx::Postgres",
            DbDialect::Sqlite => "sqlx::Sqlite",
        }
    }

    fn ph(&self, i: usize) -> String {
        match self {
            DbDialect::MySql => "?".to_string(),
            DbDialect::Postgres => format!("${}", i),
            DbDialect::Sqlite => "?".to_string(),
        }
    }

    /// Échappe un nom de colonne si c'est un mot-clé réservé SQL.
    /// MySQL/MariaDB : `backticks`, PostgreSQL/SQLite : "double quotes".
    fn escape_sql_col(&self, name: &str) -> String {
        const SQL_KEYWORDS: &[&str] = &[
            "type", "match", "order", "group", "select", "insert", "update", "delete",
            "where", "from", "table", "index", "key", "primary", "foreign", "check",
            "default", "column", "create", "alter", "drop", "values", "set", "into",
            "join", "on", "as", "and", "or", "not", "null", "is", "in", "like",
            "between", "exists", "having", "limit", "offset", "union", "all", "any",
            "case", "when", "then", "else", "end", "distinct", "asc", "desc",
            "constraint", "references", "grant", "revoke", "trigger", "view",
            "begin", "commit", "rollback", "do", "for", "if", "return", "use",
            "user", "role", "schema", "sequence", "function", "procedure",
        ];
        if SQL_KEYWORDS.contains(&name.to_lowercase().as_str()) {
            match self {
                DbDialect::MySql => format!("`{}`", name),
                // Les guillemets doivent être échappés car le résultat est injecté dans un string literal Rust
                DbDialect::Postgres | DbDialect::Sqlite => format!("\\\"{}\\\"", name),
            }
        } else {
            name.to_string()
        }
    }
}

// --- STRUCTURES DE MÉTADONNÉES ---

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary: bool,
    pub is_auto_increment: bool,
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
    dialect: DbDialect,
}

impl DaoxGenerator {
    pub fn new(database_url: &str, output_dir: &str) -> Self {
        let dialect = if database_url.starts_with("postgres") {
            DbDialect::Postgres
        } else if database_url.starts_with("sqlite") {
            DbDialect::Sqlite
        } else {
            DbDialect::MySql
        };

        Self {
            database_url: database_url.to_string(),
            output_dir: output_dir.to_string(),
            dialect,
        }
    }

    pub async fn generate(&self) -> Result<()> {
        println!("cargo:warning=🚀 Démarrage de la génération daox...");
        
        let tables = match self.dialect {
            DbDialect::MySql => {
                println!("cargo:warning=🐬 Détection Moteur MySQL/MariaDB");
                let pool = MySqlPoolOptions::new()
                    .max_connections(2)
                    .connect(&self.database_url)
                    .await
                    .context("Impossible de se connecter à MySQL. Docker tourne-t-il ?")?;

                let db_name = self.database_url.rsplit('/').next()
                    .unwrap_or("")
                    .split('?')
                    .next()
                    .unwrap_or("");

                self.introspect_mysql(&pool, db_name).await?
            }
            DbDialect::Postgres => {
                println!("cargo:warning=🐘 Détection Moteur PostgreSQL");
                let pool = PgPoolOptions::new()
                    .max_connections(2)
                    .connect(&self.database_url)
                    .await
                    .context("Impossible de se connecter à PostgreSQL. Docker tourne-t-il ?")?;
                
                self.introspect_postgres(&pool).await?
            }
            DbDialect::Sqlite => {
                println!("cargo:warning=🪶 Détection Moteur SQLite");
                let pool = SqlitePoolOptions::new()
                    .max_connections(2)
                    .connect(&self.database_url)
                    .await
                    .context("Impossible de se connecter à SQLite.")?;
                
                self.introspect_sqlite(&pool).await?
            }
        };

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
            code.push_str("// Code generated automatically by daox. DO NOT EDIT.\n\n");
            code.push_str("#[derive(Debug, Clone, sqlx::FromRow)]\n");
            code.push_str(&format!("pub struct {} {{\n", struct_name));

            for col in &table.columns {
                let rust_type = map_sql_type(&col.data_type, col.is_nullable, self.dialect);
                let snake_col_name = format!("{}", AsSnakeCase(&col.name));
                let rust_field = escape_rust_keyword(&snake_col_name);
                code.push_str(&format!("    pub {}: {},\n", rust_field, rust_type));
            }
            code.push_str("}\n\n");

            // S'applique aux TABLES et aux VUES de manière égale !
            code.push_str(&generate_read_methods(table, self.dialect));

            if !table.is_view {
                code.push_str(&generate_write_methods(table, self.dialect));
                code.push_str(&generate_patch_struct(table, self.dialect));
                code.push_str(&generate_partial_update_method(table, self.dialect));
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
        let rows = sqlx::query("SELECT TABLE_NAME, TABLE_TYPE FROM information_schema.TABLES WHERE TABLE_SCHEMA = ?")
            .bind(db_name).fetch_all(pool).await?;

        for row in rows {
            let table_name: String = row.get("TABLE_NAME");
            let is_view = row.get::<String, _>("TABLE_TYPE") == "VIEW";

            let col_rows = sqlx::query("SELECT COLUMN_NAME, DATA_TYPE, IS_NULLABLE, COLUMN_KEY, EXTRA FROM information_schema.COLUMNS WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? ORDER BY ORDINAL_POSITION")
                .bind(db_name).bind(&table_name).fetch_all(pool).await?;

            let mut columns = Vec::new();
            for col in col_rows {
                let extra: String = col.get("EXTRA");
                columns.push(Column {
                    name: col.get("COLUMN_NAME"),
                    data_type: col.get("DATA_TYPE"),
                    is_nullable: col.get::<String, _>("IS_NULLABLE") == "YES",
                    is_primary: col.get::<String, _>("COLUMN_KEY") == "PRI",
                    is_auto_increment: extra.to_lowercase().contains("auto_increment"),
                });
            }

            let mut indexes: Vec<Index> = Vec::new();
            if !is_view {
                let idx_rows = sqlx::query("SELECT INDEX_NAME, COLUMN_NAME, NON_UNIQUE FROM information_schema.STATISTICS WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ? ORDER BY INDEX_NAME, SEQ_IN_INDEX")
                    .bind(db_name).bind(&table_name).fetch_all(pool).await?;

                let mut index_map: HashMap<String, Index> = HashMap::new();
                for idx in idx_rows {
                    let index_name: String = idx.get("INDEX_NAME");
                    if index_name == "PRIMARY" { continue; }
                    let column_name: String = idx.get("COLUMN_NAME");
                    let non_unique: i64 = idx.try_get::<i64, _>("NON_UNIQUE").or_else(|_| idx.try_get::<i32, _>("NON_UNIQUE").map(|v| v as i64)).unwrap_or(1);
                    index_map.entry(index_name.clone()).and_modify(|e| e.columns.push(column_name.clone())).or_insert(Index { name: index_name, columns: vec![column_name], is_unique: non_unique == 0 });
                }
                indexes = index_map.into_values().collect();
            }
            tables.push(Table { name: table_name, is_view, columns, indexes });
        }
        Ok(tables)
    }

    // --- LOGIQUE D'INTROSPECTION POSTGRESQL ---
    async fn introspect_postgres(&self, pool: &sqlx::PgPool) -> Result<Vec<Table>> {
        let mut tables = Vec::new();
        let rows = sqlx::query("SELECT table_name, table_type FROM information_schema.tables WHERE table_schema = 'public'").fetch_all(pool).await?;

        for row in rows {
            let table_name: String = row.get("table_name");
            let is_view = row.get::<String, _>("table_type") == "VIEW";

            let col_rows = sqlx::query(
                "SELECT column_name, udt_name as data_type, is_nullable, column_default,
                 (SELECT COUNT(*) > 0 FROM information_schema.key_column_usage kcu JOIN information_schema.table_constraints tc ON kcu.constraint_name = tc.constraint_name WHERE tc.table_schema = 'public' AND tc.table_name = $1 AND kcu.column_name = c.column_name AND tc.constraint_type = 'PRIMARY KEY') as is_primary,
                 is_identity
                 FROM information_schema.columns c WHERE table_schema = 'public' AND table_name = $1 ORDER BY ordinal_position"
            ).bind(&table_name).fetch_all(pool).await?;

            let mut columns = Vec::new();
            for col in col_rows {
                let default_val: Option<String> = col.try_get("column_default").unwrap_or_default();
                let is_identity: Option<String> = col.try_get("is_identity").unwrap_or_default();
                let is_auto_inc = default_val.unwrap_or_default().contains("nextval") || is_identity.unwrap_or_default() == "YES";
                columns.push(Column {
                    name: col.get("column_name"), 
                    data_type: col.get("data_type"), 
                    is_nullable: col.get::<String, _>("is_nullable") == "YES",
                    is_primary: col.get::<bool, _>("is_primary"), 
                    is_auto_increment: is_auto_inc,
                });
            }

            let mut indexes: Vec<Index> = Vec::new();
            if !is_view {
                let idx_rows = sqlx::query(
                    "SELECT ix.relname as index_name, a.attname as column_name, i.indisunique as is_unique
                     FROM pg_catalog.pg_class t JOIN pg_catalog.pg_index i ON t.oid = i.indrelid JOIN pg_catalog.pg_class ix ON i.indexrelid = ix.oid CROSS JOIN unnest(i.indkey) WITH ORDINALITY AS k(attnum, pos) JOIN pg_catalog.pg_attribute a ON a.attrelid = t.oid AND a.attnum = k.attnum JOIN pg_catalog.pg_namespace n ON t.relnamespace = n.oid
                     WHERE n.nspname = 'public' AND t.relkind = 'r' AND t.relname = $1 AND i.indisprimary = false ORDER BY ix.relname, k.pos"
                ).bind(&table_name).fetch_all(pool).await?;

                let mut index_map: HashMap<String, Index> = HashMap::new();
                for idx in idx_rows {
                    let index_name: String = idx.get("index_name");
                    let column_name: String = idx.get("column_name");
                    let is_unique: bool = idx.get("is_unique");
                    index_map.entry(index_name.clone()).and_modify(|e| e.columns.push(column_name.clone())).or_insert(Index { name: index_name, columns: vec![column_name], is_unique });
                }
                indexes = index_map.into_values().collect();
            }
            tables.push(Table { name: table_name, is_view, columns, indexes });
        }
        Ok(tables)
    }

    async fn introspect_sqlite(&self, pool: &sqlx::sqlite::SqlitePool) -> Result<Vec<Table>> {
        let mut tables = Vec::new();
        
        let db_tables = sqlx::query("SELECT name, type FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%'")
            .fetch_all(pool).await?;

        for row in db_tables {
            let table_name: String = row.get("name");
            let table_type: String = row.get("type");
            let is_view = table_type == "view";
            
            let cols = sqlx::query(&format!("PRAGMA table_info('{}')", table_name))
                .fetch_all(pool).await?;
            
            let mut columns = Vec::new();
            for col_row in cols {
                let name: String = col_row.get("name");
                let data_type: String = col_row.get("type");
                let notnull: i32 = col_row.try_get("notnull").unwrap_or(0);
                let pk: i32 = col_row.try_get("pk").unwrap_or(0);
                
                let is_nullable = notnull == 0 && pk == 0;
                let is_primary = pk > 0;
                let is_auto_increment = is_primary && data_type.to_uppercase().contains("INT");
                
                columns.push(Column {
                    name,
                    data_type: data_type.to_lowercase(),
                    is_nullable,
                    is_primary,
                    is_auto_increment,
                });
            }

            let mut indexes = Vec::new();
            let idxs = sqlx::query(&format!("PRAGMA index_list('{}')", table_name))
                .fetch_all(pool).await?;
            
            for idx_row in idxs {
                let name: String = idx_row.get("name");
                let unique: i32 = idx_row.get("unique");
                let origin: String = idx_row.try_get("origin").unwrap_or_else(|_| "".to_string());
                
                if origin == "pk" { continue; }
                let is_unique = unique != 0;
                
                let idx_cols = sqlx::query(&format!("PRAGMA index_info('{}')", name))
                    .fetch_all(pool).await?;
                
                let mut idx_columns = Vec::new();
                for c_row in idx_cols {
                    if let Ok(col_name) = c_row.try_get::<String, _>("name") {
                        if !col_name.is_empty() {
                            idx_columns.push(col_name);
                        }
                    }
                }
                
                indexes.push(Index { name, columns: idx_columns, is_unique });
            }

            tables.push(Table { name: table_name, is_view, columns, indexes });
        }
        Ok(tables)
    }
}

// --- TRADUCTION DES TYPES SQL EN TYPES RUST (Unifié) ---

fn map_sql_type(sql_type: &str, is_nullable: bool, dialect: DbDialect) -> String {
    let rust_type = match sql_type.to_lowercase().as_str() {
        // Entiers — SQLite INTEGER est toujours 64-bit (rowid)
        "bigint" | "int8" | "bigserial" => "i64",
        "integer" if dialect == DbDialect::Sqlite => "i64",
        "int" | "integer" | "int4" | "mediumint" | "serial" => "i32",
        "smallint" | "int2" => "i16",
        "tinyint" => "i8",
        // Flottants
        "double" | "real" | "double precision" | "float8" => "f64",
        "float" | "float4" => "f32",
        // Numériques exacts (safe fallback String sans dépendance rust_decimal)
        "numeric" | "decimal" => "String",
        // Texte
        "varchar" | "char" | "bpchar" | "text" | "longtext" | "mediumtext" | "tinytext" | "character varying" | "name" => "String",
        // Dates / Temps
        "date" => "chrono::NaiveDate",
        "time" | "time without time zone" | "timetz" => "chrono::NaiveTime",
        "datetime" | "timestamp" | "timestamp without time zone" => "chrono::NaiveDateTime",
        "timestamptz" | "timestamp with time zone" => "chrono::DateTime<chrono::Utc>",
        // Binaire
        "blob" | "binary" | "varbinary" | "longblob" | "bytea" => "Vec<u8>",
        // Booléen
        "boolean" | "bool" => "bool",
        // JSON / UUID (safe fallback String)
        "json" | "jsonb" | "uuid" => "String",
        _ => "String",
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

fn generate_write_methods(table: &Table, dialect: DbDialect) -> String {
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    
    code.push_str(&format!("impl {} {{\n", struct_name));

    let pk_col = table.columns.iter().find(|c| c.is_primary);
    
    // On exclut la colonne auto-incrémentée de l'insertion
    let insert_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_auto_increment).collect();
    let col_names = insert_cols.iter().map(|c| dialect.escape_sql_col(&c.name)).collect::<Vec<_>>().join(", ");
    
    let mut placeholders = String::new();
    for i in 0..insert_cols.len() {
        if i > 0 { placeholders.push_str(", "); }
        placeholders.push_str(&dialect.ph(i + 1));
    }

    // 1. --- INSERT ---
    code.push_str("    /// Inserts the current record into the database.\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Best Practice:** Use this method when you want to create a brand new row.\n");
    code.push_str("    /// If the table has an auto-increment primary key, the database will generate the ID automatically.\n");
    code.push_str("    /// \n");
    code.push_str("    /// Returns the generated ID (or 0 if the table doesn't have an auto-increment ID).\n");
    code.push_str(&format!("    pub async fn insert<'e, E: sqlx::Executor<'e, Database = {}>>(&self, executor: E) -> sqlx::Result<u64> {{\n", dialect.db_type()));
    
    let is_numeric_pk = pk_col.map_or(false, |pk| {
        let t = pk.data_type.to_lowercase();
        t == "int2" || t == "int4" || t == "int8" || t == "integer" || t == "bigint" || t == "smallint" || t == "serial" || t == "bigserial"
    });

    if dialect == DbDialect::Postgres && is_numeric_pk {
        let pk = pk_col.unwrap();
        // Le cast ::bigint est essentiel pour sqlx Postgres si le PK est un INT (i32) car on fetch dans (i64,)
        code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({}) RETURNING {}::bigint\";\n", table.name, col_names, placeholders, dialect.escape_sql_col(&pk.name)));
        code.push_str("        let (id,): (i64,) = sqlx::query_as(&query)\n");
        for col in &insert_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind(&self.{})\n", field));
        }
        code.push_str("            .fetch_one(executor).await?;\n        Ok(id as u64)\n    }\n\n");
    } else {
        code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({})\";\n", table.name, col_names, placeholders));
        code.push_str("        let result = sqlx::query(&query)\n");
        for col in &insert_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind(&self.{})\n", field));
        }
        code.push_str("            .execute(executor).await?;\n");
        if dialect == DbDialect::MySql {
            code.push_str("        Ok(result.last_insert_id())\n    }\n\n");
        } else if dialect == DbDialect::Sqlite {
            code.push_str("        Ok(result.last_insert_rowid() as u64)\n    }\n\n");
        } else {
            code.push_str("        Ok(result.rows_affected())\n    }\n\n");
        }
    }

    // 2. --- INSERT BATCH ---
    code.push_str("    /// Inserts multiple records in a single network round-trip (Batch Insert).\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Performance:** This is heavily optimized. Instead of running 100 individual `INSERT` queries,\n");
    code.push_str("    /// this method groups them into one massive `INSERT INTO ... VALUES (...), (...), ...` query.\n");
    code.push_str("    /// Always prefer this method over looping with `.insert()` when saving large amounts of data.\n");
    code.push_str("    /// \n");
    code.push_str("    /// Returns the number of rows successfully inserted.\n");
    code.push_str(&format!("    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {{\n", dialect.db_type()));
    code.push_str("        if items.is_empty() { return Ok(0); }\n");
    code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<{}> = sqlx::QueryBuilder::new(\"INSERT INTO {} ({}) \");\n", dialect.db_type(), table.name, col_names));
    code.push_str("        query_builder.push_values(items, |mut b, item| {\n");
    for col in &insert_cols {
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("            b.push_bind(&item.{});\n", field));
    }
    code.push_str("        });\n        let result = query_builder.build().execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");

    // 3. --- UPSERT ---
    code.push_str("    /// Inserts the record, or updates it if a unique constraint is violated (Upsert).\n");
    code.push_str("    /// \n");
    code.push_str("    /// **How it works:** \n");
    code.push_str("    /// 1. The database attempts to insert the row.\n");
    code.push_str("    /// 2. If a collision occurs (e.g., an email already exists in a UNIQUE index),\n");
    code.push_str("    ///    it automatically updates the existing row with the new data instead of crashing.\n");
    code.push_str("    /// \n");
    code.push_str("    /// This is highly recommended for data synchronization tasks.\n");
    code.push_str(&format!("    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = {}>>(&self, executor: E) -> sqlx::Result<u64> {{\n", dialect.db_type()));
    
    let mut conflict_cols = Vec::new();
    let pk_cols_ref: Vec<_> = table.columns.iter().filter(|c| c.is_primary).collect();
    let has_auto_inc_pk = pk_cols_ref.iter().any(|c| c.is_auto_increment);
    let pk_col_names: Vec<_> = pk_cols_ref.iter().map(|c| c.name.clone()).collect();
    
    // Si la PK est auto-incrémentée, elle est exclue de l'INSERT. Le conflit sur PK ne peut donc jamais arriver.
    // Dans ce cas précis, on doit obligatoirement utiliser un index unique comme cible de conflit.
    if !pk_col_names.is_empty() && !has_auto_inc_pk {
        conflict_cols = pk_col_names;
    } else if let Some(idx) = table.indexes.iter().find(|idx| idx.is_unique) {
        conflict_cols = idx.columns.clone();
    }

    if dialect == DbDialect::Postgres || dialect == DbDialect::Sqlite {
        if !conflict_cols.is_empty() {
            let conflict_target = conflict_cols.iter().map(|c| dialect.escape_sql_col(c)).collect::<Vec<_>>().join(", ");
            let update_clauses = insert_cols.iter().map(|c| { let esc = dialect.escape_sql_col(&c.name); format!("{0} = EXCLUDED.{0}", esc) }).collect::<Vec<_>>().join(", ");
            code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}\";\n", table.name, col_names, placeholders, conflict_target, update_clauses));
        } else {
            code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({})\";\n", table.name, col_names, placeholders));
        }
    } else {
        let update_clauses = insert_cols.iter().map(|c| { let esc = dialect.escape_sql_col(&c.name); format!("{0} = VALUES({0})", esc) }).collect::<Vec<_>>().join(", ");
        code.push_str(&format!("        let query = \"INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE {}\";\n", table.name, col_names, placeholders, update_clauses));
    }
    code.push_str("        let result = sqlx::query(&query)\n");
    for col in &insert_cols {
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("            .bind(&self.{})\n", field));
    }
    code.push_str("            .execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");

    // OPÉRATIONS LIÉES À LA CLÉ PRIMAIRE
    let pk_cols: Vec<&Column> = table.columns.iter().filter(|c| c.is_primary).collect();
    if !pk_cols.is_empty() {
        let pk_args = pk_cols.iter().map(|c| format!("{}: &{}", escape_rust_keyword(&format!("{}", AsSnakeCase(&c.name))), map_sql_type(&c.data_type, c.is_nullable, dialect))).collect::<Vec<_>>().join(", ");
        let update_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_primary).collect();
        
        let mut set_clauses = String::new();
        for (i, c) in update_cols.iter().enumerate() {
            if i > 0 { set_clauses.push_str(", "); }
            set_clauses.push_str(&format!("{} = {}", dialect.escape_sql_col(&c.name), dialect.ph(i + 1)));
        }
        
        let pk_where = pk_cols.iter().enumerate().map(|(i, c)| format!("{} = {}", dialect.escape_sql_col(&c.name), dialect.ph(update_cols.len() + i + 1))).collect::<Vec<_>>().join(" AND ");

        // 4. --- UPDATE BY PK ---
        code.push_str("    /// Overwrites the entire record in the database using its Primary Key.\n");
        code.push_str("    /// \n");
        code.push_str("    /// **Warning:** This will update ALL columns in the row with the values in the current struct.\n");
        code.push_str("    /// If you only want to update one or two specific columns, use `update_partial_by_pk` instead \n");
        code.push_str("    /// to save network bandwidth and database disk I/O.\n");
        code.push_str(&format!("    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(&self, executor: E) -> sqlx::Result<u64> {{\n", dialect.db_type()));
        code.push_str(&format!("        let query = \"UPDATE {} SET {} WHERE {}\";\n", table.name, set_clauses, pk_where));
        code.push_str("        let result = sqlx::query(&query)\n");
        for col in &update_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind(&self.{})\n", field));
        }
        for col in &pk_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind(&self.{})\n", field));
        }
        code.push_str("            .execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");

        // 5. --- DELETE BY PK ---
        let pk_where_del = pk_cols.iter().enumerate().map(|(i, c)| format!("{} = {}", dialect.escape_sql_col(&c.name), dialect.ph(i + 1))).collect::<Vec<_>>().join(" AND ");
        code.push_str("    /// Deletes the specific record from the database using its Primary Key.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Returns the number of affected rows (1 if deleted, 0 if it didn't exist).\n");
        code.push_str(&format!("    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<u64> {{\n", dialect.db_type(), pk_args));
        code.push_str(&format!("        let query = \"DELETE FROM {} WHERE {}\";\n", table.name, pk_where_del));
        code.push_str("        let result = sqlx::query(&query)\n");
        for col in &pk_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind({})\n", field));
        }
        code.push_str("            .execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");

        // 6. --- DELETE MANY BY PK ---
        if pk_cols.len() == 1 {
            let pk = pk_cols[0];
            let pk_rust_type = map_sql_type(&pk.data_type, pk.is_nullable, dialect);
            
            code.push_str("    /// Deletes multiple records in a single query using an `IN (...)` clause.\n");
            code.push_str("    /// \n");
            code.push_str("    /// **Performance:** This is the most efficient way to delete a batch of specific IDs.\n");
            code.push_str("    /// Returns the total number of rows successfully deleted.\n");
            code.push_str(&format!("    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, ids: &[{k_rust_type}]) -> sqlx::Result<u64> {{\n", dialect.db_type(), k_rust_type = pk_rust_type));
            code.push_str("        if ids.is_empty() { return Ok(0); }\n");
            code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<{}> = sqlx::QueryBuilder::new(\"DELETE FROM {} WHERE {} IN \");\n", dialect.db_type(), table.name, dialect.escape_sql_col(&pk.name)));
            code.push_str("        query_builder.push(\"(\");\n        let mut separated = query_builder.separated(\", \");\n");
            code.push_str("        for id in ids { separated.push_bind(id); }\n");
            code.push_str("        separated.push_unseparated(\")\");\n        let result = query_builder.build().execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");
        }
    }

    for idx in &table.indexes {
        let func_suffix = idx.columns.iter().map(|c| format!("{}", AsSnakeCase(c))).collect::<Vec<_>>().join("_and_");
        
        let set_cols: Vec<&Column> = table.columns.iter().filter(|c| !c.is_primary && !idx.columns.contains(&c.name)).collect();
        if !set_cols.is_empty() {
            let mut set_clauses = String::new();
            for (i, c) in set_cols.iter().enumerate() {
                if i > 0 { set_clauses.push_str(", "); }
                set_clauses.push_str(&format!("{} = {}", dialect.escape_sql_col(&c.name), dialect.ph(i + 1)));
            }

            let mut offset = set_cols.len() + 1;
            let mut idx_where = String::new();
            for (i, c) in idx.columns.iter().enumerate() {
                if i > 0 { idx_where.push_str(" AND "); }
                idx_where.push_str(&format!("{} = {}", dialect.escape_sql_col(c), dialect.ph(offset)));
                offset += 1;
            }

            code.push_str(&format!("    /// Updates records matching the `{}` index.\n", idx.name));
            code.push_str("    /// \n");
            code.push_str("    /// **Warning:** This overwrites all columns (except the index columns) with the values from the current struct.\n");
            code.push_str(&format!("    pub async fn update_by_{}<'e, E: sqlx::Executor<'e, Database = {}>>(&self, executor: E) -> sqlx::Result<u64> {{\n", func_suffix, dialect.db_type()));
            code.push_str(&format!("        let query = \"UPDATE {} SET {} WHERE {}\";\n        let result = sqlx::query(&query)\n", table.name, set_clauses, idx_where));
            for col in &set_cols {
                let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
                code.push_str(&format!("            .bind(&self.{})\n", field));
            }
            for c in &idx.columns {
                let field = escape_rust_keyword(&format!("{}", AsSnakeCase(c)));
                code.push_str(&format!("            .bind(&self.{})\n", field));
            }
            code.push_str("            .execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");
        }

        let mut idx_params = Vec::new();
        let mut where_clauses = String::new();
        for (i, c) in idx.columns.iter().enumerate() {
            if let Some(col) = table.columns.iter().find(|col| &col.name == c) {
                let field = escape_rust_keyword(&format!("{}", AsSnakeCase(c)));
                idx_params.push(format!("{}: &{}", field, map_sql_type(&col.data_type, col.is_nullable, dialect)));
                if i > 0 { where_clauses.push_str(" AND "); }
                where_clauses.push_str(&format!("{} = {}", dialect.escape_sql_col(c), dialect.ph(i + 1)));
            }
        }

        code.push_str(&format!("    /// Deletes records matching the `{}` index.\n", idx.name));
        code.push_str("    /// \n");
        code.push_str("    /// Returns the number of affected rows.\n");
        code.push_str(&format!("    pub async fn delete_by_{}<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<u64> {{\n", func_suffix, dialect.db_type(), idx_params.join(", ")));
        code.push_str(&format!("        let query = \"DELETE FROM {} WHERE {}\";\n        let result = sqlx::query(&query)\n", table.name, where_clauses));
        for c in &idx.columns {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(c)));
            code.push_str(&format!("            .bind({})\n", field));
        }
        code.push_str("            .execute(executor).await?;\n        Ok(result.rows_affected())\n    }\n\n");
    }

    code.push_str("}\n\n");
    code
}

// --- GÉNÉRATEUR DES MÉTHODES DE LECTURE (TABLES & VUES) ---

fn generate_read_methods(table: &Table, dialect: DbDialect) -> String {
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    
    code.push_str(&format!("impl {} {{\n", struct_name));

    // 1. --- MÉTHODES GLOBALES ---
    code.push_str("    /// Counts the total number of rows in the table.\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.\n");
    code.push_str(&format!("    pub async fn count<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E) -> sqlx::Result<u64> {{\n", dialect.db_type()));
    code.push_str(&format!("        let query = \"SELECT COUNT(*) FROM {}\";\n", table.name));
    code.push_str("        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;\n");
    code.push_str("        Ok(count as u64)\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Creates a zero-allocation Asynchronous Stream over the entire table.\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Performance:** This is the absolute best way to process millions of rows.\n");
    code.push_str("    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),\n");
    code.push_str("    /// the Stream fetches and yields rows one by one directly from the database connection.\n");
    code.push_str(&format!("    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = {}> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n", dialect.db_type()));
    code.push_str(&format!("        let query = \"SELECT * FROM {}\";\n", table.name));
    code.push_str("        sqlx::query_as::<_, Self>(query).fetch(executor)\n");
    code.push_str("    }\n\n");

    code.push_str("    /// Classic Offset/Limit pagination with dynamic sorting.\n");
    code.push_str("    /// \n");
    code.push_str("    /// **SECURITY WARNING:** The `order_by` parameter is NOT bound via prepared statements \n");
    code.push_str("    /// (SQL does not allow binding column names). You MUST strictly whitelist the user input \n");
    code.push_str("    /// before passing it here to prevent SQL Injection!\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Performance:** Offset pagination becomes very slow on deep pages. Consider `list_by_cursor` instead.\n");
    code.push_str(&format!("    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {{\n", dialect.db_type()));
    code.push_str("        let offset = page.saturating_sub(1) * page_size;\n");
    code.push_str(&format!("        let query = format!(\"SELECT * FROM {} ORDER BY {{}} LIMIT {} OFFSET {}\", order_by);\n", table.name, dialect.ph(1), dialect.ph(2)));
    if dialect == DbDialect::Postgres {
        code.push_str("        sqlx::query_as::<_, Self>(&query).bind(page_size as i64).bind(offset as i64).fetch_all(executor).await\n");
    } else {
        code.push_str("        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await\n");
    }
    code.push_str("    }\n\n");

    let pk_cols: Vec<&Column> = table.columns.iter().filter(|c| c.is_primary).collect();

    // 2. --- OPÉRATIONS LIÉES À LA CLÉ PRIMAIRE ---
    if !pk_cols.is_empty() {
        let pk_args = pk_cols.iter().map(|c| format!("{}: &{}", escape_rust_keyword(&format!("{}", AsSnakeCase(&c.name))), map_sql_type(&c.data_type, c.is_nullable, dialect))).collect::<Vec<_>>().join(", ");
        let pk_where = pk_cols.iter().enumerate().map(|(i, c)| format!("{} = {}", dialect.escape_sql_col(&c.name), dialect.ph(i + 1))).collect::<Vec<_>>().join(" AND ");

        code.push_str("    /// Retrieves a single record using its Primary Key.\n");
        code.push_str("    /// \n");
        code.push_str("    /// Returns `Some(Self)` if the record exists, or `None` if it does not.\n");
        code.push_str(&format!("    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<Option<Self>> {{\n", dialect.db_type(), pk_args));
        code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, pk_where));
        code.push_str("        sqlx::query_as::<_, Self>(query)\n");
        for col in &pk_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind({})\n", field));
        }
        code.push_str("            .fetch_optional(executor).await\n    }\n\n");

        code.push_str("    /// Checks if a record exists using its Primary Key.\n");
        code.push_str("    /// \n");
        code.push_str("    /// **Performance:** This uses a `SELECT 1 ... LIMIT 1` query. It is infinitely faster \n");
        code.push_str("    /// and lighter than `get_by_pk` when you only need to check for existence, because it avoids \n");
        code.push_str("    /// transferring and deserializing the full row data.\n");
        code.push_str(&format!("    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<bool> {{\n", dialect.db_type(), pk_args));
        code.push_str(&format!("        let query = \"SELECT 1 FROM {} WHERE {} LIMIT 1\";\n", table.name, pk_where));
        code.push_str("        let exists: Option<(i32,)> = sqlx::query_as(query)\n");
        for col in &pk_cols {
            let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
            code.push_str(&format!("            .bind({})\n", field));
        }
        code.push_str("            .fetch_optional(executor).await?;\n        Ok(exists.is_some())\n    }\n\n");

        if pk_cols.len() == 1 {
            let pk = pk_cols[0];
            let pk_rust_type = map_sql_type(&pk.data_type, pk.is_nullable, dialect);
            code.push_str("    /// Cursor-based Pagination (Keyset Pagination).\n");
            code.push_str("    /// \n");
            code.push_str("    /// **Performance:** This is the SOTA (State of the Art) standard for pagination.\n");
            code.push_str("    /// Unlike `OFFSET` which scans and discards thousands of rows, this jumps immediately to the \n");
            code.push_str("    /// correct row using the B-Tree index, offering O(1) constant-time absolute performance.\n");
            code.push_str(&format!("    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, last_id: &{}, limit: u32) -> sqlx::Result<Vec<Self>> {{\n", dialect.db_type(), pk_rust_type));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {} > {} ORDER BY {} ASC LIMIT {}\";\n", table.name, dialect.escape_sql_col(&pk.name), dialect.ph(1), dialect.escape_sql_col(&pk.name), dialect.ph(2)));
            if dialect == DbDialect::Postgres {
                 code.push_str("        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await\n");
            } else {
                 code.push_str("        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit).fetch_all(executor).await\n");
            }
            code.push_str("    }\n\n");
        }
    }

    // 3. --- OPÉRATIONS LIÉES AUX INDEX ---
    for idx in &table.indexes {
        let func_suffix = idx.columns.iter().map(|c| format!("{}", AsSnakeCase(c))).collect::<Vec<_>>().join("_and_");
        
        let mut idx_params = Vec::new();
        let mut where_clauses = String::new();
        let mut bind_calls = String::new();
        let mut stream_binds = String::new();
        
        for (i, c) in idx.columns.iter().enumerate() {
            if let Some(col) = table.columns.iter().find(|col| &col.name == c) {
                let snake = format!("{}", AsSnakeCase(c));
                let field = escape_rust_keyword(&snake);
                idx_params.push(format!("{}: &{}", field, map_sql_type(&col.data_type, col.is_nullable, dialect)));
                
                if i > 0 { where_clauses.push_str(" AND "); }
                where_clauses.push_str(&format!("{} = {}", dialect.escape_sql_col(c), dialect.ph(i + 1)));

                bind_calls.push_str(&format!(".bind({})", field));
                stream_binds.push_str(&format!(".bind({}.clone())", field));
            }
        }
        let params_str = idx_params.join(", ");

        code.push_str(&format!("    /// Checks if a record exists using the `{}` index.\n", idx.name));
        code.push_str("    /// \n");
        code.push_str("    /// **Performance:** Extremely fast, uses `SELECT 1 ... LIMIT 1`.\n");
        code.push_str(&format!("    pub async fn exists_by_{}<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<bool> {{\n", func_suffix, dialect.db_type(), params_str));
        code.push_str(&format!("        let query = \"SELECT 1 FROM {} WHERE {} LIMIT 1\";\n", table.name, where_clauses));
        code.push_str(&format!("        let exists: Option<(i32,)> = sqlx::query_as(query){}.fetch_optional(executor).await?;\n", bind_calls));
        code.push_str("        Ok(exists.is_some())\n");
        code.push_str("    }\n\n");

        if idx.is_unique {
            code.push_str(&format!("    /// Retrieves a single record using the unique `{}` index.\n", idx.name));
            code.push_str(&format!("    pub async fn get_by_{}<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<Option<Self>> {{\n", func_suffix, dialect.db_type(), params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch_optional(executor).await\n", bind_calls));
            code.push_str("    }\n\n");
        } else {
            code.push_str(&format!("    /// Retrieves all records matching the `{}` index.\n", idx.name));
            code.push_str(&format!("    pub async fn list_by_{}<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}) -> sqlx::Result<Vec<Self>> {{\n", func_suffix, dialect.db_type(), params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch_all(executor).await\n", bind_calls));
            code.push_str("    }\n\n");

            code.push_str(&format!("    /// Creates a zero-allocation Asynchronous Stream using the `{}` index.\n", idx.name));
            code.push_str(&format!("    pub fn stream_by_{}<'e, E: sqlx::Executor<'e, Database = {}> + 'e>(executor: E, {}) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n", func_suffix, dialect.db_type(), params_str));
            code.push_str(&format!("        let query = \"SELECT * FROM {} WHERE {}\";\n", table.name, where_clauses));
            code.push_str(&format!("        sqlx::query_as::<_, Self>(query){}.fetch(executor)\n", stream_binds));
            code.push_str("    }\n\n");
        }
    }

    code.push_str("}\n\n");
    code
}

// --- GÉNÉRATEUR DE LA STRUCTURE PATCH (UPDATE PARTIEL) ---

fn generate_patch_struct(table: &Table, dialect: DbDialect) -> String {
    let mut code = String::new();
    let struct_name = format!("{}", AsPascalCase(&table.name));
    let patch_struct_name = format!("{}Patch", struct_name);

    code.push_str(&format!("/// Structure used for partial updates (Patching) of `{}`.\n", table.name));
    code.push_str("/// \n");
    code.push_str("/// Each field is wrapped in an `Option`. If a field is `None`, it will be completely ignored during the update.\n");
    code.push_str("/// If it is `Some(value)`, that column will be updated in the database.\n");
    code.push_str("#[derive(Debug, Clone, Default)]\n");
    code.push_str(&format!("pub struct {} {{\n", patch_struct_name));

    for col in &table.columns {
        if col.is_primary { continue; } 
        
        let rust_type = map_sql_type(&col.data_type, col.is_nullable, dialect);
        let field_name = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        
        code.push_str(&format!("    pub {}: Option<{}>,\n", field_name, rust_type));
    }
    code.push_str("}\n\n");
    
    code
}

// --- GÉNÉRATEUR DE LA FONCTION UPDATE_PARTIAL_BY_PK ---

fn generate_partial_update_method(table: &Table, dialect: DbDialect) -> String {
    
    let pk_cols: Vec<&Column> = table.columns.iter().filter(|c| c.is_primary).collect();
    if pk_cols.is_empty() {
        return String::new(); // Pas de clé primaire, pas de mise à jour ciblée possible
    }
    
    let struct_name = format!("{}", AsPascalCase(&table.name));
    let patch_struct_name = format!("{}Patch", struct_name);
    let pk_args = pk_cols.iter().map(|c| format!("{}: &{}", escape_rust_keyword(&format!("{}", AsSnakeCase(&c.name))), map_sql_type(&c.data_type, c.is_nullable, dialect))).collect::<Vec<_>>().join(", ");

    let mut code = String::new();
    code.push_str(&format!("impl {} {{\n", struct_name));
    code.push_str("    /// Updates ONLY the columns that contain data in the `patch` struct.\n");
    code.push_str("    /// \n");
    code.push_str("    /// **Performance:** This is the most optimized way to update data.\n");
    code.push_str("    /// It dynamically builds the SQL query to only include the changed columns, which saves network bandwidth\n");
    code.push_str("    /// and significantly reduces database disk I/O (WAL logging) compared to a full row update.\n");
    
    code.push_str(&format!("    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = {}>>(executor: E, {}, patch: &{}) -> sqlx::Result<u64> {{\n", dialect.db_type(), pk_args, patch_struct_name));
    code.push_str(&format!("        let mut query_builder: sqlx::QueryBuilder<{}> = sqlx::QueryBuilder::new(\"UPDATE {} SET \");\n", dialect.db_type(), table.name));
    code.push_str("        let mut has_fields = false;\n");
    
    code.push_str("        let mut separated = query_builder.separated(\", \");\n\n");

    for col in &table.columns {
        if col.is_primary { continue; }
        
        let field_name = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        
        code.push_str(&format!("        if let Some(val) = &patch.{} {{\n", field_name));
        code.push_str("            has_fields = true;\n");
        code.push_str(&format!("            separated.push(\"{} = \");\n", dialect.escape_sql_col(&col.name)));
        code.push_str("            separated.push_bind_unseparated(val.clone());\n");
        code.push_str("        }\n");
    }

    code.push_str("\n        if !has_fields {\n");
    code.push_str("            // Si le patch est vide, on économise un aller-retour réseau\n");
    code.push_str("            return Ok(0);\n");
    code.push_str("        }\n\n");

    let mut is_first = true;
    for col in &pk_cols {
        if is_first {
            code.push_str(&format!("        query_builder.push(\" WHERE {} = \");\n", dialect.escape_sql_col(&col.name)));
            is_first = false;
        } else {
            code.push_str(&format!("        query_builder.push(\" AND {} = \");\n", dialect.escape_sql_col(&col.name)));
        }
        let field = escape_rust_keyword(&format!("{}", AsSnakeCase(&col.name)));
        code.push_str(&format!("        query_builder.push_bind({}.clone());\n", field));
    }

    code.push_str("\n        let result = query_builder.build().execute(executor).await?;\n");
    code.push_str("        Ok(result.rows_affected())\n");
    code.push_str("    }\n");
    code.push_str("}\n\n");

    code
}