#[cfg(not(feature = "validation"))]
compile_error!(
    "Daox: the 'validation' feature is disabled. \
     Format regex checks in validate() will be skipped. \
     Enable with: features = [\"validation\"]"
);
#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesActiveView {
    pub f_date: Option<chrono::NaiveDate>,
    pub f_int: Option<i32>,
    pub f_varchar: Option<String>,
    pub id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesActiveViewOrderBy {
    FDateAsc,
    FDateDesc,
    FIntAsc,
    FIntDesc,
    FVarcharAsc,
    FVarcharDesc,
    IdAsc,
    IdDesc,
}

impl CompTypesActiveViewOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesActiveViewOrderBy::FDateAsc => r#"`f_date` ASC"#,
            CompTypesActiveViewOrderBy::FDateDesc => r#"`f_date` DESC"#,
            CompTypesActiveViewOrderBy::FIntAsc => r#"`f_int` ASC"#,
            CompTypesActiveViewOrderBy::FIntDesc => r#"`f_int` DESC"#,
            CompTypesActiveViewOrderBy::FVarcharAsc => r#"`f_varchar` ASC"#,
            CompTypesActiveViewOrderBy::FVarcharDesc => r#"`f_varchar` DESC"#,
            CompTypesActiveViewOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesActiveViewOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesActiveView {
    #[allow(unused_comparisons, unused_mut)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        #[cfg(not(feature = "validation"))]
        {
            // Formats validation is disabled
        }
        let mut errors = Vec::new();
        if let Some(v) = self.f_int.as_ref() {
            if (*v as i128) < (0 as i128) {
                errors.push("f_int: minimum value '0' not met".into());
            }
        }
        if let Some(v) = self.f_int.as_ref() {
            if (*v as i128) > (2147483647 as i128) {
                errors.push("f_int: maximum value '2147483647' exceeded".into());
            }
        }
        if let Some(v) = self.f_varchar.as_ref() {
            if v.len() > 255 {
                errors.push("f_varchar: exceeds max_length 255".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_varchar.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_varchar: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_varchar: configured regex is invalid".into());
                }
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i128) < (0 as i128) {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i128) > (9223372036854775807 as i128) {
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
        let query = r#"SELECT COUNT(*) FROM `comp_types_active_view`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// WARNING (MySQL): For InnoDB tables, this value is an estimate and can vary significantly from the actual count.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = ? AND table_schema = DATABASE()"#;
        let count: Option<(u64,)> = sqlx::query_as(query)
            .bind("comp_types_active_view")
            .fetch_optional(executor)
            .await?;
        Ok(count.map(|(c,)| c).unwrap_or(0))
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
        let query = r#"SELECT `f_date`, `f_int`, `f_varchar`, `id` FROM `comp_types_active_view` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }
}
