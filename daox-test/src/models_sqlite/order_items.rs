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
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("SELECT `order_id`, `product_id`, `quantity` FROM order_items ORDER BY ");
qb.push(valid_order);
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

