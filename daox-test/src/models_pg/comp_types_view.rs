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

