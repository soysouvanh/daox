// Code generated automatically by daox. DO NOT EDIT.
// Pure sqlx layer – zero framework dependency.


#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<i32>,
    pub last_name: Option<String>,
}

#[allow(clippy::all)]
impl ActiveUsers {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM active_users";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `email`, `first_name`, `id`, `last_name` FROM active_users ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["email", "first_name", "id", "last_name"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `email`, `first_name`, `id`, `last_name` FROM active_users ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesActiveView {
    pub f_date: Option<chrono::NaiveDate>,
    pub f_int: Option<i32>,
    pub f_varchar: Option<String>,
    pub id: Option<i32>,
}

#[allow(clippy::all)]
impl CompTypesActiveView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_active_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `f_date`, `f_int`, `f_varchar`, `id` FROM comp_types_active_view ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_date", "f_int", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `f_date`, `f_int`, `f_varchar`, `id` FROM comp_types_active_view ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMatView {
    pub f_blob: Option<String>,
    pub f_bool: Option<String>,
    pub f_date: Option<String>,
    pub f_datetime: Option<String>,
    pub f_decimal: Option<String>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_json: Option<String>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<String>,
    pub f_varchar: Option<String>,
    pub id: Option<i32>,
}

#[allow(clippy::all)]
impl CompTypesMatView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_mat_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_mat_view ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_mat_view ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_mat_view (`f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id`) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.f_blob).bind(&self.f_bool).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_json).bind(&self.f_text).bind(&self.f_timestamp).bind(&self.f_varchar).bind(&self.id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 13;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_mat_view (`f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_bool);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_decimal);
            b.push_bind(&item.f_double);
            b.push_bind(&item.f_float);
            b.push_bind(&item.f_int);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_text);
            b.push_bind(&item.f_timestamp);
            b.push_bind(&item.f_varchar);
            b.push_bind(&item.id);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i32,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i32,
}

#[allow(clippy::all)]
impl CompTypesMetadata {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_metadata";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM comp_types_metadata WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).execute(executor).await?;
Ok(result.last_insert_rowid() as u64)
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 6;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.comp_types_id);
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_timestamp);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 6;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.comp_types_id);
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_timestamp);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE comp_types_metadata SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.comp_types_id);
        query = query.bind(&self.f_blob);
        query = query.bind(&self.f_date);
        query = query.bind(&self.f_datetime);
        query = query.bind(&self.f_json);
        query = query.bind(&self.f_timestamp);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM comp_types_metadata WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM comp_types_metadata WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32, patch: &CompTypesMetadataPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE comp_types_metadata SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.comp_types_id {
has = true;
sep.push("`comp_types_id` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_blob {
has = true;
sep.push("`f_blob` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_date {
has = true;
sep.push("`f_date` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_datetime {
has = true;
sep.push("`f_datetime` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_json {
has = true;
sep.push("`f_json` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_timestamp {
has = true;
sep.push("`f_timestamp` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesMetadataPatch {
    pub comp_types_id: Option<i32>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<String>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesTable {
    pub f_bool: Option<bool>,
    pub f_decimal: Option<String>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_text: Option<String>,
    pub f_varchar: Option<String>,
    pub id: i32,
}

#[allow(clippy::all)]
impl CompTypesTable {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_table";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM comp_types_table ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM comp_types_table WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM comp_types_table WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`, `id` FROM comp_types_table WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_table (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) VALUES (?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.f_bool).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_text).bind(&self.f_varchar).execute(executor).await?;
Ok(result.last_insert_rowid() as u64)
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 7;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_table (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) ");
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

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_table (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) VALUES (?, ?, ?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `f_bool` = EXCLUDED.`f_bool`, `f_decimal` = EXCLUDED.`f_decimal`, `f_double` = EXCLUDED.`f_double`, `f_float` = EXCLUDED.`f_float`, `f_int` = EXCLUDED.`f_int`, `f_text` = EXCLUDED.`f_text`, `f_varchar` = EXCLUDED.`f_varchar`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.f_bool).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_text).bind(&self.f_varchar).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 7;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_table (`f_bool`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_text`, `f_varchar`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.f_bool);
            b.push_bind(&item.f_decimal);
            b.push_bind(&item.f_double);
            b.push_bind(&item.f_float);
            b.push_bind(&item.f_int);
            b.push_bind(&item.f_text);
            b.push_bind(&item.f_varchar);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `f_bool` = EXCLUDED.`f_bool`, `f_decimal` = EXCLUDED.`f_decimal`, `f_double` = EXCLUDED.`f_double`, `f_float` = EXCLUDED.`f_float`, `f_int` = EXCLUDED.`f_int`, `f_text` = EXCLUDED.`f_text`, `f_varchar` = EXCLUDED.`f_varchar`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE comp_types_table SET `f_bool` = ?, `f_decimal` = ?, `f_double` = ?, `f_float` = ?, `f_int` = ?, `f_text` = ?, `f_varchar` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM comp_types_table WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM comp_types_table WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32, patch: &CompTypesTablePatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE comp_types_table SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.f_bool {
has = true;
sep.push("`f_bool` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_decimal {
has = true;
sep.push("`f_decimal` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_double {
has = true;
sep.push("`f_double` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_float {
has = true;
sep.push("`f_float` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_int {
has = true;
sep.push("`f_int` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_text {
has = true;
sep.push("`f_text` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_varchar {
has = true;
sep.push("`f_varchar` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesTablePatch {
    pub f_bool: Option<Option<bool>>,
    pub f_decimal: Option<Option<String>>,
    pub f_double: Option<Option<f64>>,
    pub f_float: Option<Option<f64>>,
    pub f_int: Option<Option<i32>>,
    pub f_text: Option<Option<String>>,
    pub f_varchar: Option<Option<String>>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesView {
    pub f_blob: Option<Vec<u8>>,
    pub f_bool: Option<bool>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_decimal: Option<String>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_json: Option<String>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub f_varchar: Option<String>,
    pub id: Option<i32>,
}

#[allow(clippy::all)]
impl CompTypesView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_view ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `f_blob`, `f_bool`, `f_date`, `f_datetime`, `f_decimal`, `f_double`, `f_float`, `f_int`, `f_json`, `f_text`, `f_timestamp`, `f_varchar`, `id` FROM comp_types_view ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Configurations {
    pub id: i32,
    pub r#match: Option<String>,
    pub r#type: String,
    pub value: Option<String>,
}

#[allow(clippy::all)]
impl Configurations {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM configurations";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `id`, `match`, `type`, `value` FROM configurations ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT `id`, `match`, `type`, `value` FROM configurations WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM configurations WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `id`, `match`, `type`, `value` FROM configurations WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO configurations (`match`, `type`, `value`) VALUES (?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.r#match).bind(&self.r#type).bind(&self.value).execute(executor).await?;
Ok(result.last_insert_rowid() as u64)
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO configurations (`match`, `type`, `value`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.r#match);
            b.push_bind(&item.r#type);
            b.push_bind(&item.value);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO configurations (`match`, `type`, `value`) VALUES (?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `match` = EXCLUDED.`match`, `type` = EXCLUDED.`type`, `value` = EXCLUDED.`value`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.r#match).bind(&self.r#type).bind(&self.value).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO configurations (`match`, `type`, `value`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.r#match);
            b.push_bind(&item.r#type);
            b.push_bind(&item.value);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `match` = EXCLUDED.`match`, `type` = EXCLUDED.`type`, `value` = EXCLUDED.`value`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE configurations SET `match` = ?, `type` = ?, `value` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.r#match);
        query = query.bind(&self.r#type);
        query = query.bind(&self.value);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM configurations WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM configurations WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32, patch: &ConfigurationsPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE configurations SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.r#match {
has = true;
sep.push("`match` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.r#type {
has = true;
sep.push("`type` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.value {
has = true;
sep.push("`value` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct ConfigurationsPatch {
    pub r#match: Option<Option<String>>,
    pub r#type: Option<String>,
    pub value: Option<Option<String>>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Currencies {
    pub code: String,
    pub name: String,
}

#[allow(clippy::all)]
impl Currencies {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM currencies";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `code`, `name` FROM currencies ORDER BY `code` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, code: &str) -> sqlx::Result<Option<Self>> {
let query = "SELECT `code`, `name` FROM currencies WHERE `code` = ?";
sqlx::query_as::<_, Self>(query).bind(code).fetch_optional(executor).await
}

    pub async fn exists_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, code: &str) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM currencies WHERE `code` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(code).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: &str, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `code`, `name` FROM currencies WHERE `code` > ? ORDER BY `code` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO currencies (`code`, `name`) VALUES (?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.code).bind(&self.name).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 2;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO currencies (`code`, `name`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.code);
            b.push_bind(&item.name);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO currencies (`code`, `name`) VALUES (?, ?) ON CONFLICT (`code`) DO UPDATE SET `name` = EXCLUDED.`name`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.code).bind(&self.name).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 2;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO currencies (`code`, `name`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.code);
            b.push_bind(&item.name);
            });
qb.push(" ON CONFLICT (`code`) DO UPDATE SET `name` = EXCLUDED.`name`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE currencies SET `name` = ? WHERE `code` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.name);
        query = query.bind(&self.code);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, code: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM currencies WHERE `code` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(code).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_code<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[String]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM currencies WHERE `code` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, code: &str, patch: &CurrenciesPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE currencies SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.name {
has = true;
sep.push("`name` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `code` = ");
qb.push_bind(code);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CurrenciesPatch {
    pub name: Option<String>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderItems {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

#[allow(clippy::all)]
impl OrderItems {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM order_items";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `order_id`, `product_id`, `quantity` FROM order_items ORDER BY `order_id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["order_id", "product_id", "quantity"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `order_id`, `product_id`, `quantity` FROM order_items ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn get_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT `order_id`, `product_id`, `quantity` FROM order_items WHERE `order_id` = ? AND `product_id` = ?";
sqlx::query_as::<_, Self>(query).bind(order_id).bind(product_id).fetch_optional(executor).await
}

    pub async fn exists_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM order_items WHERE `order_id` = ? AND `product_id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(order_id).bind(product_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO order_items (`order_id`, `product_id`, `quantity`) VALUES (?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.order_id).bind(&self.product_id).bind(&self.quantity).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO order_items (`order_id`, `product_id`, `quantity`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.order_id);
            b.push_bind(&item.product_id);
            b.push_bind(&item.quantity);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO order_items (`order_id`, `product_id`, `quantity`) VALUES (?, ?, ?) ON CONFLICT (`order_id`, `product_id`) DO UPDATE SET `quantity` = EXCLUDED.`quantity`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.order_id).bind(&self.product_id).bind(&self.quantity).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO order_items (`order_id`, `product_id`, `quantity`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.order_id);
            b.push_bind(&item.product_id);
            b.push_bind(&item.quantity);
            });
qb.push(" ON CONFLICT (`order_id`, `product_id`) DO UPDATE SET `quantity` = EXCLUDED.`quantity`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE order_items SET `quantity` = ? WHERE `order_id` = ? AND `product_id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.quantity);
        query = query.bind(&self.order_id);
        query = query.bind(&self.product_id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM order_items WHERE `order_id` = ? AND `product_id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(order_id).bind(product_id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn update_partial_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_id: i64, product_id: i64, patch: &OrderItemsPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE order_items SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.quantity {
has = true;
sep.push("`quantity` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `order_id` = ");
qb.push_bind(order_id);
        qb.push(" AND `product_id` = ");
qb.push_bind(product_id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct OrderItemsPatch {
    pub quantity: Option<i32>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ProductMetadata {
    pub attributes: Option<String>,
    pub category: String,
    pub id: String,
    pub raw_data: Option<Vec<u8>>,
}

#[allow(clippy::all)]
impl ProductMetadata {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM product_metadata";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `attributes`, `category`, `id`, `raw_data` FROM product_metadata ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: &str) -> sqlx::Result<Option<Self>> {
let query = "SELECT `attributes`, `category`, `id`, `raw_data` FROM product_metadata WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: &str) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM product_metadata WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: &str, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `attributes`, `category`, `id`, `raw_data` FROM product_metadata WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO product_metadata (`attributes`, `category`, `id`, `raw_data`) VALUES (?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.attributes).bind(&self.category).bind(&self.id).bind(&self.raw_data).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 4;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO product_metadata (`attributes`, `category`, `id`, `raw_data`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.attributes);
            b.push_bind(&item.category);
            b.push_bind(&item.id);
            b.push_bind(&item.raw_data);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO product_metadata (`attributes`, `category`, `id`, `raw_data`) VALUES (?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `attributes` = EXCLUDED.`attributes`, `category` = EXCLUDED.`category`, `raw_data` = EXCLUDED.`raw_data`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.attributes).bind(&self.category).bind(&self.id).bind(&self.raw_data).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 4;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO product_metadata (`attributes`, `category`, `id`, `raw_data`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.attributes);
            b.push_bind(&item.category);
            b.push_bind(&item.id);
            b.push_bind(&item.raw_data);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `attributes` = EXCLUDED.`attributes`, `category` = EXCLUDED.`category`, `raw_data` = EXCLUDED.`raw_data`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE product_metadata SET `attributes` = ?, `category` = ?, `raw_data` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.attributes);
        query = query.bind(&self.category);
        query = query.bind(&self.raw_data);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM product_metadata WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[String]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM product_metadata WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: &str, patch: &ProductMetadataPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE product_metadata SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.attributes {
has = true;
sep.push("`attributes` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.category {
has = true;
sep.push("`category` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.raw_data {
has = true;
sep.push("`raw_data` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct ProductMetadataPatch {
    pub attributes: Option<Option<String>>,
    pub category: Option<String>,
    pub raw_data: Option<Option<Vec<u8>>>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoles {
    pub assigned_at: Option<chrono::DateTime<chrono::Utc>>,
    pub role_name: String,
    pub user_id: i64,
}

#[allow(clippy::all)]
impl UserRoles {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM user_roles";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles ORDER BY `assigned_at` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["assigned_at", "role_name", "user_id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) VALUES (?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.assigned_at).bind(&self.role_name).bind(&self.user_id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO user_roles (`assigned_at`, `role_name`, `user_id`) ");
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

    pub async fn get_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? AND `role_name` = ?";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(role_name).fetch_optional(executor).await
}

    pub async fn exists_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE `user_id` = ? AND `role_name` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).bind(role_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64, role_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE `user_id` = ? AND `role_name` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(user_id).bind(role_name).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn list_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? ORDER BY `assigned_at` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(user_id).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E, user_id: i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `assigned_at`, `role_name`, `user_id` FROM user_roles WHERE `user_id` = ? ORDER BY `assigned_at` ASC";
sqlx::query_as::<_, Self>(query).bind(user_id).fetch(executor)
}

    pub async fn exists_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM user_roles WHERE `user_id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, user_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM user_roles WHERE `user_id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(user_id).execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub email: String,
    pub first_name: Option<String>,
    pub id: i32,
    pub last_name: String,
    pub status: String,
}

#[allow(clippy::all)]
impl Users {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM users";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).execute(executor).await?;
Ok(result.last_insert_rowid() as u64)
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 5;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) ");
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

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) VALUES (?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `created_at` = EXCLUDED.`created_at`, `email` = EXCLUDED.`email`, `first_name` = EXCLUDED.`first_name`, `last_name` = EXCLUDED.`last_name`, `status` = EXCLUDED.`status`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 5;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO users (`created_at`, `email`, `first_name`, `last_name`, `status`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.created_at);
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `created_at` = EXCLUDED.`created_at`, `email` = EXCLUDED.`email`, `first_name` = EXCLUDED.`first_name`, `last_name` = EXCLUDED.`last_name`, `status` = EXCLUDED.`status`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE users SET `created_at` = ?, `email` = ?, `first_name` = ?, `last_name` = ?, `status` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
        query = query.bind(&self.created_at);
        query = query.bind(&self.email);
        query = query.bind(&self.first_name);
        query = query.bind(&self.last_name);
        query = query.bind(&self.status);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM users WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32, patch: &UsersPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE users SET ");
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

    pub async fn list_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_name: &String, first_name: &String, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E, last_name: &'e String, first_name: &'e String) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `last_name` = ? AND `first_name` = ? ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).fetch(executor)
}

    pub async fn exists_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_name: &String, first_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `last_name` = ? AND `first_name` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(last_name).bind(first_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_name: &String, first_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `last_name` = ? AND `first_name` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(last_name).bind(first_name).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, email: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT `created_at`, `email`, `first_name`, `id`, `last_name`, `status` FROM users WHERE `email` = ?";
sqlx::query_as::<_, Self>(query).bind(email).fetch_optional(executor).await
}

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, email: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE `email` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(email).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, email: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE `email` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(email).execute(executor).await?;
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

