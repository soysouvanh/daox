//! # DAOx Pipeline Code Generator
//!
//! Provides a "Database-First", zero-allocation Object-Relational Mapper logic framework.
//! It translates live database schemas (`information_schema`) into a strongly-typed `schema/` TOML dictionary
//! which is then materialized into memory-safe Rust Structs.
//!
//! ## Generated API Reference
//! Each generated Model struct yields the following SOTA (State of the Art) API:
//! - **Reads**: `find_all`, `get_by_pk`, `exists_by_pk`, `count`
//! - **Writes**: `insert` (Returns specific Last-ID per dialect), `insert_many` (Batch processing)
//! - **Updates**: `update_by_pk` (Full Update), and granular partial updates (`update_{col}` for every mutable column)
//! - **Deletes**: `delete_by_pk`, `delete_all` (Fast truncations)
//! - **Indexes**: Dynamic multi-parameter generation `get_by_{colA}_and_{colB}` supporting unique constraints and B-Tree indexes!
//!
//! The execution strictly bypasses transactions for Reads, keeping overhead essentially zero via `Requestor Pool`.

use serde::Deserialize;
use sqlx::Row;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Property<T> {
    Complex {
        value: T,
        message: Option<String>,
        default: Option<String>,
    },
    Simple(T),
}

impl<T: Clone> Property<T> {
    pub fn value(&self) -> T {
        match self {
            Property::Complex { value, .. } => value.clone(),
            Property::Simple(v) => v.clone(),
        }
    }

    pub fn message(&self) -> Option<String> {
        match self {
            Property::Complex { message, .. } => message.clone(),
            Property::Simple(_) => None,
        }
    }

    pub fn default_val(&self) -> Option<String> {
        match self {
            Property::Complex { default, .. } => default.clone(),
            Property::Simple(_) => None,
        }
    }
}

fn default_prop_false() -> Property<bool> {
    Property::Simple(false)
}

///  `ColumnMetadata`: The In-Memory Representation of a Data Dictionary Column.
///
/// This struct strictly maps to the individual `<column>.toml` files generated in the
/// `schema/` directory. It acts as the "Single Source of Truth" for the entire framework,
/// dictating not just the Rust type, but also runtime validation rules.
#[derive(Debug, Deserialize, Clone)]
pub struct ColumnMetadata {
    /// The expected Rust type string (e.g., `"i32"`, `"String"`, `"chrono::DateTime<chrono::Utc>"`).
    #[serde(rename = "type")]
    pub rust_type: Property<String>,

    /// If `true`, the SQL column allows NULL and the Rust field will be wrapped in `Option<T>`.
    pub is_optional: Property<bool>,

    /// Minimum required length (used later by Handlers for immediate Fail-Fast payload validation).
    pub min_length: Option<Property<usize>>,

    /// Maximum allowed length (aligns with `VARCHAR(n)` size, used for validation).
    pub max_length: Option<Property<usize>>,

    pub min_value: Option<Property<i64>>,
    pub max_value: Option<Property<i64>>,

    /// Regex format rule (e.g., email or uuid pattern) used by the validation layer.
    pub format: Option<Property<String>>,

    /// Optimized array matcher, parsed and evaluated natively
    pub enum_values: Option<Property<Vec<String>>>,

    /// Indicates if the column is part of the Primary Key. Triggers generation of `get_by_pk`, `update_by_pk`.
    #[serde(default = "default_prop_false")]
    pub is_primary_key: Property<bool>,

    /// Indicates if the database handles the increment. Automatically excludes the column from the `insert()` signature.
    #[serde(default = "default_prop_false")]
    pub is_auto_increment: Property<bool>,

    #[serde(default = "default_prop_false")]
    pub is_unique: Property<bool>,

    #[serde(default = "default_prop_false")]
    pub is_index: Property<bool>,

    #[serde(default)]
    pub business_rules: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TableConfig {
    pub database: String,
    pub description: Option<String>,
}

///  `ColumnOverride`: The manual override configuration.
///
/// Contains optional fields to allow humans to override introspection defaults.
/// Only the values defined in the TOML file will override the SQL-derived metadata.
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct ColumnOverride {
    #[serde(rename = "type")]
    pub rust_type: Option<Property<String>>,
    pub is_optional: Option<Property<bool>>,
    pub min_length: Option<Property<usize>>,
    pub max_length: Option<Property<usize>>,
    pub min_value: Option<Property<i64>>,
    pub max_value: Option<Property<i64>>,
    pub format: Option<Property<String>>,
    pub enum_values: Option<Property<Vec<String>>>,
    pub is_unique: Option<Property<bool>>,
    pub is_index: Option<Property<bool>>,
    pub business_rules: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct TableConfigOverride {
    pub database: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct IndexMetadata {
    pub name: String,
    #[serde(default)]
    pub is_unique: bool,
    pub columns: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, Default)]
pub struct TableIndexesConfig {
    #[serde(default)]
    pub indexes: Vec<IndexMetadata>,
}

pub type TableDirParseResult = Result<
    (
        TableConfig,
        HashMap<String, ColumnMetadata>,
        Vec<IndexMetadata>,
    ),
    Box<dyn std::error::Error>,
>;

///  `TableMetadata`: The In-Memory Representation of a Table.
#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub sql_table_name: Option<String>,
    pub config: TableConfig,
    pub columns: HashMap<String, ColumnMetadata>,
    pub indexes: Vec<IndexMetadata>,
}

///  `DaoGenerator`: The core orchestrator of the LightX code generation pipeline.
///
/// It performs two distinct tasks:
/// 1. `introspect_mysql`: Reads live SQL metadata to generate TOML files (Database-First).
/// 2. `generate_dao`: Parses the TOML files to produce zero-overhead, fully typed Rust code (`lightx_dao_generated.rs`).
pub struct DaoGenerator {
    schema_dir: String,
    out_dir: String,
}

impl DaoGenerator {
    pub fn new(schema_dir: &str, out_dir: &str) -> Self {
        Self {
            schema_dir: schema_dir.to_string(),
            out_dir: out_dir.to_string(),
        }
    }

