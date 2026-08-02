// Code generated automatically by daox. DO NOT EDIT.
// Pure sqlx layer – zero framework dependency.


#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<i64>,
    pub last_name: Option<String>,
}

#[allow(clippy::all)]
impl ActiveUsers {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM active_users";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"email\", \"first_name\", \"id\", \"last_name\" FROM active_users ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["email", "first_name", "id", "last_name"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"email\", \"first_name\", \"id\", \"last_name\" FROM active_users ORDER BY ");
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
    pub id: Option<i64>,
}

#[allow(clippy::all)]
impl CompTypesActiveView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_active_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"f_date\", \"f_int\", \"f_varchar\", \"id\" FROM comp_types_active_view ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_date", "f_int", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"f_date\", \"f_int\", \"f_varchar\", \"id\" FROM comp_types_active_view ORDER BY ");
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
    pub f_blob: Option<Vec<u8>>,
    pub f_bool: Option<bool>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_decimal: Option<String>,
    pub f_double: Option<f64>,
    pub f_float: Option<f32>,
    pub f_int: Option<i32>,
    pub f_json: Option<String>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub f_varchar: Option<String>,
    pub id: i64,
}

#[allow(clippy::all)]
impl CompTypesMatView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_mat_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\" FROM comp_types_mat_view ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\" FROM comp_types_mat_view ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_mat_view (\"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\") VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)";
        let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.f_blob).bind(&self.f_bool).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_json).bind(&self.f_text).bind(&self.f_timestamp).bind(&self.f_varchar).bind(&self.id).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY comp_types_mat_view (\"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.f_blob { format!("\\\\x{}", v.iter().fold(String::new(), |mut acc, b| { std::fmt::Write::write_fmt(&mut acc, format_args!("{:02x}", b)).ok(); acc })) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_bool { if *v { "true".to_string() } else { "false".to_string() } } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_date { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_datetime { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_decimal { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_double { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_float { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_int { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_json { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_text { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_timestamp { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_varchar { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.id; v.to_string() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i64,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<serde_json::Value>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i64,
}

