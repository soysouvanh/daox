#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesView {
    pub f_blob: Option<Vec<u8>>,
    pub f_bool: Option<bool>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_decimal: Option<f64>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_json: Option<serde_json::Value>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub f_varchar: Option<String>,
    pub id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesViewOrderBy {
    FBlobAsc,
    FBlobDesc,
    FBoolAsc,
    FBoolDesc,
    FDateAsc,
    FDateDesc,
    FDatetimeAsc,
    FDatetimeDesc,
    FDecimalAsc,
    FDecimalDesc,
    FDoubleAsc,
    FDoubleDesc,
    FFloatAsc,
    FFloatDesc,
    FIntAsc,
    FIntDesc,
    FJsonAsc,
    FJsonDesc,
    FTextAsc,
    FTextDesc,
    FTimestampAsc,
    FTimestampDesc,
    FVarcharAsc,
    FVarcharDesc,
    IdAsc,
    IdDesc,
}

impl CompTypesViewOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesViewOrderBy::FBlobAsc => r#"`f_blob` ASC"#,
            CompTypesViewOrderBy::FBlobDesc => r#"`f_blob` DESC"#,
            CompTypesViewOrderBy::FBoolAsc => r#"`f_bool` ASC"#,
            CompTypesViewOrderBy::FBoolDesc => r#"`f_bool` DESC"#,
            CompTypesViewOrderBy::FDateAsc => r#"`f_date` ASC"#,
            CompTypesViewOrderBy::FDateDesc => r#"`f_date` DESC"#,
            CompTypesViewOrderBy::FDatetimeAsc => r#"`f_datetime` ASC"#,
            CompTypesViewOrderBy::FDatetimeDesc => r#"`f_datetime` DESC"#,
            CompTypesViewOrderBy::FDecimalAsc => r#"`f_decimal` ASC"#,
            CompTypesViewOrderBy::FDecimalDesc => r#"`f_decimal` DESC"#,
            CompTypesViewOrderBy::FDoubleAsc => r#"`f_double` ASC"#,
            CompTypesViewOrderBy::FDoubleDesc => r#"`f_double` DESC"#,
            CompTypesViewOrderBy::FFloatAsc => r#"`f_float` ASC"#,
            CompTypesViewOrderBy::FFloatDesc => r#"`f_float` DESC"#,
            CompTypesViewOrderBy::FIntAsc => r#"`f_int` ASC"#,
            CompTypesViewOrderBy::FIntDesc => r#"`f_int` DESC"#,
            CompTypesViewOrderBy::FJsonAsc => r#"`f_json` ASC"#,
            CompTypesViewOrderBy::FJsonDesc => r#"`f_json` DESC"#,
            CompTypesViewOrderBy::FTextAsc => r#"`f_text` ASC"#,
            CompTypesViewOrderBy::FTextDesc => r#"`f_text` DESC"#,
            CompTypesViewOrderBy::FTimestampAsc => r#"`f_timestamp` ASC"#,
            CompTypesViewOrderBy::FTimestampDesc => r#"`f_timestamp` DESC"#,
            CompTypesViewOrderBy::FVarcharAsc => r#"`f_varchar` ASC"#,
            CompTypesViewOrderBy::FVarcharDesc => r#"`f_varchar` DESC"#,
            CompTypesViewOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesViewOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesView {
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
        if let Some(v) = self.f_text.as_ref() {
            if v.len() > 65535 {
                errors.push("f_text: exceeds max_length 65535".into());
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
        let query = r#"SELECT COUNT(*) FROM comp_types_view"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'comp_types_view' AND table_schema = DATABASE()"#;
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
        let query = r#"SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_view ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        order_by: &[CompTypesViewOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
            r#"SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_view"#,
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
