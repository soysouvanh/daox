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

use std::fmt::Write;

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
    pub has_default: Property<bool>,
    #[serde(default = "default_prop_false")]
    pub is_index: Property<bool>,
    #[serde(default = "default_prop_false")]
    pub is_generated: Property<bool>,
    #[serde(default)]
    pub business_rules: std::collections::HashMap<String, String>,
}

impl ColumnMetadata {
    pub fn rust_type_resolved(&self) -> String {
        let v = self.rust_type.value();
        if v == "json" || v == "jsonb" {
            "serde_json::Value".to_string()
        } else {
            v
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    pub database: String,
    pub description: Option<String>,
    #[serde(default)]
    pub is_view: bool,
}

#[derive(serde::Serialize)]
struct TableToml {
    database: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_view: Option<bool>,
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
    pub is_generated: Option<Property<bool>>,
    pub business_rules: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
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

const MAX_TOML_SIZE: u64 = 1_048_576; // 1 MB

pub fn read_toml_file_limited(
    path: &std::path::Path,
) -> Result<String, Box<dyn std::error::Error>> {
    let meta = fs::metadata(path)?;
    if meta.len() > MAX_TOML_SIZE {
        return Err(format!(
            "TOML file {:?} exceeds size limit ({} > {} bytes)",
            path,
            meta.len(),
            MAX_TOML_SIZE
        )
        .into());
    }
    Ok(fs::read_to_string(path)?)
}

pub fn safe_remove_dir_all(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        fs::remove_file(path)?;
        return Ok(());
    }
    if !meta.is_dir() {
        fs::remove_file(path)?;
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let child_path = entry?.path();
        let child_meta = fs::symlink_metadata(&child_path)?;
        if child_meta.file_type().is_symlink() || !child_meta.is_dir() {
            fs::remove_file(&child_path)?;
        } else {
            safe_remove_dir_all(&child_path)?;
        }
    }
    fs::remove_dir(path)?;
    Ok(())
}

fn write_if_changed<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P,
    content: C,
) -> std::io::Result<bool> {
    let p = path.as_ref();
    let c = content.as_ref();
    if let Ok(existing) = std::fs::read(p)
        && existing == c
    {
        return Ok(false);
    }
    std::fs::write(p, c)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o644);
        std::fs::set_permissions(p, perms)?;
    }
    Ok(true)
}

pub fn safe_write_if_changed<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P,
    content: C,
) -> Result<bool, String> {
    let p = path.as_ref();
    let c = content.as_ref();

    // 1. Symlink check on the target
    match fs::symlink_metadata(p) {
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                return Err(format!("Refusing to write to symlink: {:?}", p));
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(format!("Cannot stat {:?}: {}", p, e)),
    }

    // 2. Symlink check on the parent
    if let Some(parent) = p.parent() {
        match fs::symlink_metadata(parent) {
            Ok(parent_meta) => {
                if parent_meta.file_type().is_symlink() {
                    return Err(format!(
                        "Refusing to write into symlinked directory: {:?}",
                        parent
                    ));
                }
            }
            Err(e) => return Err(format!("Cannot stat parent {:?}: {}", parent, e)),
        }
    }

    // 3. Vérification contenu existant (pas de réécriture inutile)
    if let Ok(existing) = fs::read(p)
        && existing == c
    {
        return Ok(false);
    }

    // 4. Écriture atomique : fichier temporaire create_new + rename
    let file_name = p
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("Cannot determine filename for path: {:?}", p))?;
    let tmp_path = p.with_file_name(format!(
        ".{}.tmp-{}-{}",
        file_name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    // create_new(true) garantit qu'on ne suit pas un symlink préexistant
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp_path)
        .map_err(|e| format!("Cannot create temp file {:?}: {}", tmp_path, e))?;

    use std::io::Write;
    file.write_all(c).map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    drop(file);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&tmp_path, perms).map_err(|e| e.to_string())?;
    }

    fs::rename(&tmp_path, p).map_err(|e| {
        let _ = fs::remove_file(&tmp_path);
        format!("Cannot rename {:?} to {:?}: {}", tmp_path, p, e)
    })?;

    Ok(true)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dialect {
    Postgres,
    MySql,
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

    fn max_bind_params(self) -> usize {
        match self {
            Dialect::MySql => 65535,
            Dialect::Postgres => 65535,
            Dialect::Sqlite => 32766,
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
            Dialect::MySql => format!("`{}`", name.replace('`', "``")),
            Dialect::Postgres => format!("\"{}\"", name.replace('"', "\"\"")),
            Dialect::Sqlite => format!("`{}`", name.replace('`', "``")),
        }
    }

    fn upsert_suffix(self, pk_cols: &[String], update_cols: &[String]) -> String {
        match self {
            Dialect::MySql => {
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("{} = VALUES({})", self.quote_ident(c), self.quote_ident(c)))
                    .collect();
                format!(" ON DUPLICATE KEY UPDATE {}", updates.join(", "))
            }
            Dialect::Postgres => {
                let conflict: Vec<String> = pk_cols.iter().map(|c| self.quote_ident(c)).collect();
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("{} = EXCLUDED.{}", self.quote_ident(c), self.quote_ident(c)))
                    .collect();
                format!(
                    " ON CONFLICT ({}) DO UPDATE SET {}",
                    conflict.join(", "),
                    updates.join(", ")
                )
            }
            Dialect::Sqlite => {
                let conflict: Vec<String> = pk_cols.iter().map(|c| self.quote_ident(c)).collect();
                let updates: Vec<String> = update_cols
                    .iter()
                    .map(|c| format!("{} = EXCLUDED.{}", self.quote_ident(c), self.quote_ident(c)))
                    .collect();
                format!(
                    " ON CONFLICT ({}) DO UPDATE SET {}",
                    conflict.join(", "),
                    updates.join(", ")
                )
            }
        }
    }
}

fn is_safe_identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
        && !s.chars().next().unwrap().is_ascii_digit()
        && s.chars().any(|c| c.is_ascii_alphanumeric())
}

pub fn parse_mysql_enum(input: &str) -> Result<Vec<String>, String> {
    let mut values = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escape = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if escape {
            current.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' if in_quote => {
                escape = true;
            }
            '\'' => {
                if in_quote {
                    if chars.peek() == Some(&'\'') {
                        current.push('\'');
                        chars.next();
                    } else {
                        in_quote = false;
                    }
                } else {
                    in_quote = true;
                }
            }
            ',' if !in_quote => {
                values.push(std::mem::take(&mut current));
            }
            _ => current.push(c),
        }
    }

    values.push(current);
    Ok(values)
}

fn validate_regex(value: &str) -> Result<(), Box<dyn std::error::Error>> {
    if value.len() > 1024 {
        return Err("regex too long (max 1024 bytes)".into());
    }

    regex::Regex::new(value).map_err(|e| format!("invalid regex {:?}: {}", value, e))?;

    Ok(())
}

fn ensure_safe_dir(
    base: &std::path::Path,
    p: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let meta = fs::symlink_metadata(p)?;
    if meta.file_type().is_symlink() {
        return Err(format!("symlink directory rejected: {:?}", p).into());
    }

    let canonical_base = fs::canonicalize(base)?;
    let canonical_p = fs::canonicalize(p)?;

    if !canonical_p.starts_with(&canonical_base) {
        return Err(format!(
            "path traversal detected: {:?} outside {:?}",
            canonical_p, canonical_base
        )
        .into());
    }

    Ok(())
}

