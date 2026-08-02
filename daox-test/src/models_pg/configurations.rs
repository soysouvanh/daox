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

