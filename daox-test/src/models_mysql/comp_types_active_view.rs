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
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = self.f_int.as_ref() {
            if (*v as i64) < 0 {
                errors.push("f_int: minimum value '0' not met".into());
            }
        }
        if let Some(v) = self.f_int.as_ref() {
            if (*v as i64) > 2147483647 {
                errors.push("f_int: maximum value '2147483647' exceeded".into());
            }
        }
        if let Some(v) = self.f_varchar.as_ref() {
            if v.len() > 255 {
                errors.push("f_varchar: exceeds max_length 255".into());
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM comp_types_active_view"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'comp_types_active_view' AND table_schema = DATABASE()"#;
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
        let query = r#"SELECT `f_date`, `f_int`, `f_varchar`, `id` FROM comp_types_active_view ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        order_by: &[CompTypesActiveViewOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"SELECT `f_date`, `f_int`, `f_varchar`, `id` FROM comp_types_active_view"#,
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