fn validate_identifier_for_dialect(name: &str, dialect: Dialect) -> Result<(), String> {
    let max_len = match dialect {
        Dialect::MySql => 64,
        Dialect::Postgres => 63,
        Dialect::Sqlite => 1000,
    };
    if name.len() > max_len {
        return Err(format!(
            "Identifier '{}' ({} chars) exceeds {} char limit for {:?}",
            name,
            name.len(),
            max_len,
            dialect
        ));
    }
    Ok(())
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
        let is_mysql = database_url.starts_with("mysql") || database_url.starts_with("mariadb");
        let fallback_dialect = if is_postgres {
            "postgres"
        } else if is_sqlite {
            "sqlite"
        } else {
            if !is_mysql {
                println!(
                    "cargo:warning=Daox: Unknown dialect for url '{}'. Falling back to 'mysql'.",
                    database_url
                );
            }
            "mysql"
        };

        let schema_path = Path::new(&self.schema_dir);
        fs::create_dir_all(schema_path)?;

        let databases_toml_path = schema_path.join("databases.toml");
        if !databases_toml_path.exists() {
            let mut db_root = toml::map::Map::new();
            let mut db_section = toml::map::Map::new();
            db_section.insert(
                "dialect".into(),
                toml::Value::String(fallback_dialect.to_string()),
            );
            db_section.insert(
                "description".into(),
                toml::Value::String("Default main database autogenerated by Daox".to_string()),
            );
            db_root.insert("default".into(), toml::Value::Table(db_section));
            let databases_toml_content = toml::to_string_pretty(&toml::Value::Table(db_root))
                .map_err(|e| format!("TOML serialization error for databases.toml: {}", e))?;
            safe_write_if_changed(&databases_toml_path, &databases_toml_content)?;
        }

        sqlx::any::install_default_drivers();
        let pool_future = sqlx::any::AnyPoolOptions::new()
            .max_connections(16)
            .acquire_timeout(std::time::Duration::from_secs(5))
            .connect(database_url);
        let pool = tokio::time::timeout(std::time::Duration::from_secs(10), pool_future)
            .await
            .map_err(|_| "Connection timeout after 10s")??;

        let (tables_query, cols_query) = if is_postgres {
            (
                "
                SELECT tablename::text AS \"TABLE_NAME\", 'BASE TABLE' AS \"TABLE_TYPE\" FROM pg_tables WHERE schemaname = 'public'
                UNION ALL
                SELECT viewname::text AS \"TABLE_NAME\", 'VIEW' AS \"TABLE_TYPE\" FROM pg_views WHERE schemaname = 'public'
                ",
                "
                SELECT
                    c.column_name::text AS \"COLUMN_NAME\",
                    c.data_type::text AS \"DATA_TYPE\",
                    c.is_nullable::text AS \"IS_NULLABLE\",
                    COALESCE(kc.constraint_type, '') AS \"COLUMN_KEY\",
                    CASE WHEN c.column_default LIKE 'nextval(%' THEN 'auto_increment' ELSE '' END AS \"EXTRA\",
                    c.data_type::text AS \"COLUMN_TYPE\",
                    c.column_default::text AS \"COLUMN_DEFAULT\",
                    c.character_maximum_length::int AS \"CHARACTER_MAXIMUM_LENGTH\",
                    CASE WHEN c.is_generated = 'ALWAYS' THEN 1 ELSE 0 END AS \"IS_GENERATED\"
                FROM information_schema.columns c
                LEFT JOIN (
                    SELECT kcu.table_schema, kcu.table_name, kcu.column_name,
                           CASE WHEN tc.constraint_type = 'PRIMARY KEY' THEN 'PRI' WHEN tc.constraint_type = 'UNIQUE' THEN 'UNI' ELSE '' END as constraint_type
                    FROM information_schema.key_column_usage kcu
                    JOIN information_schema.table_constraints tc ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema
                ) kc ON kc.table_schema = c.table_schema AND kc.table_name = c.table_name AND kc.column_name = c.column_name
                WHERE c.table_schema = 'public' AND c.table_name = $1
                "
            )
        } else if is_sqlite {
            (
                "SELECT name AS TABLE_NAME, type AS TABLE_TYPE FROM sqlite_master WHERE type IN ('table', 'view') AND name NOT LIKE 'sqlite_%'",
                "
                SELECT
                    ti.name AS COLUMN_NAME,
                    ti.type AS DATA_TYPE,
                    CASE WHEN ti.\"notnull\" = 1 THEN 'NO' ELSE 'YES' END AS IS_NULLABLE,
                    CASE WHEN ti.pk > 0 THEN 'PRI' ELSE '' END AS COLUMN_KEY,
                    '' AS EXTRA,
                    ti.type AS COLUMN_TYPE,
                    CAST(ti.dflt_value AS TEXT) AS COLUMN_DEFAULT,
                    NULL AS CHARACTER_MAXIMUM_LENGTH,
                    CASE WHEN ti.hidden >= 2 THEN 1 ELSE 0 END AS IS_GENERATED
                FROM sqlite_master m
                JOIN pragma_table_xinfo(m.name) ti
                WHERE m.type IN ('table', 'view') AND m.name = ?
                ",
            )
        } else {
            (
                "SELECT CAST(TABLE_NAME AS CHAR) AS TABLE_NAME, CAST(TABLE_TYPE AS CHAR) AS TABLE_TYPE FROM information_schema.TABLES WHERE TABLE_SCHEMA = DATABASE()",
                "
                SELECT
                    CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME,
                    CAST(DATA_TYPE AS CHAR) AS DATA_TYPE,
                    CAST(IS_NULLABLE AS CHAR) AS IS_NULLABLE,
                    CAST(COLUMN_KEY AS CHAR) AS COLUMN_KEY,
                    CAST(EXTRA AS CHAR) AS EXTRA,
                    CAST(COLUMN_TYPE AS CHAR) AS COLUMN_TYPE,
                    CAST(COLUMN_DEFAULT AS CHAR) AS COLUMN_DEFAULT,
                    CHARACTER_MAXIMUM_LENGTH,
                    CASE WHEN CAST(EXTRA AS CHAR) LIKE '%GENERATED%' THEN 1 ELSE 0 END AS IS_GENERATED
                FROM information_schema.COLUMNS
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ?
                ",
            )
        };

        let tables_rows = tokio::time::timeout(
            std::time::Duration::from_secs(10),
            sqlx::query(tables_query).fetch_all(&pool),
        )
        .await
        .map_err(|_| "Table listing timeout after 10s")??;
        let mut tables: Vec<(String, String)> = Vec::new();
        for row in tables_rows {
            let table_name: String = match row.try_get("TABLE_NAME") {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "cargo:warning=Daox: skipping row with missing TABLE_NAME: {}",
                        e
                    );
                    continue;
                }
            };
            let table_type: String = match row.try_get("TABLE_TYPE") {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "cargo:warning=Daox: skipping table '{}' with missing TABLE_TYPE: {}",
                        table_name, e
                    );
                    continue;
                }
            };
            tables.push((table_name, table_type));
        }

        if schema_path.exists() {
            let table_names: std::collections::HashSet<_> =
                tables.iter().map(|(name, _)| name.clone()).collect();
            for entry in fs::read_dir(schema_path)? {
                let entry = entry?;
                let meta = fs::symlink_metadata(entry.path())?;
                if meta.file_type().is_symlink() {
                    return Err(
                        format!("Symlink rejected in schema dir: {:?}", entry.path()).into(),
                    );
                }
                let name = entry.file_name().to_string_lossy().to_string();
                if name != "overrides"
                    && name != "databases.toml"
                    && !table_names.contains(&name)
                    && let Err(e) = safe_remove_dir_all(&entry.path())
                {
                    eprintln!(
                        "Daox: warning: could not remove orphaned entry {:?}: {}",
                        entry.path(),
                        e
                    );
                }
            }
        }

        let mut spawn_handles = Vec::new();
        let semaphore = std::sync::Arc::new(tokio::sync::Semaphore::new(8));

        for (table_name, table_type) in tables {
            let schema_path = schema_path.to_path_buf();
            let pool = pool.clone();
            let sem = std::sync::Arc::clone(&semaphore);

            spawn_handles.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.map_err(|e| e.to_string())?;
                let res: Result<(), Box<dyn std::error::Error + Send + Sync>> = async {
                    if !is_safe_identifier(&table_name) {
                        println!(
                            "cargo:warning=Daox: table '{}' skipped (invalid identifier characters)",
                            table_name
                        );
                        return Ok(());
                    }
            let is_view = table_type.to_uppercase() == "VIEW";
            let table_dir = schema_path.join(&table_name);
            let table_dir_c = table_dir.clone();
            tokio::task::spawn_blocking(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
                fs::create_dir_all(&table_dir_c)?;

                let table_toml_path = table_dir_c.join("_table.toml");
                let mut db_name = "default".to_string();
                let mut existing_desc = None;
                if let Ok(content) = read_toml_file_limited(&table_toml_path)
                    && let Ok(config) = toml::from_str::<TableConfig>(&content)
                {
                    db_name = config.database;
                    existing_desc = config.description;
                }
                
                let table_toml_data = TableToml {
                    database: db_name,
                    description: existing_desc,
                    is_view: is_view.then_some(true),
                };

                let toml_str = toml::to_string_pretty(&table_toml_data)
                    .map_err(|e| format!("TOML serialization error: {}", e))?;

                safe_write_if_changed(&table_toml_path, &toml_str)?;
                Ok(())
            }).await.map_err(|e| e.to_string())??;

            let columns = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                sqlx::query(cols_query).bind(&table_name).fetch_all(&pool)
            ).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

            for col in columns {
                let col_name: String = match col.try_get("COLUMN_NAME") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("cargo:warning=Daox: skipping column with missing COLUMN_NAME in table '{}': {}", table_name, e);
                        continue;
                    }
                };
                if !is_safe_identifier(&col_name) {
                    println!(
                        "cargo:warning=Daox: column '{}' in table '{}' skipped (invalid identifier characters)",
                        col_name, table_name
                    );
                    continue;
                }
                let data_type: String = match col.try_get("DATA_TYPE") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("cargo:warning=Daox: skipping column '{}' with missing DATA_TYPE: {}", col_name, e);
                        continue;
                    }
                };
                let is_nullable: String = match col.try_get("IS_NULLABLE") { Ok(v) => v, Err(_) => "YES".to_string() };
                let column_key: String = match col.try_get("COLUMN_KEY") { Ok(v) => v, Err(_) => "".to_string() };
                let extra: String = match col.try_get("EXTRA") { Ok(v) => v, Err(_) => "".to_string() };
                let column_type: String = match col.try_get("COLUMN_TYPE") { Ok(v) => v, Err(_) => data_type.clone() };
                let column_default: Option<String> = col.try_get("COLUMN_DEFAULT").unwrap_or(None);
                let max_length_val: Option<i32> =
                    col.try_get("CHARACTER_MAXIMUM_LENGTH").unwrap_or(None);

                let rust_type = if column_type == "tinyint(1)"
                    || data_type == "boolean"
                    || data_type.to_lowercase() == "boolean"
                {
                    "bool"
                } else {
                    let d_type = data_type.to_lowercase();
                    let cn = col_name.to_lowercase();
                    match d_type.as_str() {
                        "json" | "jsonb" => "json",
                        "binary" | "blob" | "bytea" | "varbinary" | "longblob" | "mediumblob" | "tinyblob" => "Vec<u8>",
                        "decimal" | "float" | "double" | "real" | "numeric" | "double precision" => "f64",
                        "boolean" | "bool" => "bool",
                        "bigint" | "bigserial" => "i64",
                        "int" | "integer" | "serial" => "i32",
                        "smallint" | "tinyint" | "smallserial" => "i16",
                        "varchar" | "text" | "enum" | "char" | "character varying" | "longtext" | "mediumtext" | "tinytext" => "String",
                        "timestamp"
                        | "timestamp without time zone"
                        | "timestamp with time zone"
                        | "datetime" => "chrono::DateTime<chrono::Utc>",
                        "date" => "chrono::NaiveDate",
                        _ => {
                            if cn.contains("json") {
                                "json"
                            } else if cn.contains("blob") || cn.contains("binary") || cn.contains("bytea") {
                                "Vec<u8>"
                            } else if cn.contains("decimal") || cn.contains("float") || cn.contains("double") || cn.contains("real") || cn.contains("numeric") {
                                "f64"
                            } else if cn.contains("bool") || cn.starts_with("is_") || cn.starts_with("has_") {
                                "bool"
                            } else {
                                "String"
                            }
                        }
                    }
                };

                let is_primary_key = column_key == "PRI";
                let is_optional = is_nullable == "YES" && !is_primary_key;
                let is_unique = column_key == "UNI";
                let is_index = column_key == "MUL";
                let is_auto_increment = extra.contains("auto_increment")
                    || (is_sqlite && is_primary_key && data_type.to_lowercase() == "integer");
                let min_length = if is_optional { 0 } else { 1 };
                let has_default_val = column_default.is_some();
                let is_generated_val: i32 = col.try_get("IS_GENERATED").unwrap_or(0);
                let is_generated = is_generated_val > 0;

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

                let format_regex = if col_name.starts_with("is_") || col_name.starts_with("has_") {
                    "^[01]$".to_string()
                } else if data_type == "enum" {
                    let trimmed = column_type.trim();
                    let vals_str = if trimmed.to_lowercase().starts_with("enum(") && trimmed.ends_with(")") {
                        &trimmed[5..trimmed.len()-1]
                    } else {
                        trimmed
                    };

                    let parsed_vals = match parse_mysql_enum(vals_str) {
                        Ok(v) if !v.is_empty() => v,
                        Ok(_) => {
                            println!("cargo:warning=Daox: ENUM in table '{}' column '{}' parsed to empty values, using permissive regex", table_name, col_name);
                            Vec::new()
                        }
                        Err(e) => {
                            println!("cargo:warning=Daox: failed to parse ENUM in table '{}' column '{}': {}, using permissive regex", table_name, col_name, e);
                            Vec::new()
                        }
                    };

                    if parsed_vals.is_empty() {
                        "^.*$".to_string()
                    } else {
                        let rx_inner = parsed_vals.iter()
                            .map(|s| std::borrow::Cow::Owned(regex::escape(s)))
                            .collect::<Vec<_>>()
                            .join("|");
                        let format_regex = format!("^({})$", rx_inner);
                        if let Err(e) = regex::Regex::new(&format_regex) {
                            println!("cargo:warning=Daox: invalid regex for enum in table '{}': {}", table_name, e);
                            "^.*$".to_string()
                        } else {
                            format_regex
                        }
                    }
                } else if rust_type == "String" {
                    if col_name.contains("email") {
                        r#"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#.to_string()
                    } else if col_name.contains("password") {
                        r#"^[A-Za-z\d]{8,}$"#.to_string()
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

                use toml::Value as TomlValue;
                use toml::map::Map as TomlMap;

                #[allow(clippy::too_many_arguments)]
                fn build_column_toml(
                    rust_type: &str,
                    is_optional: bool,
                    min_length: usize,
                    inferred_max_length: Option<usize>,
                    min_val: Option<i64>,
                    max_val: Option<i64>,
                    format_regex: &str,
                    is_primary_key: bool,
                    is_unique: bool,
                    is_index: bool,
                    is_auto_increment: bool,
                    has_default_val: bool,
                    is_generated: bool,
                    enum_values: Option<&[String]>,
                ) -> Result<String, String> {
                    let mut root = TomlMap::new();

                    let mut type_section = TomlMap::new();
                    type_section.insert("value".into(), TomlValue::String(rust_type.to_string()));
                    type_section.insert("message".into(), TomlValue::String("schema.type.message".into()));
                    root.insert("type".into(), TomlValue::Table(type_section));

                    let mut opt_section = TomlMap::new();
                    opt_section.insert("value".into(), TomlValue::Boolean(is_optional));
                    opt_section.insert("message".into(), TomlValue::String(
                        if is_optional { "".into() } else { "schema.is_optional.message".into() }
                    ));
                    if is_optional {
                        let dv = match rust_type {
                            "bool" => "false",
                            "i8"|"i16"|"i32"|"i64"|"u8"|"u16"|"u32"|"u64"|"f32"|"f64" => "0",
                            _ => "",
                        };
                        opt_section.insert("default".into(), TomlValue::String(dv.into()));
                    }
                    root.insert("is_optional".into(), TomlValue::Table(opt_section));

                    if let Some(len) = inferred_max_length {
                        let mut ml_section = TomlMap::new();
                        ml_section.insert("value".into(), TomlValue::Integer(len as i64));
                        ml_section.insert("message".into(), TomlValue::String(
                            format!("schema.max_length.message|{}", len)
                        ));
                        root.insert("max_length".into(), TomlValue::Table(ml_section));
                    }

                    let mut minl_section = TomlMap::new();
                    minl_section.insert("value".into(), TomlValue::Integer(min_length as i64));
                    minl_section.insert("message".into(), TomlValue::String(
                        format!("schema.min_length.message|{}", min_length)
                    ));
                    root.insert("min_length".into(), TomlValue::Table(minl_section));

                    if let Some(v) = min_val {
                        let mut mv_section = TomlMap::new();
                        mv_section.insert("value".into(), TomlValue::Integer(v));
                        mv_section.insert("message".into(), TomlValue::String(
                            format!("schema.min_value.message|{}", v)
                        ));
                        root.insert("min_value".into(), TomlValue::Table(mv_section));
                    }
                    if let Some(v) = max_val {
                        let mut mv_section = TomlMap::new();
                        mv_section.insert("value".into(), TomlValue::Integer(v));
                        mv_section.insert("message".into(), TomlValue::String(
                            format!("schema.max_value.message|{}", v)
                        ));
                        root.insert("max_value".into(), TomlValue::Table(mv_section));
                    }

                    let mut fmt_section = TomlMap::new();
                    fmt_section.insert("value".into(), TomlValue::String(format_regex.to_string()));
                    fmt_section.insert("message".into(), TomlValue::String("schema.format.message".into()));
                    root.insert("format".into(), TomlValue::Table(fmt_section));

                    for (key, val) in [
                        ("is_primary_key", is_primary_key),
                        ("is_unique", is_unique),
                        ("is_index", is_index),
                        ("is_auto_increment", is_auto_increment),
                        ("has_default", has_default_val),
                        ("is_generated", is_generated),
                    ] {
                        let mut section = TomlMap::new();
                        section.insert("value".into(), TomlValue::Boolean(val));
                        root.insert(key.into(), TomlValue::Table(section));
                    }

                    root.insert("business_rules".into(), TomlValue::Table(TomlMap::new()));

                    if let Some(evs) = enum_values {
                        if !evs.is_empty() {
                            let mut ev_section = TomlMap::new();
                            let vals: Vec<TomlValue> = evs
                                .iter()
                                .map(|s| TomlValue::String(s.clone()))
                                .collect();
                            ev_section.insert("value".into(), TomlValue::Array(vals));
                            ev_section.insert(
                                "message".into(),
                                TomlValue::String("schema.enum_values.message".into()),
                            );
                            root.insert("enum_values".into(), TomlValue::Table(ev_section));
                        }
                    } else {
                        let mut ev_section = TomlMap::new();
                        ev_section.insert("value".into(), TomlValue::Array(vec![]));
                        root.insert("enum_values".into(), TomlValue::Table(ev_section));
                    }

                    let toml_value = TomlValue::Table(root);
                    let final_content = toml::to_string_pretty(&toml_value).map_err(|e| e.to_string())?;

                    Ok(final_content)
                }

                let parsed_enum = if data_type == "enum" {
                    let trimmed = column_type.trim();
                    let vals_str = if trimmed.to_lowercase().starts_with("enum(") && trimmed.ends_with(")") {
                        &trimmed[5..trimmed.len()-1]
                    } else {
                        trimmed
                    };
                    Some(parse_mysql_enum(vals_str).unwrap_or_default())
                } else {
                    None
                };

                let toml_content = build_column_toml(
                    rust_type, is_optional, min_length, inferred_max_length,
                    min_val, max_val, &format_regex, is_primary_key,
                    is_unique, is_index, is_auto_increment, has_default_val,
                    is_generated,
                    parsed_enum.as_deref()
                ).map_err(|e| e.to_string())?;

                let col_file = table_dir.join(format!("{}.toml", col_name));
                safe_write_if_changed(&col_file, &toml_content)?;
            }

            // Indexes
            let idx_query = if is_postgres {
                "
                SELECT
                    i.relname::text AS \"INDEX_NAME\",
                    a.attname::text AS \"COLUMN_NAME\",
                    (CASE WHEN ix.indisunique THEN 0 ELSE 1 END)::bigint AS \"NON_UNIQUE\"
                FROM pg_class t
                JOIN pg_index ix ON t.oid = ix.indrelid
                JOIN pg_class i ON i.oid = ix.indexrelid
                JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
                WHERE t.relkind = 'r' AND t.relname = $1 AND a.attnum > 0 AND NOT ix.indisprimary
                ORDER BY i.relname, a.attnum
                "
            } else if is_sqlite {
                "
                SELECT
                    il.name AS INDEX_NAME,
                    ii.name AS COLUMN_NAME,
                    CASE WHEN il.\"unique\" = 1 THEN 0 ELSE 1 END AS NON_UNIQUE
                FROM sqlite_master m
                JOIN pragma_index_list(m.name) il
                JOIN pragma_index_info(il.name) ii
                WHERE il.origin != 'pk' AND m.type IN ('table', 'view') AND m.name = ?
                ORDER BY m.name, il.name, ii.seqno
                "
            } else {
                "
                SELECT
                    CAST(INDEX_NAME AS CHAR) AS INDEX_NAME,
                    CAST(COLUMN_NAME AS CHAR) AS COLUMN_NAME,
                    NON_UNIQUE
                FROM information_schema.STATISTICS
                WHERE TABLE_SCHEMA = DATABASE() AND TABLE_NAME = ? AND INDEX_NAME != 'PRIMARY'
                ORDER BY INDEX_NAME, SEQ_IN_INDEX
                "
            };
            let indexes_rows = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                sqlx::query(idx_query).bind(&table_name).fetch_all(&pool)
            ).await.map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;

            let mut grouped: HashMap<String, (bool, Vec<String>)> = HashMap::new();
            for row in indexes_rows {
                let idx_name: String = match row.try_get("INDEX_NAME") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let col_name: String = match row.try_get("COLUMN_NAME") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let non_unique: i64 = match row.try_get("NON_UNIQUE") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let entry = grouped
                    .entry(idx_name)
                    .or_insert((non_unique == 0, Vec::new()));
                entry.1.push(col_name);
            }

            let idx_file = table_dir.join("_indexes.toml");
            if !grouped.is_empty() {
                let mut indexes = Vec::new();
                for (name, (is_unique, columns)) in grouped {
                    indexes.push(IndexMetadata {
                        name,
                        is_unique,
                        columns,
                    });
                }

                let content = toml::to_string_pretty(&TableIndexesConfig { indexes })
                    .map_err(|e| format!("TOML serialization error: {}", e))?;

                safe_write_if_changed(&idx_file, &content).map_err(|e| e.to_string())?;
            } else if idx_file.exists() {
                fs::remove_file(&idx_file)?;
            }
            Ok(())
        }.await;
        res
        }));
        }

        for handle in spawn_handles {
            handle
                .await
                .map_err(|e| format!("Spawn error: {}", e).into())
                .and_then(|res| res.map_err(|e| e as Box<dyn std::error::Error>))?;
        }
        println!("cargo:warning=Daox: Database introspection complete.");

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
                    let col_meta_fs = fs::symlink_metadata(&col_path)
                        .map_err(|e| format!("Cannot stat {:?}: {}", col_path, e))?;
                    if col_meta_fs.file_type().is_symlink() {
                        return Err(format!("Symlink rejected in schema: {:?}", col_path).into());
                    }
                    let file_stem = col_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .ok_or("Invalid path")?
                        .to_string();
                    if file_stem != "_table"
                        && file_stem != "_indexes"
                        && (!file_stem
                            .chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_')
                            || file_stem.chars().next().is_none_or(|c| c.is_ascii_digit()))
                    {
                        continue;
                    }
                    let content = read_toml_file_limited(&col_path)?;
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

                    if let Some(fmt) = &col_meta.format {
                        validate_regex(&fmt.value())?;
                    }

                    const ALLOWED_TYPES: &[&str] = &[
                        "bool",
                        "i8",
                        "i16",
                        "i32",
                        "i64",
                        "u8",
                        "u16",
                        "u32",
                        "u64",
                        "f32",
                        "f64",
                        "String",
                        "Vec<u8>",
                        "serde_json::Value",
                        "chrono::NaiveDate",
                        "chrono::DateTime<chrono::Utc>",
                        "chrono::NaiveDateTime",
                        "json",
                        "jsonb",
                    ];
                    let resolved = col_meta.rust_type_resolved();
                    if !ALLOWED_TYPES.contains(&resolved.as_str()) {
                        return Err(format!(
                            "Column '{}': forbidden type '{}'",
                            file_stem, resolved
                        )
                        .into());
                    }
                    columns.insert(file_stem, col_meta);
                }
            }
            Ok((config, columns, indexes))
        };

        let mut dirs_to_parse = Vec::new();
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
                    let table_name = entry
                        .file_name()
                        .into_string()
                        .map_err(|_| "Invalid utf8 table name")?;
                    if !is_safe_identifier(&table_name) {
                        eprintln!(
                            "Daox: table directory '{}' rejected (invalid identifier)",
                            table_name
                        );
                        continue;
                    }
                    ensure_safe_dir(schema_path, &path)?;
                    dirs_to_parse.push((path, None, table_name));
                } else {
                    let db_name = entry
                        .file_name()
                        .into_string()
                        .map_err(|_| "Invalid utf8 db name")?;
                    if !is_safe_identifier(&db_name) {
                        eprintln!(
                            "Daox: database directory '{}' rejected (invalid identifier)",
                            db_name
                        );
                        continue;
                    }
                    ensure_safe_dir(schema_path, &path)?;
                    if let Ok(subs) = fs::read_dir(&path) {
                        for sub in subs.flatten() {
                            if sub.path().is_dir() {
                                let table_name = sub
                                    .file_name()
                                    .into_string()
                                    .map_err(|_| "Invalid utf8 table name")?;
                                if !is_safe_identifier(&table_name) {
                                    eprintln!(
                                        "Daox: table directory '{}/{}' rejected (invalid identifier)",
                                        db_name, table_name
                                    );
                                    continue;
                                }
                                ensure_safe_dir(schema_path, &sub.path())?;
                                dirs_to_parse.push((sub.path(), Some(db_name.clone()), table_name));
                            }
                        }
                    }
                }
            }
        }

        use rayon::prelude::*;
        let parsed_results: Result<Vec<TableMetadata>, String> = dirs_to_parse
            .into_par_iter()
            .map(
                |(path, db_name_opt, table_name)| -> Result<TableMetadata, String> {
                    let (mut config, columns, indexes) =
                        parse_table_dir(&path).map_err(|e| e.to_string())?;
                    if let Some(db_name) = &db_name_opt {
                        config.database = db_name.clone();
                        Ok(TableMetadata {
                            name: format!("{}_{}", db_name, table_name),
                            sql_table_name: Some(table_name),
                            config,
                            columns,
                            indexes,
                        })
                    } else {
                        Ok(TableMetadata {
                            name: table_name,
                            sql_table_name: None,
                            config,
                            columns,
                            indexes,
                        })
                    }
                },
            )
            .collect();

        tables.extend(parsed_results.map_err(|e| -> Box<dyn std::error::Error> { e.into() })?);

        // Overrides
        let overrides_path = match std::env::var("DAOX_OVERRIDES_DIR") {
            Ok(dir) => {
                let p = std::path::PathBuf::from(dir);
                let meta = fs::symlink_metadata(&p)?;
                if meta.file_type().is_symlink() {
                    return Err(format!("Symlink rejected: {:?}", p).into());
                }
                let canonical = fs::canonicalize(&p)?;
                let canonical_base = fs::canonicalize(
                    schema_path
                        .parent()
                        .ok_or("schema_dir has no parent directory")?,
                )?;
                if !canonical.starts_with(&canonical_base) {
                    return Err(format!(
                        "Path traversal detected: {:?} is outside {:?}",
                        canonical, canonical_base
                    )
                    .into());
                }
                canonical
            }
            Err(_) => {
                let default = schema_path
                    .parent()
                    .ok_or("schema_dir has no parent directory")?
                    .join("overrides");
                if default.exists() {
                    let meta = fs::symlink_metadata(&default)?;
                    if meta.file_type().is_symlink() {
                        return Err(format!("Symlink rejected: {:?}", default).into());
                    }
                    fs::canonicalize(&default)?
                } else {
                    fs::create_dir_all(&default)?;
                    default
                }
            }
        };
        if !overrides_path.exists() {
            fs::create_dir_all(&overrides_path)?;
            write_if_changed(
                overrides_path.join("README.md"),
                "# Daox Overrides\n\nReplicate `schema/<table>/<column>.toml` here to override introspected values.\n",
            )?;
        }

        if overrides_path.is_dir() {
            for entry in fs::read_dir(overrides_path)? {
                let entry = entry?;
                let path = entry.path();
                let dir_meta = fs::symlink_metadata(&path)?;
                if dir_meta.file_type().is_symlink() {
                    return Err(
                        format!("Symlink rejected for override table dir: {:?}", path).into(),
                    );
                }
                if path.is_dir() {
                    let table_name = entry
                        .file_name()
                        .into_string()
                        .map_err(|_| "Invalid UTF-8 in override filename")?;
                    if let Some(table) = tables.iter_mut().find(|t| t.name == table_name) {
                        for col_entry in fs::read_dir(&path)? {
                            let col_entry = col_entry?;
                            let col_path = col_entry.path();
                            if col_path.is_file()
                                && col_path.extension().and_then(|s| s.to_str()) == Some("toml")
                            {
                                let col_meta_fs = fs::symlink_metadata(&col_path).map_err(|e| {
                                    format!("Cannot stat override file {:?}: {}", col_path, e)
                                })?;
                                if col_meta_fs.file_type().is_symlink() {
                                    return Err(format!(
                                        "Symlink rejected in overrides: {:?}",
                                        col_path
                                    )
                                    .into());
                                }
                                let file_stem = col_path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .ok_or("Invalid utf8 override path")?
                                    .to_string();
                                let content = read_toml_file_limited(&col_path)?;
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

                                        let resolved = col_meta.rust_type_resolved();
                                        const ALLOWED_TYPES: &[&str] = &[
                                            "bool",
                                            "i8",
                                            "i16",
                                            "i32",
                                            "i64",
                                            "u8",
                                            "u16",
                                            "u32",
                                            "u64",
                                            "f32",
                                            "f64",
                                            "String",
                                            "Vec<u8>",
                                            "serde_json::Value",
                                            "chrono::NaiveDate",
                                            "chrono::DateTime<chrono::Utc>",
                                            "chrono::NaiveDateTime",
                                            "json",
                                            "jsonb",
                                        ];
                                        if !ALLOWED_TYPES.contains(&resolved.as_str()) {
                                            return Err(format!(
                                                "Column override '{}': forbidden type '{}'",
                                                file_stem, resolved
                                            )
                                            .into());
                                        }
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
                                        validate_regex(&v.value())?;
                                        col_meta.format = Some(v);
                                    }
                                    if let Some(v) = ov.is_unique {
                                        col_meta.is_unique = v;
                                    }
                                    if let Some(v) = ov.is_index {
                                        col_meta.is_index = v;
                                    }
                                    if let Some(v) = ov.is_generated {
                                        col_meta.is_generated = v;
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
        if let Ok(content) = read_toml_file_limited(&path)
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
        println!(
            "cargo:warning=Daox: Unknown dialect alias for DB '{}' in databases.toml. Falling back to 'mysql'.",
            db_name
        );
        Dialect::MySql
    }

    // ── 3d. Code generation – PURE SQLX ONLY ──

    pub fn generate_dao(&self) -> Result<(), Box<dyn std::error::Error>> {
        let tables = self.parse_schema()?;
        const MAX_TABLES: usize = 1000;
        if tables.len() > MAX_TABLES {
            println!(
                "cargo:warning=Daox: {} tables detected, exceeding limit of {}. Truncating.",
                tables.len(),
                MAX_TABLES
            );
            return Err(format!(
                "Too many tables ({}) > limit ({})",
                tables.len(),
                MAX_TABLES
            )
            .into());
        }
        let tables: Vec<TableMetadata> = tables
            .into_iter()
            .filter(|t| {
                if t.columns.is_empty() {
                    println!("cargo:warning=Daox: table '{}' ignored: no columns", t.name);
                    false
                } else {
                    true
                }
            })
            .collect();

        const MAX_TOTAL_COLUMNS: usize = 10_000;
        let total_cols: usize = tables.iter().map(|t| t.columns.len()).sum();
        if total_cols > MAX_TOTAL_COLUMNS {
            return Err(format!(
                "Total column count ({}) exceeds limit ({}). Refusing to generate.",
                total_cols, MAX_TOTAL_COLUMNS
            )
            .into());
        }
        let out_path = Path::new(&self.out_dir);
        let mut mod_code = String::with_capacity(4096);
        mod_code.push_str("// Code generated automatically by daox. DO NOT EDIT.\n");
        mod_code.push_str("// Pure sqlx layer – zero framework dependency.\n\n");

        use rayon::prelude::*;
        let mut rustfmt_args = vec![];
        let results: Result<Vec<_>, String> = tables.par_iter().map(|table| {
            let mut code = String::with_capacity(1024 * 100);

            let struct_name = Self::to_pascal_case(&table.name);
            let db_name = if table.config.database.is_empty() {
                "default"
            } else {
                table.config.database.as_str()
            };
            let dialect = self.resolve_dialect(db_name);
            let db_type = dialect.sqlx_db_type();
            let sql_table = table.sql_table_name.as_ref().unwrap_or(&table.name);
            if !is_safe_identifier(sql_table) {
                return Err(format!(
                    "Table '{}' has unsafe SQL identifier after parsing",
                    sql_table
                ));
            }
            validate_identifier_for_dialect(sql_table, dialect)?;
            let sql_table_q = dialect.quote_ident(sql_table);

            // Sorted columns
            let mut cols: Vec<_> = table.columns.iter().collect();
            cols.sort_by(|a, b| a.0.cmp(b.0));

            const MAX_COLUMNS_PER_TABLE: usize = 200;
            if cols.len() > MAX_COLUMNS_PER_TABLE {
                return Err(format!(
                    "Table '{}' has {} columns, exceeding the limit of {}. \
                     Consider splitting the table.",
                    table.name, cols.len(), MAX_COLUMNS_PER_TABLE
                ));
            }

            for (col_name, _) in &cols {
                if !is_safe_identifier(col_name) {
                    return Err(format!(
                        "Column '{}' in table '{}' has unsafe identifier",
                        col_name, sql_table
                    ));
                }
                if let Err(e) = validate_identifier_for_dialect(col_name, dialect) {
                    return Err(format!("Column '{}' in table '{}' error: {}", col_name, sql_table, e));
                }
            }

            // ── Struct ──
            code.push_str("#[cfg(not(feature = \"validation\"))]\ncompile_error!(\n    \"Daox: the 'validation' feature is disabled. \\\n     Format regex checks in validate() will be skipped. \\\n     Enable with: features = [\\\"validation\\\"]\"\n);\n");
            code.push_str("#[allow(clippy::all)]\n#[derive(Debug, Clone, sqlx::FromRow)]\n");
            writeln!(&mut code, "pub struct {} {{", struct_name).unwrap();
            for (col_name, col_meta) in &cols {
                let field = Self::escape_rust_keyword(col_name);
                let mut ty = col_meta.rust_type_resolved();
                if col_meta.is_optional.value() {
                    ty = format!("Option<{}>", ty);
                }
                writeln!(&mut code, "    pub {}: {},", field, ty).unwrap();
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
                if col_meta.is_auto_increment.value() || col_meta.is_generated.value() {
                    // skip auto increment and generated columns from insert/update payload
                    if col_meta.is_auto_increment.value() {
                        auto_inc_col = Some(col_name.to_string());
                    }
                    continue;
                }
                insert_cols.push((col_name.to_string(), (*col_meta).clone()));
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
            let _max_params = dialect.max_bind_params();

            // ── OrderBy Enum ──
            let order_enum_name = format!("{}OrderBy", struct_name);
            code.push_str(&format!("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum {} {{\n", order_enum_name));
            for (col_name, _) in &cols {
                let pascal = Self::to_pascal_case(col_name);
                code.push_str(&format!("    {}Asc,\n    {}Desc,\n", pascal, pascal));
            }
            code.push_str("}\n\n");
            code.push_str(&format!("impl {} {{\n    pub fn as_str(&self) -> &'static str {{\n        match self {{\n", order_enum_name));
            for (col_name, _) in &cols {
                let pascal = Self::to_pascal_case(col_name);
                let quoted = dialect.quote_ident(col_name);
                code.push_str(&format!("            {}::{}Asc => r#\"{} ASC\"#,\n", order_enum_name, pascal, quoted));
                code.push_str(&format!("            {}::{}Desc => r#\"{} DESC\"#,\n", order_enum_name, pascal, quoted));
            }
            code.push_str("        }\n    }\n}\n\n");

            // ── impl block ──
            write!(&mut code, "#[allow(clippy::all)]\nimpl {} {{\n", struct_name).unwrap();

            // validate method
            write!(&mut code, "    #[allow(unused_comparisons, unused_mut)]\n    pub fn validate(&self) -> Result<(), Vec<String>> {{\n        #[cfg(not(feature = \"validation\"))]\n        {{\n            // Formats validation is disabled\n        }}\n        let mut errors = Vec::new();\n").unwrap();
            for (col_name, col_meta) in &cols {
                let field = Self::escape_rust_keyword(col_name);
                let is_opt = col_meta.is_optional.value();
                let ty = col_meta.rust_type_resolved();
                let val_ref = if is_opt { format!("self.{}.as_ref()", field) } else { format!("Some(&self.{})", field) };

                if ty == "String" || ty == "Vec<u8>" {
                    if let Some(prop) = &col_meta.min_length {
                        let min = prop.value();
                        if min > 0 {
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             if v.len() < {min} {{\n\
                                                 errors.push(\"{field}: min_length {min} not met\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, min = min
                            ).unwrap();
                        }
                    }
                    if let Some(prop) = &col_meta.max_length {
                        let max = prop.value();
                        write!(&mut code,
                            "        if let Some(v) = {val_ref} {{\n\
                                         if v.len() > {max} {{\n\
                                             errors.push(\"{field}: exceeds max_length {max}\".into());\n\
                                         }}\n\
                                     }}\n",
                            val_ref = val_ref, field = field, max = max
                        ).unwrap();
                    }
                }

                if ty == "String" {
                    if let Some(prop) = &col_meta.enum_values {
                        let evs = prop.value();
                        if !evs.is_empty() {
                            let evs_str = evs.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(", ");
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             let valid_enums = [{evs}];\n\
                                             if !valid_enums.contains(&v.as_str()) {{\n\
                                                 errors.push(\"{field}: invalid enum value\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, evs = evs_str
                            ).unwrap();
                        }
                    }
                    if let Some(prop) = &col_meta.format {
                        let re_str = prop.value();
                        if !re_str.is_empty() {
                            if let Err(err) = regex::Regex::new(&re_str) {
                                return Err(format!("Column '{}': invalid regex format '{}': {}", field, re_str, err));
                            }
                            let re_escaped = format!("{:?}", re_str);
                            write!(&mut code,
                                "        #[cfg(feature = \"validation\")]\n\
                                         if let Some(v) = {val_ref} {{\n\
                                             static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();\n\
                                             let re = RE.get_or_init(|| regex::Regex::new({re}).ok());\n\
                                             match re {{\n\
                                                 Some(re) => {{\n\
                                                     if !re.is_match(v) {{\n\
                                                         errors.push(\"{field}: format constraint not met\".into());\n\
                                                     }}\n\
                                                 }}\n\
                                                 None => {{\n\
                                                     errors.push(\"{field}: configured regex is invalid\".into());\n\
                                                 }}\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, re = re_escaped
                            ).unwrap();
                        }
                    }
                }

                if ty != "String" && ty != "Vec<u8>" && ty != "bool" && ty != "serde_json::Value" && !ty.starts_with("chrono::") {
                    if ty == "f32" || ty == "f64" {
                        write!(&mut code,
                            "        if let Some(v) = {val_ref} {{\n\
                                         if !v.is_finite() {{\n\
                                             errors.push(\"{field}: value must be finite (NaN/Infinity rejected)\".into());\n\
                                         }}\n\
                                     }}\n",
                            val_ref = val_ref, field = field
                        ).unwrap();
                    }
                    if let Some(prop) = &col_meta.min_value {
                        let min = prop.value();
                        if ty == "f32" || ty == "f64" {
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             if !v.is_finite() {{\n\
                                                 errors.push(\"{field}: value must be finite\".into());\n\
                                             }} else if (*v as f64) < ({min} as f64) {{\n\
                                                 errors.push(\"{field}: minimum value '{min}' not met\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, min = min
                            ).unwrap();
                        } else {
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             if (*v as i128) < ({min} as i128) {{\n\
                                                 errors.push(\"{field}: minimum value '{min}' not met\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, min = min
                            ).unwrap();
                        }
                    }
                    if let Some(prop) = &col_meta.max_value {
                        let max = prop.value();
                        if ty == "f32" || ty == "f64" {
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             if !v.is_finite() {{\n\
                                                 errors.push(\"{field}: value must be finite\".into());\n\
                                             }} else if (*v as f64) > ({max} as f64) {{\n\
                                                 errors.push(\"{field}: maximum value '{max}' exceeded\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, max = max
                            ).unwrap();
                        } else {
                            write!(&mut code,
                                "        if let Some(v) = {val_ref} {{\n\
                                             if (*v as i128) > ({max} as i128) {{\n\
                                                 errors.push(\"{field}: maximum value '{max}' exceeded\".into());\n\
                                             }}\n\
                                         }}\n",
                                val_ref = val_ref, field = field, max = max
                            ).unwrap();
                        }
                    }
                }
            }
            write!(&mut code, "        if errors.is_empty() {{ Ok(()) }} else {{ Err(errors) }}\n    }}\n\n").unwrap();

            // count
            write!(&mut code,
                "    /// Returns the total number of rows in the table.\n\
                     /// \n\
                     /// **⚠️ Performance Warning:** On some databases (e.g., MySQL/InnoDB, PostgreSQL), \n\
                     /// a `COUNT(*)` without a `WHERE` clause can cause a full table scan, \n\
                     /// which may take a long time on large tables (e.g. >10M rows).\n\
                     /// Consider caching this value or using an approximate row count from \n\
                     /// `information_schema.tables` or `pg_class` if exact precision is not required.\n\
                     #[deprecated(since = \"0.2.0\", note = \"Use `approximate_count` instead to prevent full table scans.\")]\n\
                     pub async fn count<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                         let query = r#\"SELECT COUNT(*) FROM {tbl}\"#;\n\
                         let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;\n\
                         Ok(count as u64)\n\
                     }}\n\n",
                db = db_type,
                tbl = sql_table_q,
            ).unwrap();

            // approximate_count
            if dialect == Dialect::Postgres {
                write!(&mut code,
                    "    /// Returns an approximate total number of rows in the table using database statistics (O(1)).\n\
                         /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.\n\
                         pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                             let query = r#\"SELECT reltuples::bigint FROM pg_class WHERE relname = $1\"#;\n\
                             let count: Option<(i64,)> = sqlx::query_as(query).bind(\"{tbl}\").fetch_optional(executor).await?;\n\
                             Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))\n\
                         }}\n\n",
                    db = db_type,
                    tbl = sql_table,
                ).unwrap();
            } else if dialect == Dialect::MySql {
                write!(&mut code,
                    "    /// Returns an approximate total number of rows in the table using database statistics (O(1)).\n\
                         /// WARNING (MySQL): For InnoDB tables, this value is an estimate and can vary significantly from the actual count.\n\
                         pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                             let query = r#\"SELECT table_rows FROM information_schema.tables WHERE table_name = ? AND table_schema = DATABASE()\"#;\n\
                             let count: Option<(i64,)> = sqlx::query_as(query).bind(\"{tbl}\").fetch_optional(executor).await?;\n\
                             Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))\n\
                         }}\n\n",
                    db = db_type,
                    tbl = sql_table,
                ).unwrap();
            } else {
                if table.config.is_view {
                    write!(&mut code,
                        "    /// Returns an approximate total number of rows in the view.\n\
                             /// (SQLite views do not support O(1) approximation, so this falls back to COUNT(*)).\n\
                             #[deprecated(since = \"0.2.0\", note = \"SQLite views fallback to COUNT(*). Avoid using this to prevent full table scans.\")]\n\
                             pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                                 #[allow(deprecated)]\n\
                                 Self::count(executor).await\n\
                             }}\n\n",
                        db = db_type,
                    ).unwrap();
                } else {
                    write!(&mut code,
                        "    /// Returns an estimated count upper bound using `MAX(rowid)` (O(1)).\n\
                              /// WARNING (SQLite): This overestimates the count if rows have been deleted.\n\
                              pub async fn estimated_count_upper_bound<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E) -> sqlx::Result<u64> {{\n\
                                  let query = r#\"SELECT MAX(rowid) FROM {tbl}\"#;\n\
                                  let count: Option<(Option<i64>,)> = sqlx::query_as(query).fetch_optional(executor).await?;\n\
                                  Ok(count.and_then(|(c,)| c).map(|c| c.max(0) as u64).unwrap_or(0))\n\
                              }}\n\n",
                        db = db_type, tbl = sql_table_q
                    ).unwrap();
                }
            }

            let default_order_sql = if pk_cols.is_empty() {
                let fallback = table
                    .indexes
                    .first()
                    .and_then(|idx| idx.columns.first())
                    .filter(|c| table.columns.contains_key(*c))
                    .map(|s| s.as_str())
                    .or_else(|| {
                        cols.iter()
                            .find(|(n, _)| **n == "created_at" || **n == "id")
                            .map(|(n, _)| n.as_str())
                    })
                    .unwrap_or_else(|| {
                        cols.first()
                            .expect("Daox invariant: table must have at least one column after filter")
                            .0
                            .as_str()
                    });
                format!("ORDER BY {} ASC", dialect.quote_ident(fallback))
            } else {
                let order_cols: Vec<String> = pk_cols.iter()
                    .map(|(n, _)| format!("{} ASC", dialect.quote_ident(n)))
                    .collect();
                format!("ORDER BY {}", order_cols.join(", "))
            };

            // stream_all
            write!(&mut code,
                "    /// Streams rows from the table, ordered by the primary key.\n\
                     /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.\n\
                     /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.\n\
                     #[deprecated(since = \"0.2.0\", note = \"Use cursor-based pagination instead to prevent pool starvation.\")]\n\
                     pub fn stream_all<'e, E: sqlx::Executor<'e, Database = {db}> + 'e>(executor: E, limit: i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n\
                         let limit = limit.clamp(1, 10000);\n\
                         let query = r#\"SELECT {cols} FROM {tbl} {order} LIMIT {lph}\"#;\n\
                         sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)\n\
                     }}\n\n",
                db = db_type,
                cols = all_cols_str,
                tbl = sql_table_q,
                order = default_order_sql,
                lph = dialect.placeholder(1)
            ).unwrap();

            // list_paginated removed

            // ── PK methods ──
            if !pk_cols.is_empty() {
                let pk_args: Vec<String> = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw = m.rust_type_resolved();
                        let at = match raw.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
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
                write!(&mut code,
                    "    pub async fn get_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<Option<Self>> {{\n\
                             let query = r#\"SELECT {cols} FROM {tbl} WHERE {wh}\"#;\n\
                             sqlx::query_as::<_, Self>(query){binds}.fetch_optional(executor).await\n\
                         }}\n\n",
                    sfx = pk_suffix, db = db_type, args = pk_args_str,
                    cols = all_cols_str, tbl = sql_table_q, wh = pk_where_str, binds = pk_binds_str,
                ).unwrap();

                // exists_by_pk
                write!(&mut code,
                    "    pub async fn exists_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<bool> {{\n\
                             let query = r#\"SELECT 1 FROM {tbl} WHERE {wh} LIMIT 1\"#;\n\
                             let exists: Option<(i32,)> = sqlx::query_as(query){binds}.fetch_optional(executor).await?;\n\
                             Ok(exists.is_some())\n\
                         }}\n\n",
                    sfx = pk_suffix, db = db_type, args = pk_args_str,
                    tbl = sql_table_q, wh = pk_where_str, binds = pk_binds_str,
                ).unwrap();

                // list_by_cursor (single PK only)
                if pk_cols.len() == 1 {
                    let (pk_n, pk_m) = &pk_cols[0];
                    let raw = pk_m.rust_type_resolved();
                    let cursor_at = match raw.as_str() {
                        "String" => "&str",
                        "Vec<u8>" => "&[u8]",
                        other => other,
                    };
                    let p1 = dialect.placeholder(1);
                    let p2 = dialect.placeholder(2);
                    write!(&mut code,
                        "    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, last_id: {cat}, limit: u32) -> sqlx::Result<Vec<Self>> {{\n\
                                 let limit = limit.clamp(1, 10000);\n\
                                 let query = r#\"SELECT {cols} FROM {tbl} WHERE {pk} > {p1} ORDER BY {pk} ASC LIMIT {p2}\"#;\n\
                                 sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await\n\
                             }}\n\n",
                        db = db_type, cat = cursor_at, cols = all_cols_str, tbl = sql_table_q,
                        pk = dialect.quote_ident(pk_n), p1 = p1, p2 = p2,
                    ).unwrap();
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
                            sql_table_q,
                            ins_cols_str,
                            ins_phs_str,
                            dialect.quote_ident(ai)
                        )
                    } else {
                        format!(
                            "INSERT INTO {} ({}) VALUES ({})",
                            sql_table_q, ins_cols_str, ins_phs_str
                        )
                    }
                } else {
                    format!(
                        "INSERT INTO {} ({}) VALUES ({})",
                        sql_table_q, ins_cols_str, ins_phs_str
                    )
                };

                write!(&mut code,
                    "    pub async fn insert_unchecked<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                             let query = r#\"{sql}\"#;\n",
                    db = db_type, sql = insert_sql,
                ).unwrap();

                if auto_inc_col.is_some() {
                    if dialect == Dialect::Postgres {
                        write!(&mut code,
                            "        let (id,): (i64,) = sqlx::query_as(query){binds}.fetch_one(executor).await?;\n\
                                     Ok(id as u64)\n\
                                 }}\n\n",
                            binds = ins_binds_str,
                        ).unwrap();
                    } else {
                        write!(&mut code,
                            "        let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                                     Ok({last})\n\
                                 }}\n\n",
                            binds = ins_binds_str,
                            last = if dialect == Dialect::Sqlite { "result.last_insert_rowid() as u64" } else { "result.last_insert_id()" },
                            db = db_type,
                        ).unwrap();
                    }
                } else {
                    write!(&mut code,
                        "        let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                                 Ok(result.rows_affected())\n\
                             }}\n\n",
                        binds = ins_binds_str,
                        db = db_type,
                    ).unwrap();
                }

                write!(&mut code,
                    "    pub async fn insert<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                             if let Err(e) = self.validate() {{\n\
                                 return Err(sqlx::Error::Protocol(e.join(\", \").into()));\n\
                             }}\n\
                             self.insert_unchecked(executor).await\n\
                         }}\n\n",
                    db = db_type
                ).unwrap();

                // insert_batch
                let col_count = insert_cols.len();
                let max_params = dialect.max_bind_params();
                if col_count > max_params {
                    return Err(format!(
                        "Table '{}' has {} columns, exceeding max bind parameters {} for dialect {:?}",
                        table.name, col_count, max_params, dialect
                    ));
                }
                if db_type == "sqlx::Postgres" {
                    write!(&mut code,
                        "    /// Inserts a batch of records using Postgres COPY (ultra-fast). \n\
                             /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.\n\
                              pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                  if items.is_empty() {{ return Ok(0); }}\n\
                                  for (idx, item) in items.iter().enumerate() {{\n\
                                      if let Err(e) = item.validate() {{\n\
                                          return Err(sqlx::Error::Protocol(format!(\"insert_batch: item {{}} failed validation: {{}}\", idx, e.join(\", \")).into()));\n\
                                      }}\n\
                                  }}\n\
                                  let mut copy_in = executor.copy_in_raw(r#\"COPY {tbl} ({cols}) FROM STDIN WITH (FORMAT csv)\"#).await?;\n\
                                 for chunk in items.chunks(1000) {{\n\
                                     let est: usize = chunk.iter().map(|item| {{\n\
                                         let mut s = 0usize;\n\
                                         let _ = item;\n",
                        db = db_type, tbl = sql_table_q, cols = ins_cols_str
                    ).unwrap();

                    for (n, m) in &insert_cols {
                        let field = Self::escape_rust_keyword(n);
                        let rust_type = m.rust_type_resolved();
                        let is_opt = m.is_optional.value();
                        match rust_type.as_str() {
                            "String" => {
                                if is_opt {
                                    writeln!(&mut code, "                                         s += item.{f}.as_ref().map_or(1, |v| v.len() + 2);", f = field).unwrap();
                                } else {
                                    writeln!(&mut code, "                                         s += item.{f}.len() + 2;", f = field).unwrap();
                                }
                            }
                            "Vec<u8>" => {
                                if is_opt {
                                    writeln!(&mut code, "                                         s += item.{f}.as_ref().map_or(1, |v| v.len() * 2 + 4);", f = field).unwrap();
                                } else {
                                    writeln!(&mut code, "                                         s += item.{f}.len() * 2 + 4;", f = field).unwrap();
                                }
                            }
                            "serde_json::Value" => {
                                if is_opt {
                                    writeln!(&mut code, "                                         s += item.{f}.as_ref().map_or(1, |v| v.to_string().len() + 2);", f = field).unwrap();
                                } else {
                                    writeln!(&mut code, "                                         s += item.{f}.to_string().len() + 2;", f = field).unwrap();
                                }
                            }
                            _ => {
                                writeln!(&mut code, "                                         s += 32;").unwrap();
                            }
                        }
                    }

                    write!(&mut code,
                        "                                         s\n\
                                     }}).sum();\n\
                                     let mut payload = String::with_capacity(est);\n\
                                     #[allow(unused_imports)]\n\
                                     use std::fmt::Write;\n\
                                     for item in chunk {{\n",
                    ).unwrap();

                    for (i, (n, m)) in insert_cols.iter().enumerate() {
                        let field = Self::escape_rust_keyword(n);
                        let rust_type = m.rust_type_resolved();
                        let is_opt = m.is_optional.value();
                        let write_stmt = match rust_type.as_str() {
                            "String" => {
                                r#"payload.push('"'); for c in v.chars() { if c == '"' { payload.push_str("\"\""); } else { payload.push(c); } } payload.push('"');"#
                            }
                            "chrono::DateTime<chrono::Utc>"
                            | "chrono::NaiveDate"
                            | "chrono::NaiveDateTime" => "write!(&mut payload, \"\\\"{}\\\"\", v).unwrap();",
                            "Vec<u8>" => {
                                r#"payload.push_str("\"\\x"); for b in v { write!(&mut payload, "{:02x}", b).unwrap(); } payload.push('"');"#
                            }
                            "serde_json::Value" => {
                                r#"let json_str = v.to_string(); payload.push('"'); for c in json_str.chars() { if c == '"' { payload.push_str("\"\""); } else { payload.push(c); } } payload.push('"');"#
                            }
                            "bool" => {
                                "if *v { payload.push_str(\"true\"); } else { payload.push_str(\"false\"); }"
                            }
                            _ => "write!(&mut payload, \"{}\", v).unwrap();", // Numeric
                        };
                        let val_expr = if is_opt {
                            format!("if let Some(v) = &item.{} {{ {} }}", field, write_stmt)
                        } else {
                            format!("{{ let v = &item.{}; {} }}", field, write_stmt)
                        };
                        if i > 0 {
                            code.push_str("                payload.push(',');\n");
                        }
                        writeln!(&mut code,
                            "                {}",
                            val_expr
                        ).unwrap();
                    }
                    code.push_str(
                        "                payload.push('\\n');\n\
                                         const MAX_COPY_VALUE_SIZE: usize = 100 * 1024 * 1024;\n\
                                         if payload.len() > MAX_COPY_VALUE_SIZE {\n\
                                             return Err(sqlx::Error::Protocol(format!(\"COPY payload exceeds {} bytes limit per chunk\", MAX_COPY_VALUE_SIZE).into()));\n\
                                         }\n\
                                         if payload.len() > 10 * 1024 * 1024 {{\n\
                                             copy_in.send(payload.as_bytes()).await?;\n\
                                             payload.clear();\n\
                                         }}\n\
                                     }\n\
                                     if !payload.is_empty() {{\n\
                                         copy_in.send(payload.as_bytes()).await?;\n\
                                     }}\n\
                                 }\n\
                                 copy_in.finish().await?;\n\
                                 Ok(items.len() as u64)\n\
                             }\n\n",
                    );
                } else {
                    write!(&mut code,
                        "    /// Inserts a batch of records. \n\
                             /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.\n\
                             pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 for (idx, item) in items.iter().enumerate() {{\n\
                                     if let Err(e) = item.validate() {{\n\
                                         return Err(sqlx::Error::Protocol(format!(\"insert_batch: item {{}} failed validation: {{}}\", idx, e.join(\", \")).into()));\n\
                                     }}\n\
                                 }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\
                                 let mut total_affected = 0;\n\
                                 for chunk in items.chunks(chunk_size.max(1)) {{\n\
                                     let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(r#\"INSERT INTO {tbl} ({cols}) \"#);\n\
                                     qb.push_values(chunk, |mut b, item| {{\n",
                        db = db_type, tbl = sql_table_q, cols = ins_cols_str, col_count = col_count, max_params = max_params
                    ).unwrap();
                    for (n, _) in &insert_cols {
                        writeln!(&mut code,
                            "            b.push_bind(&item.{});",
                            Self::escape_rust_keyword(n)
                        ).unwrap();
                    }
                    code.push_str(
                        "            });\n\
                                     let result = qb.build().execute(&mut **executor).await?;\n\
                                     total_affected += result.rows_affected();\n\
                                 }\n\
                                 Ok(total_affected)\n\
                             }\n\n",
                    );
                }

                // upsert
                if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
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
                        sql_table_q, upsert_cols_str, upsert_phs_str, suffix
                    );

                    write!(&mut code,
                        "    pub async fn upsert_unchecked<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                                 let query = r#\"{sql}\"#;\n\
                                 let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                                 Ok(result.rows_affected())\n\
                             }}\n\n",
                        db = db_type, sql = upsert_sql, binds = upsert_binds_str,
                    ).unwrap();

                    write!(&mut code,
                        "    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                                 if let Err(e) = self.validate() {{\n\
                                     return Err(sqlx::Error::Protocol(e.join(\", \").into()));\n\
                                 }}\n\
                                 self.upsert_unchecked(executor).await\n\
                             }}\n\n",
                        db = db_type
                    ).unwrap();

                    write!(&mut code,
                        "    /// Upserts a batch of records. \n\
                             /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.\n\
                             pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 for (idx, item) in items.iter().enumerate() {{\n\
                                     if let Err(e) = item.validate() {{\n\
                                         return Err(sqlx::Error::Protocol(format!(\"upsert_batch: item {{}} failed validation: {{}}\", idx, e.join(\", \")).into()));\n\
                                     }}\n\
                                 }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\
                                 let mut total_affected = 0;\n\
                                 for chunk in items.chunks(chunk_size.max(1)) {{\n\
                                     let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(r#\"INSERT INTO {tbl} ({cols}) \"#);\n\
                                     qb.push_values(chunk, |mut b, item| {{\n",
                        db = db_type, tbl = sql_table_q, cols = ins_cols_str, col_count = col_count, max_params = max_params
                    ).unwrap();
                    for (n, _) in &insert_cols {
                        writeln!(&mut code,
                            "            b.push_bind(&item.{});",
                            Self::escape_rust_keyword(n)
                        ).unwrap();
                    }
                    write!(&mut code,
                        "            }});\n\
                                     qb.push(r#\"{suffix}\"#);\n\
                                     let result = qb.build().execute(&mut **executor).await?;\n\
                                     total_affected += result.rows_affected();\n\
                                 }}\n\
                                 Ok(total_affected)\n\
                             }}\n\n",
                        suffix = suffix
                    ).unwrap();
                }

                // update_by_pk
                if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
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
                        writeln!(&mut bind_lines,
                            "        query = query.bind(&self.{});",
                            Self::escape_rust_keyword(n)
                        ).unwrap();
                    }
                    for (n, _) in &pk_cols {
                        writeln!(&mut bind_lines,
                            "        query = query.bind(&self.{});",
                            Self::escape_rust_keyword(n)
                        ).unwrap();
                    }

                    write!(&mut code,
                        "    pub async fn update_unchecked_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                                 let query_str = r#\"UPDATE {tbl} SET {set} WHERE {wh}\"#;\n\
                                 let mut query = sqlx::query::<{db}>(query_str);\n\
                         {binds}\
                                 let result = query.execute(executor).await?;\n\
                                 Ok(result.rows_affected())\n\
                             }}\n\n",
                        sfx = pk_suffix, db = db_type, tbl = sql_table_q,
                        set = set_str, wh = pk_where_str, binds = bind_lines,
                    ).unwrap();

                    write!(&mut code,
                        "    pub async fn update_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(&self, executor: E) -> sqlx::Result<u64> {{\n\
                                 if let Err(e) = self.validate() {{\n\
                                     return Err(sqlx::Error::Protocol(e.join(\", \").into()));\n\
                                 }}\n\
                                 self.update_unchecked_by_{sfx}(executor).await\n\
                             }}\n\n",
                        sfx = pk_suffix, db = db_type
                    ).unwrap();
                }

                // delete_by_pk
                if !table.config.is_view && !pk_cols.is_empty() {
                    let pk_args: Vec<String> = pk_cols
                        .iter()
                        .map(|(n, m)| {
                            let raw = m.rust_type_resolved();
                            let at = match raw.as_str() {
                                "String" => "&str",
                                "Vec<u8>" => "&[u8]",
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

                    write!(&mut code,
                        "    pub async fn delete_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<u64> {{\n\
                                 let query = r#\"DELETE FROM {tbl} WHERE {wh}\"#;\n\
                                 let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                                 Ok(result.rows_affected())\n\
                             }}\n\n",
                        sfx = pk_suffix, db = db_type, args = pk_args.join(", "),
                        tbl = sql_table_q, wh = pk_where.join(" AND "), binds = pk_binds.join(""),
                    ).unwrap();

                    // delete_many_by_pk (single PK)
                    if pk_cols.len() == 1 {
                        let (pk_n, pk_m) = &pk_cols[0];
                        let raw = pk_m.rust_type_resolved();
                        let id_type = match raw.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        let base_chunk = if id_type == "String" || id_type == "Vec<u8>" { 500 } else { 5000 };
                        write!(&mut code,
                            "    pub async fn delete_many_by_{sfx}<'e>(executor: &mut sqlx::Transaction<'e, {db}>, ids: &[{idt}]) -> sqlx::Result<u64> {{\n\
                                     if ids.is_empty() {{ return Ok(0); }}\n\
                                     let mut total_affected = 0;\n\
                                     let chunk_size = {base_chunk}_usize.min({max_params});\n\
                                     for chunk in ids.chunks(chunk_size) {{\n\
                                         let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(r#\"DELETE FROM {tbl} WHERE {pk} IN \"#);\n\
                                         qb.push(\"(\");\n\
                                         let mut sep = qb.separated(\", \");\n\
                                         for id in chunk {{ sep.push_bind(id); }}\n\
                                         sep.push_unseparated(\")\");\n\
                                         let result = qb.build().execute(&mut **executor).await?;\n\
                                         total_affected += result.rows_affected();\n\
                                     }}\n\
                                     Ok(total_affected)\n\
                                 }}\n\n",
                            sfx = pk_suffix, db = db_type, idt = id_type,
                            tbl = sql_table_q, pk = dialect.quote_ident(pk_n), max_params = max_params,
                        ).unwrap();
                    }
                }
            }

            // ── Patch struct + update_partial_by_pk ──
            if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
                if update_cols.len() > 10 {
                    return Err(format!(
                        "Table '{}': {} update columns exceeds the 10-column limit for update_partial_by_id. \
                         Consider splitting the table or using update_by_{} instead.",
                        table.name, update_cols.len(), pk_suffix
                    ));
                }

                let patch_name = format!("{}Patch", struct_name);
                let pk_args: Vec<String> = pk_cols
                    .iter()
                    .map(|(n, m)| {
                        let raw = m.rust_type_resolved();
                        let at = match raw.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
                            other => other,
                        };
                        format!("{}: {}", Self::escape_rust_keyword(n), at)
                    })
                    .collect();

                let tbl_escaped = sql_table_q.replace('"', "\\\"");
                write!(&mut code,
                    "    #[allow(unused_assignments, unused_comparisons, unused_mut, unused_variables)]\n\
                         pub async fn update_partial_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}, patch: &{patch}) -> sqlx::Result<u64> {{\n\
                             let mut errors: Vec<String> = Vec::new();\n",
                    sfx = pk_suffix, db = db_type,
                    args = pk_args.join(", "), patch = patch_name
                ).unwrap();

                for (n, col_meta) in update_cols.iter() {
                    let field = Self::escape_rust_keyword(n);
                    let is_opt = col_meta.is_optional.value();
                    let ty = col_meta.rust_type_resolved();

                    writeln!(&mut code, "                             if let Some(val) = &patch.{field} {{").unwrap();
                    let val_ref = if is_opt { "v" } else { "val" };
                    if is_opt {
                        writeln!(&mut code, "                                 if let Some(v) = val.as_ref() {{").unwrap();
                    }

                    if ty == "String" || ty == "Vec<u8>" {
                        if let Some(prop) = &col_meta.min_length {
                            let min = prop.value();
                            if min > 0 { writeln!(&mut code, "                                     if {val_ref}.len() < {min} {{ errors.push(\"{field}: min_length {min} not met\".into()); }}").unwrap(); }
                        }
                        if let Some(prop) = &col_meta.max_length {
                            let max = prop.value();
                            writeln!(&mut code, "                                     if {val_ref}.len() > {max} {{ errors.push(\"{field}: exceeds max_length {max}\".into()); }}").unwrap();
                        }
                    }

                    if ty == "String" {
                        if let Some(prop) = &col_meta.enum_values {
                            let evs = prop.value();
                            if !evs.is_empty() {
                                let evs_str = evs.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(", ");
                                write!(&mut code, "                                     let valid_enums = [{evs_str}];\n                                     if !valid_enums.contains(&{val_ref}.as_str()) {{ errors.push(\"{field}: invalid enum value\".into()); }}\n").unwrap();
                            }
                        }
                        if let Some(prop) = &col_meta.format {
                            let re_str = prop.value();
                            if !re_str.is_empty() {
                                let re_escaped = format!("{:?}", re_str);
                                write!(&mut code, "                                     #[cfg(feature = \"validation\")]\n                                     {{\n                                         static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();\n                                         let re = RE.get_or_init(|| regex::Regex::new({re_escaped}).ok());\n                                         if let Some(re) = re {{ if !re.is_match({val_ref}) {{ errors.push(\"{field}: format constraint not met\".into()); }} }}\n                                     }}\n").unwrap();
                            }
                        }
                    }

                    if ty != "String" && ty != "Vec<u8>" && ty != "bool" && ty != "serde_json::Value" && !ty.starts_with("chrono::") {
                        if ty == "f32" || ty == "f64" {
                            writeln!(&mut code, "                                     if !{val_ref}.is_finite() {{ errors.push(\"{field}: value must be finite (NaN/Infinity rejected)\".into()); }}").unwrap();
                        }
                        if let Some(prop) = &col_meta.min_value {
                            let min = prop.value();
                            let cast = if ty == "f32" || ty == "f64" { "f64" } else { "i128" };
                            writeln!(&mut code, "                                     if (*{val_ref} as {cast}) < ({min} as {cast}) {{ errors.push(\"{field}: minimum value '{min}' not met\".into()); }}").unwrap();
                        }
                        if let Some(prop) = &col_meta.max_value {
                            let max = prop.value();
                            let cast = if ty == "f32" || ty == "f64" { "f64" } else { "i128" };
                            writeln!(&mut code, "                                     if (*{val_ref} as {cast}) > ({max} as {cast}) {{ errors.push(\"{field}: exceeds max_value '{max}'\".into()); }}").unwrap();
                        }
                    }

                    if is_opt { writeln!(&mut code, "                                 }}").unwrap(); }
                    writeln!(&mut code, "                             }}").unwrap();
                }

                write!(&mut code,
                    "                             if !errors.is_empty() {{\n\
                                                      return Err(sqlx::Error::Protocol(errors.join(\", \").into()));\n\
                                                  }}\n\
                                                  let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"UPDATE {tbl} SET \");\n\
                                                  let mut first = true;\n",
                    db = db_type, tbl = tbl_escaped
                ).unwrap();

                for (n, _) in update_cols.iter() {
                    let field = Self::escape_rust_keyword(n);
                    let col_q = dialect.quote_ident(n).replace('"', "\\\"");
                    write!(&mut code,
                        "        if let Some(val) = &patch.{f} {{\n\
                                     if !first {{ qb.push(\", \"); }}\n\
                                     qb.push(\"{col} = \");\n\
                                     qb.push_bind(val.clone());\n\
                                     first = false;\n\
                                 }}\n",
                        f = field, col = col_q
                    ).unwrap();
                }

                writeln!(&mut code,
                    "        if first {{ return Ok(0); }}"
                ).unwrap();

                for (i, (n, _)) in pk_cols.iter().enumerate() {
                    let col_q = dialect.quote_ident(n).replace('"', "\\\"");
                    let conj = if i == 0 { " WHERE " } else { " AND " };
                    write!(&mut code,
                        "        qb.push(\"{conj}{col} = \");\n\
                                 qb.push_bind({f});\n",
                        conj = conj, col = col_q,
                        f = Self::escape_rust_keyword(n)
                    ).unwrap();
                }

                write!(&mut code,
                    "        let result = qb.build().execute(executor).await?;\n\
                             Ok(result.rows_affected())\n\
                         }}\n\n"
                ).unwrap();
            }

            // ── Index methods ──
            const MAX_INDEXES_PER_TABLE: usize = 50;
            if table.indexes.len() > MAX_INDEXES_PER_TABLE {
                println!(
                    "cargo:warning=Daox: table '{}' has {} indexes, truncating to {}",
                    table.name,
                    table.indexes.len(),
                    MAX_INDEXES_PER_TABLE
                );
            }
            for index in table.indexes.iter().take(MAX_INDEXES_PER_TABLE) {
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
                        let raw = m.rust_type_resolved();
                        let at = match raw.as_str() {
                            "String" => "&str",
                            "Vec<u8>" => "&[u8]",
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
                    write!(&mut code,
                        "    pub async fn get_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<Option<Self>> {{\n\
                                 let query = r#\"SELECT {cols} FROM {tbl} WHERE {wh}\"#;\n\
                                 sqlx::query_as::<_, Self>(query){binds}.fetch_optional(executor).await\n\
                             }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        cols = all_cols_str, tbl = sql_table_q, wh = where_str, binds = binds_str,
                    ).unwrap();
                } else {
                    // list_by (non-unique → Vec)
                    write!(&mut code,
                        "    pub async fn list_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}, limit: i64) -> sqlx::Result<Vec<Self>> {{\n\
                                 let limit = limit.clamp(1, 10000);\n\
                                 let query = r#\"SELECT {cols} FROM {tbl} WHERE {wh} {order} LIMIT {lph}\"#;\n\
                                 sqlx::query_as::<_, Self>(query){binds}.bind(limit).fetch_all(executor).await\n\
                             }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        cols = all_cols_str, tbl = sql_table_q, wh = where_str, binds = binds_str,
                        lph = dialect.placeholder(idx_args.len() + 1), order = default_order_sql,
                    ).unwrap();

                    // stream_by
                    let stream_args_str = args_str
                        .replace("&str", "&'e str")
                        .replace("&[u8]", "&'e [u8]");
                    write!(&mut code,
                        "    /// Streams rows from the table, filtered by {sfx}.\n\
                                 /// **⚠️ Performance Warning:** Unbounded streaming is potentially dangerous.\n\
                                 /// A `limit` parameter is now mandatory to prevent connection pool starvation. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.\n\
                                 #[deprecated(since = \"0.2.0\", note = \"Use cursor-based pagination instead to prevent pool starvation.\")]\n\
                                 pub fn stream_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}> + 'e>(executor: E, {args}, limit: i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {{\n\
                                 let limit = limit.clamp(1, 10000);\n\
                                 let query = r#\"SELECT {cols} FROM {tbl} WHERE {wh} {order} LIMIT {lph}\"#;\n\
                                 sqlx::query_as::<_, Self>(query){binds}.bind(limit).fetch(executor)\n\
                             }}\n\n",
                        sfx = method_suffix, db = db_type, args = stream_args_str,
                        cols = all_cols_str, tbl = sql_table_q, wh = where_str, binds = binds_str,
                        order = default_order_sql,
                        lph = dialect.placeholder(idx_args.len() + 1)
                    ).unwrap();
                }

                // exists_by
                write!(&mut code,
                    "    pub async fn exists_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<bool> {{\n\
                             let query = r#\"SELECT 1 FROM {tbl} WHERE {wh} LIMIT 1\"#;\n\
                             let exists: Option<(i32,)> = sqlx::query_as(query){binds}.fetch_optional(executor).await?;\n\
                             Ok(exists.is_some())\n\
                         }}\n\n",
                    sfx = method_suffix, db = db_type, args = args_str,
                    tbl = sql_table_q, wh = where_str, binds = binds_str,
                ).unwrap();

                // delete_by (non-view)
                if !table.config.is_view {
                    write!(&mut code,
                        "    pub async fn delete_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}) -> sqlx::Result<u64> {{\n\
                                 let query = r#\"DELETE FROM {tbl} WHERE {wh}\"#;\n\
                                 let result = sqlx::query::<{db}>(query){binds}.execute(executor).await?;\n\
                                 Ok(result.rows_affected())\n\
                             }}\n\n",
                        sfx = method_suffix, db = db_type, args = args_str,
                        tbl = sql_table_q, wh = where_str, binds = binds_str,
                    ).unwrap();
                }
            }

            code.push_str("}\n\n");

            if !table.config.is_view && !pk_cols.is_empty() && !update_cols.is_empty() {
                let patch_name = format!("{}Patch", struct_name);
                write!(&mut code,
                    "#[allow(clippy::all)]\n#[derive(Debug, Clone, Default)]\npub struct {} {{\n",
                    patch_name
                ).unwrap();
                for (n, m) in &update_cols {
                    let raw = m.rust_type_resolved();
                    let inner = if m.is_optional.value() {
                        format!("Option<{}>", raw)
                    } else {
                        raw
                    };
                    writeln!(&mut code,
                        "    pub {}: Option<{}>,",
                        Self::escape_rust_keyword(n),
                        inner
                    ).unwrap();
                }
                code.push_str("}\n\n");
            }

            let file_name = format!("{}.rs", table.name);
            let table_file = out_path.join(&file_name);
            let changed = write_if_changed(&table_file, &code).map_err(|e| e.to_string())?;

            let mod_ident = Self::escape_rust_keyword(&table.name);
            let m1 = format!("pub mod {};\n", mod_ident);
            let m2 = format!("pub use {}::*;\n", mod_ident);

            Ok((table_file, m1, m2, changed))
        }).collect();

        let results = results.map_err(Box::<dyn std::error::Error>::from)?;
        for (table_file, m1, m2, changed) in results {
            mod_code.push_str(&m1);
            mod_code.push_str(&m2);
            if changed {
                rustfmt_args.push(table_file);
            }
        }

        let mod_file = out_path.join("mod.rs");
        let mod_changed = write_if_changed(&mod_file, &mod_code)?;
        if mod_changed {
            rustfmt_args.push(mod_file);
        }

        // --- ORPHAN CLEANUP ---
        let mut expected_files = std::collections::HashSet::new();
        expected_files.insert("mod.rs".to_string());
        for t in &tables {
            expected_files.insert(format!("{}.rs", t.name));
        }
        if let Ok(entries) = fs::read_dir(out_path) {
            for entry in entries.flatten() {
                if let Ok(ft) = entry.file_type()
                    && ft.is_file()
                {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.ends_with(".rs") && !expected_files.contains(&name) {
                        let _ = fs::remove_file(entry.path());
                    }
                }
            }
        }

        #[allow(unused_imports)]
        use rayon::prelude::*;
        rustfmt_args.par_chunks(50).for_each(|chunk| {
            let _ = std::process::Command::new("rustfmt")
                .arg("--edition")
                .arg("2024")
                .args(chunk)
                .status();
        });

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
