#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
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
            ActiveUsersOrderBy::EmailAsc => r#""email" ASC"#,
            ActiveUsersOrderBy::EmailDesc => r#""email" DESC"#,
            ActiveUsersOrderBy::FirstNameAsc => r#""first_name" ASC"#,
            ActiveUsersOrderBy::FirstNameDesc => r#""first_name" DESC"#,
            ActiveUsersOrderBy::IdAsc => r#""id" ASC"#,
            ActiveUsersOrderBy::IdDesc => r#""id" DESC"#,
            ActiveUsersOrderBy::LastNameAsc => r#""last_name" ASC"#,
            ActiveUsersOrderBy::LastNameDesc => r#""last_name" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl ActiveUsers {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = self.email.as_ref() {
            if v.len() > 255 {
                errors.push("email: exceeds max_length 255".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.email.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^([a-zA-Z0-9_\\-\\.]+)@([a-zA-Z0-9_\\-\\.]+)\\.([a-zA-Z]{2,5})$")
                    .ok()
            });
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("email: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("email: configured regex is invalid".into());
                }
            }
        }
        if let Some(v) = self.first_name.as_ref() {
            if v.len() > 100 {
                errors.push("first_name: exceeds max_length 100".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.first_name.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("first_name: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("first_name: configured regex is invalid".into());
                }
            }
        }
        if let Some(v) = self.id.as_ref() {
            if (*v as i128) < (0 as i128) {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = self.id.as_ref() {
            if (*v as i128) > (9223372036854775807 as i128) {
                errors.push("id: maximum value '9223372036854775807' exceeded".into());
            }
        }
        if let Some(v) = self.last_name.as_ref() {
            if v.len() > 100 {
                errors.push("last_name: exceeds max_length 100".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.last_name.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("last_name: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("last_name: configured regex is invalid".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM "active_users""#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = $1"#;
        let count: Option<(i64,)> = sqlx::query_as(query)
            .bind("active_users")
            .fetch_optional(executor)
            .await?;
        Ok(count.map(|(c,)| c.max(0) as u64).unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT "email", "first_name", "id", "last_name" FROM "active_users" ORDER BY "id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }
}
