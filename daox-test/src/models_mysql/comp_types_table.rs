#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesTable {
    pub f_bool: Option<bool>,
    pub f_decimal: Option<f64>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_text: Option<String>,
    pub f_varchar: Option<String>,
    pub id: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesTableOrderBy {
    FBoolAsc,
    FBoolDesc,
    FDecimalAsc,
    FDecimalDesc,
    FDoubleAsc,
    FDoubleDesc,
    FFloatAsc,
    FFloatDesc,
    FIntAsc,
    FIntDesc,
    FTextAsc,
    FTextDesc,
    FVarcharAsc,
    FVarcharDesc,
    IdAsc,
    IdDesc,
}

impl CompTypesTableOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesTableOrderBy::FBoolAsc => r#"`f_bool` ASC"#,
            CompTypesTableOrderBy::FBoolDesc => r#"`f_bool` DESC"#,
            CompTypesTableOrderBy::FDecimalAsc => r#"`f_decimal` ASC"#,
            CompTypesTableOrderBy::FDecimalDesc => r#"`f_decimal` DESC"#,
            CompTypesTableOrderBy::FDoubleAsc => r#"`f_double` ASC"#,
            CompTypesTableOrderBy::FDoubleDesc => r#"`f_double` DESC"#,
            CompTypesTableOrderBy::FFloatAsc => r#"`f_float` ASC"#,
            CompTypesTableOrderBy::FFloatDesc => r#"`f_float` DESC"#,
            CompTypesTableOrderBy::FIntAsc => r#"`f_int` ASC"#,
            CompTypesTableOrderBy::FIntDesc => r#"`f_int` DESC"#,
            CompTypesTableOrderBy::FTextAsc => r#"`f_text` ASC"#,
            CompTypesTableOrderBy::FTextDesc => r#"`f_text` DESC"#,
            CompTypesTableOrderBy::FVarcharAsc => r#"`f_varchar` ASC"#,
            CompTypesTableOrderBy::FVarcharDesc => r#"`f_varchar` DESC"#,
            CompTypesTableOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesTableOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesTable {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
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
        if let Some(v) = self.f_text.as_ref() {
            if v.len() > 65535 {
                errors.push("f_text: exceeds max_length 65535".into());
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
        let query = r#"SELECT COUNT(*) FROM `comp_types_table`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// WARNING (MySQL): For InnoDB tables, this value is an estimate and can vary significantly from the actual count.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = ? AND table_schema = DATABASE()"#;
        let count: Option<(i64,)> = sqlx::query_as(query)
            .bind("comp_types_table")
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
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM `comp_types_table` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM `comp_types_table` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `comp_types_table` WHERE `id` = ? LIMIT 1"#;
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
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM `comp_types_table` WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
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
        let query = r#"INSERT INTO `comp_types_table` (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) VALUES (?, ?, ?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.f_bool)
            .bind(&self.f_decimal)
            .bind(&self.f_double)
            .bind(&self.f_float)
            .bind(&self.f_int)
            .bind(&self.f_text)
            .bind(&self.f_varchar)
            .execute(executor)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn insert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.insert(executor).await.map_err(|e| e.into())
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
        let chunk_size = 65535 / 7;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_table` (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.f_bool);
                b.push_bind(&item.f_decimal);
                b.push_bind(&item.f_double);
                b.push_bind(&item.f_float);
                b.push_bind(&item.f_int);
                b.push_bind(&item.f_text);
                b.push_bind(&item.f_varchar);
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
        let query = r#"INSERT INTO `comp_types_table` (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) VALUES (?, ?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `f_bool` = VALUES(`f_bool`), `f_decimal` = VALUES(`f_decimal`), `f_double` = VALUES(`f_double`), `f_float` = VALUES(`f_float`), `f_int` = VALUES(`f_int`), `f_text` = VALUES(`f_text`), `f_varchar` = VALUES(`f_varchar`)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.f_bool)
            .bind(&self.f_decimal)
            .bind(&self.f_double)
            .bind(&self.f_float)
            .bind(&self.f_int)
            .bind(&self.f_text)
            .bind(&self.f_varchar)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn upsert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.upsert(executor).await.map_err(|e| e.into())
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
        let chunk_size = 65535 / 7;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_table` (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.f_bool);
                b.push_bind(&item.f_decimal);
                b.push_bind(&item.f_double);
                b.push_bind(&item.f_float);
                b.push_bind(&item.f_int);
                b.push_bind(&item.f_text);
                b.push_bind(&item.f_varchar);
            });
            qb.push(r#" ON DUPLICATE KEY UPDATE `f_bool` = VALUES(`f_bool`), `f_decimal` = VALUES(`f_decimal`), `f_double` = VALUES(`f_double`), `f_float` = VALUES(`f_float`), `f_int` = VALUES(`f_int`), `f_text` = VALUES(`f_text`), `f_varchar` = VALUES(`f_varchar`)"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::MySql>(query_str);
        query = query.bind(&self.f_bool);
        query = query.bind(&self.f_decimal);
        query = query.bind(&self.f_double);
        query = query.bind(&self.f_float);
        query = query.bind(&self.f_int);
        query = query.bind(&self.f_text);
        query = query.bind(&self.f_varchar);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_validated_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.update_by_id(executor).await.map_err(|e| e.into())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `comp_types_table` WHERE `id` = ?"#;
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
                sqlx::QueryBuilder::new(r#"DELETE FROM `comp_types_table` WHERE `id` IN "#);
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
        patch: &CompTypesTablePatch,
    ) -> sqlx::Result<u64> {
        let mut qb: sqlx::QueryBuilder<sqlx::MySql> =
            sqlx::QueryBuilder::new("UPDATE `comp_types_table` SET ");
        let mut first = true;
        if let Some(val) = &patch.f_bool {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_bool` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_decimal {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_decimal` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_double {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_double` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_float {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_float` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_int {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_int` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_text {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_text` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if let Some(val) = &patch.f_varchar {
            if !first {
                qb.push(", ");
            }
            qb.push("`f_varchar` = ");
            qb.push_bind(val.clone());
            first = false;
        }
        if first {
            return Ok(0);
        }
        qb.push(" WHERE `id` = ");
        qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesTablePatch {
    pub f_bool: Option<Option<bool>>,
    pub f_decimal: Option<Option<f64>>,
    pub f_double: Option<Option<f64>>,
    pub f_float: Option<Option<f64>>,
    pub f_int: Option<Option<i32>>,
    pub f_text: Option<Option<String>>,
    pub f_varchar: Option<Option<String>>,
}
