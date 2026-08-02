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

