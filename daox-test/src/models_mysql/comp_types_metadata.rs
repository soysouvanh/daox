#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i64,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<serde_json::Value>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i64,
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
            if (*v as i64) > 9223372036854775807 {
                errors.push("comp_types_id: maximum value '9223372036854775807' exceeded".into());
            }
        }
        if let Some(v) = self.f_blob.as_ref() {
            if v.len() > 65535 {
                errors.push("f_blob: exceeds max_length 65535".into());
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
        let query = r#"SELECT COUNT(*) FROM `comp_types_metadata`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'comp_types_metadata' AND table_schema = DATABASE()"#;
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
        let limit = limit.clamp(1, 10000);
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` ORDER BY `id` ASC LIMIT ?"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` WHERE `id` = ?"#;
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<bool> {
        let query = r#"SELECT 1 FROM `comp_types_metadata` WHERE `id` = ? LIMIT 1"#;
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
        let query = r#"SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM `comp_types_metadata` WHERE `id` > ? ORDER BY `id` ASC LIMIT ?"#;
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
        let query = r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
            .bind(&self.comp_types_id)
            .bind(&self.f_blob)
            .bind(&self.f_date)
            .bind(&self.f_datetime)
            .bind(&self.f_json)
            .bind(&self.f_timestamp)
            .execute(executor)
            .await?;
        Ok(result.last_insert_id())
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
        let chunk_size = 65535 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
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

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `comp_types_id` = VALUES(`comp_types_id`), `f_blob` = VALUES(`f_blob`), `f_date` = VALUES(`f_date`), `f_datetime` = VALUES(`f_datetime`), `f_json` = VALUES(`f_json`), `f_timestamp` = VALUES(`f_timestamp`)"#;
        let result = sqlx::query::<sqlx::MySql>(query)
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
        executor: &mut sqlx::Transaction<'e, sqlx::MySql>,
        items: &[Self],
    ) -> sqlx::Result<u64> {
        if items.is_empty() {
            return Ok(0);
        }
        let chunk_size = 65535 / 6;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new(
                r#"INSERT INTO `comp_types_metadata` (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.comp_types_id);
                b.push_bind(&item.f_blob);
                b.push_bind(&item.f_date);
                b.push_bind(&item.f_datetime);
                b.push_bind(&item.f_json);
                b.push_bind(&item.f_timestamp);
            });
            qb.push(r#" ON DUPLICATE KEY UPDATE `comp_types_id` = VALUES(`comp_types_id`), `f_blob` = VALUES(`f_blob`), `f_date` = VALUES(`f_date`), `f_datetime` = VALUES(`f_datetime`), `f_json` = VALUES(`f_json`), `f_timestamp` = VALUES(`f_timestamp`)"#);
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str = r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#;
        let mut query = sqlx::query::<sqlx::MySql>(query_str);
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
        id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM `comp_types_metadata` WHERE `id` = ?"#;
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
                sqlx::QueryBuilder::new(r#"DELETE FROM `comp_types_metadata` WHERE `id` IN "#);
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
        patch: &CompTypesMetadataPatch,
    ) -> sqlx::Result<u64> {
        let mut mask = 0u64;
        let mut has = false;
        if patch.comp_types_id.is_some() {
            mask |= 1 << 0;
            has = true;
        }
        if patch.f_blob.is_some() {
            mask |= 1 << 1;
            has = true;
        }
        if patch.f_date.is_some() {
            mask |= 1 << 2;
            has = true;
        }
        if patch.f_datetime.is_some() {
            mask |= 1 << 3;
            has = true;
        }
        if patch.f_json.is_some() {
            mask |= 1 << 4;
            has = true;
        }
        if patch.f_timestamp.is_some() {
            mask |= 1 << 5;
            has = true;
        }
        if !has {
            return Ok(0);
        }

        let query_str = match mask {
            1 => r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ? WHERE `id` = ?"#,
            2 => r#"UPDATE `comp_types_metadata` SET `f_blob` = ? WHERE `id` = ?"#,
            3 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ? WHERE `id` = ?"#
            }
            4 => r#"UPDATE `comp_types_metadata` SET `f_date` = ? WHERE `id` = ?"#,
            5 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ? WHERE `id` = ?"#
            }
            6 => r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ? WHERE `id` = ?"#,
            7 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ? WHERE `id` = ?"#
            }
            8 => r#"UPDATE `comp_types_metadata` SET `f_datetime` = ? WHERE `id` = ?"#,
            9 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            10 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            11 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            12 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            13 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            14 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            15 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ? WHERE `id` = ?"#
            }
            16 => r#"UPDATE `comp_types_metadata` SET `f_json` = ? WHERE `id` = ?"#,
            17 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            18 => r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_json` = ? WHERE `id` = ?"#,
            19 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            20 => r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_json` = ? WHERE `id` = ?"#,
            21 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            22 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            23 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            24 => {
                r#"UPDATE `comp_types_metadata` SET `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            25 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            26 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            27 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            28 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            29 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            30 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            31 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ? WHERE `id` = ?"#
            }
            32 => r#"UPDATE `comp_types_metadata` SET `f_timestamp` = ? WHERE `id` = ?"#,
            33 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            34 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            35 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            36 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            37 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            38 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            39 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            40 => {
                r#"UPDATE `comp_types_metadata` SET `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            41 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            42 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            43 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            44 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            45 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            46 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            47 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            48 => {
                r#"UPDATE `comp_types_metadata` SET `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            49 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            50 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            51 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            52 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            53 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            54 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            55 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            56 => {
                r#"UPDATE `comp_types_metadata` SET `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            57 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            58 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            59 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            60 => {
                r#"UPDATE `comp_types_metadata` SET `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            61 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            62 => {
                r#"UPDATE `comp_types_metadata` SET `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            63 => {
                r#"UPDATE `comp_types_metadata` SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?"#
            }
            _ => unreachable!(),
        };

        let mut query = sqlx::query::<sqlx::MySql>(query_str);
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
    pub comp_types_id: Option<i64>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<serde_json::Value>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}
