#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Currencies {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrenciesOrderBy {
    CodeAsc,
    CodeDesc,
    NameAsc,
    NameDesc,
}

impl CurrenciesOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrenciesOrderBy::CodeAsc => r#"`code` ASC"#,
            CurrenciesOrderBy::CodeDesc => r#"`code` DESC"#,
            CurrenciesOrderBy::NameAsc => r#"`name` ASC"#,
            CurrenciesOrderBy::NameDesc => r#"`name` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl Currencies {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.code) {
            if v.len() < 1 {
                errors.push("code: min_length 1 not met".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.code) {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("code: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("code: configured regex is invalid".into());
                }
            }
        }
        if let Some(v) = Some(&self.name) {
            if v.len() < 1 {
                errors.push("name: min_length 1 not met".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.name) {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("name: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("name: configured regex is invalid".into());
                }
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
        let query = r#"SELECT COUNT(*) FROM `currencies`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table.
    /// WARNING (SQLite): Uses `MAX(rowid)` which overestimates the count if rows have been deleted.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT MAX(rowid) FROM `currencies`"#;
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
        let query = r#"SELECT `code`, `name` FROM `currencies` ORDER BY `code` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `code`, `name` FROM `currencies` WHERE `code` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(code)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `currencies` WHERE `code` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(code)
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
        let query = r#"SELECT `code`, `name` FROM `currencies` WHERE `code` > ? ORDER BY `code` ASC LIMIT ?"#;
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
        let query = r#"INSERT INTO `currencies` (`code`, `name`) VALUES (?, ?)"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.code)
            .bind(&self.name)
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
        let chunk_size = 32766 / 2;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
                sqlx::QueryBuilder::new(r#"INSERT INTO `currencies` (`code`, `name`) "#);
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.code);
                b.push_bind(&item.name);
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
        let query = r#"INSERT INTO `currencies` (`code`, `name`) VALUES (?, ?) ON CONFLICT (`code`) DO UPDATE SET `name` = EXCLUDED.`name`"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.code)
            .bind(&self.name)
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
        let chunk_size = 32766 / 2;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
                sqlx::QueryBuilder::new(r#"INSERT INTO `currencies` (`code`, `name`) "#);
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.code);
                b.push_bind(&item.name);
            });
            qb.push(r#" ON CONFLICT (`code`) DO UPDATE SET `name` = EXCLUDED.`name`"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `currencies` SET `name` = ? WHERE `code` = ?"#;
        let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.name);
        query = query.bind(&self.code);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_validated_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.update_by_code(executor).await.map_err(|e| e.into())
    }

    pub async fn delete_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `currencies` WHERE `code` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(code)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_code<'e>(
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
                sqlx::QueryBuilder::new(r#"DELETE FROM `currencies` WHERE `code` IN "#);
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
    pub async fn update_partial_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        code: &str,
        patch: &CurrenciesPatch,
    ) -> sqlx::Result<u64> {
        let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
            sqlx::QueryBuilder::new("UPDATE `currencies` SET ");
        let mut first = true;
        if let Some(val) = &patch.name {
            if !first {
                qb.push(", ");
            }
            qb.push("`name` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if first {
            return Ok(0);
        }
        qb.push(" WHERE `code` = ");
        qb.push_bind(code);
        let result = qb.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CurrenciesPatch {
    pub name: Option<String>,
}
