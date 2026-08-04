#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: i64,
    pub last_name: String,
    pub status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsersOrderBy {
    CreatedAtAsc,
    CreatedAtDesc,
    EmailAsc,
    EmailDesc,
    FirstNameAsc,
    FirstNameDesc,
    IdAsc,
    IdDesc,
    LastNameAsc,
    LastNameDesc,
    StatusAsc,
    StatusDesc,
}

impl UsersOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            UsersOrderBy::CreatedAtAsc => r#"`created_at` ASC"#,
            UsersOrderBy::CreatedAtDesc => r#"`created_at` DESC"#,
            UsersOrderBy::EmailAsc => r#"`email` ASC"#,
            UsersOrderBy::EmailDesc => r#"`email` DESC"#,
            UsersOrderBy::FirstNameAsc => r#"`first_name` ASC"#,
            UsersOrderBy::FirstNameDesc => r#"`first_name` DESC"#,
            UsersOrderBy::IdAsc => r#"`id` ASC"#,
            UsersOrderBy::IdDesc => r#"`id` DESC"#,
            UsersOrderBy::LastNameAsc => r#"`last_name` ASC"#,
            UsersOrderBy::LastNameDesc => r#"`last_name` DESC"#,
            UsersOrderBy::StatusAsc => r#"`status` ASC"#,
            UsersOrderBy::StatusDesc => r#"`status` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl Users {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.email) {
            if v.len() < 1 {
                errors.push("email: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.email) {
            if v.len() > 255 {
                errors.push("email: exceeds max_length 255".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.email) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^([a-zA-Z0-9_\\-\\.]+)@([a-zA-Z0-9_\\-\\.]+)\\.([a-zA-Z]{2,5})$")
                    .expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("email: format constraint not met".into());
            }
        }
        if let Some(v) = self.first_name.as_ref() {
            if v.len() > 100 {
                errors.push("first_name: exceeds max_length 100".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.first_name.as_ref() {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("first_name: format constraint not met".into());
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
        if let Some(v) = Some(&self.last_name) {
            if v.len() < 1 {
                errors.push("last_name: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.last_name) {
            if v.len() > 100 {
                errors.push("last_name: exceeds max_length 100".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.last_name) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("last_name: format constraint not met".into());
            }
        }
        if let Some(v) = Some(&self.status) {
            if v.len() < 1 {
                errors.push("status: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.status) {
            if v.len() > 50 {
                errors.push("status: exceeds max_length 50".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.status) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("status: format constraint not met".into());
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
        let query = r#"SELECT COUNT(*) FROM `users`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// WARNING (MySQL): For InnoDB tables, this value is an estimate and can vary significantly from the actual count.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'users' AND table_schema = DATABASE()"#;
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
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `users` WHERE `id` = ? LIMIT 1"#;
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
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
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
        let query = r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.created_at)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
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
        let chunk_size = 65535 / 5;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.created_at);
                b.push_bind(&item.email);
                b.push_bind(&item.first_name);
                b.push_bind(&item.last_name);
                b.push_bind(&item.status);
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
        let query = r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.created_at)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
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
        let chunk_size = 65535 / 5;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.created_at);
                b.push_bind(&item.email);
                b.push_bind(&item.first_name);
                b.push_bind(&item.last_name);
                b.push_bind(&item.status);
            });
            qb.push(r#" ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::MySql>(query_str);
        query = query.bind(&self.created_at);
        query = query.bind(&self.email);
        query = query.bind(&self.first_name);
        query = query.bind(&self.last_name);
        query = query.bind(&self.status);
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
        let query = r#"DELETE FROM `users` WHERE `id` = ?"#;
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
                sqlx::QueryBuilder::new(r#"DELETE FROM `users` WHERE `id` IN "#);
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
        patch: &UsersPatch,
    ) -> sqlx::Result<u64> {
        let mut set_clauses: Vec<String> = Vec::new();
        let mut _param_idx = 1usize;
        if patch.created_at.is_some() {
            set_clauses.push("`created_at` = ?".to_string());
            _param_idx += 1;
        }
        if patch.email.is_some() {
            set_clauses.push("`email` = ?".to_string());
            _param_idx += 1;
        }
        if patch.first_name.is_some() {
            set_clauses.push("`first_name` = ?".to_string());
            _param_idx += 1;
        }
        if patch.last_name.is_some() {
            set_clauses.push("`last_name` = ?".to_string());
            _param_idx += 1;
        }
        if patch.status.is_some() {
            set_clauses.push("`status` = ?".to_string());
            _param_idx += 1;
        }
        if set_clauses.is_empty() {
            return Ok(0);
        }
        let mut query_str = format!("UPDATE `users` SET {}", set_clauses.join(", "));
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
        if let Some(val) = &patch.created_at {
            query = query.bind(val);
        }
        if let Some(val) = &patch.email {
            query = query.bind(val);
        }
        if let Some(val) = &patch.first_name {
            query = query.bind(val);
        }
        if let Some(val) = &patch.last_name {
            query = query.bind(val);
        }
        if let Some(val) = &patch.status {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        email: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `email` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(email)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        email: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `users` WHERE `email` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        email: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `users` WHERE `email` = ?"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(email)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::MySql>,
    >(
        executor: E,
        last_name: &str,
        first_name: &str,
        limit: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_name)
            .bind(first_name)
            .bind(limit)
            .fetch_all(executor)
            .await
    }

    /// Streams rows from the table, filtered by last_name_and_first_name.
    /// **⚠️ Performance Warning:** Unbounded streaming is potentially dangerous.
    /// A `limit` parameter is now mandatory to prevent connection pool starvation. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e,
    >(
        executor: E,
        last_name: &'e str,
        first_name: &'e str,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_name)
            .bind(first_name)
            .bind(limit)
            .fetch(executor)
    }

    pub async fn exists_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::MySql>,
    >(
        executor: E,
        last_name: &str,
        first_name: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `users` WHERE `last_name` = ? AND `first_name` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(last_name)
            .bind(first_name)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn delete_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::MySql>,
    >(
        executor: E,
        last_name: &str,
        first_name: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `users` WHERE `last_name` = ? AND `first_name` = ?"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(last_name)
            .bind(first_name)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct UsersPatch {
    pub created_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub email: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<String>,
    pub status: Option<String>,
}
