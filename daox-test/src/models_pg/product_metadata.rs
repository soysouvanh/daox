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
                payload.push_str(&if let Some(v) = &item.raw_data { { let mut s = String::with_capacity(3 + v.len() * 2); s.push_str("\\\\x"); for b in v { use std::fmt::Write; write!(&mut s, "{:02x}", b).ok(); } s } } else { String::new() });
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