    /// Introspects the database (automatically bypassed for Postgres/SQLite relying on offline schema)
    pub async fn introspect(&self, database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let is_postgres = database_url.starts_with("postgres");
        let is_sqlite = database_url.starts_with("sqlite");
        let fallback_dialect = if is_postgres {
            "postgres"
        } else if is_sqlite {
            "sqlite"
        } else {
            "mysql"
        };

        let schema_path = Path::new(&self.schema_dir);
        fs::create_dir_all(schema_path)?;

        // Ensure global databases.toml exists
        let databases_toml_path = schema_path.join("databases.toml");
        if !databases_toml_path.exists() {
            let databases_toml_content = format!(
                r#"# ==============================================================================
# MULTI-DATABASE CONFIGURATION (databases.toml)
# ==============================================================================
# This file allows the LightX framework to configure the data access layer.
# LightX natively supports a multi-database architecture (e.g., MySQL for users,
# PostgreSQL for analytics).
#
# SECTION HEADER EXPLANATION (e.g., [default]):
# The section header (here `default`) represents the internal database identifier.
# - Format: Must be in `snake_case` (lowercase letters, numbers, and underscores `_`).
# - Usage 1 (Rust Code): It will automatically generate the transaction method in the RequestContext.
#               Example: [default] will generate `ctx.get_or_create_default_tx()`.
# - Usage 2 (Tables): It will be used in the `database = "..."` key of your `_table.toml` files
#               to indicate which database each table belongs to.
#
# USAGE RULES:
# 1. Each block defines a distinct connection Pool.
# 2. The environment variable will be expected in the format `{{UPPERCASE_IDENTIFIER}}_DATABASE_URL`.
#    Example: for [default], define `DEFAULT_DATABASE_URL` in your `.env` file.
#             (NB: DATABASE_URL alone is used by default during the introspection build).
#
# SUPPORTED PROPERTIES:
# - dialect     : (Required) "mysql", "postgres", or "sqlite".
# - description : (Optional) Business explanation for the development team.
#
# MULTI-DATABASE EXAMPLE (TO COPY/PASTE):
#
# [default]
# dialect = "mysql"
# description = "Main database containing users"
#
# [analytics]
# dialect = "postgres"
# description = "Telemetry database"
#
# ==============================================================================

[default]
dialect = "{}"
description = "Default main database autogenerated by LightX"
"#,
                fallback_dialect
            );
            fs::write(&databases_toml_path, databases_toml_content)?;
        }

        if is_postgres {
            println!(
                "cargo:warning= PostgreSQL live introspection is currently bypassed. Relying purely on offline definitions in TOML schemas."
            );
            return Ok(());
        } else if is_sqlite {
            println!(
                "cargo:warning= SQLite live introspection is currently bypassed. Relying purely on offline definitions in TOML schemas."
            );
            return Ok(());
        }

        #[cfg(not(feature = "mysql"))]
        {
            println!("cargo:warning= MySQL feature is disabled. Bypassing live introspection.");
            return Ok(());
        }

        #[cfg(feature = "mysql")]
        let pool = sqlx::mysql::MySqlPoolOptions::new()
            .max_connections(2)
            .connect(database_url)
            .await?;

        // 1. Fetch all tables from current database
        let tables_query = "SELECT CAST(TABLE_NAME AS CHAR) AS TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()";
        let tables: Vec<String> = sqlx::query(tables_query)
            .fetch_all(&pool)
            .await?
            .into_iter()
            .map(|row| row.get::<String, _>("TABLE_NAME"))
            .collect();

        for table_name in tables {
            let table_dir = schema_path.join(&table_name);
            fs::create_dir_all(&table_dir)?;

            let table_toml = table_dir.join("_table.toml");
            if !table_toml.exists() {
                fs::write(&table_toml, "database = \"default\"\n")?;
            }

            // 2. Fetch all columns for the table
            let cols_query = r#"
                SELECT 
                    CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME, 
                    CAST(DATA_TYPE AS CHAR) AS DATA_TYPE, 
                    CAST(IS_NULLABLE AS CHAR) AS IS_NULLABLE, 
                    CAST(COLUMN_KEY AS CHAR) AS COLUMN_KEY, 
                    CAST(EXTRA AS CHAR) AS EXTRA, 
                    CAST(COLUMN_TYPE AS CHAR) AS COLUMN_TYPE,
                    CHARACTER_MAXIMUM_LENGTH 
                FROM information_schema.COLUMNS 
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
            "#;

            let columns = sqlx::query(cols_query)
                .bind(&table_name)
                .fetch_all(&pool)
                .await?;

            for col in columns {
                let col_name: String = col.get("COLUMN_NAME");
                let data_type: String = col.get("DATA_TYPE");
                let is_nullable: String = col.get("IS_NULLABLE"); // "YES" or "NO"
                let column_key: String = col.get("COLUMN_KEY"); // "PRI"
                let extra: String = col.get("EXTRA"); // e.g. "auto_increment"
                let column_type: String = col.get("COLUMN_TYPE"); // e.g. "enum('A','B')"
                // Handle CHARACTER_MAXIMUM_LENGTH which can be NULL
                let max_length_val: Option<i32> =
                    col.try_get("CHARACTER_MAXIMUM_LENGTH").unwrap_or(None);

                // SQL to Rust strict mapping
                let rust_type = match data_type.as_str() {
                    "bigint" => "i64",
                    "int" | "integer" => "i32",
                    "smallint" | "tinyint" => "i16",
                    "varchar" | "text" | "enum" => "String",
                    "timestamp" | "datetime" => "chrono::DateTime<chrono::Utc>",
                    "date" => "chrono::NaiveDate",
                    "json" => "serde_json::Value",
                    "binary" | "blob" => "Vec<u8>",
                    _ => "String",
                };

                let is_optional = is_nullable == "YES";
                let is_primary_key = column_key == "PRI";
                let is_unique = column_key == "UNI";
                let is_index = column_key == "MUL";
                let is_auto_increment = extra.contains("auto_increment");

                let min_length = if is_optional { 0 } else { 1 };

                let mut min_val: Option<i64> = None;
                let mut max_val: Option<i64> = None;

                let numeric_max_len = match rust_type {
                    "i16" => {
                        min_val = Some(0);
                        max_val = Some(32767);
                        Some(5)
                    }
                    "i32" => {
                        min_val = Some(0);
                        max_val = Some(2147483647);
                        Some(10)
                    }
                    "i64" => {
                        min_val = Some(0);
                        max_val = Some(9223372036854775807);
                        Some(19)
                    }
                    _ => None,
                };

                let inferred_max_length = max_length_val.map(|v| v as usize).or(numeric_max_len);

                let mut enum_values_toml = String::new();
                let format_regex = if col_name.starts_with("is_") || col_name.starts_with("has_") {
                    "^[01]$".to_string()
                } else if data_type == "enum" {
                    let mut vals_str = column_type
                        .replace("enum(", "")
                        .replace(")", "")
                        .replace("'", "");
                    let vals_arr = vals_str
                        .split(',')
                        .map(|s| format!("\"{}\"", s))
                        .collect::<Vec<_>>()
                        .join(", ");
                    enum_values_toml = format!(
                        "[enum_values]\nvalue = [{}]\nmessage = \"schema.enum_values.message\"\n\n",
                        vals_arr
                    );
                    vals_str = vals_str.replace(",", "|");
                    format!("^({})$", vals_str)
                } else if rust_type == "String" {
                    if col_name.contains("email") {
                        r#"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#.to_string()
                    } else if col_name.contains("password") {
                        r#"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$"#.to_string()
                    } else {
                        "^[À-ÿA-Za-z0-9_ -]*$".to_string() // Slightly restricted generic string
                    }
                } else if rust_type == "bool" {
                    "^[01]$".to_string()
                } else if col_name == "id"
                    || col_name.ends_with("_id")
                    || rust_type == "i16"
                    || rust_type == "i32"
                    || rust_type == "i64"
                {
                    if let Some(ml) = inferred_max_length {
                        format!("^[0-9]{{1,{}}}$", ml)
                    } else {
                        "^[0-9]+$".to_string()
                    }
                } else if rust_type == "chrono::NaiveDateTime"
                    || rust_type == "chrono::DateTime<chrono::Utc>"
                {
                    r#"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$"#.to_string()
                } else {
                    "^.*$".to_string()
                };

                let mut toml_content = String::new();

                toml_content.push_str(&format!(
                    "[type]\nvalue = \"{}\"\nmessage = \"schema.type.message\"\n\n",
                    rust_type
                ));
                let opt_msg = if is_optional {
                    "\"\""
                } else {
                    "\"schema.is_optional.message\""
                };
                let default_line = if is_optional {
                    let default_val_str = match rust_type {
                        "bool" => "false",
                        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32"
                        | "f64" => "0",
                        _ => "",
                    };
                    format!("default = \"{}\"\n", default_val_str)
                } else {
                    "".to_string()
                };

                toml_content.push_str(&format!(
                    "[is_optional]\nvalue = {}\nmessage = {}\n{}\n",
                    is_optional, opt_msg, default_line
                ));

                if !enum_values_toml.is_empty() {
                    toml_content.push_str(&enum_values_toml);
                } else {
                    toml_content.push_str("# [enum_values]\n# value = []\n# message = \n\n");
                }

                if let Some(len) = inferred_max_length {
                    toml_content.push_str(&format!(
                        "[max_length]\nvalue = {}\nmessage = \"schema.max_length.message|{}\"\n\n",
                        len, len
                    ));
                }

                toml_content.push_str(&format!(
                    "[min_length]\nvalue = {}\nmessage = \"schema.min_length.message|{}\"\n\n",
                    min_length, min_length
                ));

                if let Some(min_v) = min_val {
                    toml_content.push_str(&format!(
                        "[min_value]\nvalue = {}\nmessage = \"schema.min_value.message|{}\"\n\n",
                        min_v, min_v
                    ));
                } else {
                    toml_content.push_str("# [min_value]\n# value = \n# message = \n\n");
                }

                if let Some(max_v) = max_val {
                    toml_content.push_str(&format!(
                        "[max_value]\nvalue = {}\nmessage = \"schema.max_value.message|{}\"\n\n",
                        max_v, max_v
                    ));
                } else {
                    toml_content.push_str("# [max_value]\n# value = \n# message = \n\n");
                }

                toml_content.push_str(&format!(
                    "[format]\nvalue = '{}'\nmessage = \"schema.format.message\"\n\n",
                    format_regex
                ));

                toml_content.push_str(&format!("[is_primary_key]\nvalue = {}\n\n", is_primary_key));
                toml_content.push_str(&format!("[is_unique]\nvalue = {}\n\n", is_unique));
                toml_content.push_str(&format!("[is_index]\nvalue = {}\n\n", is_index));
                toml_content.push_str(&format!(
                    "[is_auto_increment]\nvalue = {}\n\n",
                    is_auto_increment
                ));
                toml_content.push_str("[business_rules]\n\n");

                let col_file = table_dir.join(format!("{}.toml", col_name));
                // We overwrite to ensure it's synced with the actual DB
                fs::write(col_file, toml_content)?;
            }

            // 3. Fetch composite/named indexes for the table (excluding PRIMARY)
            let idx_query = r#"
                SELECT 
                    CAST(INDEX_NAME AS CHAR) AS INDEX_NAME, 
                    CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME, 
                    NON_UNIQUE 
                FROM information_schema.STATISTICS 
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ? AND INDEX_NAME != 'PRIMARY'
                ORDER BY INDEX_NAME, SEQ_IN_INDEX
            "#;
            let indexes_rows = sqlx::query(idx_query)
                .bind(&table_name)
                .fetch_all(&pool)
                .await?;
            let mut grouped_indexes: std::collections::HashMap<String, (bool, Vec<String>)> =
                std::collections::HashMap::new();

            for row in indexes_rows {
                let idx_name: String = row.get("INDEX_NAME");
                let col_name: String = row.get("COLUMN_NAME");
                let non_unique: i64 = row.get("NON_UNIQUE");
                let is_unique = non_unique == 0;
                let entry = grouped_indexes
                    .entry(idx_name)
                    .or_insert((is_unique, Vec::new()));
                entry.1.push(col_name);
            }

            let idx_file = table_dir.join("_indexes.toml");
            if !grouped_indexes.is_empty() {
                let mut indexes_toml = String::new();
                for (idx_name, (is_unique, cols)) in grouped_indexes {
                    indexes_toml.push_str("[[indexes]]\n");
                    indexes_toml.push_str(&format!("name = \"{}\"\n", idx_name));
                    indexes_toml.push_str(&format!("is_unique = {}\n", is_unique));
                    let cols_str = cols
                        .iter()
                        .map(|c| format!("\"{}\"", c))
                        .collect::<Vec<_>>()
                        .join(", ");
                    indexes_toml.push_str(&format!("columns = [{}]\n\n", cols_str));
                }
                fs::write(&idx_file, indexes_toml)?;
            } else if idx_file.exists() {
                fs::remove_file(&idx_file)?;
            }
        }

        println!("cargo:warning= Database introspection complete. TOML schemas generated.");
        Ok(())
    }

