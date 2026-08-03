#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub email: String,
    pub first_name: Option<String>,
    pub id: i64,
    pub last_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveUsersOrderBy {
    EmailAsc,
    EmailDesc,
    FirstNameAsc,
    FirstNameDesc,
    IdAsc,
    IdDesc,
    LastNameAsc,
    LastNameDesc,
}

impl ActiveUsersOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActiveUsersOrderBy::EmailAsc => r#"`email` ASC"#,
            ActiveUsersOrderBy::EmailDesc => r#"`email` DESC"#,
            ActiveUsersOrderBy::FirstNameAsc => r#"`first_name` ASC"#,
            ActiveUsersOrderBy::FirstNameDesc => r#"`first_name` DESC"#,
            ActiveUsersOrderBy::IdAsc => r#"`id` ASC"#,
            ActiveUsersOrderBy::IdDesc => r#"`id` DESC"#,
            ActiveUsersOrderBy::LastNameAsc => r#"`last_name` ASC"#,
            ActiveUsersOrderBy::LastNameDesc => r#"`last_name` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl ActiveUsers {
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
        let query = r#"SELECT COUNT(*) FROM active_users"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'active_users' AND table_schema = DATABASE()"#;
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
        let query = r#"SELECT `email`, `first_name`, `id`, `last_name` FROM active_users ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        order_by: &[ActiveUsersOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"SELECT `email`, `first_name`, `id`, `last_name` FROM active_users"#,
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
}
