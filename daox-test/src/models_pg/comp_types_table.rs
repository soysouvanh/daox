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
            CompTypesTableOrderBy::FBoolAsc => r#""f_bool" ASC"#,
            CompTypesTableOrderBy::FBoolDesc => r#""f_bool" DESC"#,
            CompTypesTableOrderBy::FDecimalAsc => r#""f_decimal" ASC"#,
            CompTypesTableOrderBy::FDecimalDesc => r#""f_decimal" DESC"#,
            CompTypesTableOrderBy::FDoubleAsc => r#""f_double" ASC"#,
            CompTypesTableOrderBy::FDoubleDesc => r#""f_double" DESC"#,
            CompTypesTableOrderBy::FFloatAsc => r#""f_float" ASC"#,
            CompTypesTableOrderBy::FFloatDesc => r#""f_float" DESC"#,
            CompTypesTableOrderBy::FIntAsc => r#""f_int" ASC"#,
            CompTypesTableOrderBy::FIntDesc => r#""f_int" DESC"#,
            CompTypesTableOrderBy::FTextAsc => r#""f_text" ASC"#,
            CompTypesTableOrderBy::FTextDesc => r#""f_text" DESC"#,
            CompTypesTableOrderBy::FVarcharAsc => r#""f_varchar" ASC"#,
            CompTypesTableOrderBy::FVarcharDesc => r#""f_varchar" DESC"#,
            CompTypesTableOrderBy::IdAsc => r#""id" ASC"#,
            CompTypesTableOrderBy::IdDesc => r#""id" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesTable {
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
    #[deprecated(
        since = "0.2.0",
        note = "Use `approximate_count` instead to prevent full table scans."
    )]
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM "comp_types_table""#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'comp_types_table'"#;
        let count: Option<(i64,)> = sqlx::query_as(query).fetch_optional(executor).await?;
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
        let query = r#"SELECT "f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar", "id" FROM "comp_types_table" ORDER BY "id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT "f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar", "id" FROM "comp_types_table" WHERE "id" = $1"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM "comp_types_table" WHERE "id" = $1 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        last_id: i64,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT "f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar", "id" FROM "comp_types_table" WHERE "id" > $1 ORDER BY "id" ASC LIMIT $2"#;
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
        let query = r#"INSERT INTO "comp_types_table" ("f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING "id"::bigint"#;
        let (id,): (i64,) = sqlx::query_as(query)
            .bind(&self.f_bool)
            .bind(&self.f_decimal)
            .bind(&self.f_double)
            .bind(&self.f_float)
            .bind(&self.f_int)
            .bind(&self.f_text)
            .bind(&self.f_varchar)
            .fetch_one(executor)
            .await?;
        Ok(id as u64)
    }

    pub async fn insert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.insert(executor).await.map_err(|e| e.into())
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
        let mut copy_in = executor.copy_in_raw(r#"COPY "comp_types_table" ("f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar") FROM STDIN WITH (FORMAT csv)"#).await?;
        for chunk in items.chunks(1000) {
            let est: usize = chunk
                .iter()
                .map(|item| {
                    let mut s = 0usize;
                    let _ = item;
                    s += 32;
                    s += 32;
                    s += 32;
                    s += 32;
                    s += 32;
                    s += item.f_text.as_ref().map_or(1, |v| v.len() + 2);
                    s += item.f_varchar.as_ref().map_or(1, |v| v.len() + 2);
                    s
                })
                .sum();
            let mut payload = String::with_capacity(est);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                if let Some(v) = &item.f_bool {
                    if *v {
                        payload.push_str("true");
                    } else {
                        payload.push_str("false");
                    }
                }
                payload.push(',');
                if let Some(v) = &item.f_decimal {
                    write!(&mut payload, "{}", v).unwrap();
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
                if let Some(v) = &item.f_text {
                    payload.push('"');
                    for c in v.chars() {
                        if c == '"' {
                            payload.push_str("\"\"");
                        } else {
                            payload.push(c);
                        }
                    }
                    payload.push('"');
                }
                payload.push(',');
                if let Some(v) = &item.f_varchar {
                    payload.push('"');
                    for c in v.chars() {
                        if c == '"' {
                            payload.push_str("\"\"");
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
        let query = r#"INSERT INTO "comp_types_table" ("f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar") VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT ("id") DO UPDATE SET "f_bool" = EXCLUDED."f_bool", "f_decimal" = EXCLUDED."f_decimal", "f_double" = EXCLUDED."f_double", "f_float" = EXCLUDED."f_float", "f_int" = EXCLUDED."f_int", "f_text" = EXCLUDED."f_text", "f_varchar" = EXCLUDED."f_varchar""#;
        let result = sqlx::query::<sqlx::Postgres>(query)
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

    pub async fn upsert_validated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.upsert(executor).await.map_err(|e| e.into())
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
        let chunk_size = 65535 / 7;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
                r#"INSERT INTO "comp_types_table" ("f_bool", "f_decimal", "f_double", "f_float", "f_int", "f_text", "f_varchar") "#,
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
            qb.push(r#" ON CONFLICT ("id") DO UPDATE SET "f_bool" = EXCLUDED."f_bool", "f_decimal" = EXCLUDED."f_decimal", "f_double" = EXCLUDED."f_double", "f_float" = EXCLUDED."f_float", "f_int" = EXCLUDED."f_int", "f_text" = EXCLUDED."f_text", "f_varchar" = EXCLUDED."f_varchar""#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_int" = $5, "f_text" = $6, "f_varchar" = $7 WHERE "id" = $8"#;
        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
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

    pub async fn update_validated_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
        self.validate().map_err(|e| e.join(", "))?;
        self.update_by_id(executor).await.map_err(|e| e.into())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM "comp_types_table" WHERE "id" = $1"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        ids: &[i64],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(65535);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
                sqlx::QueryBuilder::new(r#"DELETE FROM "comp_types_table" WHERE "id" IN "#);
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
    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: i64,
        patch: &CompTypesTablePatch,
    ) -> sqlx::Result<u64> {
        let mut mask = 0u64;
        let mut has = false;
        if patch.f_bool.is_some() {
            mask |= 1 << 0;
            has = true;
        }
        if patch.f_decimal.is_some() {
            mask |= 1 << 1;
            has = true;
        }
        if patch.f_double.is_some() {
            mask |= 1 << 2;
            has = true;
        }
        if patch.f_float.is_some() {
            mask |= 1 << 3;
            has = true;
        }
        if patch.f_int.is_some() {
            mask |= 1 << 4;
            has = true;
        }
        if patch.f_text.is_some() {
            mask |= 1 << 5;
            has = true;
        }
        if patch.f_varchar.is_some() {
            mask |= 1 << 6;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        let query_str = match mask {
            1 => r#"UPDATE "comp_types_table" SET "f_bool" = $1 WHERE "id" = $2"#,
            2 => r#"UPDATE "comp_types_table" SET "f_decimal" = $1 WHERE "id" = $2"#,
            3 => r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2 WHERE "id" = $3"#,
            4 => r#"UPDATE "comp_types_table" SET "f_double" = $1 WHERE "id" = $2"#,
            5 => r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2 WHERE "id" = $3"#,
            6 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2 WHERE "id" = $3"#
            }
            7 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3 WHERE "id" = $4"#
            }
            8 => r#"UPDATE "comp_types_table" SET "f_float" = $1 WHERE "id" = $2"#,
            9 => r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2 WHERE "id" = $3"#,
            10 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2 WHERE "id" = $3"#
            }
            11 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3 WHERE "id" = $4"#
            }
            12 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2 WHERE "id" = $3"#
            }
            13 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3 WHERE "id" = $4"#
            }
            14 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3 WHERE "id" = $4"#
            }
            15 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4 WHERE "id" = $5"#
            }
            16 => r#"UPDATE "comp_types_table" SET "f_int" = $1 WHERE "id" = $2"#,
            17 => r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_int" = $2 WHERE "id" = $3"#,
            18 => r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_int" = $2 WHERE "id" = $3"#,
            19 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            20 => r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_int" = $2 WHERE "id" = $3"#,
            21 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            22 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            23 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_int" = $4 WHERE "id" = $5"#
            }
            24 => r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_int" = $2 WHERE "id" = $3"#,
            25 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            26 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            27 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_int" = $4 WHERE "id" = $5"#
            }
            28 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_int" = $3 WHERE "id" = $4"#
            }
            29 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4 WHERE "id" = $5"#
            }
            30 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4 WHERE "id" = $5"#
            }
            31 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_int" = $5 WHERE "id" = $6"#
            }
            32 => r#"UPDATE "comp_types_table" SET "f_text" = $1 WHERE "id" = $2"#,
            33 => r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_text" = $2 WHERE "id" = $3"#,
            34 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_text" = $2 WHERE "id" = $3"#
            }
            35 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            36 => r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_text" = $2 WHERE "id" = $3"#,
            37 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            38 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            39 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            40 => r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_text" = $2 WHERE "id" = $3"#,
            41 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            42 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            43 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            44 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            45 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            46 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            47 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_text" = $5 WHERE "id" = $6"#
            }
            48 => r#"UPDATE "comp_types_table" SET "f_int" = $1, "f_text" = $2 WHERE "id" = $3"#,
            49 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_int" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            50 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_int" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            51 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            52 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_int" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            53 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            54 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            55 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_int" = $4, "f_text" = $5 WHERE "id" = $6"#
            }
            56 => {
                r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_int" = $2, "f_text" = $3 WHERE "id" = $4"#
            }
            57 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            58 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            59 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5 WHERE "id" = $6"#
            }
            60 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4 WHERE "id" = $5"#
            }
            61 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5 WHERE "id" = $6"#
            }
            62 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5 WHERE "id" = $6"#
            }
            63 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_int" = $5, "f_text" = $6 WHERE "id" = $7"#
            }
            64 => r#"UPDATE "comp_types_table" SET "f_varchar" = $1 WHERE "id" = $2"#,
            65 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_varchar" = $2 WHERE "id" = $3"#
            }
            66 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_varchar" = $2 WHERE "id" = $3"#
            }
            67 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            68 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_varchar" = $2 WHERE "id" = $3"#
            }
            69 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            70 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            71 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            72 => {
                r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_varchar" = $2 WHERE "id" = $3"#
            }
            73 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            74 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            75 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            76 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            77 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            78 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            79 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            80 => r#"UPDATE "comp_types_table" SET "f_int" = $1, "f_varchar" = $2 WHERE "id" = $3"#,
            81 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_int" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            82 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_int" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            83 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            84 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_int" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            85 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            86 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            87 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_int" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            88 => {
                r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_int" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            89 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            90 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            91 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_int" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            92 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_int" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            93 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            94 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            95 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_int" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            96 => {
                r#"UPDATE "comp_types_table" SET "f_text" = $1, "f_varchar" = $2 WHERE "id" = $3"#
            }
            97 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_text" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            98 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_text" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            99 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            100 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_text" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            101 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            102 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            103 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            104 => {
                r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_text" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            105 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            106 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            107 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            108 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            109 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            110 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            111 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_text" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            112 => {
                r#"UPDATE "comp_types_table" SET "f_int" = $1, "f_text" = $2, "f_varchar" = $3 WHERE "id" = $4"#
            }
            113 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_int" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            114 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_int" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            115 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            116 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_int" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            117 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            118 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            119 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_int" = $4, "f_text" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            120 => {
                r#"UPDATE "comp_types_table" SET "f_float" = $1, "f_int" = $2, "f_text" = $3, "f_varchar" = $4 WHERE "id" = $5"#
            }
            121 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            122 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            123 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            124 => {
                r#"UPDATE "comp_types_table" SET "f_double" = $1, "f_float" = $2, "f_int" = $3, "f_text" = $4, "f_varchar" = $5 WHERE "id" = $6"#
            }
            125 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            126 => {
                r#"UPDATE "comp_types_table" SET "f_decimal" = $1, "f_double" = $2, "f_float" = $3, "f_int" = $4, "f_text" = $5, "f_varchar" = $6 WHERE "id" = $7"#
            }
            127 => {
                r#"UPDATE "comp_types_table" SET "f_bool" = $1, "f_decimal" = $2, "f_double" = $3, "f_float" = $4, "f_int" = $5, "f_text" = $6, "f_varchar" = $7 WHERE "id" = $8"#
            }
            _ => return Err(sqlx::Error::Protocol("invalid patch mask".into())),
        };

        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        if let Some(val) = &patch.f_bool {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_decimal {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_double {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_float {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_int {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_text {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_varchar {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
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
