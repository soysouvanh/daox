// Code généré automatiquement par daox. NE PAS MODIFIER.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ActiveUsers {
    pub id: Option<i64>,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl ActiveUsers {
    /// Compte le nombre total de lignes.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM active_users";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM active_users";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Pagination par numéro de page et tri dynamique (Offset/Limit).
    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::Postgres>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM active_users ORDER BY {} LIMIT $1 OFFSET $2", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size as i64).bind(offset as i64).fetch_all(executor).await
    }

}

