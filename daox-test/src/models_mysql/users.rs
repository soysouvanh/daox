#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: i64,
    pub last_name: String,
    pub status: String,
}

#[allow(clippy::all)]
impl Users {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM users";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_id: i64, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::MySql>(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).execute(executor).await?;
Ok(result.last_insert_id())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::MySql>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 5;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.created_at);
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)";
let result = sqlx::query::<sqlx::MySql>(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::MySql>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 5;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.created_at);
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            });
qb.push(" ON DUPLICATE KEY UPDATE `created_at` = VALUES(`created_at`), `email` = VALUES(`email`), `first_name` = VALUES(`first_name`), `last_name` = VALUES(`last_name`), `status` = VALUES(`status`)");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE users SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::MySql>(query_str);
        query = query.bind(&self.created_at);
        query = query.bind(&self.email);
        query = query.bind(&self.first_name);
        query = query.bind(&self.last_name);
        query = query.bind(&self.status);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `id` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::MySql>, ids: &[i64]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("DELETE FROM users WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: i64, patch: &UsersPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE users SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.created_at {
has = true;
sep.push("`created_at` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.email {
has = true;
sep.push("`email` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.first_name {
has = true;
sep.push("`first_name` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.last_name {
has = true;
sep.push("`last_name` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.status {
has = true;
sep.push("`status` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `email` = ?";
sqlx::query_as::<_, Self>(query).bind(email).fetch_optional(executor).await
}

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `email` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(email).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `email` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(email).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn list_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &String, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, last_name: &'e String, first_name: &'e String) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).fetch(executor)
}

    pub async fn exists_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `last_name` = ? AND `first_name` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(last_name).bind(first_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `last_name` = ? AND `first_name` = ?";
let result = sqlx::query::<sqlx::MySql>(query).bind(last_name).bind(first_name).execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct UsersPatch {
    pub created_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub email: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<String>,
    pub status: Option<String>,
}

