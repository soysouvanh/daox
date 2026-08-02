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

