// Code généré automatiquement par daox. NE PAS MODIFIER.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Users {
    pub id: i64,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: String,
    pub status: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl Users {
    /// Compte le nombre total de lignes.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM users";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM users";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Pagination par numéro de page et tri dynamique (Offset/Limit).
    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM users ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Récupère une ligne via sa clé primaire.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM users WHERE id = ?";
        sqlx::query_as::<_, Self>(query).bind(id).fetch_optional(executor).await
    }

    /// Vérifie si une ligne existe (Très léger, évite la RAM).
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE id = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(id).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Pagination par curseur (Performance absolue O(1) sur le B-Tree).
    pub async fn list_by_cursor<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_id: &i64, limit: u32) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM users WHERE id > ? ORDER BY id ASC LIMIT ?";
        sqlx::query_as::<_, Self>(query).bind(last_id).bind(limit).fetch_all(executor).await
    }

    /// Vérifie l'existence via l'index `idx_email`.
    pub async fn exists_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE email = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(email).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Récupère une ligne (unique) via l'index `idx_email`.
    pub async fn get_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM users WHERE email = ?";
        sqlx::query_as::<_, Self>(query).bind(email).fetch_optional(executor).await
    }

    /// Vérifie l'existence via l'index `idx_name`.
    pub async fn exists_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM users WHERE last_name = ? AND first_name = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(last_name).bind(first_name).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Liste des lignes via l'index `idx_name`.
    pub async fn list_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM users WHERE last_name = ? AND first_name = ?";
        sqlx::query_as::<_, Self>(query).bind(last_name).bind(first_name).fetch_all(executor).await
    }

    /// Flux asynchrone (Stream) zéro-allocation sur l'index `idx_name`.
    pub fn stream_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, last_name: &String, first_name: &Option<String>) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM users WHERE last_name = ? AND first_name = ?";
        sqlx::query_as::<_, Self>(query).bind(last_name.clone()).bind(first_name.clone()).fetch(executor)
    }

}

impl Users {
    /// Insère la ligne en base de données. Retourne l'ID généré (ou 0).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO users (email, first_name, last_name, status, created_at) VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(&self.created_at)
            .execute(executor).await?;
        Ok(result.last_insert_id())
    }

    /// Insère de multiples lignes en une seule requête réseau (Batch).
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO users (email, first_name, last_name, status, created_at) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.email);
            b.push_bind(&item.first_name);
            b.push_bind(&item.last_name);
            b.push_bind(&item.status);
            b.push_bind(&item.created_at);
        });
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Insère ou met à jour la ligne si une contrainte d'unicité est violée (Upsert).
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO users (email, first_name, last_name, status, created_at) VALUES (?, ?, ?, ?, ?) ON DUPLICATE KEY UPDATE email = VALUES(email), first_name = VALUES(first_name), last_name = VALUES(last_name), status = VALUES(status), created_at = VALUES(created_at)";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(&self.created_at)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne entière via sa clé primaire.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET email = ?, first_name = ?, last_name = ?, status = ?, created_at = ? WHERE id = ?";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(&self.created_at)
            .bind(&self.id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime la ligne via sa clé primaire.
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE id = ?";
        let result = sqlx::query(query).bind(id).execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime de multiples lignes via leurs clés primaires (Batch).
    pub async fn delete_many_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, ids: &[i64]) -> sqlx::Result<u64> {
        if ids.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("DELETE FROM users WHERE id IN ");
        query_builder.push("(");
        let mut separated = query_builder.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne via l'index `idx_email`.
    pub async fn update_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET first_name = ?, last_name = ?, status = ?, created_at = ? WHERE email = ?";
        let result = sqlx::query(query)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.status)
            .bind(&self.created_at)
            .bind(&self.email)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime des lignes via l'index `idx_email`.
    pub async fn delete_by_email<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, email: &String) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE email = ?";
        let result = sqlx::query(query)
            .bind(email)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne via l'index `idx_name`.
    pub async fn update_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE users SET email = ?, status = ?, created_at = ? WHERE last_name = ? AND first_name = ?";
        let result = sqlx::query(query)
            .bind(&self.email)
            .bind(&self.status)
            .bind(&self.created_at)
            .bind(&self.last_name)
            .bind(&self.first_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime des lignes via l'index `idx_name`.
    pub async fn delete_by_last_name_and_first_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, last_name: &String, first_name: &Option<String>) -> sqlx::Result<u64> {
        let query = "DELETE FROM users WHERE last_name = ? AND first_name = ?";
        let result = sqlx::query(query)
            .bind(last_name)
            .bind(first_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure pour la mise à jour partielle (Patch) de `users`.
#[derive(Debug, Clone, Default)]
pub struct UsersPatch {
    pub email: Option<String>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<Option<chrono::NaiveDateTime>>,
}

impl Users {
    /// Met à jour uniquement les colonnes renseignées (Patch).
    /// Économise le réseau et les écritures disque de la base de données.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, id: &i64, patch: &UsersPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.email {
            has_fields = true;
            separated.push("email = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.first_name {
            has_fields = true;
            separated.push("first_name = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.last_name {
            has_fields = true;
            separated.push("last_name = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.status {
            has_fields = true;
            separated.push("status = ");
            separated.push_bind_unseparated(val.clone());
        }
        if let Some(val) = &patch.created_at {
            has_fields = true;
            separated.push("created_at = ");
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

