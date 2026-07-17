// Code generated automatically by daox. DO NOT EDIT.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub id: Option<i64>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl ActiveUsers {
    /// Counts the total number of rows in the table.
    /// 
    /// **Note:** On large tables, `COUNT(*)` can be slow. Use it thoughtfully.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM active_users";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Creates a zero-allocation Asynchronous Stream over the entire table.
    /// 
    /// **Performance:** This is the absolute best way to process millions of rows.
    /// Instead of loading all rows into RAM (which would cause out-of-memory crashes),
    /// the Stream fetches and yields rows one by one directly from the database connection.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM active_users";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Classic Offset/Limit pagination with dynamic sorting.
    /// 
    /// **SECURITY WARNING:** The `order_by` parameter is NOT bound via prepared statements 
    /// (SQL does not allow binding column names). You MUST strictly whitelist the user input 
    /// before passing it here to prevent SQL Injection!
    /// 
    /// **Performance:** Offset pagination becomes very slow on deep pages. Consider `list_by_cursor` instead.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM active_users ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

}

