//! # DAOx – Database-First DAO Generator
//!
//! Generates strongly typed Rust structs and pure `sqlx` CRUD operations from
//! a live database schema. **Zero framework dependency.**
//!
//! ## Generated API per table
//! - **Read** : `count`, `stream_all`, `list_paginated`, `get_by_pk`,
//!   `exists_by_pk`, `list_by_cursor`
//! - **Write** : `insert`, `insert_batch`, `upsert`
//! - **Update** : `update_by_pk`, `update_partial_by_pk` (patch)
//! - **Delete** : `delete_by_pk`, `delete_many_by_pk`
//! - **Index** : `get_by_{idx}`, `exists_by_{idx}`, `stream_by_{idx}`,
//!   `update_by_{idx}`, `delete_by_{idx}`

use serde::Deserialize;
use sqlx::Row;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// ──────────────────────────────────────────────
// 1. Metadata structs (TOML ↔ Rust)
// ──────────────────────────────────────────────

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

#[derive(Debug, Deserialize, Clone)]
pub struct ColumnMetadata {
    #[serde(rename = "type")]
    pub rust_type: Property<String>,
    pub is_optional: Property<bool>,
    pub min_length: Option<Property<usize>>,
    pub max_length: Option<Property<usize>>,
    pub min_value: Option<Property<i64>>,
    pub max_value: Option<Property<i64>>,
    pub format: Option<Property<String>>,
    pub enum_values: Option<Property<Vec<String>>>,
    #[serde(default = "default_prop_false")]
    pub is_primary_key: Property<bool>,
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
    #[serde(default)]
    pub is_view: bool,
}

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
    pub is_view: Option<bool>,
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

#[derive(Debug, Clone)]
pub struct TableMetadata {
    pub name: String,
    pub sql_table_name: Option<String>,
    pub config: TableConfig,
    pub columns: HashMap<String, ColumnMetadata>,
    pub indexes: Vec<IndexMetadata>,
}

// ──────────────────────────────────────────────
// 2. Dialect helpers
// ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dialect {
    MySql,
    Postgres,
    Sqlite,
}

impl Dialect {
    fn sqlx_db_type(self) -> &'static str {
        match self {
            Dialect::MySql => "sqlx::MySql",
            Dialect::Postgres => "sqlx::Postgres",
            Dialect::Sqlite => "sqlx::Sqlite",
        }
    }

    fn placeholder(self, idx: usize) -> String {
        match self {
            Dialect::MySql | Dialect::Sqlite => "?".to_string(),
            Dialect::Postgres => format!("${}", idx),
        }
    }

    fn quote_ident(self, name: &str) -> String {
        match self {
            Dialect::MySql => format!("`{}`", name),
            Dialect::Postgres => format!("\\\"{}\\\"", name),
            Dialect::Sqlite => format!("`{}`", name),
        }
    }

    fn upsert_suffix(self, pk_cols: &[String], update_cols: &[String]) -> String {
        match self {
            Dialect::MySql => {
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("`{c}` = VALUES(`{c}`)"))
                    .collect();
                format!(" ON DUPLICATE KEY UPDATE {}", updates.join(", "))
            }
            Dialect::Postgres => {
                let conflict: Vec<String> =
                    pk_cols.iter().map(|c| format!("\\\"{c}\\\"")).collect();
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("\\\"{c}\\\" = EXCLUDED.\\\"{c}\\\""))
                    .collect();
                format!(
                    " ON CONFLICT ({}) DO UPDATE SET {}",
                    conflict.join(", "),
                    updates.join(", ")
                )
            }
            Dialect::Sqlite => {
                let conflict: Vec<String> = pk_cols.iter().map(|c| format!("`{c}`")).collect();
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("`{c}` = EXCLUDED.`{c}`"))
                    .collect();
                format!(
                    " ON CONFLICT ({}) DO UPDATE SET {}",
                    conflict.join(", "),
                    updates.join(", ")
                )
            }
        }
    }

    fn last_insert_id_expr(self) -> &'static str {
        match self {
            Dialect::MySql => "result.last_insert_id()",
            Dialect::Postgres => "result.rows_affected()",
            Dialect::Sqlite => "result.last_insert_rowid() as u64",
        }
    }
}

