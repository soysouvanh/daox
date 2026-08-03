#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoles {
    pub assigned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_name: String,
    pub user_id: i64,
}

#[allow(clippy::all)]
impl UserRoles {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM user_roles";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles ORDER BY `role_name` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
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
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles ORDER BY ");
qb.push(valid_order);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn get_by_role_name_and_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, role_name: &str, user_id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `role_name` = ? AND `user_id` = ?";
sqlx::query_as::<_, Self>(query).bind(role_name).bind(user_id).fetch_optional(executor).await
}

    pub async fn exists_by_role_name_and_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, role_name: &str, user_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE `role_name` = ? AND `user_id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(role_name).bind(user_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) VALUES (?, ?, ?)";
        let result = sqlx::query::<sqlx::MySql>(query).bind(&self.assigned_at).bind(&self.role_name).bind(&self.user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::MySql>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.assigned_at);
            b.push_bind(&item.role_name);
            b.push_bind(&item.user_id);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE `assigned_at` = VALUES(`assigned_at`)";
let result = sqlx::query::<sqlx::MySql>(query).bind(&self.assigned_at).bind(&self.role_name).bind(&self.user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::MySql>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.assigned_at);
            b.push_bind(&item.role_name);
            b.push_bind(&item.user_id);
            });
qb.push(" ON DUPLICATE KEY UPDATE `assigned_at` = VALUES(`assigned_at`)");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_role_name_and_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE user_roles SET `assigned_at` = ? WHERE `role_name` = ? AND `user_id` = ?";
let mut query = sqlx::query::<sqlx::MySql>(query_str);
        query = query.bind(&self.assigned_at);
        query = query.bind(&self.role_name);
        query = query.bind(&self.user_id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_role_name_and_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, role_name: &String, user_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE `role_name` = ? AND `user_id` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(role_name).bind(user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn update_partial_by_role_name_and_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, role_name: &str, user_id: i64, patch: &UserRolesPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE user_roles SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.assigned_at {
has = true;
sep.push("`assigned_at` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `role_name` = ");
qb.push_bind(role_name);
        qb.push(" AND `user_id` = ");
qb.push_bind(user_id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn list_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? ORDER BY `role_name` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, user_id: i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? ORDER BY `role_name` ASC";
sqlx::query_as::<_, Self>(query).bind(user_id).fetch(executor)
}

    pub async fn exists_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE `user_id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE `user_id` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn get_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? AND `role_name` = ?";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(role_name).fetch_optional(executor).await
}

    pub async fn exists_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE `user_id` = ? AND `role_name` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).bind(role_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE `user_id` = ? AND `role_name` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(user_id).bind(role_name).execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct UserRolesPatch {
    pub assigned_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
}