#[allow(clippy::all)]
impl CompTypesMetadata {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_metadata";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\", \"id\" FROM comp_types_metadata ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\", \"id\" FROM comp_types_metadata WHERE \"id\" = $1";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM comp_types_metadata WHERE \"id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: i64, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\", \"id\" FROM comp_types_metadata WHERE \"id\" > $1 ORDER BY \"id\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (\"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\") VALUES ($1, $2, $3, $4, $5, $6) RETURNING \"id\"::bigint";
        let (id,): (i64,) = sqlx::query_as(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).fetch_one(executor).await?;
Ok(id as u64)
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY comp_types_metadata (\"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&{ let v = &item.comp_types_id; v.to_string() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_blob { format!("\\\\x{}", v.iter().fold(String::new(), |mut acc, b| { std::fmt::Write::write_fmt(&mut acc, format_args!("{:02x}", b)).ok(); acc })) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_date { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_datetime { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_json { format!("\"{}\"", v.to_string().replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_timestamp { format!("\"{}\"", v) } else { String::new() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (\"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\") VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (\"id\") DO UPDATE SET \"comp_types_id\" = EXCLUDED.\"comp_types_id\", \"f_blob\" = EXCLUDED.\"f_blob\", \"f_date\" = EXCLUDED.\"f_date\", \"f_datetime\" = EXCLUDED.\"f_datetime\", \"f_json\" = EXCLUDED.\"f_json\", \"f_timestamp\" = EXCLUDED.\"f_timestamp\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 6;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO comp_types_metadata (\"comp_types_id\", \"f_blob\", \"f_date\", \"f_datetime\", \"f_json\", \"f_timestamp\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.comp_types_id);
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_timestamp);
            });
qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"comp_types_id\" = EXCLUDED.\"comp_types_id\", \"f_blob\" = EXCLUDED.\"f_blob\", \"f_date\" = EXCLUDED.\"f_date\", \"f_datetime\" = EXCLUDED.\"f_datetime\", \"f_json\" = EXCLUDED.\"f_json\", \"f_timestamp\" = EXCLUDED.\"f_timestamp\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE comp_types_metadata SET \"comp_types_id\" = $1, \"f_blob\" = $2, \"f_date\" = $3, \"f_datetime\" = $4, \"f_json\" = $5, \"f_timestamp\" = $6 WHERE \"id\" = $7";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM comp_types_metadata WHERE \"id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[i64]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM comp_types_metadata WHERE \"id\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64, patch: &CompTypesMetadataPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE comp_types_metadata SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.comp_types_id {
has = true;
sep.push("\"comp_types_id\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_blob {
has = true;
sep.push("\"f_blob\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_date {
has = true;
sep.push("\"f_date\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_datetime {
has = true;
sep.push("\"f_datetime\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_json {
has = true;
sep.push("\"f_json\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_timestamp {
has = true;
sep.push("\"f_timestamp\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"id\" = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesMetadataPatch {
    pub comp_types_id: Option<i64>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<serde_json::Value>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}

#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesTable {
    pub f_bool: Option<bool>,
    pub f_decimal: Option<f64>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_text: Option<String>,
    pub f_varchar: Option<String>,
    pub id: i64,
}

#[allow(clippy::all)]
impl CompTypesTable {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_table";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\", \"id\" FROM comp_types_table ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\", \"id\" FROM comp_types_table WHERE \"id\" = $1";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM comp_types_table WHERE \"id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: i64, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\", \"id\" FROM comp_types_table WHERE \"id\" > $1 ORDER BY \"id\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_table (\"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\") VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING \"id\"::bigint";
        let (id,): (i64,) = sqlx::query_as(query).bind(&self.f_bool).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_text).bind(&self.f_varchar).fetch_one(executor).await?;
Ok(id as u64)
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY comp_types_table (\"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.f_bool { if *v { "true".to_string() } else { "false".to_string() } } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_decimal { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_double { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_float { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_int { v.to_string() } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_text { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.f_varchar { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_table (\"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\") VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (\"id\") DO UPDATE SET \"f_bool\" = EXCLUDED.\"f_bool\", \"f_decimal\" = EXCLUDED.\"f_decimal\", \"f_double\" = EXCLUDED.\"f_double\", \"f_float\" = EXCLUDED.\"f_float\", \"f_int\" = EXCLUDED.\"f_int\", \"f_text\" = EXCLUDED.\"f_text\", \"f_varchar\" = EXCLUDED.\"f_varchar\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.f_bool).bind(&self.f_decimal).bind(&self.f_double).bind(&self.f_float).bind(&self.f_int).bind(&self.f_text).bind(&self.f_varchar).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 7;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO comp_types_table (\"f_bool\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_text\", \"f_varchar\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.f_bool);
            b.push_bind(&item.f_decimal);
            b.push_bind(&item.f_double);
            b.push_bind(&item.f_float);
            b.push_bind(&item.f_int);
            b.push_bind(&item.f_text);
            b.push_bind(&item.f_varchar);
            });
qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"f_bool\" = EXCLUDED.\"f_bool\", \"f_decimal\" = EXCLUDED.\"f_decimal\", \"f_double\" = EXCLUDED.\"f_double\", \"f_float\" = EXCLUDED.\"f_float\", \"f_int\" = EXCLUDED.\"f_int\", \"f_text\" = EXCLUDED.\"f_text\", \"f_varchar\" = EXCLUDED.\"f_varchar\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE comp_types_table SET \"f_bool\" = $1, \"f_decimal\" = $2, \"f_double\" = $3, \"f_float\" = $4, \"f_int\" = $5, \"f_text\" = $6, \"f_varchar\" = $7 WHERE \"id\" = $8";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM comp_types_table WHERE \"id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[i64]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM comp_types_table WHERE \"id\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64, patch: &CompTypesTablePatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE comp_types_table SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.f_bool {
has = true;
sep.push("\"f_bool\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_decimal {
has = true;
sep.push("\"f_decimal\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_double {
has = true;
sep.push("\"f_double\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_float {
has = true;
sep.push("\"f_float\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_int {
has = true;
sep.push("\"f_int\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_text {
has = true;
sep.push("\"f_text\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_varchar {
has = true;
sep.push("\"f_varchar\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"id\" = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesTablePatch {
    pub f_bool: Option<Option<bool>>,
    pub f_decimal: Option<Option<f64>>,
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
    pub f_decimal: Option<f64>,
    pub f_double: Option<f64>,
    pub f_float: Option<f64>,
    pub f_int: Option<i32>,
    pub f_json: Option<serde_json::Value>,
    pub f_text: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub f_varchar: Option<String>,
    pub id: Option<i64>,
}

#[allow(clippy::all)]
impl CompTypesView {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_view";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\" FROM comp_types_view ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["f_blob", "f_bool", "f_date", "f_datetime", "f_decimal", "f_double", "f_float", "f_int", "f_json", "f_text", "f_timestamp", "f_varchar", "id"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"f_blob\", \"f_bool\", \"f_date\", \"f_datetime\", \"f_decimal\", \"f_double\", \"f_float\", \"f_int\", \"f_json\", \"f_text\", \"f_timestamp\", \"f_varchar\", \"id\" FROM comp_types_view ORDER BY ");
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM configurations";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"id\", \"match\", \"type\", \"value\" FROM configurations ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"id\", \"match\", \"type\", \"value\" FROM configurations WHERE \"id\" = $1";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM configurations WHERE \"id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"id\", \"match\", \"type\", \"value\" FROM configurations WHERE \"id\" > $1 ORDER BY \"id\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO configurations (\"match\", \"type\", \"value\") VALUES ($1, $2, $3) RETURNING \"id\"::bigint";
        let (id,): (i64,) = sqlx::query_as(query).bind(&self.r#match).bind(&self.r#type).bind(&self.value).fetch_one(executor).await?;
Ok(id as u64)
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY configurations (\"match\", \"type\", \"value\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.r#match { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.r#type; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.value { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO configurations (\"match\", \"type\", \"value\") VALUES ($1, $2, $3) ON CONFLICT (\"id\") DO UPDATE SET \"match\" = EXCLUDED.\"match\", \"type\" = EXCLUDED.\"type\", \"value\" = EXCLUDED.\"value\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.r#match).bind(&self.r#type).bind(&self.value).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO configurations (\"match\", \"type\", \"value\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.r#match);
            b.push_bind(&item.r#type);
            b.push_bind(&item.value);
            });
qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"match\" = EXCLUDED.\"match\", \"type\" = EXCLUDED.\"type\", \"value\" = EXCLUDED.\"value\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE configurations SET \"match\" = $1, \"type\" = $2, \"value\" = $3 WHERE \"id\" = $4";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.r#match);
        query = query.bind(&self.r#type);
        query = query.bind(&self.value);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM configurations WHERE \"id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM configurations WHERE \"id\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i32, patch: &ConfigurationsPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE configurations SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.r#match {
has = true;
sep.push("\"match\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.r#type {
has = true;
sep.push("\"type\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.value {
has = true;
sep.push("\"value\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"id\" = ");
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM currencies";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"code\", \"name\" FROM currencies ORDER BY \"code\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &str) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"code\", \"name\" FROM currencies WHERE \"code\" = $1";
sqlx::query_as::<_, Self>(query).bind(code).fetch_optional(executor).await
}

    pub async fn exists_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &str) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM currencies WHERE \"code\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(code).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: &str, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"code\", \"name\" FROM currencies WHERE \"code\" > $1 ORDER BY \"code\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO currencies (\"code\", \"name\") VALUES ($1, $2)";
        let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.code).bind(&self.name).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY currencies (\"code\", \"name\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&{ let v = &item.code; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&{ let v = &item.name; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO currencies (\"code\", \"name\") VALUES ($1, $2) ON CONFLICT (\"code\") DO UPDATE SET \"name\" = EXCLUDED.\"name\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.code).bind(&self.name).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 2;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO currencies (\"code\", \"name\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.code);
            b.push_bind(&item.name);
            });
qb.push(" ON CONFLICT (\"code\") DO UPDATE SET \"name\" = EXCLUDED.\"name\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE currencies SET \"name\" = $1 WHERE \"code\" = $2";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.name);
        query = query.bind(&self.code);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM currencies WHERE \"code\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(code).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_code<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[String]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM currencies WHERE \"code\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_code<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, code: &str, patch: &CurrenciesPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE currencies SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.name {
has = true;
sep.push("\"name\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"code\" = ");
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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM order_items";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"order_id\", \"product_id\", \"quantity\" FROM order_items ORDER BY \"order_id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
const ALLOWED: &[&str] = &["order_id", "product_id", "quantity"];
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"order_id\", \"product_id\", \"quantity\" FROM order_items ORDER BY ");
qb.push(order_by);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

    pub async fn get_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"order_id\", \"product_id\", \"quantity\" FROM order_items WHERE \"order_id\" = $1 AND \"product_id\" = $2";
sqlx::query_as::<_, Self>(query).bind(order_id).bind(product_id).fetch_optional(executor).await
}

    pub async fn exists_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM order_items WHERE \"order_id\" = $1 AND \"product_id\" = $2 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(order_id).bind(product_id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO order_items (\"order_id\", \"product_id\", \"quantity\") VALUES ($1, $2, $3)";
        let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.order_id).bind(&self.product_id).bind(&self.quantity).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY order_items (\"order_id\", \"product_id\", \"quantity\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&{ let v = &item.order_id; v.to_string() });
                payload.push(',');
                payload.push_str(&{ let v = &item.product_id; v.to_string() });
                payload.push(',');
                payload.push_str(&{ let v = &item.quantity; v.to_string() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO order_items (\"order_id\", \"product_id\", \"quantity\") VALUES ($1, $2, $3) ON CONFLICT (\"order_id\", \"product_id\") DO UPDATE SET \"quantity\" = EXCLUDED.\"quantity\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.order_id).bind(&self.product_id).bind(&self.quantity).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 3;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO order_items (\"order_id\", \"product_id\", \"quantity\") ");
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

    pub async fn update_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE order_items SET \"quantity\" = $1 WHERE \"order_id\" = $2 AND \"product_id\" = $3";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.quantity);
        query = query.bind(&self.order_id);
        query = query.bind(&self.product_id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_id: i64, product_id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM order_items WHERE \"order_id\" = $1 AND \"product_id\" = $2";
let result = sqlx::query::<sqlx::Postgres>(query).bind(order_id).bind(product_id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn update_partial_by_order_id_and_product_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_id: i64, product_id: i64, patch: &OrderItemsPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE order_items SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.quantity {
has = true;
sep.push("\"quantity\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"order_id\" = ");
qb.push_bind(order_id);
        qb.push(" AND \"product_id\" = ");
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
    pub attributes: Option<serde_json::Value>,
    pub category: String,
    pub id: String,
    pub raw_data: Option<Vec<u8>>,
}

#[allow(clippy::all)]
impl ProductMetadata {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM product_metadata";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"attributes\", \"category\", \"id\", \"raw_data\" FROM product_metadata ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &str) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"attributes\", \"category\", \"id\", \"raw_data\" FROM product_metadata WHERE \"id\" = $1";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &str) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM product_metadata WHERE \"id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: &str, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"attributes\", \"category\", \"id\", \"raw_data\" FROM product_metadata WHERE \"id\" > $1 ORDER BY \"id\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO product_metadata (\"attributes\", \"category\", \"id\", \"raw_data\") VALUES ($1, $2, $3, $4)";
        let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.attributes).bind(&self.category).bind(&self.id).bind(&self.raw_data).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY product_metadata (\"attributes\", \"category\", \"id\", \"raw_data\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.attributes { format!("\"{}\"", v.to_string().replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.category; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&{ let v = &item.id; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.raw_data { format!("\\\\x{}", v.iter().fold(String::new(), |mut acc, b| { std::fmt::Write::write_fmt(&mut acc, format_args!("{:02x}", b)).ok(); acc })) } else { String::new() });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO product_metadata (\"attributes\", \"category\", \"id\", \"raw_data\") VALUES ($1, $2, $3, $4) ON CONFLICT (\"id\") DO UPDATE SET \"attributes\" = EXCLUDED.\"attributes\", \"category\" = EXCLUDED.\"category\", \"raw_data\" = EXCLUDED.\"raw_data\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.attributes).bind(&self.category).bind(&self.id).bind(&self.raw_data).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 4;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO product_metadata (\"attributes\", \"category\", \"id\", \"raw_data\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.attributes);
            b.push_bind(&item.category);
            b.push_bind(&item.id);
            b.push_bind(&item.raw_data);
            });
qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"attributes\" = EXCLUDED.\"attributes\", \"category\" = EXCLUDED.\"category\", \"raw_data\" = EXCLUDED.\"raw_data\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE product_metadata SET \"attributes\" = $1, \"category\" = $2, \"raw_data\" = $3 WHERE \"id\" = $4";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.attributes);
        query = query.bind(&self.category);
        query = query.bind(&self.raw_data);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM product_metadata WHERE \"id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[String]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM product_metadata WHERE \"id\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: &str, patch: &ProductMetadataPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE product_metadata SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.attributes {
has = true;
sep.push("\"attributes\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.category {
has = true;
sep.push("\"category\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.raw_data {
has = true;
sep.push("\"raw_data\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"id\" = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct ProductMetadataPatch {
    pub attributes: Option<Option<serde_json::Value>>,
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
for part in order_by.split(',') {
let col = part.trim().trim_end_matches(" ASC").trim_end_matches(" DESC").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"assigned_at\", \"role_name\", \"user_id\" FROM user_roles ORDER BY ");
qb.push(order_by);
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

}

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
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM users";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users WHERE \"id\" = $1";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE \"id\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, last_id: i64, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users WHERE \"id\" > $1 ORDER BY \"id\" ASC LIMIT $2";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (\"created_at\", \"email\", \"first_name\", \"last_name\", \"status\") VALUES ($1, $2, $3, $4, $5) RETURNING \"id\"::bigint";
        let (id,): (i64,) = sqlx::query_as(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).fetch_one(executor).await?;
Ok(id as u64)
}

    /// Inserts a batch of records using Postgres COPY (ultra-fast). 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let mut copy_in = executor.copy_in_raw("COPY users (\"created_at\", \"email\", \"first_name\", \"last_name\", \"status\") FROM STDIN WITH (FORMAT csv)").await?;
for chunk in items.chunks(10000) {
let mut payload = String::with_capacity(chunk.len() * 128);
for item in chunk {
                payload.push_str(&if let Some(v) = &item.created_at { format!("\"{}\"", v) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.email; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&if let Some(v) = &item.first_name { format!("\"{}\"", v.replace("\"", "\"\"")) } else { String::new() });
                payload.push(',');
                payload.push_str(&{ let v = &item.last_name; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push(',');
                payload.push_str(&{ let v = &item.status; format!("\"{}\"", v.replace("\"", "\"\"")) });
                payload.push('\n');
}
copy_in.send(payload.as_bytes()).await?;
}
copy_in.finish().await?;
Ok(items.len() as u64)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO users (\"created_at\", \"email\", \"first_name\", \"last_name\", \"status\") VALUES ($1, $2, $3, $4, $5) ON CONFLICT (\"id\") DO UPDATE SET \"created_at\" = EXCLUDED.\"created_at\", \"email\" = EXCLUDED.\"email\", \"first_name\" = EXCLUDED.\"first_name\", \"last_name\" = EXCLUDED.\"last_name\", \"status\" = EXCLUDED.\"status\"";
let result = sqlx::query::<sqlx::Postgres>(query).bind(&self.created_at).bind(&self.email).bind(&self.first_name).bind(&self.last_name).bind(&self.status).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 65535 / 5;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("INSERT INTO users (\"created_at\", \"email\", \"first_name\", \"last_name\", \"status\") ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.created_at);
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            });
qb.push(" ON CONFLICT (\"id\") DO UPDATE SET \"created_at\" = EXCLUDED.\"created_at\", \"email\" = EXCLUDED.\"email\", \"first_name\" = EXCLUDED.\"first_name\", \"last_name\" = EXCLUDED.\"last_name\", \"status\" = EXCLUDED.\"status\"");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE users SET \"created_at\" = $1, \"email\" = $2, \"first_name\" = $3, \"last_name\" = $4, \"status\" = $5 WHERE \"id\" = $6";
let mut query = sqlx::query::<sqlx::Postgres>(query_str);
        query = query.bind(&self.created_at);
        query = query.bind(&self.email);
        query = query.bind(&self.first_name);
        query = query.bind(&self.last_name);
        query = query.bind(&self.status);
        query = query.bind(&self.id);
let result = query.execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE \"id\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Postgres>, ids: &[i64]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(65535) {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM users WHERE \"id\" IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, id: i64, patch: &UsersPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("UPDATE users SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.created_at {
has = true;
sep.push("\"created_at\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.email {
has = true;
sep.push("\"email\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.first_name {
has = true;
sep.push("\"first_name\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.last_name {
has = true;
sep.push("\"last_name\" = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.status {
has = true;
sep.push("\"status\" = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE \"id\" = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn list_by_first_name_and_last_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, first_name: &String, last_name: &String, limit: i64) -> sqlx::Result<Vec<Self>> {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users WHERE \"first_name\" = $1 AND \"last_name\" = $2 ORDER BY \"id\" ASC LIMIT $3";
sqlx::query_as::<_, Self>(query).bind(first_name).bind(last_name).bind(limit).fetch_all(executor).await
}

    pub fn stream_by_first_name_and_last_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E, first_name: &'e String, last_name: &'e String) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users WHERE \"first_name\" = $1 AND \"last_name\" = $2 ORDER BY \"id\" ASC";
sqlx::query_as::<_, Self>(query).bind(first_name).bind(last_name).fetch(executor)
}

    pub async fn exists_by_first_name_and_last_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, first_name: &String, last_name: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE \"first_name\" = $1 AND \"last_name\" = $2 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(first_name).bind(last_name).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_first_name_and_last_name<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, first_name: &String, last_name: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE \"first_name\" = $1 AND \"last_name\" = $2";
let result = sqlx::query::<sqlx::Postgres>(query).bind(first_name).bind(last_name).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, email: &String) -> sqlx::Result<Option<Self>> {
let query = "SELECT \"created_at\", \"email\", \"first_name\", \"id\", \"last_name\", \"status\" FROM users WHERE \"email\" = $1";
sqlx::query_as::<_, Self>(query).bind(email).fetch_optional(executor).await
}

    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, email: &String) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM users WHERE \"email\" = $1 LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(email).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, email: &String) -> sqlx::Result<u64> {
let query = "DELETE FROM users WHERE \"email\" = $1";
let result = sqlx::query::<sqlx::Postgres>(query).bind(email).execute(executor).await?;
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