// ──────────────────────────────────────────────
// 3. DaoGenerator
// ──────────────────────────────────────────────

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

    // ── 3a. Introspection ──

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

        let databases_toml_path = schema_path.join("databases.toml");
        if !databases_toml_path.exists() {
            let databases_toml_content = format!(
                r#"[default]
dialect = "{}"
description = "Default main database autogenerated by Daox"
"#,
                fallback_dialect
            );
            fs::write(&databases_toml_path, databases_toml_content)?;
        }

        if is_postgres {
            println!(
                "cargo:warning=Daox: PostgreSQL live introspection bypassed. Relying on offline TOML schemas."
            );
            return Ok(());
        }
        if is_sqlite {
            println!(
                "cargo:warning=Daox: SQLite live introspection bypassed. Relying on offline TOML schemas."
            );
            return Ok(());
        }

        #[cfg(not(feature = "mysql"))]
        {
            println!("cargo:warning=Daox: MySQL feature disabled. Bypassing live introspection.");
            return Ok(());
        }

        #[cfg(feature = "mysql")]
        {
            let pool = sqlx::mysql::MySqlPoolOptions::new()
                .max_connections(2)
                .connect(database_url)
                .await?;

            let tables_query = "SELECT CAST(TABLE_NAME AS CHAR) AS TABLE_NAME, CAST(TABLE_TYPE AS CHAR) AS TABLE_TYPE FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()";
            let tables_rows = sqlx::query(tables_query).fetch_all(&pool).await?;
            let tables: Vec<(String, String)> = tables_rows
                .into_iter()
                .map(|row| {
                    (
                        row.get::<String, _>("TABLE_NAME"),
                        row.get::<String, _>("TABLE_TYPE"),
                    )
                })
                .collect();

            for (table_name, table_type) in tables {
                let is_view = table_type == "VIEW";
                let table_dir = schema_path.join(&table_name);
                fs::create_dir_all(&table_dir)?;

                let table_toml = table_dir.join("_table.toml");
                let mut db_name = "default".to_string();
                let mut existing_desc = None;
                if let Ok(content) = fs::read_to_string(&table_toml)
                    && let Ok(config) = toml::from_str::<TableConfig>(&content)
                {
                    db_name = config.database;
                    existing_desc = config.description;
                }
                let mut toml_str = format!("database = \"{}\"\n", db_name);
                if let Some(desc) = existing_desc {
                    toml_str.push_str(&format!("description = \"{}\"\n", desc));
                }
                if is_view {
                    toml_str.push_str("is_view = true\n");
                }
                fs::write(&table_toml, toml_str)?;

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
                    let is_nullable: String = col.get("IS_NULLABLE");
                    let column_key: String = col.get("COLUMN_KEY");
                    let extra: String = col.get("EXTRA");
                    let column_type: String = col.get("COLUMN_TYPE");
                    let max_length_val: Option<i32> =
                        col.try_get("CHARACTER_MAXIMUM_LENGTH").unwrap_or(None);

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
                    let inferred_max_length =
                        max_length_val.map(|v| v as usize).or(numeric_max_len);

                    let mut enum_values_toml = String::new();
                    let format_regex = if col_name.starts_with("is_")
                        || col_name.starts_with("has_")
                    {
                        "^[01]$".to_string()
                    } else if data_type == "enum" {
                        let mut vals_str =
                            column_type.replace("enum(", "").replace([')', '\''], "");
                        let vals_arr = vals_str
                            .split(',')
                            .map(|s| format!("\"{}\"", s))
                            .collect::<Vec<_>>()
                            .join(", ");
                        enum_values_toml = format!(
                            "[enum_values]\nvalue = [{}]\nmessage = \"schema.enum_values.message\"\n\n",
                            vals_arr
                        );
                        vals_str = vals_str.replace(',', "|");
                        format!("^({})$", vals_str)
                    } else if rust_type == "String" {
                        if col_name.contains("email") {
                            r#"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#
                                .to_string()
                        } else if col_name.contains("password") {
                            r#"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$"#.to_string()
                        } else {
                            "^[À-ÿA-Za-z0-9_ -]*$".to_string()
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
                        let dv = match rust_type {
                            "bool" => "false",
                            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32"
                            | "f64" => "0",
                            _ => "",
                        };
                        format!("default = \"{}\"\n", dv)
                    } else {
                        String::new()
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
                    if let Some(v) = min_val {
                        toml_content.push_str(&format!(
                            "[min_value]\nvalue = {}\nmessage = \"schema.min_value.message|{}\"\n\n", v, v
                        ));
                    } else {
                        toml_content.push_str("# [min_value]\n# value = \n# message = \n\n");
                    }
                    if let Some(v) = max_val {
                        toml_content.push_str(&format!(
                            "[max_value]\nvalue = {}\nmessage = \"schema.max_value.message|{}\"\n\n", v, v
                        ));
                    } else {
                        toml_content.push_str("# [max_value]\n# value = \n# message = \n\n");
                    }
                    toml_content.push_str(&format!(
                        "[format]\nvalue = '{}'\nmessage = \"schema.format.message\"\n\n",
                        format_regex
                    ));
                    toml_content
                        .push_str(&format!("[is_primary_key]\nvalue = {}\n\n", is_primary_key));
                    toml_content.push_str(&format!("[is_unique]\nvalue = {}\n\n", is_unique));
                    toml_content.push_str(&format!("[is_index]\nvalue = {}\n\n", is_index));
                    toml_content.push_str(&format!(
                        "[is_auto_increment]\nvalue = {}\n\n",
                        is_auto_increment
                    ));
                    toml_content.push_str("[business_rules]\n\n");

                    let col_file = table_dir.join(format!("{}.toml", col_name));
                    fs::write(col_file, toml_content)?;
                }

                // Indexes
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

                let mut grouped: HashMap<String, (bool, Vec<String>)> = HashMap::new();
                for row in indexes_rows {
                    let idx_name: String = row.get("INDEX_NAME");
                    let col_name: String = row.get("COLUMN_NAME");
                    let non_unique: i64 = row.get("NON_UNIQUE");
                    let entry = grouped
                        .entry(idx_name)
                        .or_insert((non_unique == 0, Vec::new()));
                    entry.1.push(col_name);
                }

                let idx_file = table_dir.join("_indexes.toml");
                if !grouped.is_empty() {
                    let mut s = String::new();
                    for (name, (is_unique, cols)) in grouped {
                        s.push_str("[[indexes]]\n");
                        s.push_str(&format!("name = \"{}\"\n", name));
                        s.push_str(&format!("is_unique = {}\n", is_unique));
                        let cols_str = cols
                            .iter()
                            .map(|c| format!("\"{}\"", c))
                            .collect::<Vec<_>>()
                            .join(", ");
                        s.push_str(&format!("columns = [{}]\n\n", cols_str));
                    }
                    fs::write(&idx_file, s)?;
                } else if idx_file.exists() {
                    fs::remove_file(&idx_file)?;
                }
            }
            println!("cargo:warning=Daox: Database introspection complete.");
        }

        Ok(())
    }

    // ── 3b. Parse schema ──

    pub fn parse_schema(&self) -> Result<Vec<TableMetadata>, Box<dyn std::error::Error>> {
        let mut tables = Vec::new();
        let schema_path = Path::new(&self.schema_dir);
        if !schema_path.exists() || !schema_path.is_dir() {
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
                        config = toml::from_str(&content)
                            .map_err(|e| format!("Parse error {:?}: {}", col_path, e))?;
                        continue;
                    }
                    if file_stem == "_indexes" {
                        let parsed: TableIndexesConfig = toml::from_str(&content)
                            .map_err(|e| format!("Indexes parse error {:?}: {}", col_path, e))?;
                        indexes = parsed.indexes;
                        continue;
                    }
                    let col_meta: ColumnMetadata = toml::from_str(&content)
                        .map_err(|e| format!("Parse error {:?}: {}", col_path, e))?;
                    columns.insert(file_stem, col_meta);
                }
            }
            Ok((config, columns, indexes))
        };

        for entry in fs::read_dir(schema_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && entry.file_name() == "databases.toml" {
                continue;
            }
            if path.is_dir() {
                let mut has_toml = false;
                if let Ok(subs) = fs::read_dir(&path) {
                    for sub in subs.flatten() {
                        if sub.path().is_file()
                            && sub.path().extension().and_then(|s| s.to_str()) == Some("toml")
                        {
                            has_toml = true;
                            break;
                        }
                    }
                }
                if has_toml {
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
                    let db_name = entry.file_name().into_string().unwrap();
                    if let Ok(subs) = fs::read_dir(&path) {
                        for sub in subs.flatten() {
                            if sub.path().is_dir() {
                                let table_name = sub.file_name().into_string().unwrap();
                                let (mut config, columns, indexes) = parse_table_dir(&sub.path())?;
                                config.database = db_name.clone();
                                tables.push(TableMetadata {
                                    name: format!("{}_{}", db_name, table_name),
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

        // Overrides
        let overrides_path = std::env::var("DAOX_OVERRIDES_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| schema_path.parent().unwrap().join("overrides"));

        if !overrides_path.exists() {
            fs::create_dir_all(&overrides_path)?;
            fs::write(
                overrides_path.join("README.md"),
                "# Daox Overrides\n\nReplicate `schema/<table>/<column>.toml` here to override introspected values.\n",
            )?;
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
                                    if let Ok(ov) = toml::from_str::<TableConfigOverride>(&content)
                                    {
                                        if let Some(v) = ov.database {
                                            table.config.database = v;
                                        }
                                        if let Some(v) = ov.description {
                                            table.config.description = Some(v);
                                        }
                                        if let Some(v) = ov.is_view {
                                            table.config.is_view = v;
                                        }
                                    }
                                    continue;
                                }
                                if file_stem == "_indexes" {
                                    if let Ok(ov) = toml::from_str::<TableIndexesConfig>(&content) {
                                        table.indexes = ov.indexes;
                                    }
                                    continue;
                                }
                                if let Some(col_meta) = table.columns.get_mut(&file_stem) {
                                    let ov: ColumnOverride =
                                        toml::from_str(&content).map_err(|e| {
                                            format!("Override error {:?}: {}", col_path, e)
                                        })?;
                                    if let Some(v) = ov.rust_type {
                                        col_meta.rust_type = v;
                                    }
                                    if let Some(v) = ov.is_optional {
                                        col_meta.is_optional = v;
                                    }
                                    if let Some(v) = ov.min_length {
                                        col_meta.min_length = Some(v);
                                    }
                                    if let Some(v) = ov.max_length {
                                        col_meta.max_length = Some(v);
                                    }
                                    if let Some(v) = ov.min_value {
                                        col_meta.min_value = Some(v);
                                    }
                                    if let Some(v) = ov.max_value {
                                        col_meta.max_value = Some(v);
                                    }
                                    if let Some(v) = ov.format {
                                        col_meta.format = Some(v);
                                    }
                                    if let Some(v) = ov.is_unique {
                                        col_meta.is_unique = v;
                                    }
                                    if let Some(v) = ov.is_index {
                                        col_meta.is_index = v;
                                    }
                                    if let Some(v) = ov.business_rules {
                                        col_meta.business_rules.extend(v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        tables.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(tables)
    }

    // ── 3c. Helpers ──

    fn escape_rust_keyword(name: &str) -> String {
        const KW: &[&str] = &[
            "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
            "for", "if", "impl", "in", "let", "loop", "match", "mut", "pub", "ref", "return",
            "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
            "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final",
            "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
        ];
        if KW.contains(&name) {
            format!("r#{}", name)
        } else {
            name.to_string()
        }
    }

    fn to_pascal_case(s: &str) -> String {
        let mut r = String::new();
        let mut cap = true;
        for c in s.chars() {
            if c == '_' || c == '-' {
                cap = true;
            } else if cap {
                r.push(c.to_ascii_uppercase());
                cap = false;
            } else {
                r.push(c);
            }
        }
        r
    }

    fn resolve_dialect(&self, db_name: &str) -> Dialect {
        let path = Path::new(&self.schema_dir).join("databases.toml");
        if let Ok(content) = fs::read_to_string(&path)
            && let Ok(val) = content.parse::<toml::Value>()
            && let Some(d) = val
                .get(db_name)
                .and_then(|t| t.as_table())
                .and_then(|t| t.get("dialect"))
                .and_then(|v| v.as_str())
        {
            return match d {
                "postgres" => Dialect::Postgres,
                "sqlite" => Dialect::Sqlite,
                _ => Dialect::MySql,
            };
        }
        Dialect::MySql
    }

    // ── 3d. Code generation – PURE SQLX ONLY ──

    pub fn generate_dao(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tables = self.parse_schema()?;
        let dest_path = Path::new(&self.out_dir).join("daox_generated.rs");
        let mut code = String::with_capacity(1024 * 1024);

        code.push_str("// Code generated automatically by daox. DO NOT EDIT.\n");
        code.push_str("// Pure sqlx layer – zero framework dependency.\n");
        code.push_str("#![allow(clippy::all)]\n\n");

        for table in &tables {
            let struct_name = Self::to_pascal_case(&table.name);
            let db_name = if table.config.database.is_empty() {
                "default"
            } else {
                table.config.database.as_str()
            };
            let dialect = self.resolve_dialect(db_name);
            let db_type = dialect.sqlx_db_type();
            let sql_table = table.sql_table_name.as_ref().unwrap_or(&table.name);

            // Sorted columns
            let mut cols: Vec<_> = table.columns.iter().collect();
            cols.sort_by(|a, b| a.0.cmp(b.0));

            // ── Struct ──
            code.push_str("#[derive(Debug, Clone, sqlx::FromRow)]\n");
            code.push_str(&format!("pub struct {} {{\n", struct_name));
            for (col_name, col_meta) in &cols {
                let field = Self::escape_rust_keyword(col_name);
                let mut ty = col_meta.rust_type.value();
                if col_meta.is_optional.value() {
                    ty = format!("Option<{}>", ty);
                }
                code.push_str(&format!("    pub {}: {},\n", field, ty));
            }
            code.push_str("}\n\n");

            // ── Column classification ──
            let mut pk_cols: Vec<(String, ColumnMetadata)> = Vec::new();
            let mut insert_cols: Vec<(String, ColumnMetadata)> = Vec::new();
            let mut update_cols: Vec<(String, ColumnMetadata)> = Vec::new();
            let mut auto_inc_col: Option<String> = None;

            for (col_name, col_meta) in &cols {
                if col_meta.is_primary_key.value() {
                    pk_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
                if table.config.is_view {
                    continue;
                }
                if col_meta.is_auto_increment.value() {
                    auto_inc_col = Some(col_name.to_string());
                } else {
                    insert_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
                if !col_meta.is_primary_key.value() {
                    update_cols.push((col_name.to_string(), (*col_meta).clone()));
                }
            }

            let pk_suffix = pk_cols
                .iter()
                .map(|(n, _)| n.as_str())
                .collect::<Vec<_>>()
                .join("_and_");

            let all_cols_sql: Vec<String> =
                cols.iter().map(|(n, _)| dialect.quote_ident(n)).collect();
            let all_cols_str = all_cols_sql.join(", ");

            // ── impl block ──
            code.push_str(&format!("impl {} {{\n", struct_name));

            // count
            code.push_str(&format!(
                "    pub async fn count<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                 \x20       let query = \"SELECT COUNT(*) FROM {tbl}\";\n\
                 \x20       let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;\n\
                 \x20       Ok(count as u64)\n\
                 \x20   }}\n\n",
                db = db_type,
                tbl = sql_table,
            ));

            // stream_all
            code.push_str(&format!(
                "    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = {db}> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n\
                 \x20       let query = \"SELECT {cols} FROM {tbl}\";\n\
                 \x20       sqlx::query_as::<_, Self>(query).fetch(executor)\n\
                 \x20   }}\n\n",
                db = db_type,
                cols = all_cols_str,
                tbl = sql_table,
            ));

            // list_paginated
            code.push_str(&format!(
                "    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {{\n\
                 \x20       let is_valid = order_by.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ' || c == ',' || c == '`' || c == '\"');\n\
                 \x20       if !is_valid {{\n\
                 \x20           return Err(sqlx::Error::Configuration(\"Invalid characters in ORDER BY. Potential SQL injection.\".into()));\n\
                 \x20       }}\n\
                 \x20       let offset = page.saturating_sub(1) * page_size;\n\
                 \x20       let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"SELECT {cols} FROM {tbl} ORDER BY \");\n\
                 \x20       qb.push(order_by);\n\
                 \x20       qb.push(\" LIMIT \");\n\
                 \x20       qb.push_bind(page_size as i64);\n\
                 \x20       qb.push(\" OFFSET \");\n\
                 \x20       qb.push_bind(offset as i64);\n\
                 \x20       qb.build_query_as::<Self>().fetch_all(executor).await\n\
                 \x20   }}\n\n",
                db = db_type,
                cols = all_cols_str,
                tbl = sql_table,
            ));

            // ── PK methods ──
            if !pk_cols.is_empty() {
                let pk_args: Vec<String> = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw = m.rust_type.value();
                        let at = match raw.as_str() {
                            "String" => "&String",
                            "Vec<u8>" => "&Vec<u8>",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), at)
                    })
                    .collect();
                let pk_args_str = pk_args.join(", ");

                let pk_where: Vec<String> = pk_cols
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| {
                        format!(
                            "{} = {}",
                            dialect.quote_ident(n),
                            dialect.placeholder(i + 1)
                        )
                    })
                    .collect();
                let pk_where_str = pk_where.join(" AND ");

                let pk_binds: Vec<String> = pk_cols
                    .iter()
                    .map(|(n, _)| format!(".bind({})", Self::escape_rust_keyword(n)))
                    .collect();
                let pk_binds_str = pk_binds.join("");

                // get_by_pk
                code.push_str(&format!(
                    "    pub async fn get_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<Option<Self>> {{\n\
                     \x20       let query = \"SELECT {cols} FROM {tbl} WHERE {wh}\";\n\
                     \x20       sqlx::query_as::<_, Self>(query){binds}.fetch_optional(executor).await\n\
                     \x20   }}\n\n",
                    sfx = pk_suffix, db = db_type, args = pk_args_str,
                    cols = all_cols_str, tbl = sql_table, wh = pk_where_str, binds = pk_binds_str,
                ));

                // exists_by_pk
                code.push_str(&format!(
                    "    pub async fn exists_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<bool> {{\n\
                     \x20       let query = \"SELECT 1 FROM {tbl} WHERE {wh} LIMIT 1\";\n\
                     \x20       let exists: Option<(i32,)> = sqlx::query_as(query){binds}.fetch_optional(executor).await?;\n\
                     \x20       Ok(exists.is_some())\n\
                     \x20   }}\n\n",
                    sfx = pk_suffix, db = db_type, args = pk_args_str,
                    tbl = sql_table, wh = pk_where_str, binds = pk_binds_str,
                ));

                // list_by_cursor (single PK only)
                if pk_cols.len() == 1 {
                    let (pk_n, pk_m) = &pk_cols[0];
                    let raw = pk_m.rust_type.value();
                    let cursor_at = match raw.as_str() {
                        "String" => "&String",
                        "Vec<u8>" => "&Vec<u8>",
                        other => other,
                    };
                    let p1 = dialect.placeholder(1);
                    let p2 = dialect.placeholder(2);
                    code.push_str(&format!(
                        "    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, last_id: {cat}, limit: u32) -> sqlx::Result<Vec<Self>> {{\n\
                         \x20       let query = \"SELECT {cols} FROM {tbl} WHERE {pk} > {p1} ORDER BY {pk} ASC LIMIT {p2}\";\n\
                         \x20       sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await\n\
                         \x20   }}\n\n",
                        db = db_type, cat = cursor_at, cols = all_cols_str, tbl = sql_table,
                        pk = dialect.quote_ident(pk_n), p1 = p1, p2 = p2,
                    ));
                }
            }

            // ── Write methods (skip for views) ──
            if !table.config.is_view && !insert_cols.is_empty() {
                let ins_cols_sql: Vec<String> = insert_cols
                    .iter()
                    .map(|(n, _)| dialect.quote_ident(n))
                    .collect();
                let ins_cols_str = ins_cols_sql.join(", ");
                let ins_phs: Vec<String> = insert_cols
                    .iter()
                    .enumerate()
                    .map(|(i, _)| dialect.placeholder(i + 1))
                    .collect();
                let ins_phs_str = ins_phs.join(", ");
                let ins_binds: Vec<String> = insert_cols
                    .iter()
                    .map(|(n, _)| format!(".bind(&self.{})", Self::escape_rust_keyword(n)))
                    .collect();
                let ins_binds_str = ins_binds.join("");

                // insert
                let insert_sql = if dialect == Dialect::Postgres {
                    if let Some(ai) = auto_inc_col.as_ref() {
                        format!(
                            "INSERT INTO {} ({}) VALUES ({}) RETURNING {}::bigint",
                            sql_table,
                            ins_cols_str,
                            ins_phs_str,
                            dialect.quote_ident(ai)
                        )
                    } else {
                        format!(
                            "INSERT INTO {} ({}) VALUES ({})",
                            sql_table, ins_cols_str, ins_phs_str
                        )
                    }
                } else {
                    format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        sql_table, ins_cols_str, ins_phs_str
                    )
                };

                code.push_str(&format!(
                    "    pub async fn insert<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                     \x20       let query = \"{sql}\";\n",
                    db = db_type, sql = insert_sql,
                ));

                if dialect == Dialect::Postgres && auto_inc_col.is_some() {
                    code.push_str(&format!(
                        "        let (id,): (i64,) = sqlx::query_as(query){binds}.fetch_one(executor).await?;\n\
                         \x20       Ok(id as u64)\n\
                         \x20   }}\n\n",
                        binds = ins_binds_str,
                    ));
                } else {
                    code.push_str(&format!(
                        "        let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                         \x20       Ok({last})\n\
                         \x20   }}\n\n",
                        binds = ins_binds_str,
                        last = dialect.last_insert_id_expr(),
                        db = db_type,
                    ));
                }

                // insert_batch
                code.push_str(&format!(
                    "    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {{\n\
                     \x20       if items.is_empty() {{ return Ok(0); }}\n\
                     \x20       let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"INSERT INTO {tbl} ({cols}) \");\n\
                     \x20       qb.push_values(items, |mut b, item| {{\n",
                    db = db_type, tbl = sql_table, cols = ins_cols_str,
                ));
                for (n, _) in &insert_cols {
                    code.push_str(&format!(
                        "            b.push_bind(&item.{});\n",
                        Self::escape_rust_keyword(n)
                    ));
                }
                code.push_str(
                    "        });\n\
                     \x20       let result = qb.build().execute(executor).await?;\n\
                     \x20       Ok(result.rows_affected())\n\
                     \x20   }\n\n",
                );

                // upsert
                if !pk_cols.is_empty() && !update_cols.is_empty() {
                    let pk_names: Vec<String> = pk_cols.iter().map(|(n, _)| n.clone()).collect();
                    let upd_names: Vec<String> =
                        update_cols.iter().map(|(n, _)| n.clone()).collect();
                    let suffix = dialect.upsert_suffix(&pk_names, &upd_names);

                    let upsert_cols: Vec<String> = insert_cols
                        .iter()
                        .map(|(n, _)| dialect.quote_ident(n))
                        .collect();
                    let upsert_cols_str = upsert_cols.join(", ");
                    let upsert_phs: Vec<String> = insert_cols
                        .iter()
                        .enumerate()
                        .map(|(i, _)| dialect.placeholder(i + 1))
                        .collect();
                    let upsert_phs_str = upsert_phs.join(", ");
                    let upsert_binds: Vec<String> = insert_cols
                        .iter()
                        .map(|(n, _)| format!(".bind(&self.{})", Self::escape_rust_keyword(n)))
                        .collect();
                    let upsert_binds_str = upsert_binds.join("");

                    let upsert_sql = format!(
                        "INSERT INTO {} ({}) VALUES ({}){}",
                        sql_table, upsert_cols_str, upsert_phs_str, suffix
                    );

                    code.push_str(&format!(
                        "    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                         \x20       let query = \"{sql}\";\n\
                         \x20       let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                         \x20       Ok(result.rows_affected())\n\
                         \x20   }}\n\n",
                        db = db_type, sql = upsert_sql, binds = upsert_binds_str,
                    ));
                }

                // update_by_pk
                if !pk_cols.is_empty() && !update_cols.is_empty() {
                    let set_clauses: Vec<String> = update_cols
                        .iter()
                        .enumerate()
                        .map(|(i, (n, _))| {
                            format!(
                                "{} = {}",
                                dialect.quote_ident(n),
                                dialect.placeholder(i + 1)
                            )
                        })
                        .collect();
                    let set_str = set_clauses.join(", ");
                    let pk_where: Vec<String> = pk_cols
                        .iter()
                        .enumerate()
                        .map(|(i, (n, _))| {
                            format!(
                                "{} = {}",
                                dialect.quote_ident(n),
                                dialect.placeholder(i + 1 + update_cols.len())
                            )
                        })
                        .collect();
                    let pk_where_str = pk_where.join(" AND ");

                    let mut bind_lines = String::new();
                    for (n, _) in &update_cols {
                        bind_lines.push_str(&format!(
                            "        query = query.bind(&self.{});\n",
                            Self::escape_rust_keyword(n)
                        ));
                    }
                    for (n, _) in &pk_cols {
                        bind_lines.push_str(&format!(
                            "        query = query.bind(&self.{});\n",
                            Self::escape_rust_keyword(n)
                        ));
                    }

                    code.push_str(&format!(
                        "    pub async fn update_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                         \x20       let query_str = \"UPDATE {tbl} SET {set} WHERE {wh}\";\n\
                         \x20       let mut query = sqlx::query::<{db}>(query_str);\n\
                         {binds}\
                         \x20       let result = query.execute(executor).await?;\n\
                         \x20       Ok(result.rows_affected())\n\
                         \x20   }}\n\n",
                        sfx = pk_suffix, db = db_type, tbl = sql_table,
                        set = set_str, wh = pk_where_str, binds = bind_lines,
                    ));
                }

                // delete_by_pk
                if !pk_cols.is_empty() {
                    let pk_args: Vec<String> = pk_cols
                        .iter()
                        .map(|(n, m)| {
                            let raw = m.rust_type.value();
                            let at = match raw.as_str() {
                                "String" => "&String",
                                "Vec<u8>" => "&Vec<u8>",
                                other => other,
                            };
                            format!("{}: {}", Self::escape_rust_keyword(n), at)
                        })
                        .collect();
                    let pk_where: Vec<String> = pk_cols
                        .iter()
                        .enumerate()
                        .map(|(i, (n, _))| {
                            format!(
                                "{} = {}",
                                dialect.quote_ident(n),
                                dialect.placeholder(i + 1)
                            )
                        })
                        .collect();
                    let pk_binds: Vec<String> = pk_cols
                        .iter()
                        .map(|(n, _)| format!(".bind({})", Self::escape_rust_keyword(n)))
                        .collect();

                    code.push_str(&format!(
                        "    pub async fn delete_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<u64> {{\n\
                         \x20       let query = \"DELETE FROM {tbl} WHERE {wh}\";\n\
                         \x20       let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                         \x20       Ok(result.rows_affected())\n\
                         \x20   }}\n\n",
                        sfx = pk_suffix, db = db_type, args = pk_args.join(", "),
                        tbl = sql_table, wh = pk_where.join(" AND "), binds = pk_binds.join(""),
                    ));

                    // delete_many_by_pk (single PK)
                    if pk_cols.len() == 1 {
                        let (pk_n, pk_m) = &pk_cols[0];
                        let raw = pk_m.rust_type.value();
                        let id_type = raw.as_str();
                        code.push_str(&format!(
                            "    pub async fn delete_many_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, ids: &[{idt}]) -> sqlx::Result<u64> {{\n\
                             \x20       if ids.is_empty() {{ return Ok(0); }}\n\
                             \x20       let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"DELETE FROM {tbl} WHERE {pk} IN \");\n\
                             \x20       qb.push(\"(\");\n\
                             \x20       let mut sep = qb.separated(\", \");\n\
                             \x20       for id in ids {{ sep.push_bind(id); }}\n\
                             \x20       sep.push_unseparated(\")\");\n\
                             \x20       let result = qb.build().execute(executor).await?;\n\
                             \x20       Ok(result.rows_affected())\n\
                             \x20   }}\n\n",
                            sfx = pk_suffix, db = db_type, idt = id_type,
                            tbl = sql_table, pk = dialect.quote_ident(pk_n),
                        ));
                    }
                }
            }

            // ── Patch struct + update_partial_by_pk ──
            if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
                let patch_name = format!("{}Patch", struct_name);
                let pk_args: Vec<String> = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw = m.rust_type.value();
                        let at = match raw.as_str() {
                            "String" => "&String",
                            "Vec<u8>" => "&Vec<u8>",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), at)
                    })
                    .collect();

                code.push_str(&format!(
                    "    pub async fn update_partial_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}, patch: &{patch}) -> sqlx::Result<u64> {{\n\
                     \x20       let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"UPDATE {tbl} SET \");\n\
                     \x20       let mut has = false;\n\
                     \x20       let mut sep = qb.separated(\", \");\n",
                    sfx = pk_suffix, db = db_type,
                    args = pk_args.join(", "), patch = patch_name, tbl = sql_table,
                ));

                for (n, _) in &update_cols {
                    let field = Self::escape_rust_keyword(n);
                    let col_q = dialect.quote_ident(n);
                    code.push_str(&format!(
                        "        if let Some(val) = &patch.{f} {{\n\
                         \x20           has = true;\n\
                         \x20           sep.push(\"{cq} = \");\n\
                         \x20           sep.push_bind_unseparated(val.clone());\n\
                         \x20       }}\n",
                        f = field,
                        cq = col_q,
                    ));
                }

                code.push_str("        if !has { return Ok(0); }\n");

                let mut first_pk = true;
                for (n, _) in &pk_cols {
                    let field = Self::escape_rust_keyword(n);
                    let col_q = dialect.quote_ident(n);
                    if first_pk {
                        code.push_str(&format!(
                            "        qb.push(\" WHERE {cq} = \");\n\
                             \x20       qb.push_bind({f}.clone());\n",
                            cq = col_q,
                            f = field,
                        ));
                        first_pk = false;
                    } else {
                        code.push_str(&format!(
                            "        qb.push(\" AND {cq} = \");\n\
                             \x20       qb.push_bind({f}.clone());\n",
                            cq = col_q,
                            f = field,
                        ));
                    }
                }

                code.push_str(
                    "        let result = qb.build().execute(executor).await?;\n\
                     \x20       Ok(result.rows_affected())\n\
                     \x20   }\n\n",
                );
            }

            // ── Index methods ──
            for index in &table.indexes {
                if index.columns.is_empty() {
                    continue;
                }

                let mut valid = true;
                let mut idx_args: Vec<String> = Vec::new();
                let mut idx_where: Vec<String> = Vec::new();
                let mut idx_binds: Vec<String> = Vec::new();
                let mut method_parts: Vec<String> = Vec::new();

                for (i, col_name) in index.columns.iter().enumerate() {
                    method_parts.push(col_name.clone());
                    if let Some((_, m)) = cols.iter().find(|(n, _)| *n == col_name) {
                        let raw = m.rust_type.value();
                        let at = match raw.as_str() {
                            "String" => "&String",
                            "Vec<u8>" => "&Vec<u8>",
                            other => other,
                        };
                        idx_args.push(format!("{}: {}", Self::escape_rust_keyword(col_name), at));
                        idx_where.push(format!(
                            "{} = {}",
                            dialect.quote_ident(col_name),
                            dialect.placeholder(i + 1)
                        ));
                        idx_binds.push(format!(".bind({})", Self::escape_rust_keyword(col_name)));
                    } else {
                        valid = false;
                    }
                }
                if !valid {
                    continue;
                }

                let method_suffix = method_parts.join("_and_");
                let args_str = idx_args.join(", ");
                let where_str = idx_where.join(" AND ");
                let binds_str = idx_binds.join("");

                if index.is_unique {
                    // get_by (unique → Option)
                    code.push_str(&format!(
                        "    pub async fn get_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<Option<Self>> {{\n\
                         \x20       let query = \"SELECT {cols} FROM {tbl} WHERE {wh}\";\n\
                         \x20       sqlx::query_as::<_, Self>(query){binds}.fetch_optional(executor).await\n\
                         \x20   }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        cols = all_cols_str, tbl = sql_table, wh = where_str, binds = binds_str,
                    ));
                } else {
                    // list_by (non-unique → Vec)
                    code.push_str(&format!(
                        "    pub async fn list_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<Vec<Self>> {{\n\
                         \x20       let query = \"SELECT {cols} FROM {tbl} WHERE {wh}\";\n\
                         \x20       sqlx::query_as::<_, Self>(query){binds}.fetch_all(executor).await\n\
                         \x20   }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        cols = all_cols_str, tbl = sql_table, wh = where_str, binds = binds_str,
                    ));

                    // stream_by
                    let stream_args_str = args_str
                        .replace("&String", "&'e String")
                        .replace("&Vec<u8>", "&'e Vec<u8>");
                    code.push_str(&format!(
                        "    pub fn stream_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}> + 'e>(executor: E, {args}) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n\
                         \x20       let query = \"SELECT {cols} FROM {tbl} WHERE {wh}\";\n\
                         \x20       sqlx::query_as::<_, Self>(query){binds}.fetch(executor)\n\
                         \x20   }}\n\n",
                        sfx = method_suffix, db = db_type, args = stream_args_str,
                        cols = all_cols_str, tbl = sql_table, wh = where_str, binds = binds_str,
                    ));
                }

                // exists_by
                code.push_str(&format!(
                    "    pub async fn exists_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<bool> {{\n\
                     \x20       let query = \"SELECT 1 FROM {tbl} WHERE {wh} LIMIT 1\";\n\
                     \x20       let exists: Option<(i32,)> = sqlx::query_as(query){binds}.fetch_optional(executor).await?;\n\
                     \x20       Ok(exists.is_some())\n\
                     \x20   }}\n\n",
                    sfx = method_suffix, db = db_type, args = args_str,
                    tbl = sql_table, wh = where_str, binds = binds_str,
                ));

                // delete_by (non-view)
                if !table.config.is_view {
                    code.push_str(&format!(
                        "    pub async fn delete_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<u64> {{\n\
                         \x20       let query = \"DELETE FROM {tbl} WHERE {wh}\";\n\
                         \x20       let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                         \x20       Ok(result.rows_affected())\n\
                         \x20   }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        tbl = sql_table, wh = where_str, binds = binds_str,
                    ));
                }
            }

            code.push_str("}\n\n");

            if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
                let patch_name = format!("{}Patch", struct_name);
                code.push_str(&format!(
                    "#[derive(Debug, Clone, Default)]\npub struct {} {{\n",
                    patch_name
                ));
                for (n, m) in &update_cols {
                    let raw = m.rust_type.value();
                    let inner = if m.is_optional.value() {
                        format!("Option<{}>", raw)
                    } else {
                        raw
                    };
                    code.push_str(&format!(
                        "    pub {}: Option<{}>,\n",
                        Self::escape_rust_keyword(n),
                        inner
                    ));
                }
                code.push_str("}\n\n");
            }
        }

        fs::write(dest_path, code)?;
        println!(
            "cargo:warning=Daox: DAO generation completed (pure sqlx, zero framework dependency)."
        );
        Ok(())
    }
}

// ──────────────────────────────────────────────
// 4. Public facade
// ──────────────────────────────────────────────

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
