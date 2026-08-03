#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoles {
    pub assigned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_name: String,
    pub user_id: i64,
}

#[allow(clippy::all)]
impl UserRoles {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM user_roles";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles ORDER BY \"assigned_at\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["assigned_at", "role_name", "user_id"];
let mut valid_order = String::new();
for (i, part) in order_by.split(',').enumerate() {
let part = part.trim();
let is_desc = part.ends_with(" DESC") || part.ends_with(" desc");
let col = part.trim_end_matches(" ASC").trim_end_matches(" DESC").trim_end_matches(" asc").trim_end_matches(" desc").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
if i > 0 { valid_order.push_str(", "); }
valid_order.push_str(col);
if is_desc { valid_order.push_str(" DESC"); } else { valid_order.push_str(" ASC"); }
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles ORDER BY ");
qb.push(valid_order);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO user_roles (\"assigned_at\", \"role_name\", \"user_id\") VALUES ($1, $2, $3)";
        let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.assigned_at).bind(&self.role_name).bind(&self.user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY user_roles (\"assigned_at\", \"role_name\", \"user_id\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.assigned_at { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.role_name; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&{ let v = &item.user_id; v.to_string() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn list_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles WHERE \"user_id\" = $1 ORDER BY \"assigned_at\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E, user_id: i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles WHERE \"user_id\" = $1 ORDER BY \"assigned_at\" ASC";
sqlx::query_as::<_, Self>(query).bind(user_id).fetch(executor)
}

    pub async fn exists_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE \"user_id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE \"user_id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn get_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles WHERE \"user_id\" = $1 AND \"role_name\" = $2";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(role_name).fetch_optional(executor).await
}

    pub async fn exists_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE \"user_id\" = $1 AND \"role_name\" = $2 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).bind(role_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE \"user_id\" = $1 AND \"role_name\" = $2";
let result = sqlx::query::<sqlx::Postgres>(query).bind(user_id).bind(role_name).execute(executor).await?;
Ok(result.rows_affected())
}

}

