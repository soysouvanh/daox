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
let mut valid_order = String::new();
for (i, part) in order_by.split(',').enumerate() {
let part = part.trim();
let is_desc = part.ends_with(" DESC") || part.ends_with(" desc");
let col = part.trim_end_matches(" ASC").trim_end_matches(" DESC").trim_end_matches(" asc").trim_end_matches(" desc").trim();
if !ALLOWED.contains(&col) {
return Err(sqlx::Error::Protocol(format!("Invalid ORDER BY: {}", col).into()));
}
if i > 0 { valid_order.push_str(", "); }
valid_order.push_str(col);
if is_desc { valid_order.push_str(" DESC"); } else { valid_order.push_str(" ASC"); }
}
let page_size = page_size.clamp(1, 10000);
let offset = page.saturating_sub(1) * page_size;
let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("SELECT \"email\", \"first_name\", \"id\", \"last_name\" FROM active_users ORDER BY ");
qb.push(valid_order);
qb.push(" LIMIT ");
qb.push_bind(page_size as i64);
qb.push(" OFFSET ");
qb.push_bind(offset as i64);
qb.build_query_as::<Self>().fetch_all(executor).await
}

}

