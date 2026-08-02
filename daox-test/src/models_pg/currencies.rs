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

