#[allow(clippy::all)]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CompTypesMetadata {
    pub comp_types_id: i32,
    pub f_blob: Option<Vec<u8>>,
    pub f_date: Option<chrono::NaiveDate>,
    pub f_datetime: Option<chrono::DateTime<chrono::Utc>>,
    pub f_json: Option<String>,
    pub f_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub id: i32,
}

#[allow(clippy::all)]
impl CompTypesMetadata {
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E) -> sqlx::Result<u64> {
let query = "SELECT COUNT(*) FROM comp_types_metadata";
let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
Ok(count as u64)
}

    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata ORDER BY `id` ASC";
sqlx::query_as::<_, Self>(query).fetch(executor)
}

    pub async fn get_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<Option<Self>> {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` = ?";
sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
}

    pub async fn exists_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<bool> {
let query = "SELECT 1 FROM comp_types_metadata WHERE `id` = ? LIMIT 1";
let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
Ok(exists.is_some())
}

    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, last_id: i32, limit: u32) -> sqlx::Result<Vec<Self>> {
let query = "SELECT `comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`, `id` FROM comp_types_metadata WHERE `id` > ? ORDER BY `id` ASC LIMIT ?";
sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit as i64).fetch_all(executor).await
}

    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?)";
        let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).execute(executor).await?;
Ok(result.last_insert_rowid() as u64)
}

    /// Inserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 6;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.comp_types_id);
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_timestamp);
            });
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query = "INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(&self.comp_types_id).bind(&self.f_blob).bind(&self.f_date).bind(&self.f_datetime).bind(&self.f_json).bind(&self.f_timestamp).execute(executor).await?;
Ok(result.rows_affected())
}

    /// Upserts a batch of records. 
/// WARNING: To guarantee atomicity across all chunks, you MUST pass an explicit `sqlx::Transaction` as the `executor`.
pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, items: &[Self]) -> sqlx::Result<u64> {
if items.is_empty() { return Ok(0); }
let chunk_size = 32766 / 6;
let mut total_affected = 0;
for chunk in items.chunks(chunk_size.max(1)) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("INSERT INTO comp_types_metadata (`comp_types_id`, `f_blob`, `f_date`, `f_datetime`, `f_json`, `f_timestamp`) ");
qb.push_values(chunk, |mut b, item| {
            b.push_bind(&item.comp_types_id);
            b.push_bind(&item.f_blob);
            b.push_bind(&item.f_date);
            b.push_bind(&item.f_datetime);
            b.push_bind(&item.f_json);
            b.push_bind(&item.f_timestamp);
            });
qb.push(" ON CONFLICT (`id`) DO UPDATE SET `comp_types_id` = EXCLUDED.`comp_types_id`, `f_blob` = EXCLUDED.`f_blob`, `f_date` = EXCLUDED.`f_date`, `f_datetime` = EXCLUDED.`f_datetime`, `f_json` = EXCLUDED.`f_json`, `f_timestamp` = EXCLUDED.`f_timestamp`");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(&self, executor: E) -> sqlx::Result<u64> {
let query_str = "UPDATE comp_types_metadata SET `comp_types_id` = ?, `f_blob` = ?, `f_date` = ?, `f_datetime` = ?, `f_json` = ?, `f_timestamp` = ? WHERE `id` = ?";
let mut query = sqlx::query::<sqlx::Sqlite>(query_str);
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

    pub async fn delete_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32) -> sqlx::Result<u64> {
let query = "DELETE FROM comp_types_metadata WHERE `id` = ?";
let result = sqlx::query::<sqlx::Sqlite>(query).bind(id).execute(executor).await?;
Ok(result.rows_affected())
}

    pub async fn delete_many_by_id<'e>(executor: &mut sqlx::Transaction<'e, sqlx::Sqlite>, ids: &[i32]) -> sqlx::Result<u64> {
if ids.is_empty() { return Ok(0); }
let mut total_affected = 0;
for chunk in ids.chunks(32766) {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("DELETE FROM comp_types_metadata WHERE `id` IN ");
qb.push("(");
let mut sep = qb.separated(", ");
for id in chunk { sep.push_bind(id); }
sep.push_unseparated(")");
let result = qb.build().execute(&mut **executor).await?;
total_affected += result.rows_affected();
}
Ok(total_affected)
}

    pub async fn update_partial_by_id<'e, E: sqlx::Executor<'e, Database = sqlx::Sqlite>>(executor: E, id: i32, patch: &CompTypesMetadataPatch) -> sqlx::Result<u64> {
let mut qb: sqlx::QueryBuilder<sqlx::Sqlite> = sqlx::QueryBuilder::new("UPDATE comp_types_metadata SET ");
let mut has = false;
let mut sep = qb.separated(", ");
        if let Some(val) = &patch.comp_types_id {
has = true;
sep.push("`comp_types_id` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_blob {
has = true;
sep.push("`f_blob` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_date {
has = true;
sep.push("`f_date` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_datetime {
has = true;
sep.push("`f_datetime` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_json {
has = true;
sep.push("`f_json` = ");
sep.push_bind_unseparated(val);
}
        if let Some(val) = &patch.f_timestamp {
has = true;
sep.push("`f_timestamp` = ");
sep.push_bind_unseparated(val);
}
        if !has { return Ok(0); }
        qb.push(" WHERE `id` = ");
qb.push_bind(id);
        let result = qb.build().execute(executor).await?;
Ok(result.rows_affected())
}

}

#[allow(clippy::all)]
#[derive(Debug, Clone, Default)]
pub struct CompTypesMetadataPatch {
    pub comp_types_id: Option<i32>,
    pub f_blob: Option<Option<Vec<u8>>>,
    pub f_date: Option<Option<chrono::NaiveDate>>,
    pub f_datetime: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub f_json: Option<Option<String>>,
    pub f_timestamp: Option<Option<chrono::DateTime<chrono::Utc>>>,
}