    ///  Step 1 : Parsing the Data Dictionary (TOML)
    ///
    /// This method traverses the `/schema` directory and builds the Single Source of Truth
    /// in memory, before generating the actual code.
    pub fn parse_schema(&self) -> Result<Vec<TableMetadata>, Box<dyn std::error::Error>> {
        let mut tables = Vec::new();
        let schema_path = Path::new(&self.schema_dir);

        if !schema_path.exists() || !schema_path.is_dir() {
            println!(
                "cargo:warning= Schema directory does not exist: {}",
                self.schema_dir
            );
            return Ok(tables);
        }

        let parse_table_dir = |dir_path: &Path| -> TableDirParseResult {
            let mut columns = HashMap::new();
            let mut config = TableConfig::default();
            let mut indexes = Vec::new();
            for col_entry in fs::read_dir(dir_path)? {
                let col_entry = col_entry?;
                let col_path = col_entry.path();
                if col_path.is_file()
                    && col_path.extension().and_then(|s| s.to_str()) == Some("toml")
                {
                    let file_stem = col_path.file_stem().unwrap().to_str().unwrap().to_string();
                    let content = fs::read_to_string(&col_path)?;
                    if file_stem == "_table" {
                        config = toml::from_str(&content).map_err(|e| {
                            format!("Strict parsing error in {:?}: {}", col_path, e)
                        })?;
                        continue;
                    }
                    if file_stem == "_indexes" {
                        let parsed: TableIndexesConfig = toml::from_str(&content).map_err(|e| {
                            format!("Indexes parsing error in {:?}: {}", col_path, e)
                        })?;
                        indexes = parsed.indexes;
                        continue;
                    }
                    let col_meta: ColumnMetadata = toml::from_str(&content)
                        .map_err(|e| format!("Strict parsing error in {:?}: {}", col_path, e))?;
                    columns.insert(file_stem, col_meta);
                }
            }
            Ok((config, columns, indexes))
        };

        // 1. Traverse directories representing tables (FLAT MODE) or databases (NESTED MODE)
        for entry in fs::read_dir(schema_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && entry.file_name() == "databases.toml" {
                continue;
            }

            if path.is_dir() {
                let mut has_toml = false;
                if let Ok(sub_entries) = fs::read_dir(&path) {
                    for sub in sub_entries.flatten() {
                        if sub.path().is_file()
                            && sub.path().extension().and_then(|s| s.to_str()) == Some("toml")
                        {
                            has_toml = true;
                            break;
                        }
                    }
                }

                if has_toml {
                    // Flat Mode (Legacy)
                    let table_name = entry.file_name().into_string().unwrap();
                    let (config, columns, indexes) = parse_table_dir(&path)?;
                    tables.push(TableMetadata {
                        name: table_name,
                        sql_table_name: None,
                        config,
                        columns,
                        indexes,
                    });
                } else {
                    // Nested Mode (Multi-Database Routing)
                    let db_name = entry.file_name().into_string().unwrap();
                    if let Ok(sub_entries) = fs::read_dir(&path) {
                        for sub in sub_entries.flatten() {
                            if sub.path().is_dir() {
                                let table_name = sub.file_name().into_string().unwrap();
                                let (mut config, columns, indexes) = parse_table_dir(&sub.path())?;
                                config.database = db_name.clone();
                                let prefixed = format!("{}_{}", db_name, table_name);
                                tables.push(TableMetadata {
                                    name: prefixed,
                                    sql_table_name: Some(table_name),
                                    config,
                                    columns,
                                    indexes,
                                });
                            }
                        }
                    }
                }
            }
        }

        // --- OVERRIDE STRATEGY ---
        // Reads the `/overrides` directory and overrides in-memory values seamlessly.
        let overrides_path = std::env::var("LIGHTX_OVERRIDES_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| schema_path.parent().unwrap().join("overrides"));

        if !overrides_path.exists() {
            fs::create_dir_all(&overrides_path)?;

            let readme_en_content = r#"#  LightX Overrides Strategy

Welcome to the overrides directory!

The "Database-First" philosophy of LightX completely overwrites the `schema/` directory on each compilation.
**Never modify the files in `schema/` as they will be overwritten!**

If you want to override business validation rules (e.g., forcing `min_length = 5` on an SQL column), you must replicate the table's directory structure here.

## Example
To override the `last_name` column of the `users` table:
1. Create a `users/` directory here.
2. Create the file `users/last_name.toml` here with ONLY the values you wish to override.

```toml
[min_length]
value = 5
message = "The last name must be at least 5 characters long (Manual override!)"
```
"#;
            fs::write(overrides_path.join("README.md"), readme_en_content)?;

            let readme_fr_content = r#"#  LightX Overrides Strategy

Bienvenue dans le dossier d'overrides !

La philosophie "Database-First" de LightX écrase entièrement le dossier `schema/` à chaque compilation.
**Ne modifiez jamais les fichiers dans `schema/` car ils seront écrasés !**

Si vous souhaitez surcharger des règles de validation métier (ex: forcer `min_length = 5` sur une colonne SQL), vous devez reproduire l'arborescence de la table ici.

## Exemple
Pour surcharger la colonne `last_name` de la table `users` :
1. Créez le dossier `users/` ici.
2. Créez le fichier `users/last_name.toml` ici avec UNIQUEMENT les valeurs à écraser.

```toml
[min_length]
value = 5
message = "Le nom de famille doit faire au moins 5 caractères (Surcharge manuelle !)"
```
"#;
            fs::write(overrides_path.join("README.fr.md"), readme_fr_content)?;
        }

        if overrides_path.is_dir() {
            for entry in fs::read_dir(overrides_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let table_name = entry.file_name().into_string().unwrap();
                    if let Some(table) = tables.iter_mut().find(|t| t.name == table_name) {
                        for col_entry in fs::read_dir(&path)? {
                            let col_entry = col_entry?;
                            let col_path = col_entry.path();
                            if col_path.is_file()
                                && col_path.extension().and_then(|s| s.to_str()) == Some("toml")
                            {
                                let file_stem =
                                    col_path.file_stem().unwrap().to_str().unwrap().to_string();
                                let content = fs::read_to_string(&col_path)?;

                                if file_stem == "_table" {
                                    if let Ok(config_override) =
                                        toml::from_str::<TableConfigOverride>(&content)
                                    {
                                        if let Some(v) = config_override.database {
                                            table.config.database = v;
                                        }
                                        if let Some(v) = config_override.description {
                                            table.config.description = Some(v);
                                        }
                                    }
                                    continue;
                                }
                                if file_stem == "_indexes" {
                                    if let Ok(idx_override) =
                                        toml::from_str::<TableIndexesConfig>(&content)
                                    {
                                        table.indexes = idx_override.indexes;
                                    }
                                    continue;
                                }

                                if let Some(col_meta) = table.columns.get_mut(&file_stem) {
                                    let col_override: ColumnOverride = toml::from_str(&content)
                                        .map_err(|e| {
                                            format!(
                                                "Override parsing error in {:?}: {}",
                                                col_path, e
                                            )
                                        })?;
                                    if let Some(v) = col_override.rust_type {
                                        col_meta.rust_type = v;
                                    }
                                    if let Some(v) = col_override.is_optional {
                                        col_meta.is_optional = v;
                                    }
                                    if let Some(v) = col_override.min_length {
                                        col_meta.min_length = Some(v);
                                    }
                                    if let Some(v) = col_override.max_length {
                                        col_meta.max_length = Some(v);
                                    }
                                    if let Some(v) = col_override.min_value {
                                        col_meta.min_value = Some(v);
                                    }
                                    if let Some(v) = col_override.max_value {
                                        col_meta.max_value = Some(v);
                                    }
                                    if let Some(v) = col_override.format {
                                        col_meta.format = Some(v);
                                    }
                                    if let Some(v) = col_override.is_unique {
                                        col_meta.is_unique = v;
                                    }
                                    if let Some(v) = col_override.is_index {
                                        col_meta.is_index = v;
                                    }
                                    if let Some(v) = col_override.business_rules {
                                        col_meta.business_rules.extend(v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort by table name to guarantee deterministic code generation
        tables.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(tables)
    }

    ///  Helper to escape reserved Rust keywords (e.g., 'type' -> 'r#type')
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

    ///  Helper to convert snake_case to PascalCase (e.g., 'users' -> 'Users')
    fn to_pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        for c in s.chars() {
            if c == '_' || c == '-' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        result
    }

    ///  Executes the complete DAO Generation pipeline (Steps 2, 3, and 4)
    pub fn generate_dao(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tables = self.parse_schema()?;

        // Step 2 : Generation Target (OUT_DIR)
        let dest_path = Path::new(&self.out_dir).join("lightx_dao_generated.rs");
        let mut code = String::from(
            "// =====================================================================\n// THIS FILE IS AUTO-GENERATED BY LIGHTX. DO NOT EDIT.\n// =====================================================================\n#![allow(clippy::needless_borrows_for_generic_args, clippy::collapsible_if)]\n\n",
        );

        // --- GENERATING RequestContext ---
        let databases_toml_path = Path::new(&self.schema_dir).join("databases.toml");
        let databases_content = fs::read_to_string(&databases_toml_path)?;
        let databases: toml::Value = toml::from_str(&databases_content)?;
        let db_keys: Vec<String> = databases.as_table().unwrap().keys().cloned().collect();

        let get_dialect = |db_name: &str| -> String {
            databases
                .get(db_name)
                .and_then(|t| t.as_table())
                .and_then(|t| t.get("dialect"))
                .and_then(|v| v.as_str())
                .unwrap_or("mysql")
                .to_string()
        };
        let get_pool_type = |dialect: &str| -> &str {
            if dialect == "postgres" {
                "sqlx::PgPool"
            } else if dialect == "sqlite" {
                "sqlx::SqlitePool"
            } else {
                "sqlx::MySqlPool"
            }
        };
        let get_tx_type = |dialect: &str| -> &str {
            if dialect == "postgres" {
                "sqlx::Postgres"
            } else if dialect == "sqlite" {
                "sqlx::Sqlite"
            } else {
                "sqlx::MySql"
            }
        };
        let get_pool_options = |dialect: &str| -> &str {
            if dialect == "postgres" {
                "sqlx::postgres::PgPoolOptions"
            } else if dialect == "sqlite" {
                "sqlx::sqlite::SqlitePoolOptions"
            } else {
                "sqlx::mysql::MySqlPoolOptions"
            }
        };
        let get_fallback_url = |dialect: &str| -> &str {
            if dialect == "postgres" {
                "postgres://postgres:postgres@127.0.0.1:5432/my_database"
            } else if dialect == "sqlite" {
                "sqlite::memory:"
            } else {
                "mysql://root:root@127.0.0.1:3306/my_database"
            }
        };

        code.push_str("pub struct RequestContext {\n");
        code.push_str("    pub raw_body: lightx::ext::bytes::Bytes,\n");
        code.push_str("    pub global_state: std::sync::Arc<lightx::ext::tokio::sync::broadcast::Sender<lightx::ext::bytes::Bytes>>,\n");
        code.push_str("    pub rate_limiter: std::sync::Arc<lightx::ext::moka::sync::Cache<(std::net::IpAddr, &'static str), std::sync::Arc<std::sync::atomic::AtomicU32>>>,\n");
        code.push_str("    pub response_cache: std::sync::Arc<lightx::ext::moka::sync::Cache<String, (lightx::ext::hyper::StatusCode, lightx::ext::hyper::HeaderMap, lightx::ext::bytes::Bytes, std::time::Instant)>>,\n");
        code.push_str("    pub client_ip: std::net::IpAddr,\n");
        code.push_str("    pub user_id: std::option::Option<String>,\n");
        code.push_str("    pub headers: lightx::ext::hyper::HeaderMap,\n");
        code.push_str("    pub raw_req: Option<lightx::ext::hyper::Request<lightx::ext::hyper::body::Incoming>>,\n");
        for db in &db_keys {
            let dialect = get_dialect(db);
            code.push_str(&format!(
                "    pub {}_pool: {},\n",
                db,
                get_pool_type(&dialect)
            ));
            code.push_str(&format!(
                "    pub {}_tx: Option<sqlx::Transaction<'static, {}>>,\n",
                db,
                get_tx_type(&dialect)
            ));
        }
        code.push_str("}\n\n");

        code.push_str("pub struct AppContextFactory {\n");
        code.push_str("    pub global_state: std::sync::Arc<lightx::ext::tokio::sync::broadcast::Sender<lightx::ext::bytes::Bytes>>,\n");
        code.push_str("    pub rate_limiter: std::sync::Arc<lightx::ext::moka::sync::Cache<(std::net::IpAddr, &'static str), std::sync::Arc<std::sync::atomic::AtomicU32>>>,\n");
        code.push_str("    pub response_cache: std::sync::Arc<lightx::ext::moka::sync::Cache<String, (lightx::ext::hyper::StatusCode, lightx::ext::hyper::HeaderMap, lightx::ext::bytes::Bytes, std::time::Instant)>>,\n");
        for db in &db_keys {
            let dialect = get_dialect(db);
            code.push_str(&format!(
                "    pub {}_pool: {},\n",
                db,
                get_pool_type(&dialect)
            ));
        }
        code.push_str("}\n\n");

        code.push_str("impl AppContextFactory {\n");
        code.push_str("    pub async fn new() -> Result<Self, lightx::core::AppError> {\n");
        code.push_str("        Ok(Self {\n");
        code.push_str("            global_state: std::sync::Arc::new(lightx::ext::tokio::sync::broadcast::channel(1024).0),\n");
        code.push_str("            rate_limiter: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),\n");
        code.push_str("            response_cache: std::sync::Arc::new(lightx::ext::moka::sync::Cache::builder().build()),\n");
        for db in &db_keys {
            let dialect = get_dialect(db);
            code.push_str(&format!("            {}_pool: {}::new().connect(&std::env::var(\"{}_DATABASE_URL\").or_else(|_| std::env::var(\"DATABASE_URL\")).unwrap_or_else(|_| \"{}\".to_string())).await.map_err(|e| lightx::core::AppError::SystemError {{ msg: e.to_string(), file: file!(), line: line!() }})?,\n", db, get_pool_options(&dialect), db.to_uppercase(), get_fallback_url(&dialect)));
        }
        code.push_str("        })\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code.push_str("impl lightx::server::ContextFactory for AppContextFactory {\n");
        code.push_str("    type Context = RequestContext;\n");
        code.push_str("    fn create_context(&self, peer_addr: std::net::IpAddr, headers: lightx::ext::hyper::HeaderMap, raw_body: lightx::ext::bytes::Bytes, raw_req: Option<lightx::ext::hyper::Request<lightx::ext::hyper::body::Incoming>>) -> Self::Context {\n");
        code.push_str("        let mut client_ip = peer_addr;\n");
        code.push_str("        if let Some(xff) = headers.get(\"x-forwarded-for\") {\n");
        code.push_str("            if let Ok(s) = xff.to_str() {\n");
        code.push_str("                if let Some(first) = s.split(',').next() {\n");
        code.push_str("                    if let Ok(parsed) = first.trim().parse() { client_ip = parsed; }\n");
        code.push_str("                }\n");
        code.push_str("            }\n");
        code.push_str("        } else if let Some(xreal) = headers.get(\"x-real-ip\") {\n");
        code.push_str("            if let Ok(s) = xreal.to_str() {\n");
        code.push_str(
            "                if let Ok(parsed) = s.trim().parse() { client_ip = parsed; }\n",
        );
        code.push_str("            }\n");
        code.push_str("        }\n");
        code.push_str("        RequestContext {\n");
        code.push_str("            raw_body,\n");
        code.push_str("            global_state: self.global_state.clone(),\n");
        code.push_str("            rate_limiter: self.rate_limiter.clone(),\n");
        code.push_str("            response_cache: self.response_cache.clone(),\n");
        code.push_str("            client_ip,\n");
        code.push_str("            user_id: None,\n");
        code.push_str("            headers,\n");
        code.push_str("            raw_req,\n");
        for db in &db_keys {
            code.push_str(&format!(
                "            {}_pool: self.{}_pool.clone(),\n",
                db, db
            ));
            code.push_str(&format!("            {}_tx: None,\n", db));
        }
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        code.push_str("    fn commit_context<'a>(&'a self, ctx: &'a mut Self::Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send + 'a>> {\n");
        code.push_str("        Box::pin(async move {\n");
        for db in &db_keys {
            code.push_str(&format!(
                "            if let Some(tx) = ctx.{}_tx.take() {{\n",
                db
            ));
            code.push_str("                tx.commit().await.map_err(|e| e.to_string())?;\n");
            code.push_str("            }\n");
        }
        code.push_str("            Ok(())\n");
        code.push_str("        })\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code.push_str("impl RequestContext {\n");

        for db in &db_keys {
            let dialect = get_dialect(db);
            code.push_str(&format!("    pub async fn get_or_create_{}_tx(&mut self) -> Result<&mut sqlx::Transaction<'static, {}>, lightx::core::AppError> {{\n", db, get_tx_type(&dialect)));
            code.push_str(&format!("        if self.{}_tx.is_none() {{\n", db));
            code.push_str(&format!("            let tx = self.{}_pool.begin().await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db));
            code.push_str(&format!("            self.{}_tx = Some(tx);\n", db));
            code.push_str("        }\n");
            code.push_str(&format!("        self.{}_tx.as_mut().ok_or_else(|| lightx::core::AppError::SystemError {{ msg: \"Tx absent\".to_string(), file: file!(), line: line!() }})\n", db));
            code.push_str("    }\n\n");

            code.push_str(&format!("    pub async fn commit_{}_tx(&mut self) -> Result<(), lightx::core::AppError> {{\n", db));
            code.push_str(&format!(
                "        if let Some(tx) = self.{}_tx.take() {{\n",
                db
            ));
            code.push_str("            tx.commit().await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
            code.push_str("        }\n        Ok(())\n    }\n\n");

            code.push_str(&format!("    pub async fn rollback_{}_tx(&mut self) -> Result<(), lightx::core::AppError> {{\n", db));
            code.push_str(&format!(
                "        if let Some(tx) = self.{}_tx.take() {{\n",
                db
            ));
            code.push_str("            tx.rollback().await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
            code.push_str("        }\n        Ok(())\n    }\n");
        }
        code.push_str("}\n\n");

        println!(
            "cargo:warning= generate_dao processing {} tables for schema_dir {}",
            tables.len(),
            self.schema_dir
        );
        for table in tables {
            let struct_name = Self::to_pascal_case(&table.name);

            // Step 3 : Generating Rust Structures (Models)
            code.push_str("#[derive(Debug, Clone, sqlx::FromRow)]\n");
            code.push_str(&format!("pub struct {} {{\n", struct_name));

            // Sort by column name for absolute determinism of generated code
            let mut cols: Vec<_> = table.columns.iter().collect();
            cols.sort_by(|a, b| a.0.cmp(b.0));

            for (col_name, col_meta) in &cols {
                let rust_field = Self::escape_rust_keyword(col_name);
                let mut rust_type = col_meta.rust_type.value();

                // Handling optionality (NULL in DB)
                if col_meta.is_optional.value() {
                    rust_type = format!("Option<{}>", rust_type);
                }

                code.push_str(&format!("    pub {}: {},\n", rust_field, rust_type));
            }
            code.push_str("}\n\n");

            // Step 4 : Generating CRUD Operations
            code.push_str(&format!("impl {} {{\n", struct_name));

            // Separate columns according to their business role
            let mut pk_cols = Vec::new();
            let mut insert_cols = Vec::new();
            let mut update_cols = Vec::new();

            for (col_name, col_meta) in &cols {
                if col_meta.is_primary_key.value() {
                    pk_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
                if !col_meta.is_auto_increment.value() {
                    insert_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
                if !col_meta.is_primary_key.value() {
                    update_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
            }

            let pk_suffix = pk_cols
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<Vec<_>>()
                .join("_and_");

            let db_name = if table.config.database.is_empty() {
                "default".to_string()
            } else {
                table.config.database.clone()
            };
            let table_dialect = get_dialect(&db_name);
            let is_postgres = table_dialect == "postgres";
            let is_sqlite = table_dialect == "sqlite";

            // --- 1. INSERT ---
            if !insert_cols.is_empty() {
                let col_names = insert_cols
                    .iter()
                    .map(|(n, _)| {
                        if is_postgres {
                            format!("\"{}\"", n)
                        } else {
                            format!("`{}`", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let placeholders = insert_cols
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        if is_postgres {
                            format!("${}", i + 1)
                        } else {
                            "?".to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                code.push_str("    /// Inserts the current record. (Execution uses Pool unless a Transaction is active)\n");
                code.push_str("    pub async fn insert(&self, ctx: &mut RequestContext) -> Result<u64, lightx::core::AppError> {\n");
                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };
                code.push_str(&format!(
                    "        let mut query = sqlx::query(r#\"INSERT INTO {} ({}) VALUES ({})\"#);\n",
                    table_name_q, col_names, placeholders
                ));

                for (col_name, _) in &insert_cols {
                    let field = Self::escape_rust_keyword(col_name);
                    code.push_str(&format!("        query = query.bind(&self.{});\n", field));
                }
                let result_var = if is_postgres { "_result" } else { "result" };
                code.push_str(&format!(
                    "        let {} = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    result_var, db_name
                ));
                code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
                code.push_str("        };\n");
                if is_postgres {
                    code.push_str("        Ok(0) // Postgres RETURNING mapping natively bypassed in base generator\n");
                } else if is_sqlite {
                    code.push_str("        Ok(result.last_insert_rowid() as u64)\n");
                } else {
                    code.push_str("        Ok(result.last_insert_id())\n");
                }
                code.push_str("    }\n\n");

                // --- UPSERT ---
                if !pk_cols.is_empty() && !update_cols.is_empty() {
                    let conflict_keys = pk_cols
                        .iter()
                        .map(|(n, _)| {
                            if is_postgres {
                                format!("\"{}\"", n)
                            } else {
                                format!("`{}`", n)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    let update_clauses = update_cols
                        .iter()
                        .map(|(n, _)| {
                            if is_postgres {
                                format!("\"{}\" = EXCLUDED.\"{}\"", n, n)
                            } else if is_sqlite {
                                format!("`{}` = EXCLUDED.`{}`", n, n)
                            } else {
                                format!("`{}` = VALUES(`{}`)", n, n)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    let upsert_sql = if is_postgres || is_sqlite {
                        format!(
                            "INSERT INTO {} ({}) VALUES ({}) ON CONFLICT ({}) DO UPDATE SET {}",
                            table_name_q, col_names, placeholders, conflict_keys, update_clauses
                        )
                    } else {
                        format!(
                            "INSERT INTO {} ({}) VALUES ({}) ON DUPLICATE KEY UPDATE {}",
                            table_name_q, col_names, placeholders, update_clauses
                        )
                    };

                    code.push_str(
                        "    /// Upserts the current record (Insert or Update if PK conflicts).\n",
                    );
                    code.push_str("    pub async fn upsert(&self, ctx: &mut RequestContext) -> Result<(), lightx::core::AppError> {\n");
                    code.push_str(&format!(
                        "        let mut query = sqlx::query(r#\"{}\"#);\n",
                        upsert_sql
                    ));

                    for (col_name, _) in &insert_cols {
                        let field = Self::escape_rust_keyword(col_name);
                        code.push_str(&format!("        query = query.bind(&self.{});\n", field));
                    }

                    code.push_str(&format!(
                        "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                        db_name
                    ));
                    code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
                    code.push_str("        } else {\n");
                    code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db_name));
                    code.push_str("        }\n");
                    code.push_str("        Ok(())\n");
                    code.push_str("    }\n\n");
                }
            }

            // --- 2. GET_BY_PK ---
            if !pk_cols.is_empty() {
                let pk_args = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw_t = m.rust_type.value();
                        let arg_type = match raw_t.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), arg_type)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let pk_where = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        if is_postgres {
                            format!("\"{}\" = ${}", n, i + 1)
                        } else {
                            format!("`{}` = ?", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" AND ");

                code.push_str("    /// Retrieves a record via its primary key. (Zero-Transaction Overhead for pure reads)\n");
                code.push_str(&format!("    pub async fn get_by_{}(ctx: &mut RequestContext, {}) -> Result<Option<Self>, lightx::core::AppError> {{\n", pk_suffix, pk_args));
                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };

                let all_cols = cols
                    .iter()
                    .map(|(n, _)| {
                        if is_postgres {
                            format!("\"{}\"", n)
                        } else {
                            format!("`{}`", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                code.push_str(&format!(
                    "        let mut query = sqlx::query_as(r#\"SELECT {} FROM {} WHERE {}\"#);\n",
                    all_cols, table_name_q, pk_where
                ));

                for (col_name, m) in &pk_cols {
                    let field = Self::escape_rust_keyword(col_name);
                    let raw_t = m.rust_type.value();
                    if raw_t == "String" || raw_t == "Vec<u8>" {
                        code.push_str(&format!("        query = query.bind({});\n", field));
                    } else {
                        code.push_str(&format!("        query = query.bind(&{});\n", field));
                    }
                }

                code.push_str(&format!(
                    "        let record = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.fetch_optional(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.fetch_optional(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
                code.push_str("        };\n");
                code.push_str("        Ok(record)\n");
                code.push_str("    }\n\n");
            }

            // --- 3. UPDATE_BY_PK ---
            if !pk_cols.is_empty() && !update_cols.is_empty() {
                let set_clauses = update_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        if is_postgres {
                            format!("\"{}\" = ${}", n, i + 1)
                        } else {
                            format!("`{}` = ?", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let pk_where = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        if is_postgres {
                            format!("\"{}\" = ${}", n, i + 1 + update_cols.len())
                        } else {
                            format!("`{}` = ?", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" AND ");

                code.push_str("    /// Updates the entire record via its primary key. (Execution uses Pool unless a Transaction is active)\n");
                code.push_str(&format!("    pub async fn update_by_{}(&self, ctx: &mut RequestContext) -> Result<(), lightx::core::AppError> {{\n", pk_suffix));
                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };
                code.push_str(&format!(
                    "        let mut query = sqlx::query(r#\"UPDATE {} SET {} WHERE {}\"#);\n",
                    table_name_q, set_clauses, pk_where
                ));

                let mut all_binds = Vec::new();
                for (n, _) in &update_cols {
                    all_binds.push(n.to_string());
                }
                for (n, _) in &pk_cols {
                    all_binds.push(n.to_string());
                }

                for col_name in &all_binds {
                    let field = Self::escape_rust_keyword(col_name);
                    code.push_str(&format!("        query = query.bind(&self.{});\n", field));
                }
                code.push_str(&format!(
                    "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db_name));
                code.push_str("        }\n");
                code.push_str("        Ok(())\n");
                code.push_str("    }\n\n");
            }

            // --- 3.5. UPDATE_PARTIAL (Column-level PATCH) ---
            if !pk_cols.is_empty() && !update_cols.is_empty() {
                for (col_name, col_meta) in &update_cols {
                    let raw_t = col_meta.rust_type.value();
                    let arg_type = match raw_t.as_str() {
                        "String" => "&str",
                        "Vec<u8>" => "&[u8]",
                        other => other,
                    };

                    let pk_args = pk_cols
                        .iter()
                        .map(|(n, m)| {
                            let r_t = m.rust_type.value();
                            let a_t = match r_t.as_str() {
                                "String" => "&str",
                                "Vec<u8>" => "&[u8]",
                                other => other,
                            };
                            format!("{}: {}", Self::escape_rust_keyword(n), a_t)
                        })
                        .collect::<Vec<_>>()
                        .join(", ");

                    let pk_where = pk_cols
                        .iter()
                        .enumerate()
                        .map(|(i, (n, _))| {
                            if is_postgres {
                                format!("\"{}\" = ${}", n, i + 2)
                            } else {
                                format!("`{}` = ?", n)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" AND ");

                    let table_name_q = if is_postgres {
                        format!(
                            "\"{}\"",
                            table.sql_table_name.as_ref().unwrap_or(&table.name)
                        )
                    } else {
                        table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                    };
                    let set_clause = if is_postgres {
                        format!("\"{}\" = $1", col_name)
                    } else {
                        format!("`{}` = ?", col_name)
                    };

                    code.push_str(&format!("    pub async fn update_{}(ctx: &mut RequestContext, {}, new_val: {}) -> Result<(), lightx::core::AppError> {{\n", col_name, pk_args, arg_type));
                    code.push_str(&format!(
                        "        let mut query = sqlx::query(r#\"UPDATE {} SET {} WHERE {}\"#);\n",
                        table_name_q, set_clause, pk_where
                    ));

                    if raw_t == "String" || raw_t == "Vec<u8>" {
                        code.push_str("        query = query.bind(new_val);\n");
                    } else {
                        code.push_str("        query = query.bind(&new_val);\n");
                    }

                    for (pk_n, pk_m) in &pk_cols {
                        let pk_field = Self::escape_rust_keyword(pk_n);
                        let pk_r_t = pk_m.rust_type.value();
                        if pk_r_t == "String" || pk_r_t == "Vec<u8>" {
                            code.push_str(&format!("        query = query.bind({});\n", pk_field));
                        } else {
                            code.push_str(&format!("        query = query.bind(&{});\n", pk_field));
                        }
                    }

                    code.push_str(&format!(
                        "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                        db_name
                    ));
                    code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
                    code.push_str("        } else {\n");
                    code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db_name));
                    code.push_str("        }\n");
                    code.push_str("        Ok(())\n");
                    code.push_str("    }\n\n");
                }
            }

            // --- 4. DELETE_BY_PK ---
            if !pk_cols.is_empty() {
                let pk_args = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw_t = m.rust_type.value();
                        let arg_type = match raw_t.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), arg_type)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let pk_where = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        if is_postgres {
                            format!("\"{}\" = ${}", n, i + 1)
                        } else {
                            format!("`{}` = ?", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" AND ");

                code.push_str("    /// Permanently deletes the record. (Execution uses Pool unless a Transaction is active)\n");
                code.push_str(&format!("    pub async fn delete_by_{}(ctx: &mut RequestContext, {}) -> Result<(), lightx::core::AppError> {{\n", pk_suffix, pk_args));
                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };
                code.push_str(&format!(
                    "        let mut query = sqlx::query(r#\"DELETE FROM {} WHERE {}\"#);\n",
                    table_name_q, pk_where
                ));

                for (col_name, m) in &pk_cols {
                    let field = Self::escape_rust_keyword(col_name);
                    let raw_t = m.rust_type.value();
                    if raw_t == "String" || raw_t == "Vec<u8>" {
                        code.push_str(&format!("        query = query.bind({});\n", field));
                    } else {
                        code.push_str(&format!("        query = query.bind(&{});\n", field));
                    }
                }
                code.push_str(&format!(
                    "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db_name));
                code.push_str("        }\n");
                code.push_str("        Ok(())\n");
                code.push_str("    }\n\n");
            }

            // --- 4.5. DELETE_ALL ---
            code.push_str(
                "    /// Deletes all records in the table. Returns the number of affected rows.\n",
            );
            code.push_str("    pub async fn delete_all(ctx: &mut RequestContext) -> Result<u64, lightx::core::AppError> {\n");
            let table_name_q = if is_postgres {
                format!(
                    "\"{}\"",
                    table.sql_table_name.as_ref().unwrap_or(&table.name)
                )
            } else {
                table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
            };
            code.push_str(&format!(
                "        let query = sqlx::query(r#\"DELETE FROM {}\"#);\n",
                table_name_q
            ));
            code.push_str(&format!(
                "        let result = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                db_name
            ));
            code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
            code.push_str("        };\n");
            code.push_str("        Ok(result.rows_affected())\n");
            code.push_str("    }\n\n");

            // --- 5. FIND_ALL ---
            code.push_str("    /// Retrieves a paginated list of records. (Zero-Transaction Overhead for pure reads)\n");
            code.push_str("    pub async fn find_all(ctx: &mut RequestContext, limit: i64, offset: i64) -> Result<Vec<Self>, lightx::core::AppError> {\n");
            let table_name_q = if is_postgres {
                format!(
                    "\"{}\"",
                    table.sql_table_name.as_ref().unwrap_or(&table.name)
                )
            } else {
                table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
            };
            let all_cols = cols
                .iter()
                .map(|(n, _)| {
                    if is_postgres {
                        format!("\"{}\"", n)
                    } else {
                        format!("`{}`", n)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            code.push_str(&format!(
                "        let mut query = sqlx::query_as(r#\"SELECT {} FROM {} LIMIT {} OFFSET {}\"#);\n",
                all_cols, table_name_q,
                if is_postgres { "$1" } else { "?" },
                if is_postgres { "$2" } else { "?" }
            ));
            code.push_str("        query = query.bind(limit).bind(offset);\n");
            code.push_str(&format!(
                "        let records = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                db_name
            ));
            code.push_str("            query.fetch_all(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            query.fetch_all(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
            code.push_str("        };\n        Ok(records)\n    }\n\n");

            // --- 6. EXISTS_BY_PK ---
            if !pk_cols.is_empty() {
                let pk_args = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw_t = m.rust_type.value();
                        let arg_type = match raw_t.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), arg_type)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let pk_where = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        if is_postgres {
                            format!("\"{}\" = ${}", n, i + 1)
                        } else {
                            format!("`{}` = ?", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" AND ");

                code.push_str("    /// Checks if a record exists by its primary key. (Ultra-fast, doesn't fetch columns)\n");
                code.push_str(&format!("    pub async fn exists_by_{}(ctx: &mut RequestContext, {}) -> Result<bool, lightx::core::AppError> {{\n", pk_suffix, pk_args));
                code.push_str(&format!(
                    "        let mut query = sqlx::query(r#\"SELECT 1 FROM {} WHERE {}\"#);\n",
                    table_name_q, pk_where
                ));
                for (col_name, m) in &pk_cols {
                    let field = Self::escape_rust_keyword(col_name);
                    let raw_t = m.rust_type.value();
                    if raw_t == "String" || raw_t == "Vec<u8>" {
                        code.push_str(&format!("        query = query.bind({});\n", field));
                    } else {
                        code.push_str(&format!("        query = query.bind(&{});\n", field));
                    }
                }
                code.push_str(&format!(
                    "        let record = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.fetch_optional(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.fetch_optional(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
                code.push_str("        };\n        Ok(record.is_some())\n    }\n\n");
            }

            // --- 6.1. LIST_BY_CURSOR (O(1) Keyset Pagination) ---
            if pk_cols.len() == 1 {
                let (pk_name, pk_meta) = &pk_cols[0];
                let raw_t = pk_meta.rust_type.value();
                let cursor_type = if raw_t == "String" {
                    "String".to_string()
                } else if raw_t == "Vec<u8>" {
                    "Vec<u8>".to_string()
                } else {
                    raw_t.to_string()
                };
                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };
                let pk_q = if is_postgres {
                    format!("\"{}\"", pk_name)
                } else {
                    format!("`{}`", pk_name)
                };

                let all_cols = cols
                    .iter()
                    .map(|(n, _)| {
                        if is_postgres {
                            format!("\"{}\"", n)
                        } else {
                            format!("`{}`", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                code.push_str(
                    "    /// High-performance O(1) Keyset Pagination based on the Primary Key.\n",
                );
                code.push_str(&format!("    pub async fn list_by_{}_cursor(ctx: &mut RequestContext, last_{}: Option<{}>, limit: i64) -> Result<Vec<Self>, lightx::core::AppError> {{\n", pk_suffix, pk_name, cursor_type));

                code.push_str(&format!(
                    "        let base_query = if last_{}.is_some() {{\n",
                    pk_name
                ));
                code.push_str(&format!(
                    "            r#\"SELECT {} FROM {} WHERE {} > {} ORDER BY {} ASC LIMIT {}\"#\n",
                    all_cols,
                    table_name_q,
                    pk_q,
                    if is_postgres { "$1" } else { "?" },
                    pk_q,
                    if is_postgres { "$2" } else { "?" }
                ));
                code.push_str("        } else {\n");
                code.push_str(&format!(
                    "            r#\"SELECT {} FROM {} ORDER BY {} ASC LIMIT {}\"#\n",
                    all_cols,
                    table_name_q,
                    pk_q,
                    if is_postgres { "$1" } else { "?" }
                ));
                code.push_str("        };\n");

                code.push_str("        let mut query = sqlx::query_as(base_query);\n");
                code.push_str(&format!(
                    "        if let Some(last_val) = last_{} {{\n",
                    pk_name
                ));
                code.push_str("            query = query.bind(last_val);\n");
                code.push_str("        }\n");
                code.push_str("        query = query.bind(limit);\n");

                code.push_str(&format!(
                    "        let records = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.fetch_all(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.fetch_all(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
                code.push_str("        };\n        Ok(records)\n    }\n\n");
            }

            // --- 7. COUNT ---
            code.push_str("    /// Counts the total number of records in the table.\n");
            code.push_str("    pub async fn count(ctx: &mut RequestContext) -> Result<i64, lightx::core::AppError> {\n");
            code.push_str(&format!(
                "        let query = sqlx::query_scalar::<_, i64>(r#\"SELECT COUNT(*) FROM {}\"#);\n",
                table_name_q
            ));
            code.push_str(&format!(
                "        let total = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                db_name
            ));
            code.push_str("            query.fetch_one(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            query.fetch_one(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", db_name));
            code.push_str("        };\n        Ok(total)\n    }\n\n");

            // --- 7.5. STREAM_ALL ---
            code.push_str("    /// Returns an asynchronous stream to process millions of records without allocating massive memory.\n");
            code.push_str("    pub fn stream_all<'a>(ctx: &'a mut RequestContext) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Self, lightx::core::AppError>> + Send + 'a>> {\n");
            let table_name_q = if is_postgres {
                format!(
                    "\"{}\"",
                    table.sql_table_name.as_ref().unwrap_or(&table.name)
                )
            } else {
                table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
            };
            let all_cols = cols
                .iter()
                .map(|(n, _)| {
                    if is_postgres {
                        format!("\"{}\"", n)
                    } else {
                        format!("`{}`", n)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");
            code.push_str(&format!(
                "        let query = sqlx::query_as::<_, Self>(r#\"SELECT {} FROM {}\"#);\n",
                all_cols, table_name_q
            ));
            code.push_str("        use futures::StreamExt;\n");
            code.push_str(&format!(
                "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                db_name
            ));
            code.push_str("            let stream = query.fetch(&mut **tx).map(|res| res.map_err(|e| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() }));\n");
            code.push_str("            Box::pin(stream)\n");
            code.push_str("        } else {\n");
            code.push_str(&format!("            let stream = query.fetch(&ctx.{}_pool).map(|res| res.map_err(|e| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }}));\n", db_name));
            code.push_str("            Box::pin(stream)\n");
            code.push_str("        }\n");
            code.push_str("    }\n\n");

            // --- 8. INSERT_MANY ---
            if !insert_cols.is_empty() {
                code.push_str("    /// Inserts multiple records in a single atomic transaction. (Batch Insert)\n");
                code.push_str("    pub async fn insert_many(items: &[Self], ctx: &mut RequestContext) -> Result<u64, lightx::core::AppError> {\n");
                code.push_str(&format!(
                    "        ctx.get_or_create_{}_tx().await?;\n",
                    db_name
                ));
                code.push_str("        let mut count = 0;\n");
                code.push_str("        for item in items {\n");
                code.push_str("            item.insert(ctx).await?;\n");
                code.push_str("            count += 1;\n");
                code.push_str("        }\n");
                code.push_str("        Ok(count)\n    }\n\n");
            }

            // --- 8.5. UPSERT_MANY ---
            if !pk_cols.is_empty() && !update_cols.is_empty() && !insert_cols.is_empty() {
                code.push_str("    /// Upserts multiple records in a single atomic transaction. (Batch Upsert)\n");
                code.push_str("    pub async fn upsert_many(items: &[Self], ctx: &mut RequestContext) -> Result<(), lightx::core::AppError> {\n");
                code.push_str(&format!(
                    "        ctx.get_or_create_{}_tx().await?;\n",
                    db_name
                ));
                code.push_str("        for item in items {\n");
                code.push_str("            item.upsert(ctx).await?;\n");
                code.push_str("        }\n");
                code.push_str("        Ok(())\n");
                code.push_str("    }\n\n");
            }

            // --- 9. GET_BY_INDEXES (Simple & Composite) ---
            for index in &table.indexes {
                if index.columns.is_empty() {
                    continue;
                }

                let mut method_name_parts = Vec::new();
                let mut args = Vec::new();
                let mut where_clauses = Vec::new();
                let mut bind_calls = Vec::new();
                let mut valid = true;

                for (i, col_name) in index.columns.iter().enumerate() {
                    let field = Self::escape_rust_keyword(col_name);
                    method_name_parts.push(col_name.to_string());

                    if let Some(col_meta) =
                        cols.iter().find(|(n, _)| *n == col_name).map(|(_, m)| m)
                    {
                        let raw_t = col_meta.rust_type.value();
                        let arg_type = match raw_t.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        args.push(format!("{}: {}", field, arg_type));

                        let param_placeholder = if is_postgres {
                            format!("${}", i + 1)
                        } else {
                            "?".to_string()
                        };
                        if is_postgres {
                            where_clauses.push(format!("\"{}\" = {}", col_name, param_placeholder));
                        } else {
                            where_clauses.push(format!("`{}` = {}", col_name, param_placeholder));
                        }

                        if raw_t == "String" || raw_t == "Vec<u8>" {
                            bind_calls.push(format!("        query = query.bind({});\n", field));
                        } else {
                            bind_calls.push(format!("        query = query.bind(&{});\n", field));
                        }
                    } else {
                        valid = false;
                    }
                }

                if !valid {
                    continue;
                }

                let method_suffix = method_name_parts.join("_and_");
                let args_str = args.join(", ");
                let where_str = where_clauses.join(" AND ");

                let table_name_q = if is_postgres {
                    format!(
                        "\"{}\"",
                        table.sql_table_name.as_ref().unwrap_or(&table.name)
                    )
                } else {
                    table.sql_table_name.as_ref().unwrap_or(&table.name).clone()
                };
                let all_cols = cols
                    .iter()
                    .map(|(n, _)| {
                        if is_postgres {
                            format!("\"{}\"", n)
                        } else {
                            format!("`{}`", n)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let return_type = if index.is_unique {
                    "Option<Self>"
                } else {
                    "Vec<Self>"
                };
                let fetch_method = if index.is_unique {
                    "fetch_optional"
                } else {
                    "fetch_all"
                };

                code.push_str(&format!(
                    "    /// Retrieves records by index `{}`.\n",
                    index.name
                ));
                code.push_str(&format!("    pub async fn get_by_{}(ctx: &mut RequestContext, {}) -> Result<{}, lightx::core::AppError> {{\n", method_suffix, args_str, return_type));

                code.push_str(&format!(
                    "        let mut query = sqlx::query_as(r#\"SELECT {} FROM {} WHERE {}\"#);\n",
                    all_cols, table_name_q, where_str
                ));

                for bind in &bind_calls {
                    code.push_str(bind);
                }

                code.push_str(&format!(
                    "        let records = if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str(&format!("            query.{}(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", fetch_method));
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.{}(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?\n", fetch_method, db_name));
                code.push_str("        };\n        Ok(records)\n    }\n\n");

                if !index.is_unique {
                    let mut stream_args = Vec::new();
                    let mut stream_bind_calls = Vec::new();

                    for col_name in &index.columns {
                        let field = Self::escape_rust_keyword(col_name);
                        if let Some(col_meta) =
                            cols.iter().find(|(n, _)| *n == col_name).map(|(_, m)| m)
                        {
                            let raw_t = col_meta.rust_type.value();
                            let arg_type = match raw_t.as_str() {
                                "String" => "&'a str",
                                "Vec<u8>" => "&'a [u8]",
                                other => other,
                            };
                            stream_args.push(format!("{}: {}", field, arg_type));
                            stream_bind_calls
                                .push(format!("        query = query.bind({});\n", field));
                        }
                    }

                    let stream_args_str = stream_args.join(", ");
                    code.push_str(&format!(
                        "    /// Safely streams records by index `{}`.\n",
                        index.name
                    ));
                    code.push_str(&format!("    pub fn stream_by_{}<'a>(ctx: &'a mut RequestContext, {}) -> std::pin::Pin<Box<dyn futures::Stream<Item = Result<Self, lightx::core::AppError>> + Send + 'a>> {{\n", method_suffix, stream_args_str));

                    code.push_str(&format!(
                        "        let mut query = sqlx::query_as::<_, Self>(r#\"SELECT {} FROM {} WHERE {}\"#);\n",
                        all_cols, table_name_q, where_str
                    ));
                    for bind in stream_bind_calls {
                        code.push_str(&bind);
                    }
                    code.push_str("        use futures::StreamExt;\n");
                    code.push_str(&format!(
                        "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                        db_name
                    ));
                    code.push_str("            let stream = query.fetch(&mut **tx).map(|res| res.map_err(|e| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() }));\n");
                    code.push_str("            Box::pin(stream)\n");
                    code.push_str("        } else {\n");
                    code.push_str(&format!("            let stream = query.fetch(&ctx.{}_pool).map(|res| res.map_err(|e| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }}));\n", db_name));
                    code.push_str("            Box::pin(stream)\n");
                    code.push_str("        }\n");
                    code.push_str("    }\n\n");
                }

                // --- DELETE_BY_INDEX ---
                code.push_str(&format!(
                    "    /// Deletes records matching the index `{}`.\n",
                    index.name
                ));
                code.push_str(&format!("    pub async fn delete_by_{}(ctx: &mut RequestContext, {}) -> Result<(), lightx::core::AppError> {{\n", method_suffix, args_str));
                code.push_str(&format!(
                    "        let mut query = sqlx::query(r#\"DELETE FROM {} WHERE {}\"#);\n",
                    table_name_q, where_str
                ));
                for bind in &bind_calls {
                    code.push_str(bind);
                }
                code.push_str(&format!(
                    "        if let Some(tx) = ctx.{}_tx.as_mut() {{\n",
                    db_name
                ));
                code.push_str("            query.execute(&mut **tx).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError { msg: e.to_string(), file: file!(), line: line!() })?;\n");
                code.push_str("        } else {\n");
                code.push_str(&format!("            query.execute(&ctx.{}_pool).await.map_err(|e: sqlx::Error| lightx::core::AppError::DatabaseError {{ msg: e.to_string(), file: file!(), line: line!() }})?;\n", db_name));
                code.push_str("        }\n");
                code.push_str("        Ok(())\n");
                code.push_str("    }\n\n");
            }

            code.push_str("}\n\n");
        }

        // Final deterministic write into the OUT_DIR artifact directory
        fs::write(dest_path, code)?;
        println!("cargo:warning= DAO generation completed successfully in OUT_DIR!");

        Ok(())
    }
}

/// The main entrypoint mapping the Database-First facade.
pub struct DaoxGenerator {
    db_url: String,
    out_dir: String,
}

impl DaoxGenerator {
    pub fn new(db_url: &str, out_dir: &str) -> Self {
        Self {
            db_url: db_url.to_string(),
            out_dir: out_dir.to_string(),
        }
    }

    pub async fn generate(&self) -> Result<(), Box<dyn std::error::Error>> {
        let schema_dir = format!("{}/.daox_schema", self.out_dir);
        let inner = DaoGenerator::new(&schema_dir, &self.out_dir);
        inner.introspect(&self.db_url).await?;
        inner.generate_dao()?;
        Ok(())
    }
}
