// Code generated automatically by daox. DO NOT EDIT.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Configurations {
    pub id: i32,
    pub r#type: String,
    pub r#match: Option<String>,
    pub value: Option<String>,
}

impl Configurations {
    /// Counts the total number of rows in the table.
    /// 
    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM configurations";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Creates a zero-allocation Asynchronous Stream over the entire table.
    /// 
    /// **Performance:** This is the absolute best way to process millions of rows.
    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),
    /// the Stream fetches and yields rows one by one directly from the database connection.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM configurations";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Classic Offset/Limit pagination with dynamic sorting.
    /// 
    /// **SECURITY WARNING:** The `order_by` parameter is NOT bound via prepared statements 
    /// (SQL does not allow binding column names). You MUST strictly whitelist the user input 
    /// before passing it here to prevent SQL Injection!
    /// 
    /// **Performance:** Offset pagination becomes very slow on deep pages. Consider `list_by_cursor` instead.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM configurations ORDER BY {} LIMIT $1 OFFSET $2", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size as i64).bind(offset as i64).fetch_all(executor).await
    }

    /// Retrieves a single record using its Primary Key.
    /// 
    /// Returns `Some(Self)` if the record exists, or `None` if it does not.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &i32) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM configurations WHERE id = $1";
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor).await
    }

    /// Checks if a record exists using its Primary Key.
    /// 
    /// **Performance:** This uses a `SELECT 1 ... LIMIT 1` query. It is infinitely faster 
    /// and lighter than `get_by_pk` when you only need to check for existence, because it avoids 
    /// transferring and deserializing the full row data.
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &i32) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM configurations WHERE id = $1 LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(id)
            .fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Cursor-based Pagination (Keyset Pagination).
    /// 
    /// **Performance:** This is the SOTA (State of the Art) standard for pagination.
    /// Unlike `OFFSET` which scans and discards thousands of rows, this jumps immediately to the 
    /// correct row using the B-Tree index, offering O(1) constant-time absolute performance.
    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: &i32, limit: u32) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM configurations WHERE id > $1 ORDER BY id ASC LIMIT $2";
        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
    }

}

impl Configurations {
    /// Inserts the current record into the database.
    /// 
    /// **Best Practice:** Use this method when you want to create a brand new row.
    /// If the table has an auto-increment primary key, the database will generate the ID automatically.
    /// 
    /// Returns the generated ID (or 0 if the table doesn't have an auto-increment ID).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO configurations (\"type\", \"match\", value) VALUES ($1, $2, $3) RETURNING id::bigint";
        let (id,): (i64,) = sqlx::query_as(&query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .fetch_one(executor).await?;
        Ok(id as u64)
    }

    /// Inserts multiple records in a single network round-trip (Batch Insert).
    /// 
    /// **Performance:** This is heavily optimized. Instead of running 100 individual `INSERT` queries,
    /// this method groups them into one massive `INSERT INTO ... VALUES (...), (...), ...` query.
    /// Always prefer this method over looping with `.insert()` when saving large amounts of data.
    /// 
    /// Returns the number of rows successfully inserted.
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO configurations (\"type\", \"match\", value) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.r#type);
            b.push_bind(&item.r#match);
            b.push_bind(&item.value);
        });
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Inserts the record, or updates it if a unique constraint is violated (Upsert).
    /// 
    /// **How it works:** 
    /// 1. The database attempts to insert the row.
    /// 2. If a collision occurs (e.g., an email already exists in a UNIQUE index),
    ///    it automatically updates the existing row with the new data instead of crashing.
    /// 
    /// This is highly recommended for data synchronization tasks.
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO configurations (\"type\", \"match\", value) VALUES ($1, $2, $3)";
        let result = sqlx::query(&query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Overwrites the entire record in the database using its Primary Key.
    /// 
    /// **Warning:** This will update ALL columns in the row with the values in the current struct.
    /// If you only want to update one or two specific columns, use `update_partial_by_pk` instead 
    /// to save network bandwidth and database disk I/O.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE configurations SET \"type\" = $1, \"match\" = $2, value = $3 WHERE id = $4";
        let result = sqlx::query(&query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .bind(&self.id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes the specific record from the database using its Primary Key.
    /// 
    /// Returns the number of affected rows (1 if deleted, 0 if it didn't exist).
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &i32) -> sqlx::Result<u64> {
        let query = "DELETE FROM configurations WHERE id = $1";
        let result = sqlx::query(&query)
            .bind(id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes multiple records in a single query using an `IN (...)` clause.
    /// 
    /// **Performance:** This is the most efficient way to delete a batch of specific IDs.
    /// Returns the total number of rows successfully deleted.
    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, ids: &[i32]) -> sqlx::Result<u64> {
        if ids.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM configurations WHERE id IN ");
        query_builder.push("(");
        let mut separated = query_builder.separated(", ");
        for id in ids { separated.push_bind(id); }
        separated.push_unseparated(")");
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure used for partial updates (Patching) of `configurations`.
/// 
/// Each field is wrapped in an `Option`. If a field is `None`, it will be completely ignored during the update.
/// If it is `Some(value)`, that column will be updated in the database.
#[derive(Debug, Clone, Default)]
pub struct ConfigurationsPatch {
    pub r#type: Option<String>,
    pub r#match: Option<Option<String>>,
    pub value: Option<Option<String>>,
}

impl Configurations {
    /// Updates ONLY the columns that contain data in the `patch` struct.
    /// 
    /// **Performance:** This is the most optimized way to update data.
    /// It dynamically builds the SQL query to only include the changed columns, which saves network bandwidth
    /// and significantly reduces database disk I/O (WAL logging) compared to a full row update.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &i32, patch: &ConfigurationsPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE configurations SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.r#type {
            has_fields = true;
            separated.push("\"type\" = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.r#match {
            has_fields = true;
            separated.push("\"match\" = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.value {
            has_fields = true;
            separated.push("value = ");
            separated.push_bind_unseparated(val.clone());
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id.clone());

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

