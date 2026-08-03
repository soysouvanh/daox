#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoles {
    pub assigned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_name: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRolesOrderBy {
    AssignedAtAsc,
    AssignedAtDesc,
    RoleNameAsc,
    RoleNameDesc,
    UserIdAsc,
    UserIdDesc,
}

impl UserRolesOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserRolesOrderBy::AssignedAtAsc => r#""assigned_at" ASC"#,
            UserRolesOrderBy::AssignedAtDesc => r#""assigned_at" DESC"#,
            UserRolesOrderBy::RoleNameAsc => r#""role_name" ASC"#,
            UserRolesOrderBy::RoleNameDesc => r#""role_name" DESC"#,
            UserRolesOrderBy::UserIdAsc => r#""user_id" ASC"#,
            UserRolesOrderBy::UserIdDesc => r#""user_id" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl UserRoles {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.role_name) {
            if v.len() < 1 {
                errors.push("role_name: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.role_name) {
            if v.len() > 50 {
                errors.push("role_name: exceeds max_length 50".into());
            }
        }
        if let Some(v) = Some(&self.user_id) {
            if (*v as i64) < 0 {
                errors.push("user_id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.user_id) {
            if (*v as i64) > 9223372036854775807 {
                errors.push("user_id: maximum value '9223372036854775807' exceeded".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM user_roles"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'user_roles'"#;
        let count: Option<(i64,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT "assigned_at", "role_name", "user_id" FROM user_roles ORDER BY "user_id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        order_by: &[UserRolesOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"SELECT "assigned_at", "role_name", "user_id" FROM user_roles"#,
        );
        qb.push(" ORDER BY ");
        for (i, o) in order_by.iter().enumerate() {
            if i > 0 {
                qb.push(", ");
            }
            qb.push(o.as_str());
        }
        qb.push(" LIMIT ");
        qb.push_bind(page_size as i64);
        qb.push(" OFFSET ");
        qb.push_bind(offset as i64);
        qb.build_query_as::<Self>().fetch_all(executor).await
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query =
            r#"INSERT INTO user_roles ("assigned_at", "role_name", "user_id") VALUES ($1, $2, $3)"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.assigned_at)
            .bind(&self.role_name)
            .bind(&self.user_id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    /// Inserts a batch of records using Postgres COPY (ultra-fast).
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn insert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let mut copy_in = executor.copy_in_raw(r#"COPY user_roles ("assigned_at", "role_name", "user_id") FROM STDIN WITH (FORMAT csv)"#).await?;
        for chunk in items.chunks(10000) {
            let mut payload = String::with_capacity(chunk.len() * 512);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                if let Some(v) = &item.assigned_at {
                    write!(&mut payload, "\"{}\"", v).unwrap();
                }
                payload.push(',');
                {
                    let v = &item.role_name;
                    payload.push('"');
                    for c in v.chars() {
                        if c == '"' {
                            payload.push_str("\"\"");
                        } else if c == '\\' {
                            payload.push_str("\\\\");
                        } else {
                            payload.push(c);
                        }
                    }
                    payload.push('"');
                }
                payload.push(',');
                {
                    let v = &item.user_id;
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push('\n');
            }
            copy_in.send(payload.as_bytes()).await?;
        }
        copy_in.finish().await?;
        Ok(items.len() as u64)
    }

    pub async fn list_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        user_id: i64,
        limit: i64,
    ) -> sqlx::Result<Vec<Self>> {
        let query = r#"SELECT "assigned_at", "role_name", "user_id" FROM user_roles WHERE "user_id" = $1 ORDER BY "user_id" ASC LIMIT $2"#;
        sqlx::query_as::<_, Self>(query)
            .bind(user_id)
            .bind(limit)
            .fetch_all(executor)
            .await
    }

    /// Streams rows from the table, filtered by user_id.
    /// **⚠️ Performance Warning:** Unbounded streaming is potentially dangerous.
    /// A `limit` parameter is now mandatory to prevent connection pool starvation.
    pub fn stream_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(
        executor: E,
        user_id: i64,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT "assigned_at", "role_name", "user_id" FROM user_roles WHERE "user_id" = $1 ORDER BY "user_id" ASC LIMIT $2"#;
        sqlx::query_as::<_, Self>(query)
            .bind(user_id)
            .bind(limit)
            .fetch(executor)
    }

    pub async fn exists_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        user_id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM user_roles WHERE "user_id" = $1 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(user_id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn delete_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        user_id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM user_roles WHERE "user_id" = $1"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(user_id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_by_user_id_and_role_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        user_id: i64,
        role_name: &String,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT "assigned_at", "role_name", "user_id" FROM user_roles WHERE "user_id" = $1 AND "role_name" = $2"#;
        sqlx::query_as::<_, Self>(query)
            .bind(user_id)
            .bind(role_name)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_user_id_and_role_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        user_id: i64,
        role_name: &String,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM user_roles WHERE "user_id" = $1 AND "role_name" = $2 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(user_id)
            .bind(role_name)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn delete_by_user_id_and_role_name<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        user_id: i64,
        role_name: &String,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM user_roles WHERE "user_id" = $1 AND "role_name" = $2"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(user_id)
            .bind(role_name)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }
}
