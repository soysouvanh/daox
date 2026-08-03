#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<i32>,
    pub last_name: Option<String>,
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
        if let Some(v) = self.id.as_ref() {
            if (*v as i64) < 0 {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = self.id.as_ref() {
            if (*v as i64) > 2147483647 {
                errors.push("id: maximum value '2147483647' exceeded".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM active_users"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the view.
    /// (SQLite views do not support O(1) approximation, so this falls back to COUNT(*)).
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        Self::count(executor).await
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT `email`, `first_name`, `id`, `last_name` FROM active_users ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
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
        let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
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
