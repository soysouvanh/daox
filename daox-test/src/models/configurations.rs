// Code généré automatiquement par daox. NE PAS MODIFIER.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Configurations {
    pub id: i32,
    pub r#type: String,
    pub r#match: Option<String>,
    pub value: Option<String>,
}

impl Configurations {
    /// Compte le nombre total de lignes.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM configurations";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM configurations";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Pagination par numéro de page et tri dynamique (Offset/Limit).
    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM configurations ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Récupère une ligne via sa clé primaire.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i32) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM configurations WHERE id = ?";
        sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
    }

    /// Vérifie si une ligne existe (Très léger, évite la RAM).
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i32) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM configurations WHERE id = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Pagination par curseur (Performance absolue O(1) sur le B-Tree).
    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_id: &i32, limit: u32) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM configurations WHERE id > ? ORDER BY id ASC LIMIT ?";
        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit).fetch_all(executor).await
    }

}

impl Configurations {
    /// Insère la ligne en base de données. Retourne l'ID généré (ou 0).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO configurations (type, match, value) VALUES (?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .execute(executor).await?;
        Ok(result.last_insert_id())
    }

    /// Insère de multiples lignes en une seule requête réseau (Batch).
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO configurations (type, match, value) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.r#type);
            b.push_bind(&item.r#match);
            b.push_bind(&item.value);
        });
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Insère ou met à jour la ligne si une contrainte d'unicité est violée (Upsert).
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO configurations (type, match, value) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE type = VALUES(type), match = VALUES(match), value = VALUES(value)";
        let result = sqlx::query(query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne entière via sa clé primaire.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE configurations SET type = ?, match = ?, value = ? WHERE id = ?";
        let result = sqlx::query(query)
            .bind(&self.r#type)
            .bind(&self.r#match)
            .bind(&self.value)
            .bind(&self.id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime la ligne via sa clé primaire.
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i32) -> sqlx::Result<u64> {
        let query = "DELETE FROM configurations WHERE id = ?";
        let result = sqlx::query(query).bind(id).execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime de multiples lignes via leurs clés primaires (Batch).
    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, ids: &[i32]) -> sqlx::Result<u64> {
        if ids.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("DELETE FROM configurations WHERE id IN ");
        query_builder.push("(");
        let mut separated = query_builder.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure pour la mise à jour partielle (Patch) de `configurations`.
#[derive(Debug, Clone, Default)]
pub struct ConfigurationsPatch {
    pub r#type: Option<String>,
    pub r#match: Option<Option<String>>,
    pub value: Option<Option<String>>,
}

impl Configurations {
    /// Met à jour uniquement les colonnes renseignées (Patch).
    /// Économise le réseau et les écritures disque de la base de données.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i32, patch: &ConfigurationsPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE configurations SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.r#type {
            has_fields = true;
            separated.push("type = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.r#match {
            has_fields = true;
            separated.push("match = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.value {
            has_fields = true;
            separated.push("value = ");
            separated.push_bind_unseparated(val.clone());
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(id.clone());

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

