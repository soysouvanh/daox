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
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_text.as_ref() {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("f_text: format constraint not met".into());
            }
        }
        if let Some(v) = self.f_varchar.as_ref() {
            if v.len() > 255 {
                errors.push("f_varchar: exceeds max_length 255".into());
            }
        }
        #[cfg(feature = "validation")]
        if let Some(v) = self.f_varchar.as_ref() {
            static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
            let re = RE.get_or_init(|| {
                regex::Regex::new("^[À-ÿA-Za-z0-9_ -]*$").expect("Invalid regex in TOML")
            });
            if !re.is_match(v) {
                errors.push("f_varchar: format constraint not met".into());
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
        let query = r#"SELECT COUNT(*) FROM `comp_types_table`"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT table_rows FROM information_schema.tables WHERE table_name = 'comp_types_table' AND table_schema = DATABASE()"#;
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
            1 => r#"UPDATE `comp_types_table` SET `f_bool` = ? WHERE `id` = ?"#,
            2 => r#"UPDATE `comp_types_table` SET `f_decimal` = ? WHERE `id` = ?"#,
            3 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ? WHERE `id` = ?"#,
            4 => r#"UPDATE `comp_types_table` SET `f_double` = ? WHERE `id` = ?"#,
            5 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ? WHERE `id` = ?"#,
            6 => r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ? WHERE `id` = ?"#,
            7 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ? WHERE `id` = ?"#
            }
            8 => r#"UPDATE `comp_types_table` SET `f_float` = ? WHERE `id` = ?"#,
            9 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ? WHERE `id` = ?"#,
            10 => r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ? WHERE `id` = ?"#,
            11 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ? WHERE `id` = ?"#
            }
            12 => r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ? WHERE `id` = ?"#,
            13 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ? WHERE `id` = ?"#
            }
            14 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ? WHERE `id` = ?"#
            }
            15 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ? WHERE `id` = ?"#
            }
            16 => r#"UPDATE `comp_types_table` SET `f_int` = ? WHERE `id` = ?"#,
            17 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_int` = ? WHERE `id` = ?"#,
            18 => r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_int` = ? WHERE `id` = ?"#,
            19 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            20 => r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_int` = ? WHERE `id` = ?"#,
            21 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            22 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            23 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            24 => r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_int` = ? WHERE `id` = ?"#,
            25 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            26 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            27 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            28 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            29 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            30 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            31 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ? WHERE `id` = ?"#
            }
            32 => r#"UPDATE `comp_types_table` SET `f_text` = ? WHERE `id` = ?"#,
            33 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_text` = ? WHERE `id` = ?"#,
            34 => r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_text` = ? WHERE `id` = ?"#,
            35 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            36 => r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_text` = ? WHERE `id` = ?"#,
            37 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            38 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            39 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            40 => r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_text` = ? WHERE `id` = ?"#,
            41 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            42 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            43 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            44 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            45 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            46 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            47 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            48 => r#"UPDATE `comp_types_table` SET `f_int` = ?, `f_text` = ? WHERE `id` = ?"#,
            49 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            50 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            51 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            52 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            53 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            54 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            55 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            56 => {
                r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            57 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            58 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            59 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            60 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            61 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            62 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            63 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ? WHERE `id` = ?"#
            }
            64 => r#"UPDATE `comp_types_table` SET `f_varchar` = ? WHERE `id` = ?"#,
            65 => r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_varchar` = ? WHERE `id` = ?"#,
            66 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            67 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            68 => r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_varchar` = ? WHERE `id` = ?"#,
            69 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            70 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            71 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            72 => r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#,
            73 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            74 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            75 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            76 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            77 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            78 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            79 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            80 => r#"UPDATE `comp_types_table` SET `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#,
            81 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            82 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            83 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            84 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            85 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            86 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            87 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            88 => {
                r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            89 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            90 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            91 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            92 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            93 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            94 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            95 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            96 => r#"UPDATE `comp_types_table` SET `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#,
            97 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            98 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            99 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            100 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            101 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            102 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            103 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            104 => {
                r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            105 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            106 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            107 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            108 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            109 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            110 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            111 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            112 => {
                r#"UPDATE `comp_types_table` SET `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            113 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            114 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            115 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            116 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            117 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            118 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            119 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            120 => {
                r#"UPDATE `comp_types_table` SET `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            121 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            122 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            123 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            124 => {
                r#"UPDATE `comp_types_table` SET `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            125 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            126 => {
                r#"UPDATE `comp_types_table` SET `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            127 => {
                r#"UPDATE `comp_types_table` SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?"#
            }
            _ => unreachable!(),
        };

        let mut query = sqlx::query::<sqlx::MySql>(query_str);
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
