#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i32,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<serde_json::Value>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompTypesMetadataOrderBy {
    CompTypesIdAsc,
    CompTypesIdDesc,
    FBlobAsc,
    FBlobDesc,
    FDateAsc,
    FDateDesc,
    FDatetimeAsc,
    FDatetimeDesc,
    FJsonAsc,
    FJsonDesc,
    FTimestampAsc,
    FTimestampDesc,
    IdAsc,
    IdDesc,
}

impl CompTypesMetadataOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            CompTypesMetadataOrderBy::CompTypesIdAsc => r#"`comp_types_id` ASC"#,
            CompTypesMetadataOrderBy::CompTypesIdDesc => r#"`comp_types_id` DESC"#,
            CompTypesMetadataOrderBy::FBlobAsc => r#"`f_blob` ASC"#,
            CompTypesMetadataOrderBy::FBlobDesc => r#"`f_blob` DESC"#,
            CompTypesMetadataOrderBy::FDateAsc => r#"`f_date` ASC"#,
            CompTypesMetadataOrderBy::FDateDesc => r#"`f_date` DESC"#,
            CompTypesMetadataOrderBy::FDatetimeAsc => r#"`f_datetime` ASC"#,
            CompTypesMetadataOrderBy::FDatetimeDesc => r#"`f_datetime` DESC"#,
            CompTypesMetadataOrderBy::FJsonAsc => r#"`f_json` ASC"#,
            CompTypesMetadataOrderBy::FJsonDesc => r#"`f_json` DESC"#,
            CompTypesMetadataOrderBy::FTimestampAsc => r#"`f_timestamp` ASC"#,
            CompTypesMetadataOrderBy::FTimestampDesc => r#"`f_timestamp` DESC"#,
            CompTypesMetadataOrderBy::IdAsc => r#"`id` ASC"#,
            CompTypesMetadataOrderBy::IdDesc => r#"`id` DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl CompTypesMetadata {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.comp_types_id) {
            if (*v as i64) < 0 {
                errors.push("comp_types_id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.comp_types_id) {
            if (*v as i64) > 2147483647 {
                errors.push("comp_types_id: maximum value '2147483647' exceeded".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i64) < 0 {
                errors.push("id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.id) {
            if (*v as i64) > 2147483647 {
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT COUNT(*) FROM comp_types_metadata"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table.
    /// Uses `MAX(rowid)` to provide an instant O(1) estimate without a full table scan.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT MAX(rowid) FROM comp_types_metadata"#;
        let count: Option<(Option<i64>,)> = sqlx::query_as(query).fetch_optional(executor).await?;
        Ok(count
            .and_then(|(c,)| c)
            .map(|c| c.max(0) as u64)
            .unwrap_or(0))
    }

    /// Streams rows from the table, ordered by the primary key.
    /// **⚠️ Performance Warning:** Streaming a whole table without a limit or timeout can cause connection pool starvation.
    /// A `limit` parameter is now mandatory to prevent Unbounded Streaming DoS.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(
        executor: E,
        limit: i64,
    ) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM comp_types_metadata WHERE `id` = ? LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        last_id: i32,
        limit: u32,
    ) -> sqlx::Result<Vec<Self>> {
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(last_id)
            .bind(limit as i64)
            .fetch_all(executor)
            .await
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.comp_types_id)
            .bind(&self.f_blob)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_json)
            .bind(&self.f_timestamp)
            .execute(executor)
            .await?;
        Ok(result.last_insert_rowid() as u64)
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
        let chunk_size = 32766 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.comp_types_id);
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_timestamp);
            });
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(&self.comp_types_id)
            .bind(&self.f_blob)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_json)
            .bind(&self.f_timestamp)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    /// Upserts a batch of records.
    /// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
    pub async fn upsert_batch<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 32766 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new(
                r#"INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.comp_types_id);
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_timestamp);
            });
            qb.push(" ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`");
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE comp_types_metadata SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.comp_types_id);
        query = query.bind(&self.f_blob);
        query = query.bind(&self.f_date);
        query = query.bind(&self.f_datetime);
        query = query.bind(&self.f_json);
        query = query.bind(&self.f_timestamp);
        query = query.bind(&self.id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM comp_types_metadata WHERE `id` = ?"#;
        let result = sqlx::query::<sqlx::Sqlite>(query)
            .bind(id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_many_by_id<'e>(
        executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>,
        ids: &[i32],
    ) -> sqlx::Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let mut total_affected = 0;
        let chunk_size = 5000_usize.min(32766);
        for chunk in ids.chunks(chunk_size) {
            let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> =
                sqlx::QueryBuilder::new(r#"DELETE FROM comp_types_metadata WHERE `id` IN "#);
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
    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(
        executor: E,
        id: i32,
        patch: &CompTypesMetadataPatch,
    ) -> sqlx::Result<u64> {
        let mut bits = [0u8; 1];
        let mut has = false;
        if patch.comp_types_id.is_some() {
            bits[0] |= 1 << 0;
            has = true;
        }
        if patch.f_blob.is_some() {
            bits[0] |= 1 << 1;
            has = true;
        }
        if patch.f_date.is_some() {
            bits[0] |= 1 << 2;
            has = true;
        }
        if patch.f_datetime.is_some() {
            bits[0] |= 1 << 3;
            has = true;
        }
        if patch.f_json.is_some() {
            bits[0] |= 1 << 4;
            has = true;
        }
        if patch.f_timestamp.is_some() {
            bits[0] |= 1 << 5;
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
                    let mut q = String::with_capacity(448);
                    q.push_str("UPDATE comp_types_metadata SET ");
                    let mut first = true;
                    if patch.comp_types_id.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`comp_types_id` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.f_blob.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`f_blob` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.f_date.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`f_date` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.f_datetime.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`f_datetime` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.f_json.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`f_json` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    if patch.f_timestamp.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#"`f_timestamp` = "#);
                        q.push_str("?");
                        first = false;
                    }
                    q.push_str(r#" WHERE `id` = "#);
                    q.push_str("?");
                    if write.len() < 1000 {
                        write.insert(bits, q.clone());
                    }
                    q
                }
            }
        };

        let mut query = sqlx::query::<sqlx::Sqlite>(sqlx::AssertSqlSafe(query_str.as_str()));
        if let Some(val) = &patch.comp_types_id {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_blob {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_date {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_datetime {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_json {
            query = query.bind(val);
        }
        if let Some(val) = &patch.f_timestamp {
            query = query.bind(val);
        }
        query = query.bind(id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesMetadataPatch {
    pub comp_types_id: Option<i32>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<serde_json::Value>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}
