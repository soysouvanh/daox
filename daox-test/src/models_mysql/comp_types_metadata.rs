#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i64,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<serde_json::Value>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesMetadataOrderBy {
    CompTypesIdAsc,
    CompTypesIdDesc,
    FBlobAsc,
    FBlobDesc,
    FDateAsc,
    FDateDesc,
    FDatetimeAsc,
    FDatetimeDesc,
    FJsonAsc,
    FJsonDesc,
    FTimestampAsc,
    FTimestampDesc,
    IdAsc,
    IdDesc,
}

impl CompTypesMetadataOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesMetadataOrderBy::CompTypesIdAsc => r#"`comp_types_id` ASC"#,
            CompTypesMetadataOrderBy::CompTypesIdDesc => r#"`comp_types_id` DESC"#,
            CompTypesMetadataOrderBy::FBlobAsc => r#"`f_blob` ASC"#,
            CompTypesMetadataOrderBy::FBlobDesc => r#"`f_blob` DESC"#,
            CompTypesMetadataOrderBy::FDateAsc => r#"`f_date` ASC"#,
            CompTypesMetadataOrderBy::FDateDesc => r#"`f_date` DESC"#,
            CompTypesMetadataOrderBy::FDatetimeAsc => r#"`f_datetime` ASC"#,
            CompTypesMetadataOrderBy::FDatetimeDesc => r#"`f_datetime` DESC"#,
            CompTypesMetadataOrderBy::FJsonAsc => r#"`f_json` ASC"#,
            CompTypesMetadataOrderBy::FJsonDesc => r#"`f_json` DESC"#,
            CompTypesMetadataOrderBy::FTimestampAsc => r#"`f_timestamp` ASC"#,
            CompTypesMetadataOrderBy::FTimestampDesc => r#"`f_timestamp` DESC"#,
            CompTypesMetadataOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesMetadataOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesMetadata {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.comp_types_id) {
            if (*v as i64) < 0 {
                errors.push("comp_types_id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.comp_types_id) {
            if (*v as i64) > 9223372036854775807 {
                errors.push("comp_types_id: maximum value '9223372036854775807' exceeded".into());
            }
        }
        if let Some(v) = self.f_blob.as_ref() {
            if v.len() > 65535 {
                errors.push("f_blob: exceeds max_length 65535".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i64) < 0 {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i64) > 9223372036854775807 {
                errors.push("id: maximum value '9223372036854775807' exceeded".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM `comp_types_metadata`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// WARNING (MySQL): For InnoDB tables, this value is an estimate and can vary significantly from the actual count.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'comp_types_metadata' AND table_schema = DATABASE()"#;
        let count: Option<(i64,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `comp_types_metadata` WHERE `id` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        last_id: i64,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_id)
            .bind(limit as i64)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.comp_types_id)
            .bind(&self.f_blob)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_json)
            .bind(&self.f_timestamp)
            .execute(executor)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn insert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.insert(executor).await.map_err(|e| e.into())
    }

    /// Inserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn insert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::MySql>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 65535 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.comp_types_id);
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_timestamp);
            });
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `comp_types_id` = VALUES(`comp_types_id`), `f_blob` = VALUES(`f_blob`), `f_date` = VALUES(`f_date`), `f_datetime` = VALUES(`f_datetime`), `f_json` = VALUES(`f_json`), `f_timestamp` = VALUES(`f_timestamp`)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.comp_types_id)
            .bind(&self.f_blob)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_json)
            .bind(&self.f_timestamp)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn upsert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.upsert(executor).await.map_err(|e| e.into())
    }

    /// Upserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn upsert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::MySql>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 65535 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.comp_types_id);
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_timestamp);
            });
            qb.push(r#" ON DUPLICATE KEY UPDATE `comp_types_id` = VALUES(`comp_types_id`), `f_blob` = VALUES(`f_blob`), `f_date` = VALUES(`f_date`), `f_datetime` = VALUES(`f_datetime`), `f_json` = VALUES(`f_json`), `f_timestamp` = VALUES(`f_timestamp`)"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::MySql>(query_str);
        query = query.bind(&self.comp_types_id);
        query = query.bind(&self.f_blob);
        query = query.bind(&self.f_date);
        query = query.bind(&self.f_datetime);
        query = query.bind(&self.f_json);
        query = query.bind(&self.f_timestamp);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_validated_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.update_by_id(executor).await.map_err(|e| e.into())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `comp_types_metadata` WHERE `id` = ?"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::MySql>,
        ids: &[i64],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(65535);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> =
                sqlx::QueryBuilder::new(r#"DELETE FROM `comp_types_metadata` WHERE `id` IN "#);
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
    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
        patch: &CompTypesMetadataPatch,
    ) -> sqlx::Result<u64> {
        let mut set_clauses: Vec<String> = Vec::new();
        let mut _param_idx = 1usize;
        if patch.comp_types_id.is_some() {
            set_clauses.push("`comp_types_id` = ?".to_string());
            _param_idx += 1;
        }
        if patch.f_blob.is_some() {
            set_clauses.push("`f_blob` = ?".to_string());
            _param_idx += 1;
        }
        if patch.f_date.is_some() {
            set_clauses.push("`f_date` = ?".to_string());
            _param_idx += 1;
        }
        if patch.f_datetime.is_some() {
            set_clauses.push("`f_datetime` = ?".to_string());
            _param_idx += 1;
        }
        if patch.f_json.is_some() {
            set_clauses.push("`f_json` = ?".to_string());
            _param_idx += 1;
        }
        if patch.f_timestamp.is_some() {
            set_clauses.push("`f_timestamp` = ?".to_string());
            _param_idx += 1;
        }
        if set_clauses.is_empty() {
            return Ok(0);
        }
        let mut query_str = format!(
            "UPDATE `comp_types_metadata` SET {}",
            set_clauses.join(", ")
        );
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
        let mut query = sqlx::query::<sqlx::MySql>(safe_query_str);
        if let Some(val) = &patch.comp_types_id {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_blob {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_date {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_datetime {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_json {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_timestamp {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesMetadataPatch {
    pub comp_types_id: Option<i64>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<serde_json::Value>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}
