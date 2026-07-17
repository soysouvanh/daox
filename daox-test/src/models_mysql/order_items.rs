// Code généré automatiquement par daox. NE PAS MODIFIER.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct OrderItems {
    pub order_id: i64,
    pub product_id: i64,
    pub quantity: i32,
}

impl OrderItems {
    /// Compte le nombre total de lignes.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM order_items";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM order_items";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Pagination par numéro de page et tri dynamique (Offset/Limit).
    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM order_items ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Récupère une ligne via sa clé primaire.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM order_items WHERE order_id = ? AND product_id = ?";
        sqlx::query_as::<_, Self>(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor).await
    }

    /// Vérifie si une ligne existe (Très léger, évite la RAM).
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM order_items WHERE order_id = ? AND product_id = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(order_id)
            .bind(product_id)
            .fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

}

impl OrderItems {
    /// Insère la ligne en base de données. Retourne l'ID généré (ou 0).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO order_items (order_id, product_id, quantity) VALUES (?, ?, ?)";
        let result = sqlx::query(&query)
            .bind(&self.order_id)
            .bind(&self.product_id)
            .bind(&self.quantity)
            .execute(executor).await?;
        Ok(result.last_insert_id())
    }

    /// Insère de multiples lignes en une seule requête réseau (Batch).
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO order_items (order_id, product_id, quantity) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.order_id);
            b.push_bind(&item.product_id);
            b.push_bind(&item.quantity);
        });
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Insère ou met à jour la ligne si une contrainte d'unicité est violée (Upsert).
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO order_items (order_id, product_id, quantity) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE order_id = VALUES(order_id), product_id = VALUES(product_id), quantity = VALUES(quantity)";
        let result = sqlx::query(&query)
            .bind(&self.order_id)
            .bind(&self.product_id)
            .bind(&self.quantity)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne entière via sa clé primaire.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE order_items SET quantity = ? WHERE order_id = ? AND product_id = ?";
        let result = sqlx::query(&query)
            .bind(&self.quantity)
            .bind(&self.order_id)
            .bind(&self.product_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime la ligne via sa clé primaire.
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64) -> sqlx::Result<u64> {
        let query = "DELETE FROM order_items WHERE order_id = ? AND product_id = ?";
        let result = sqlx::query(&query)
            .bind(order_id)
            .bind(product_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure pour la mise à jour partielle (Patch) de `order_items`.
#[derive(Debug, Clone, Default)]
pub struct OrderItemsPatch {
    pub quantity: Option<i32>,
}

impl OrderItems {
    /// Met à jour uniquement les colonnes renseignées (Patch).
    /// Économise le réseau et les écritures disque de la base de données.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_id: &i64, product_id: &i64, patch: &OrderItemsPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE order_items SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.quantity {
            has_fields = true;
            separated.push("quantity = ");
            separated.push_bind_unseparated(val.clone());
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE order_id = ");
        query_builder.push_bind(order_id.clone());
        query_builder.push(" AND product_id = ");
        query_builder.push_bind(product_id.clone());

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

