#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Currencies {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrenciesOrderBy {
    CodeAsc,
    CodeDesc,
    NameAsc,
    NameDesc,
}

impl CurrenciesOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CurrenciesOrderBy::CodeAsc => r#""code" ASC"#,
            CurrenciesOrderBy::CodeDesc => r#""code" DESC"#,
            CurrenciesOrderBy::NameAsc => r#""name" ASC"#,
            CurrenciesOrderBy::NameDesc => r#""name" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl Currencies {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.code) {
            if v.len() < 1 {
                errors.push("code: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.code) {
            if v.len() > 3 {
                errors.push("code: exceeds max_length 3".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.code) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("code: format constraint not met".into());
            }
        }
        if let Some(v) = Some(&self.name) {
            if v.len() < 1 {
                errors.push("name: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.name) {
            if v.len() > 50 {
                errors.push("name: exceeds max_length 50".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = Some(&self.name) {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("name: format constraint not met".into());
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
        let query = r#"SELECT COUNT(*) FROM "currencies""#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'currencies'"#;
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
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT "code", "name" FROM "currencies" ORDER BY "code" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT "code", "name" FROM "currencies" WHERE "code" = $1"#;
        sqlx::query_as::<_, Self>(query)
            .bind(code)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM "currencies" WHERE "code" = $1 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(code)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        last_id: &str,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT "code", "name" FROM "currencies" WHERE "code" > $1 ORDER BY "code" ASC LIMIT $2"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_id)
            .bind(limit as i64)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO "currencies" ("code", "name") VALUES ($1, $2)"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.code)
            .bind(&self.name)
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
        let mut copy_in = executor
            .copy_in_raw(r#"COPY "currencies" ("code", "name") FROM STDIN WITH (FORMAT csv)"#)
            .await?;
        for chunk in items.chunks(1000) {
            let est: usize = chunk
                .iter()
                .map(|item| {
                    let mut s = 0usize;
                    let _ = item;
                    s += item.code.len() + 2;
                    s += item.name.len() + 2;
                    s
                })
                .sum();
            let mut payload = String::with_capacity(est);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                {
                    let v = &item.code;
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
                    let v = &item.name;
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
                payload.push('\n');
            }
            copy_in.send(payload.as_bytes()).await?;
        }
        copy_in.finish().await?;
        Ok(items.len() as u64)
    }

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO "currencies" ("code", "name") VALUES ($1, $2) ON CONFLICT ("code") DO UPDATE SET "name" = EXCLUDED."name""#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.code)
            .bind(&self.name)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    /// Upserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn upsert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 65535 / 2;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
                sqlx::QueryBuilder::new(r#"INSERT INTO "currencies" ("code", "name") "#);
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.code);
                b.push_bind(&item.name);
            });
            qb.push(r#" ON CONFLICT ("code") DO UPDATE SET "name" = EXCLUDED."name""#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE "currencies" SET "name" = $1 WHERE "code" = $2"#;
        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.name);
        query = query.bind(&self.code);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        code: &str,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM "currencies" WHERE "code" = $1"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(code)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_code<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        ids: &[&str],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(65535);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
                sqlx::QueryBuilder::new(r#"DELETE FROM "currencies" WHERE "code" IN "#);
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
    pub async fn update_partial_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        code: &str,
        patch: &CurrenciesPatch,
    ) -> sqlx::Result<u64> {
        let mut mask = 0u64;
        let mut has = false;
        if patch.name.is_some() {
            mask |= 1 << 0;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        let query_str = match mask {
            1 => r#"UPDATE "currencies" SET "name" = $1 WHERE "code" = $2"#,
            _ => unreachable!(),
        };

        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        if let Some(val) = &patch.name {
            query = query.bind(val);
        }
        query = query.bind(code);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CurrenciesPatch {
    pub name: Option<String>,
}
