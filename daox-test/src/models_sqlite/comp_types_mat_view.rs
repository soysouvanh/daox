#[cfg(not(feature = "validation"))]
compile_error!(
    "Daox: the 'validation' feature is disabled. \
     Format regex checks in validate() will be skipped. \
     Enable with: features = [\"validation\"]"
);
#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMatView {
    pub f_blob: Option<Vec<u8>>,
    pub f_bool: Option<bool>,
    pub f_date: Option<String>,
    pub f_datetime: Option<String>,
    pub f_decimal: Option<f64>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_json: Option<String>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<String>,
    pub f_varchar: Option<String>,
    pub id: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesMatViewOrderBy {
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

impl CompTypesMatViewOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesMatViewOrderBy::FBlobAsc => r#"`f_blob` ASC"#,
            CompTypesMatViewOrderBy::FBlobDesc => r#"`f_blob` DESC"#,
            CompTypesMatViewOrderBy::FBoolAsc => r#"`f_bool` ASC"#,
            CompTypesMatViewOrderBy::FBoolDesc => r#"`f_bool` DESC"#,
            CompTypesMatViewOrderBy::FDateAsc => r#"`f_date` ASC"#,
            CompTypesMatViewOrderBy::FDateDesc => r#"`f_date` DESC"#,
            CompTypesMatViewOrderBy::FDatetimeAsc => r#"`f_datetime` ASC"#,
            CompTypesMatViewOrderBy::FDatetimeDesc => r#"`f_datetime` DESC"#,
            CompTypesMatViewOrderBy::FDecimalAsc => r#"`f_decimal` ASC"#,
            CompTypesMatViewOrderBy::FDecimalDesc => r#"`f_decimal` DESC"#,
            CompTypesMatViewOrderBy::FDoubleAsc => r#"`f_double` ASC"#,
            CompTypesMatViewOrderBy::FDoubleDesc => r#"`f_double` DESC"#,
            CompTypesMatViewOrderBy::FFloatAsc => r#"`f_float` ASC"#,
            CompTypesMatViewOrderBy::FFloatDesc => r#"`f_float` DESC"#,
            CompTypesMatViewOrderBy::FIntAsc => r#"`f_int` ASC"#,
            CompTypesMatViewOrderBy::FIntDesc => r#"`f_int` DESC"#,
            CompTypesMatViewOrderBy::FJsonAsc => r#"`f_json` ASC"#,
            CompTypesMatViewOrderBy::FJsonDesc => r#"`f_json` DESC"#,
            CompTypesMatViewOrderBy::FTextAsc => r#"`f_text` ASC"#,
            CompTypesMatViewOrderBy::FTextDesc => r#"`f_text` DESC"#,
            CompTypesMatViewOrderBy::FTimestampAsc => r#"`f_timestamp` ASC"#,
            CompTypesMatViewOrderBy::FTimestampDesc => r#"`f_timestamp` DESC"#,
            CompTypesMatViewOrderBy::FVarcharAsc => r#"`f_varchar` ASC"#,
            CompTypesMatViewOrderBy::FVarcharDesc => r#"`f_varchar` DESC"#,
            CompTypesMatViewOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesMatViewOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesMatView {
    #[allow(unused_comparisons, unused_mut)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        #[cfg(not(feature = "validation"))]
        {
            // Formats validation is disabled
        }
        let mut errors = Vec::new();
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_date.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_date: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_date: configured regex is invalid".into());
                }
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_datetime.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_datetime: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_datetime: configured regex is invalid".into());
                }
            }
        }
        if let Some(v) = self.f_decimal.as_ref() {
            if !v.is_finite() {
                errors.push("f_decimal: value must be finite (NaN/Infinity rejected)".into());
            }
        }
        if let Some(v) = self.f_double.as_ref() {
            if !v.is_finite() {
                errors.push("f_double: value must be finite (NaN/Infinity rejected)".into());
            }
        }
        if let Some(v) = self.f_float.as_ref() {
            if !v.is_finite() {
                errors.push("f_float: value must be finite (NaN/Infinity rejected)".into());
            }
        }
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
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_json.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_json: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_json: configured regex is invalid".into());
                }
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_text.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_text: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_text: configured regex is invalid".into());
                }
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_timestamp.as_ref() {
            static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").ok());
            match re {
                Some(re) => {
                    if !re.is_match(v) {
                        errors.push("f_timestamp: format constraint not met".into());
                    }
                }
                None => {
                    errors.push("f_timestamp: configured regex is invalid".into());
                }
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
        if let Some(v) = self.id.as_ref() {
            if (*v as i128) < (0 as i128) {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = self.id.as_ref() {
            if (*v as i128) > (2147483647 as i128) {
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
    #[deprecated(
        since = "0.2.0",
        note = "Use `approximate_count` instead to prevent full table scans."
    )]
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM `comp_types_mat_view`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an estimated count upper bound using `MAX(rowid)` (O(1)).
    /// WARNING (SQLite): This overestimates the count if rows have been deleted.
    pub async fn estimated_count_upper_bound<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT MAX(rowid) FROM `comp_types_mat_view`"#;
        let count: Option<(Option<i64>,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count
            .and_then(|(c,)| c)
            .map(|c| c.max(0) as u64)
            .unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS. Timeouts are managed by the underlying sqlx `AnyPoolOptions` settings.
    #[deprecated(
        since = "0.2.0",
        note = "Use cursor-based pagination instead to prevent pool starvation."
    )]
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM `comp_types_mat_view` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn insert_unchecked<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `comp_types_mat_view` (`f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.f_blob)
            .bind(&self.f_bool)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_decimal)
            .bind(&self.f_double)
            .bind(&self.f_float)
            .bind(&self.f_int)
            .bind(&self.f_json)
            .bind(&self.f_text)
            .bind(&self.f_timestamp)
            .bind(&self.f_varchar)
            .bind(&self.id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        if let Err(e) = self.validate() {
            return Err(sqlx::Error::Protocol(e.join(", ").into()));
        }
        self.insert_unchecked(executor).await
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
        for (idx, item) in items.iter().enumerate() {
            if let Err(e) = item.validate() {
                return Err(sqlx::Error::Protocol(
                    format!(
                        "insert_batch: item {} failed validation: {}",
                        idx,
                        e.join(", ")
                    )
                    .into(),
                ));
            }
        }
        let chunk_size = 32766 / 13;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_mat_view` (`f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_bool);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_decimal);
                b.push_bind(&item.f_double);
                b.push_bind(&item.f_float);
                b.push_bind(&item.f_int);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_text);
                b.push_bind(&item.f_timestamp);
                b.push_bind(&item.f_varchar);
                b.push_bind(&item.id);
            });
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }
}
