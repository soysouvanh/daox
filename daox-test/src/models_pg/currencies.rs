// Code generated automatically by daox. DO NOT EDIT.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Currencies {
    pub code: String,
    pub name: String,
}

impl Currencies {
    /// Counts the total number of rows in the table.
    /// 
    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM currencies";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Creates a zero-allocation Asynchronous Stream over the entire table.
    /// 
    /// **Performance:** This is the absolute best way to process millions of rows.
    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),
    /// the Stream fetches and yields rows one by one directly from the database connection.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM currencies";
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
        let query = format!("SELECT * FROM currencies ORDER BY {} LIMIT $1 OFFSET $2", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size as i64).bind(offset as i64).fetch_all(executor).await
    }

    /// Retrieves a single record using its Primary Key.
    /// 
    /// Returns `Some(Self)` if the record exists, or `None` if it does not.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &String) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM currencies WHERE code = $1";
        sqlx::query_as::<_, Self>(query)
            .bind(code)
            .fetch_optional(executor).await
    }

    /// Checks if a record exists using its Primary Key.
    /// 
    /// **Performance:** This uses a `SELECT 1 ... LIMIT 1` query. It is infinitely faster 
    /// and lighter than `get_by_pk` when you only need to check for existence, because it avoids 
    /// transferring and deserializing the full row data.
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &String) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM currencies WHERE code = $1 LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(code)
            .fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Cursor-based Pagination (Keyset Pagination).
    /// 
    /// **Performance:** This is the SOTA (State of the Art) standard for pagination.
    /// Unlike `OFFSET` which scans and discards thousands of rows, this jumps immediately to the 
    /// correct row using the B-Tree index, offering O(1) constant-time absolute performance.
    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: &String, limit: u32) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM currencies WHERE code > $1 ORDER BY code ASC LIMIT $2";
        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
    }

}

impl Currencies {
    /// Inserts the current record into the database.
    /// 
    /// **Best Practice:** Use this method when you want to create a brand new row.
    /// If the table has an auto-increment primary key, the database will generate the ID automatically.
    /// 
    /// Returns the generated ID (or 0 if the table doesn't have an auto-increment ID).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO currencies (code, name) VALUES ($1, $2)";
        let result = sqlx::query(&query)
            .bind(&self.code)
            .bind(&self.name)
            .execute(executor).await?;
        Ok(result.rows_affected())
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
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO currencies (code, name) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.code);
            b.push_bind(&item.name);
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
        let query = "INSERT INTO currencies (code, name) VALUES ($1, $2) ON CONFLICT (code) DO UPDATE SET code = EXCLUDED.code, name = EXCLUDED.name";
        let result = sqlx::query(&query)
            .bind(&self.code)
            .bind(&self.name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Overwrites the entire record in the database using its Primary Key.
    /// 
    /// **Warning:** This will update ALL columns in the row with the values in the current struct.
    /// If you only want to update one or two specific columns, use `update_partial_by_pk` instead 
    /// to save network bandwidth and database disk I/O.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE currencies SET name = $1 WHERE code = $2";
        let result = sqlx::query(&query)
            .bind(&self.name)
            .bind(&self.code)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes the specific record from the database using its Primary Key.
    /// 
    /// Returns the number of affected rows (1 if deleted, 0 if it didn't exist).
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &String) -> sqlx::Result<u64> {
        let query = "DELETE FROM currencies WHERE code = $1";
        let result = sqlx::query(&query)
            .bind(code)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes multiple records in a single query using an `IN (...)` clause.
    /// 
    /// **Performance:** This is the most efficient way to delete a batch of specific IDs.
    /// Returns the total number of rows successfully deleted.
    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, ids: &[String]) -> sqlx::Result<u64> {
        if ids.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM currencies WHERE code IN ");
        query_builder.push("(");
        let mut separated = query_builder.separated(", ");
        for id in ids { separated.push_bind(id); }
        separated.push_unseparated(")");
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure used for partial updates (Patching) of `currencies`.
/// 
/// Each field is wrapped in an `Option`. If a field is `None`, it will be completely ignored during the update.
/// If it is `Some(value)`, that column will be updated in the database.
#[derive(Debug, Clone, Default)]
pub struct CurrenciesPatch {
    pub name: Option<String>,
}

impl Currencies {
    /// Updates ONLY the columns that contain data in the `patch` struct.
    /// 
    /// **Performance:** This is the most optimized way to update data.
    /// It dynamically builds the SQL query to only include the changed columns, which saves network bandwidth
    /// and significantly reduces database disk I/O (WAL logging) compared to a full row update.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &String, patch: &CurrenciesPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE currencies SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.name {
            has_fields = true;
            separated.push("name = ");
            separated.push_bind_unseparated(val.clone());
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE code = ");
        query_builder.push_bind(code.clone());

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

