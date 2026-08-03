#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProductMetadata {
    pub attributes: Option<serde_json::Value>,
    pub category: String,
    pub id: String,
    pub raw_data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductMetadataOrderBy {
    AttributesAsc,
    AttributesDesc,
    CategoryAsc,
    CategoryDesc,
    IdAsc,
    IdDesc,
    RawDataAsc,
    RawDataDesc,
}

impl ProductMetadataOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProductMetadataOrderBy::AttributesAsc => r#""attributes" ASC"#,
            ProductMetadataOrderBy::AttributesDesc => r#""attributes" DESC"#,
            ProductMetadataOrderBy::CategoryAsc => r#""category" ASC"#,
            ProductMetadataOrderBy::CategoryDesc => r#""category" DESC"#,
            ProductMetadataOrderBy::IdAsc => r#""id" ASC"#,
            ProductMetadataOrderBy::IdDesc => r#""id" DESC"#,
            ProductMetadataOrderBy::RawDataAsc => r#""raw_data" ASC"#,
            ProductMetadataOrderBy::RawDataDesc => r#""raw_data" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl ProductMetadata {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.category) {
            if v.len() < 1 {
                errors.push("category: min_length 1 not met".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if v.len() < 1 {
                errors.push("id: min_length 1 not met".into());
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
        let query = r#"SELECT COUNT(*) FROM product_metadata"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'product_metadata'"#;
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
        let query = r#"SELECT "attributes", "category", "id", "raw_data" FROM product_metadata ORDER BY "id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: &str,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT "attributes", "category", "id", "raw_data" FROM product_metadata WHERE "id" = $1"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: &str,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM product_metadata WHERE "id" = $1 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        last_id: &str,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let query = r#"SELECT "attributes", "category", "id", "raw_data" FROM product_metadata WHERE "id" > $1 ORDER BY "id" ASC LIMIT $2"#;
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
        let query = r#"INSERT INTO product_metadata ("attributes", "category", "id", "raw_data") VALUES ($1, $2, $3, $4)"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.attributes)
            .bind(&self.category)
            .bind(&self.id)
            .bind(&self.raw_data)
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
        let mut copy_in = executor.copy_in_raw(r#"COPY product_metadata ("attributes", "category", "id", "raw_data") FROM STDIN WITH (FORMAT csv)"#).await?;
        for chunk in items.chunks(10000) {
            let mut payload = String::with_capacity(chunk.len() * 512);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                if let Some(v) = &item.attributes {
                    let json_str = v.to_string();
                    payload.push('"');
                    for c in json_str.chars() {
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
                    let v = &item.category;
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
                if let Some(v) = &item.raw_data {
                    payload.push_str("\"\\x");
                    for b in v {
                        write!(&mut payload, "{:02x}", b).unwrap();
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
        let query = r#"INSERT INTO product_metadata ("attributes", "category", "id", "raw_data") VALUES ($1, $2, $3, $4) ON CONFLICT (\"id\") DO UPDATE SET \"attributes\" = EXCLUDED.\"attributes\", \"category\" = EXCLUDED.\"category\", \"raw_data\" = EXCLUDED.\"raw_data\""#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.attributes)
            .bind(&self.category)
            .bind(&self.id)
            .bind(&self.raw_data)
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
        let chunk_size = 65535 / 4;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
                r#"INSERT INTO product_metadata ("attributes", "category", "id", "raw_data") "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.attributes);
                b.push_bind(&item.category);
                b.push_bind(&item.id);
                b.push_bind(&item.raw_data);
            });
            qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"attributes\" = EXCLUDED.\"attributes\", \"category\" = EXCLUDED.\"category\", \"raw_data\" = EXCLUDED.\"raw_data\"");
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE product_metadata SET "attributes" = $1, "category" = $2, "raw_data" = $3 WHERE "id" = $4"#;
        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.attributes);
        query = query.bind(&self.category);
        query = query.bind(&self.raw_data);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        id: &String,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM product_metadata WHERE "id" = $1"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Postgres>,
        ids: &[String],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 500_usize.min(65535);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> =
                sqlx::QueryBuilder::new(r#"DELETE FROM product_metadata WHERE "id" IN "#);
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
        id: &str,
        patch: &ProductMetadataPatch,
    ) -> sqlx::Result<u64> {
        let mut bits = [0u8; 1];
        let mut has = false;
        if patch.attributes.is_some() {
            bits[0] |= 1 << 0;
            has = true;
        }
        if patch.category.is_some() {
            bits[0] |= 1 << 1;
            has = true;
        }
        if patch.raw_data.is_some() {
            bits[0] |= 1 << 2;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        static CACHE: std::sync::OnceLock<
            [std::sync::RwLock<std::collections::HashMap<[u8; 1], String>>; 16],
        > = std::sync::OnceLock::new();
        let cache_shards = CACHE.get_or_init(|| {
            std::array::from_fn(|_| std::sync::RwLock::new(std::collections::HashMap::new()))
        });
        let shard_idx = bits
            .iter()
            .fold(0usize, |acc, &b| acc.wrapping_add(b as usize) ^ (acc << 3))
            % 16;
        let cache_lock = &cache_shards[shard_idx];
        let query_str = {
            let read = cache_lock.read().unwrap();
            if let Some(q) = read.get(&bits) {
                q.clone()
            } else {
                drop(read);
                let mut write = cache_lock.write().unwrap();
                if let Some(q) = write.get(&bits) {
                    q.clone()
                } else {
                    let mut q = String::with_capacity(352);
                    q.push_str("UPDATE product_metadata SET ");
                    let mut first = true;
                    let mut param_idx = 1;
                    if patch.attributes.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#""attributes" = "#);
                        {
                            use std::fmt::Write;
                            write!(&mut q, "${}", param_idx).unwrap();
                            param_idx += 1;
                        }
                        first = false;
                    }
                    if patch.category.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#""category" = "#);
                        {
                            use std::fmt::Write;
                            write!(&mut q, "${}", param_idx).unwrap();
                            param_idx += 1;
                        }
                        first = false;
                    }
                    if patch.raw_data.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#""raw_data" = "#);
                        {
                            use std::fmt::Write;
                            write!(&mut q, "${}", param_idx).unwrap();
                            param_idx += 1;
                        }
                        first = false;
                    }
                    q.push_str(r#" WHERE "id" = "#);
                    {
                        use std::fmt::Write;
                        write!(&mut q, "${}", param_idx).unwrap();
                        param_idx += 1;
                    }
                    if write.len() < 1000 {
                        write.insert(bits, q.clone());
                    }
                    q
                }
            }
        };

        let mut query = sqlx::query::<sqlx::Postgres>(sqlx::AssertSqlSafe(query_str.as_str()));
        if let Some(val) = &patch.attributes {
            query = query.bind(val);
        }
        if let Some(val) = &patch.category {
            query = query.bind(val);
        }
        if let Some(val) = &patch.raw_data {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct ProductMetadataPatch {
    pub attributes: Option<Option<serde_json::Value>>,
    pub category: Option<String>,
    pub raw_data: Option<Option<Vec<u8>>>,
}
