// Code generated automatically by daox. DO NOT EDIT.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub id: i64,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Users {
    /// Counts the total number of rows in the table.
    /// 
    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM users";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Creates a zero-allocation Asynchronous Stream over the entire table.
    /// 
    /// **Performance:** This is the absolute best way to process millions of rows.
    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),
    /// the Stream fetches and yields rows one by one directly from the database connection.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM users";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Classic Offset/Limit pagination with dynamic sorting.
    /// 
    /// **SECURITY WARNING:** The `order_by` parameter is NOT bound via prepared statements 
    /// (SQL does not allow binding column names). You MUST strictly whitelist the user input 
    /// before passing it here to prevent SQL Injection!
    /// 
    /// **Performance:** Offset pagination becomes very slow on deep pages. Consider `list_by_cursor` instead.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM users ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Retrieves a single record using its Primary Key.
    /// 
    /// Returns `Some(Self)` if the record exists, or `None` if it does not.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM users WHERE id = ?";
        sqlx::query_as::<_, Self>(query)
            .bind(id)
            .fetch_optional(executor).await
    }

    /// Checks if a record exists using its Primary Key.
    /// 
    /// **Performance:** This uses a `SELECT 1 ... LIMIT 1` query. It is infinitely faster 
    /// and lighter than `get_by_pk` when you only need to check for existence, because it avoids 
    /// transferring and deserializing the full row data.
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE id = ? LIMIT 1";
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
    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_id: &i64, limit: u32) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM users WHERE id > ? ORDER BY id ASC LIMIT ?";
        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit).fetch_all(executor).await
    }

    /// Checks if a record exists using the `idx_name` index.
    /// 
    /// **Performance:** Extremely fast, uses `SELECT 1 ... LIMIT 1`.
    pub async fn exists_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE last_name = ? AND first_name = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(last_name).bind(first_name).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Retrieves all records matching the `idx_name` index.
    pub async fn list_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM users WHERE last_name = ? AND first_name = ?";
        sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).fetch_all(executor).await
    }

    /// Creates a zero-allocation Asynchronous Stream using the `idx_name` index.
    pub fn stream_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, last_name: &String, first_name: &Option<String>) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM users WHERE last_name = ? AND first_name = ?";
        sqlx::query_as::<_, Self>(query).bind(last_name.clone()).bind(first_name.clone()).fetch(executor)
    }

    /// Checks if a record exists using the `idx_email` index.
    /// 
    /// **Performance:** Extremely fast, uses `SELECT 1 ... LIMIT 1`.
    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE email = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(email).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Retrieves a single record using the unique `idx_email` index.
    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM users WHERE email = ?";
        sqlx::query_as::<_, Self>(query).bind(email).fetch_optional(executor).await
    }

}

impl Users {
    /// Inserts the current record into the database.
    /// 
    /// **Best Practice:** Use this method when you want to create a brand new row.
    /// If the table has an auto-increment primary key, the database will generate the ID automatically.
    /// 
    /// Returns the generated ID (or 0 if the table doesn't have an auto-increment ID).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO users (email, first_name, last_name, status, created_at) VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(self.created_at)
            .execute(executor).await?;
        Ok(result.last_insert_id())
    }

    /// Inserts multiple records in a single network round-trip (Batch Insert).
    /// 
    /// **Performance:** This is heavily optimized. Instead of running 100 individual `INSERT` queries,
    /// this method groups them into one massive `INSERT INTO ... VALUES (...), (...), ...` query.
    /// Always prefer this method over looping with `.insert()` when saving large amounts of data.
    /// 
    /// Returns the number of rows successfully inserted.
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO users (email, first_name, last_name, status, created_at) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            b.push_bind(item.created_at);
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
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO users (email, first_name, last_name, status, created_at) VALUES (?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE email = VALUES(email), first_name = VALUES(first_name), last_name = VALUES(last_name), status = VALUES(status), created_at = VALUES(created_at)";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(self.created_at)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Overwrites the entire record in the database using its Primary Key.
    /// 
    /// **Warning:** This will update ALL columns in the row with the values in the current struct.
    /// If you only want to update one or two specific columns, use `update_partial_by_pk` instead 
    /// to save network bandwidth and database disk I/O.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET email = ?, first_name = ?, last_name = ?, status = ?, created_at = ? WHERE id = ?";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(self.created_at)
            .bind(self.id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes the specific record from the database using its Primary Key.
    /// 
    /// Returns the number of affected rows (1 if deleted, 0 if it didn't exist).
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE id = ?";
        let result = sqlx::query(query)
            .bind(id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes multiple records in a single query using an `IN (...)` clause.
    /// 
    /// **Performance:** This is the most efficient way to delete a batch of specific IDs.
    /// Returns the total number of rows successfully deleted.
    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, ids: &[i64]) -> sqlx::Result<u64> {
        if ids.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("DELETE FROM users WHERE id IN ");
        query_builder.push("(");
        let mut separated = query_builder.separated(", ");
        for id in ids { separated.push_bind(id); }
        separated.push_unseparated(")");
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Updates records matching the `idx_name` index.
    /// 
    /// **Warning:** This overwrites all columns (except the index columns) with the values from the current struct.
    pub async fn update_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET email = ?, status = ?, created_at = ? WHERE last_name = ? AND first_name = ?";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.status)
            .bind(self.created_at)
            .bind(&self.last_name)
            .bind(&self.first_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes records matching the `idx_name` index.
    /// 
    /// Returns the number of affected rows.
    pub async fn delete_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE last_name = ? AND first_name = ?";
        let result = sqlx::query(query)
            .bind(last_name)
            .bind(first_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Updates records matching the `idx_email` index.
    /// 
    /// **Warning:** This overwrites all columns (except the index columns) with the values from the current struct.
    pub async fn update_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET first_name = ?, last_name = ?, status = ?, created_at = ? WHERE email = ?";
        let result = sqlx::query(query)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(self.created_at)
            .bind(&self.email)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes records matching the `idx_email` index.
    /// 
    /// Returns the number of affected rows.
    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE email = ?";
        let result = sqlx::query(query)
            .bind(email)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure used for partial updates (Patching) of `users`.
/// 
/// Each field is wrapped in an `Option`. If a field is `None`, it will be completely ignored during the update.
/// If it is `Some(value)`, that column will be updated in the database.
#[derive(Debug, Clone, Default)]
pub struct UsersPatch {
    pub email: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<Option<chrono::NaiveDateTime>>,
}

impl Users {
    /// Updates ONLY the columns that contain data in the `patch` struct.
    /// 
    /// **Performance:** This is the most optimized way to update data.
    /// It dynamically builds the SQL query to only include the changed columns, which saves network bandwidth
    /// and significantly reduces database disk I/O (WAL logging) compared to a full row update.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64, patch: &UsersPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.email {
            has_fields = true;
            separated.push("email = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.first_name {
            has_fields = true;
            separated.push("first_name = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.last_name {
            has_fields = true;
            separated.push("last_name = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.status {
            has_fields = true;
            separated.push("status = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.created_at {
            has_fields = true;
            separated.push("created_at = ");
            separated.push_bind_unseparated(*val);
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(*id);

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

