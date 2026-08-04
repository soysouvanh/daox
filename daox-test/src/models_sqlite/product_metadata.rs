#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProductMetadata {
    pub attributes: Option<String>,
    pub category: String,
    pub id: String,
    pub raw_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductMetadataOrderBy {
    AttributesAsc,
    AttributesDesc,
    CategoryAsc,
    CategoryDesc,
    IdAsc,
    IdDesc,
    RawDataAsc,
    RawDataDesc,
}

impl ProductMetadataOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductMetadataOrderBy::AttributesAsc => r#"`attributes` ASC"#,
            ProductMetadataOrderBy::AttributesDesc => r#"`attributes` DESC"#,
            ProductMetadataOrderBy::CategoryAsc => r#"`category` ASC"#,
            ProductMetadataOrderBy::CategoryDesc => r#"`category` DESC"#,
            ProductMetadataOrderBy::IdAsc => r#"`id` ASC"#,
            ProductMetadataOrderBy::IdDesc => r#"`id` DESC"#,
            ProductMetadataOrderBy::RawDataAsc => r#"`raw_data` ASC"#,
            ProductMetadataOrderBy::RawDataDesc => r#"`raw_data` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl ProductMetadata {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        #[cfg(feature = "validation")]
        if let Some(v) = self.attributes.as_ref() {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("attributes: format constraint not met".into());
            }
        }
        if let Some(v) = Some(&self.category) {
            if v.len() < 1 {
                errors.push("category: min_length 1 not met".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.category) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("category: format constraint not met".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if v.len() < 1 {
                errors.push("id: min_length 1 not met".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.id) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("id: format constraint not met".into());
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Returns the total number of rows in the table.
    ///
    /// **⚠️ Performance Warning:** On some databases (e.g., MySQL/InnoDB, PostgreSQL),
    /// a `COUNT(*)` without a `WHERE` clause can cause a full table scan,
    /// which may take a long time on large tables (e.g. >10M rows).
    /// Consider caching this value or using an approximate row count from
    /// `information_schema.tables` or `pg_class` if exact precision is not required.
    #[deprecated(
        since = "0.2.0",
        note = "Use `approximate_count` instead to prevent full table scans."
    )]
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM `product_metadata`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table.
    /// WARNING (SQLite): Uses `MAX(rowid)` which overestimates the count if rows have been deleted.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT MAX(rowid) FROM `product_metadata`"#;
        let count: Option<(Option<i64>,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count
            .and_then(|(c,)| c)
            .map(|c| c.max(0) as u64)
            .unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `attributes`, `category`, `id`, `raw_data` FROM `product_metadata` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `attributes`, `category`, `id`, `raw_data` FROM `product_metadata` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `product_metadata` WHERE `id` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        last_id: &str,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `attributes`, `category`, `id`, `raw_data` FROM `product_metadata` WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_id)
            .bind(limit as i64)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `product_metadata` (`attributes`, `category`, `id`, `raw_data`) VALUES (?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.attributes)
            .bind(&self.category)
            .bind(&self.id)
            .bind(&self.raw_data)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn insert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.insert(executor).await.map_err(|e| e.into())
    }

    /// Inserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn insert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 32766 / 4;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `product_metadata` (`attributes`, `category`, `id`, `raw_data`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.attributes);
                b.push_bind(&item.category);
                b.push_bind(&item.id);
                b.push_bind(&item.raw_data);
            });
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `product_metadata` (`attributes`, `category`, `id`, `raw_data`) VALUES (?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `attributes` = EXCLUDED.`attributes`, `category` = EXCLUDED.`category`, `raw_data` = EXCLUDED.`raw_data`"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.attributes)
            .bind(&self.category)
            .bind(&self.id)
            .bind(&self.raw_data)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn upsert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.upsert(executor).await.map_err(|e| e.into())
    }

    /// Upserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn upsert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 32766 / 4;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `product_metadata` (`attributes`, `category`, `id`, `raw_data`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.attributes);
                b.push_bind(&item.category);
                b.push_bind(&item.id);
                b.push_bind(&item.raw_data);
            });
            qb.push(r#" ON CONFLICT (`id`) DO UPDATE SET `attributes` = EXCLUDED.`attributes`, `category` = EXCLUDED.`category`, `raw_data` = EXCLUDED.`raw_data`"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `product_metadata` SET `attributes` = ?, `category` = ?, `raw_data` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.attributes);
        query = query.bind(&self.category);
        query = query.bind(&self.raw_data);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_validated_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.update_by_id(executor).await.map_err(|e| e.into())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `product_metadata` WHERE `id` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        ids: &[&str],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(32766);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
                sqlx::QueryBuilder::new(r#"DELETE FROM `product_metadata` WHERE `id` IN "#);
            qb.push("(");
            let mut sep = qb.separated(", ");
            for id in chunk {
                sep.push_bind(id);
            }
            sep.push_unseparated(")");
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    #[allow(unused_assignments)]
    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: &str,
        patch: &ProductMetadataPatch,
    ) -> sqlx::Result<u64> {
        let mut set_clauses: Vec<String> = Vec::new();
        let mut _param_idx = 1usize;
        if patch.attributes.is_some() {
            set_clauses.push("`attributes` = ?".to_string());
            _param_idx += 1;
        }
        if patch.category.is_some() {
            set_clauses.push("`category` = ?".to_string());
            _param_idx += 1;
        }
        if patch.raw_data.is_some() {
            set_clauses.push("`raw_data` = ?".to_string());
            _param_idx += 1;
        }
        if set_clauses.is_empty() {
            return Ok(0);
        }
        let mut query_str = format!("UPDATE `product_metadata` SET {}", set_clauses.join(", "));
        query_str.push_str(" WHERE `id` = ?");
        _param_idx += 1;
        static CACHE: std::sync::OnceLock<
            std::sync::RwLock<std::collections::HashMap<String, &'static str>>,
        > = std::sync::OnceLock::new();
        let cache = CACHE.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()));
        let safe_query_str: &'static str = {
            if let Some(s) = cache.read().unwrap().get(&query_str) {
                *s
            } else {
                let leaked = Box::leak(query_str.clone().into_boxed_str());
                cache.write().unwrap().insert(query_str, leaked);
                leaked
            }
        };
        let mut query = sqlx::query::<sqlx::Sqlite>(safe_query_str);
        if let Some(val) = &patch.attributes {
            query = query.bind(val);
        }
        if let Some(val) = &patch.category {
            query = query.bind(val);
        }
        if let Some(val) = &patch.raw_data {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct ProductMetadataPatch {
    pub attributes: Option<Option<String>>,
    pub category: Option<String>,
    pub raw_data: Option<Option<Vec<u8>>>,
}
