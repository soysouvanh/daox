#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderItems {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderItemsOrderBy {
    OrderIdAsc,
    OrderIdDesc,
    ProductIdAsc,
    ProductIdDesc,
    QuantityAsc,
    QuantityDesc,
}

impl OrderItemsOrderBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderItemsOrderBy::OrderIdAsc => r#""order_id" ASC"#,
            OrderItemsOrderBy::OrderIdDesc => r#""order_id" DESC"#,
            OrderItemsOrderBy::ProductIdAsc => r#""product_id" ASC"#,
            OrderItemsOrderBy::ProductIdDesc => r#""product_id" DESC"#,
            OrderItemsOrderBy::QuantityAsc => r#""quantity" ASC"#,
            OrderItemsOrderBy::QuantityDesc => r#""quantity" DESC"#,
        }
    }
}

#[allow(clippy::all)]
impl OrderItems {
    #[allow(unused_comparisons)]
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        if let Some(v) = Some(&self.order_id) {
            if (*v as i64) < 0 {
                errors.push("order_id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.order_id) {
            if (*v as i64) > 9223372036854775807 {
                errors.push("order_id: maximum value '9223372036854775807' exceeded".into());
            }
        }
        if let Some(v) = Some(&self.product_id) {
            if (*v as i64) < 0 {
                errors.push("product_id: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.product_id) {
            if (*v as i64) > 9223372036854775807 {
                errors.push("product_id: maximum value '9223372036854775807' exceeded".into());
            }
        }
        if let Some(v) = Some(&self.quantity) {
            if (*v as i64) < 0 {
                errors.push("quantity: minimum value '0' not met".into());
            }
        }
        if let Some(v) = Some(&self.quantity) {
            if (*v as i64) > 2147483647 {
                errors.push("quantity: maximum value '2147483647' exceeded".into());
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
        let query = r#"SELECT COUNT(*) FROM order_items"#;
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Returns an approximate total number of rows in the table using database statistics (O(1)).
    /// This is extremely fast for huge tables but the number may be slightly outdated until the next VACUUM/ANALYZE.
    pub async fn approximate_count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
    ) -> sqlx::Result<u64> {
        let query = r#"SELECT reltuples::bigint FROM pg_class WHERE relname = 'order_items'"#;
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
        let query = r#"SELECT "order_id", "product_id", "quantity" FROM order_items ORDER BY "order_id" ASC LIMIT $1"#;
        sqlx::query_as::<_, Self>(query).bind(limit).fetch(executor)
    }

    #[deprecated(note = "Use list_by_cursor for large datasets")]
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        executor: E,
        order_by: &[OrderItemsOrderBy],
        page: u32,
        page_size: u32,
    ) -> sqlx::Result<Vec<Self>> {
        if order_by.is_empty() {
            return Err(sqlx::Error::Protocol("ORDER BY cannot be empty".into()));
        }
        let page_size = page_size.clamp(1, 10000);
        let offset = page.saturating_sub(1) * page_size;
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            r#"SELECT "order_id", "product_id", "quantity" FROM order_items"#,
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

    pub async fn get_by_order_id_and_product_id<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        order_id: i64,
        product_id: i64,
    ) -> sqlx::Result<Option<Self>> {
        let query = r#"SELECT "order_id", "product_id", "quantity" FROM order_items WHERE "order_id" = $1 AND "product_id" = $2"#;
        sqlx::query_as::<_, Self>(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor)
            .await
    }

    pub async fn exists_by_order_id_and_product_id<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        order_id: i64,
        product_id: i64,
    ) -> sqlx::Result<bool> {
        let query =
            r#"SELECT 1 FROM order_items WHERE "order_id" = $1 AND "product_id" = $2 LIMIT 1"#;
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor)
            .await?;
        Ok(exists.is_some())
    }

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query =
            r#"INSERT INTO order_items ("order_id", "product_id", "quantity") VALUES ($1, $2, $3)"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.order_id)
            .bind(&self.product_id)
            .bind(&self.quantity)
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
        let mut copy_in = executor.copy_in_raw(r#"COPY order_items ("order_id", "product_id", "quantity") FROM STDIN WITH (FORMAT csv)"#).await?;
        for chunk in items.chunks(10000) {
            let mut payload = String::with_capacity(chunk.len() * 512);
            #[allow(unused_imports)]
            use std::fmt::Write;
            for item in chunk {
                {
                    let v = &item.order_id;
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push(',');
                {
                    let v = &item.product_id;
                    write!(&mut payload, "{}", v).unwrap();
                }
                payload.push(',');
                {
                    let v = &item.quantity;
                    write!(&mut payload, "{}", v).unwrap();
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
        let query = r#"INSERT INTO order_items ("order_id", "product_id", "quantity") VALUES ($1, $2, $3) ON CONFLICT (\"order_id\", \"product_id\") DO UPDATE SET \"quantity\" = EXCLUDED.\"quantity\""#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(&self.order_id)
            .bind(&self.product_id)
            .bind(&self.quantity)
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
        let chunk_size = 65535 / 3;
        let mut total_affected = 0;
        for chunk in items.chunks(chunk_size.max(1)) {
            let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
                r#"INSERT INTO order_items ("order_id", "product_id", "quantity") "#,
            );
            qb.push_values(chunk, |mut b, item| {
                b.push_bind(&item.order_id);
                b.push_bind(&item.product_id);
                b.push_bind(&item.quantity);
            });
            qb.push(" ON CONFLICT (\"order_id\", \"product_id\") DO UPDATE SET \"quantity\" = EXCLUDED.\"quantity\"");
            let result = qb.build().execute(&mut **executor).await?;
            total_affected += result.rows_affected();
        }
        Ok(total_affected)
    }

    pub async fn update_by_order_id_and_product_id<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        &self,
        executor: E,
    ) -> sqlx::Result<u64> {
        let query_str =
            r#"UPDATE order_items SET "quantity" = $1 WHERE "order_id" = $2 AND "product_id" = $3"#;
        let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.quantity);
        query = query.bind(&self.order_id);
        query = query.bind(&self.product_id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_order_id_and_product_id<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        order_id: i64,
        product_id: i64,
    ) -> sqlx::Result<u64> {
        let query = r#"DELETE FROM order_items WHERE "order_id" = $1 AND "product_id" = $2"#;
        let result = sqlx::query::<sqlx::Postgres>(query)
            .bind(order_id)
            .bind(product_id)
            .execute(executor)
            .await?;
        Ok(result.rows_affected())
    }

    #[allow(unused_assignments)]
    pub async fn update_partial_by_order_id_and_product_id<
        'e,
        E: sqlx::Executor<'e, Database = sqlx::Postgres>,
    >(
        executor: E,
        order_id: i64,
        product_id: i64,
        patch: &OrderItemsPatch,
    ) -> sqlx::Result<u64> {
        let mut bits = [0u8; 1];
        let mut has = false;
        if patch.quantity.is_some() {
            bits[0] |= 1 << 0;
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
                    let mut q = String::with_capacity(288);
                    q.push_str("UPDATE order_items SET ");
                    let mut first = true;
                    let mut param_idx = 1;
                    if patch.quantity.is_some() {
                        if !first {
                            q.push_str(", ");
                        }
                        q.push_str(r#""quantity" = "#);
                        {
                            use std::fmt::Write;
                            write!(&mut q, "${}", param_idx).unwrap();
                            param_idx += 1;
                        }
                        first = false;
                    }
                    q.push_str(r#" WHERE "order_id" = "#);
                    {
                        use std::fmt::Write;
                        write!(&mut q, "${}", param_idx).unwrap();
                        param_idx += 1;
                    }
                    q.push_str(r#" AND "product_id" = "#);
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
        if let Some(val) = &patch.quantity {
            query = query.bind(val);
        }
        query = query.bind(order_id);
        query = query.bind(product_id);
        let result = query.execute(executor).await?;
        Ok(result.rows_affected())
    }
}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct OrderItemsPatch {
    pub quantity: Option<i32>,
}
