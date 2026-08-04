#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: i32,
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
            if (*v as i64) > 2147483647 {
                errors.push("id: maximum value '2147483647' exceeded".into());
            }
        }
        if let Some(v) = Some(&self.last_name) {
            if v.len() < 1 {
                errors.push("last_name: min_length 1 not met".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM `users`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table.
    /// Uses `MAX(rowid)` to provide an instant O(1) estimate without a full table scan.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT MAX(rowid) FROM `users`"#;
        let count: Option<(Option<i64>,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count
            .and_then(|(c,)| c)
            .map(|c| c.max(0) as u64)
            .unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `users` WHERE `id` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        last_id: i32,
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

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.created_at)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .execute(executor)
            .await?;
        Ok(result.last_insert_rowid() as u64)
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
        let chunk_size = 32766 / 5;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
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

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `created_at` = EXCLUDED.`created_at`, `email` = EXCLUDED.`email`, `first_name` = EXCLUDED.`first_name`, `last_name` = EXCLUDED.`last_name`, `status` = EXCLUDED.`status`"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.created_at)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
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
        let chunk_size = 32766 / 5;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `users` (`created_at`, `email`, `first_name`, `last_name`, `status`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.created_at);
                b.push_bind(&item.email);
                b.push_bind(&item.first_name);
                b.push_bind(&item.last_name);
                b.push_bind(&item.status);
            });
            qb.push(r#" ON CONFLICT (`id`) DO UPDATE SET `created_at` = EXCLUDED.`created_at`, `email` = EXCLUDED.`email`, `first_name` = EXCLUDED.`first_name`, `last_name` = EXCLUDED.`last_name`, `status` = EXCLUDED.`status`"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.created_at);
        query = query.bind(&self.email);
        query = query.bind(&self.first_name);
        query = query.bind(&self.last_name);
        query = query.bind(&self.status);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `users` WHERE `id` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        ids: &[i32],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(32766);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
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
    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
        patch: &UsersPatch,
    ) -> sqlx::Result<u64> {
        let mut mask = 0u64;
        let mut has = false;
        if patch.created_at.is_some() {
            mask |= 1 << 0;
            has = true;
        }
        if patch.email.is_some() {
            mask |= 1 << 1;
            has = true;
        }
        if patch.first_name.is_some() {
            mask |= 1 << 2;
            has = true;
        }
        if patch.last_name.is_some() {
            mask |= 1 << 3;
            has = true;
        }
        if patch.status.is_some() {
            mask |= 1 << 4;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        let query_str = match mask {
            1 => r#"UPDATE `users` SET `created_at` = ? WHERE `id` = ?"#,
            2 => r#"UPDATE `users` SET `email` = ? WHERE `id` = ?"#,
            3 => r#"UPDATE `users` SET `created_at` = ?, `email` = ? WHERE `id` = ?"#,
            4 => r#"UPDATE `users` SET `first_name` = ? WHERE `id` = ?"#,
            5 => r#"UPDATE `users` SET `created_at` = ?, `first_name` = ? WHERE `id` = ?"#,
            6 => r#"UPDATE `users` SET `email` = ?, `first_name` = ? WHERE `id` = ?"#,
            7 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ? WHERE `id` = ?"#
            }
            8 => r#"UPDATE `users` SET `last_name` = ? WHERE `id` = ?"#,
            9 => r#"UPDATE `users` SET `created_at` = ?, `last_name` = ? WHERE `id` = ?"#,
            10 => r#"UPDATE `users` SET `email` = ?, `last_name` = ? WHERE `id` = ?"#,
            11 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `last_name` = ? WHERE `id` = ?"#
            }
            12 => r#"UPDATE `users` SET `first_name` = ?, `last_name` = ? WHERE `id` = ?"#,
            13 => {
                r#"UPDATE `users` SET `created_at` = ?, `first_name` = ?, `last_name` = ? WHERE `id` = ?"#
            }
            14 => {
                r#"UPDATE `users` SET `email` = ?, `first_name` = ?, `last_name` = ? WHERE `id` = ?"#
            }
            15 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ? WHERE `id` = ?"#
            }
            16 => r#"UPDATE `users` SET `status` = ? WHERE `id` = ?"#,
            17 => r#"UPDATE `users` SET `created_at` = ?, `status` = ? WHERE `id` = ?"#,
            18 => r#"UPDATE `users` SET `email` = ?, `status` = ? WHERE `id` = ?"#,
            19 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `status` = ? WHERE `id` = ?"#
            }
            20 => r#"UPDATE `users` SET `first_name` = ?, `status` = ? WHERE `id` = ?"#,
            21 => {
                r#"UPDATE `users` SET `created_at` = ?, `first_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            22 => {
                r#"UPDATE `users` SET `email` = ?, `first_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            23 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            24 => r#"UPDATE `users` SET `last_name` = ?, `status` = ? WHERE `id` = ?"#,
            25 => {
                r#"UPDATE `users` SET `created_at` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            26 => r#"UPDATE `users` SET `email` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#,
            27 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            28 => {
                r#"UPDATE `users` SET `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            29 => {
                r#"UPDATE `users` SET `created_at` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            30 => {
                r#"UPDATE `users` SET `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            31 => {
                r#"UPDATE `users` SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#
            }
            _ => unreachable!(),
        };

        let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
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

    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        email: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM `users` WHERE `email` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(email)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
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

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        email: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `users` WHERE `email` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(email)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
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
    /// A `limit` parameter is now mandatory to prevent connection pool starvation.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e,
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
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
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
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    >(
        executor: E,
        last_name: &str,
        first_name: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `users` WHERE `last_name` = ? AND `first_name` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
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
