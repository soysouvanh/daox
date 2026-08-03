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
        if let Some(v) = self.first_name.as_ref() {
            if v.len() > 100 {
                errors.push("first_name: exceeds max_length 100".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM users"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'users' AND table_schema = DATABASE()"#;
        let count: Option<(i64,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM users WHERE `id` = ? LIMIT 1"#;
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
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
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
        let query = r#"INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?)"#;
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
                r#"INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) "#,
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
        let query = r#"INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)"#;
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
                r#"INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.created_at);
                b.push_bind(&item.email);
                b.push_bind(&item.first_name);
                b.push_bind(&item.last_name);
                b.push_bind(&item.status);
            });
            qb.push(" ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)");
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE users SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?"#;
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM users WHERE `id` = ?"#;
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
                sqlx::QueryBuilder::new(r#"DELETE FROM users WHERE `id` IN "#);
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
        let mut bits = [0u8; 1];
        let mut has = false;
        if patch.created_at.is_some() {
            bits[0] |= 1 << 0;
            has = true;
        }
        if patch.email.is_some() {
            bits[0] |= 1 << 1;
            has = true;
        }
        if patch.first_name.is_some() {
            bits[0] |= 1 << 2;
            has = true;
        }
        if patch.last_name.is_some() {
            bits[0] |= 1 << 3;
            has = true;
        }
        if patch.status.is_some() {
            bits[0] |= 1 << 4;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        static CACHE: std::sync::OnceLock<
            [std::sync::RwLock<std::collections::HashMap<[u8; 1], String>>; 16],
        > = std::sync::OnceLock::new();
        let cache_shards = CACHE.get_or_init(|| {
            std::array::from_fn(|_| std::sync::RwLock::new(std::collections::HashMap::new()))
        });
        let shard_idx = bits
            .iter()
            .fold(0usize, |acc, &b| acc.wrapping_add(b as usize) ^ (acc << 3))
            % 16;
        let cache_lock = &cache_shards[shard_idx];
        let query_str = {
            let read = cache_lock.read().unwrap();
            if let Some(q) = read.get(&bits) {
                q.clone()
            } else {
                drop(read);
                let mut write = cache_lock.write().unwrap();
                if let Some(q) = write.get(&bits) {
                    q.clone()
                } else {
                    let mut q = String::with_capacity(416);
                    q.push_str("UPDATE users SET ");
                    let mut first = true;
                    if patch.created_at.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`created_at` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.email.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`email` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.first_name.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`first_name` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.last_name.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`last_name` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.status.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`status` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    q.push_str(r#" WHERE `id` = "#);
                    q.push_str("?");
                    if write.len() < 1000 {
                        write.insert(bits, q.clone());
                    }
                    q
                }
            }
        };

        let mut query = sqlx::query::<sqlx::MySql>(sqlx::AssertSqlSafe(query_str.as_str()));
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
        email: &String,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `email` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(email)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        email: &String,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM users WHERE `email` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(email)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        email: &String,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM users WHERE `email` = ?"#;
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
        last_name: &String,
        first_name: &String,
        limit: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?"#;
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
    pub fn stream_by_last_name_and_first_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e,
    >(
        executor: E,
        last_name: &'e String,
        first_name: &'e String,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?"#;
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
        last_name: &String,
        first_name: &String,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM users WHERE `last_name` = ? AND `first_name` = ? LIMIT 1"#;
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
        last_name: &String,
        first_name: &String,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM users WHERE `last_name` = ? AND `first_name` = ?"#;
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
