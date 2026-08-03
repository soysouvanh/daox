#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMatView {
    pub f_blob: Option<Vec<u8>>,
    pub f_bool: Option<bool>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_decimal: Option<String>,
    pub f_double: Option<f64>,
    pub f_float: Option<f32>,
    pub f_int: Option<i32>,
    pub f_json: Option<String>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub f_varchar: Option<String>,
    pub id: i64,
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
            CompTypesMatViewOrderBy::FBlobAsc => r#""f_blob" ASC"#,
            CompTypesMatViewOrderBy::FBlobDesc => r#""f_blob" DESC"#,
            CompTypesMatViewOrderBy::FBoolAsc => r#""f_bool" ASC"#,
            CompTypesMatViewOrderBy::FBoolDesc => r#""f_bool" DESC"#,
            CompTypesMatViewOrderBy::FDateAsc => r#""f_date" ASC"#,
            CompTypesMatViewOrderBy::FDateDesc => r#""f_date" DESC"#,
            CompTypesMatViewOrderBy::FDatetimeAsc => r#""f_datetime" ASC"#,
            CompTypesMatViewOrderBy::FDatetimeDesc => r#""f_datetime" DESC"#,
            CompTypesMatViewOrderBy::FDecimalAsc => r#""f_decimal" ASC"#,
            CompTypesMatViewOrderBy::FDecimalDesc => r#""f_decimal" DESC"#,
            CompTypesMatViewOrderBy::FDoubleAsc => r#""f_double" ASC"#,
            CompTypesMatViewOrderBy::FDoubleDesc => r#""f_double" DESC"#,
            CompTypesMatViewOrderBy::FFloatAsc => r#""f_float" ASC"#,
            CompTypesMatViewOrderBy::FFloatDesc => r#""f_float" DESC"#,
            CompTypesMatViewOrderBy::FIntAsc => r#""f_int" ASC"#,
            CompTypesMatViewOrderBy::FIntDesc => r#""f_int" DESC"#,
            CompTypesMatViewOrderBy::FJsonAsc => r#""f_json" ASC"#,
            CompTypesMatViewOrderBy::FJsonDesc => r#""f_json" DESC"#,
            CompTypesMatViewOrderBy::FTextAsc => r#""f_text" ASC"#,
            CompTypesMatViewOrderBy::FTextDesc => r#""f_text" DESC"#,
            CompTypesMatViewOrderBy::FTimestampAsc => r#""f_timestamp" ASC"#,
            CompTypesMatViewOrderBy::FTimestampDesc => r#""f_timestamp" DESC"#,
            CompTypesMatViewOrderBy::FVarcharAsc => r#""f_varchar" ASC"#,
            CompTypesMatViewOrderBy::FVarcharDesc => r#""f_varchar" DESC"#,
            CompTypesMatViewOrderBy::IdAsc => r#""id" ASC"#,
            CompTypesMatViewOrderBy::IdDesc => r#""id" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesMatView {
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM comp_types_mat_view"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query =
            r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'comp_types_mat_view'"#;
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
        let query = r#"SELECT "f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id" FROM comp_types_mat_view ORDER BY "id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        order_by: &[CompTypesMatViewOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"SELECT "f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id" FROM comp_types_mat_view"#,
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
        let query = r#"INSERT INTO comp_types_mat_view ("f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
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

    /// Inserts a batch of records using Postgres COPY (ultra-fast).
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn insert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let mut copy_in = executor.copy_in_raw(r#"COPY comp_types_mat_view ("f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id") FROM STDIN WITH (FORMAT csv)"#).await?;
        for chunk in items.chunks(10000) {
            let mut payload = String::with_capacity(chunk.len() * 512);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                if let Some(v) = &item.f_blob {
                    payload.push_str("\"\\x");
                    for b in v {
                        write!(&mut payload, "{:02x}", b).unwrap();
                    }
                    payload.push('"');
                }
                payload.push(',');
                if let Some(v) = &item.f_bool {
                    if *v {
                        payload.push_str("true");
                    } else {
                        payload.push_str("false");
                    }
                }
                payload.push(',');
                if let Some(v) = &item.f_date {
                    write!(&mut payload, "\"{}\"", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_datetime {
                    write!(&mut payload, "\"{}\"", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_decimal {
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
                if let Some(v) = &item.f_double {
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_float {
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_int {
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_json {
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
                if let Some(v) = &item.f_text {
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
                if let Some(v) = &item.f_timestamp {
                    write!(&mut payload, "\"{}\"", v).unwrap();
                }
                payload.push(',');
                if let Some(v) = &item.f_varchar {
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
                    let v = &item.id;
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push('\n');
            }
            copy_in.send(payload.as_bytes()).await?;
        }
        copy_in.finish().await?;
        Ok(items.len() as u64)
    }
}
