// Code generated automatically by daox. DO NOT EDIT.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderItems {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

impl OrderItems {
    /// Counts the total number of rows in the table.
    /// 
    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM order_items";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Creates a zero-allocation Asynchronous Stream over the entire table.
    /// 
    /// **Performance:** This is the absolute best way to process millions of rows.
    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),
    /// the Stream fetches and yields rows one by one directly from the database connection.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM order_items";
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
        let query = format!("SELECT * FROM order_items ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Retrieves a single record using its Primary Key.
    /// 
    /// Returns `Some(Self)` if the record exists, or `None` if it does not.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM order_items WHERE order_id = ? AND product_id = ?";
        sqlx::query_as::<_, Self>(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor).await
    }

    /// Checks if a record exists using its Primary Key.
    /// 
    /// **Performance:** This uses a `SELECT 1 ... LIMIT 1` query. It is infinitely faster 
    /// and lighter than `get_by_pk` when you only need to check for existence, because it avoids 
    /// transferring and deserializing the full row data.
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM order_items WHERE order_id = ? AND product_id = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

}

impl OrderItems {
    /// Inserts the current record into the database.
    /// 
    /// **Best Practice:** Use this method when you want to create a brand new row.
    /// If the table has an auto-increment primary key, the database will generate the ID automatically.
    /// 
    /// Returns the generated ID (or 0 if the table doesn't have an auto-increment ID).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO order_items (order_id, product_id, quantity) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(self.order_id)
            .bind(self.product_id)
            .bind(self.quantity)
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
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO order_items (order_id, product_id, quantity) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(item.order_id);
            b.push_bind(item.product_id);
            b.push_bind(item.quantity);
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
        let query = "INSERT INTO order_items (order_id, product_id, quantity) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE order_id = VALUES(order_id), product_id = VALUES(product_id), quantity = VALUES(quantity)";
        let result = sqlx::query(query)
            .bind(self.order_id)
            .bind(self.product_id)
            .bind(self.quantity)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Overwrites the entire record in the database using its Primary Key.
    /// 
    /// **Warning:** This will update ALL columns in the row with the values in the current struct.
    /// If you only want to update one or two specific columns, use `update_partial_by_pk` instead 
    /// to save network bandwidth and database disk I/O.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE order_items SET quantity = ? WHERE order_id = ? AND product_id = ?";
        let result = sqlx::query(query)
            .bind(self.quantity)
            .bind(self.order_id)
            .bind(self.product_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Deletes the specific record from the database using its Primary Key.
    /// 
    /// Returns the number of affected rows (1 if deleted, 0 if it didn't exist).
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<u64> {
        let query = "DELETE FROM order_items WHERE order_id = ? AND product_id = ?";
        let result = sqlx::query(query)
            .bind(order_id)
            .bind(product_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure used for partial updates (Patching) of `order_items`.
/// 
/// Each field is wrapped in an `Option`. If a field is `None`, it will be completely ignored during the update.
/// If it is `Some(value)`, that column will be updated in the database.
#[derive(Debug, Clone, Default)]
pub struct OrderItemsPatch {
    pub quantity: Option<i32>,
}

impl OrderItems {
    /// Updates ONLY the columns that contain data in the `patch` struct.
    /// 
    /// **Performance:** This is the most optimized way to update data.
    /// It dynamically builds the SQL query to only include the changed columns, which saves network bandwidth
    /// and significantly reduces database disk I/O (WAL logging) compared to a full row update.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64, patch: &OrderItemsPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE order_items SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.quantity {
            has_fields = true;
            separated.push("quantity = ");
            separated.push_bind_unseparated(*val);
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE order_id = ");
        query_builder.push_bind(*order_id);
        query_builder.push(" AND product_id = ");
        query_builder.push_bind(*product_id);

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

