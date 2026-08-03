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
                payload.push_str(&if let Some(v) = &item.f_blob { { let mut s = String::with_capacity(3 + v.len() * 2); s.push_str("\\\\x"); for b in v { use std::fmt::Write; write!(&mut s, "{:02x}", b).ok(); } s } } else { String::new() });
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

