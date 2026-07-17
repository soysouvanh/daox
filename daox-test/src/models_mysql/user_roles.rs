// Code généré automatiquement par daox. NE PAS MODIFIER.

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRoles {
    pub user_id: i64,
    pub role_name: String,
    pub assigned_at: Option<chrono::NaiveDateTime>,
}

impl UserRoles {
    /// Compte le nombre total de lignes.
    pub async fn count<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E) -> sqlx::Result<u64> {
        let query = "SELECT COUNT(*) FROM user_roles";
        let (count,): (i64,) = sqlx::query_as(query).fetch_one(executor).await?;
        Ok(count as u64)
    }

    /// Flux asynchrone (Stream) zéro-allocation sur toute la table.
    pub fn stream_all<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM user_roles";
        sqlx::query_as::<_, Self>(query).fetch(executor)
    }

    /// Pagination par numéro de page et tri dynamique (Offset/Limit).
    /// Attention: order_by n'est pas bindé, à valider en amont contre l'injection SQL.
    pub async fn list_paginated<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, order_by: &str, page: u32, page_size: u32) -> sqlx::Result<Vec<Self>> {
        let offset = page.saturating_sub(1) * page_size;
        let query = format!("SELECT * FROM user_roles ORDER BY {} LIMIT ? OFFSET ?", order_by);
        sqlx::query_as::<_, Self>(&query).bind(page_size).bind(offset).fetch_all(executor).await
    }

    /// Récupère une ligne via sa clé primaire.
    pub async fn get_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM user_roles WHERE user_id = ? AND role_name = ?";
        sqlx::query_as::<_, Self>(query)
            .bind(user_id)
            .bind(role_name)
            .fetch_optional(executor).await
    }

    /// Vérifie si une ligne existe (Très léger, évite la RAM).
    pub async fn exists_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM user_roles WHERE user_id = ? AND role_name = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query)
            .bind(user_id)
            .bind(role_name)
            .fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Vérifie l'existence via l'index `idx_user_id`.
    pub async fn exists_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM user_roles WHERE user_id = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Liste des lignes via l'index `idx_user_id`.
    pub async fn list_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64) -> sqlx::Result<Vec<Self>> {
        let query = "SELECT * FROM user_roles WHERE user_id = ?";
        sqlx::query_as::<_, Self>(query).bind(user_id).fetch_all(executor).await
    }

    /// Flux asynchrone (Stream) zéro-allocation sur l'index `idx_user_id`.
    pub fn stream_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql> + 'e>(executor: E, user_id: &i64) -> impl futures::Stream<Item = sqlx::Result<Self>> + 'e {
        let query = "SELECT * FROM user_roles WHERE user_id = ?";
        sqlx::query_as::<_, Self>(query).bind(user_id.clone()).fetch(executor)
    }

    /// Vérifie l'existence via l'index `idx_user_role`.
    pub async fn exists_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<bool> {
        let query = "SELECT 1 FROM user_roles WHERE user_id = ? AND role_name = ? LIMIT 1";
        let exists: Option<(i32,)> = sqlx::query_as(query).bind(user_id).bind(role_name).fetch_optional(executor).await?;
        Ok(exists.is_some())
    }

    /// Récupère une ligne (unique) via l'index `idx_user_role`.
    pub async fn get_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<Option<Self>> {
        let query = "SELECT * FROM user_roles WHERE user_id = ? AND role_name = ?";
        sqlx::query_as::<_, Self>(query).bind(user_id).bind(role_name).fetch_optional(executor).await
    }

}

impl UserRoles {
    /// Insère la ligne en base de données. Retourne l'ID généré (ou 0).
    pub async fn insert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO user_roles (user_id, role_name, assigned_at) VALUES (?, ?, ?)";
        let result = sqlx::query(&query)
            .bind(&self.user_id)
            .bind(&self.role_name)
            .bind(&self.assigned_at)
            .execute(executor).await?;
        Ok(result.last_insert_id())
    }

    /// Insère de multiples lignes en une seule requête réseau (Batch).
    pub async fn insert_batch<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, items: &[Self]) -> sqlx::Result<u64> {
        if items.is_empty() { return Ok(0); }
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("INSERT INTO user_roles (user_id, role_name, assigned_at) ");
        query_builder.push_values(items, |mut b, item| {
            b.push_bind(&item.user_id);
            b.push_bind(&item.role_name);
            b.push_bind(&item.assigned_at);
        });
        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Insère ou met à jour la ligne si une contrainte d'unicité est violée (Upsert).
    pub async fn upsert<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "INSERT INTO user_roles (user_id, role_name, assigned_at) VALUES (?, ?, ?) ON DUPLICATE KEY UPDATE user_id = VALUES(user_id), role_name = VALUES(role_name), assigned_at = VALUES(assigned_at)";
        let result = sqlx::query(&query)
            .bind(&self.user_id)
            .bind(&self.role_name)
            .bind(&self.assigned_at)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Met à jour la ligne entière via sa clé primaire.
    pub async fn update_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE user_roles SET assigned_at = ? WHERE user_id = ? AND role_name = ?";
        let result = sqlx::query(&query)
            .bind(&self.assigned_at)
            .bind(&self.user_id)
            .bind(&self.role_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    /// Supprime la ligne via sa clé primaire.
    pub async fn delete_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<u64> {
        let query = "DELETE FROM user_roles WHERE user_id = ? AND role_name = ?";
        let result = sqlx::query(&query)
            .bind(user_id)
            .bind(role_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE user_roles SET assigned_at = ? WHERE user_id = ?";
        let result = sqlx::query(&query)
            .bind(&self.assigned_at)
            .bind(&self.user_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_user_id<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64) -> sqlx::Result<u64> {
        let query = "DELETE FROM user_roles WHERE user_id = ?";
        let result = sqlx::query(&query)
            .bind(user_id)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn update_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(&self, executor: E) -> sqlx::Result<u64> {
        let query = "UPDATE user_roles SET assigned_at = ? WHERE user_id = ? AND role_name = ?";
        let result = sqlx::query(&query)
            .bind(&self.assigned_at)
            .bind(&self.user_id)
            .bind(&self.role_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

    pub async fn delete_by_user_id_and_role_name<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String) -> sqlx::Result<u64> {
        let query = "DELETE FROM user_roles WHERE user_id = ? AND role_name = ?";
        let result = sqlx::query(&query)
            .bind(user_id)
            .bind(role_name)
            .execute(executor).await?;
        Ok(result.rows_affected())
    }

}

/// Structure pour la mise à jour partielle (Patch) de `user_roles`.
#[derive(Debug, Clone, Default)]
pub struct UserRolesPatch {
    pub assigned_at: Option<Option<chrono::NaiveDateTime>>,
}

impl UserRoles {
    /// Met à jour uniquement les colonnes renseignées (Patch).
    /// Économise le réseau et les écritures disque de la base de données.
    pub async fn update_partial_by_pk<'e, E: sqlx::Executor<'e, Database = sqlx::MySql>>(executor: E, user_id: &i64, role_name: &String, patch: &UserRolesPatch) -> sqlx::Result<u64> {
        let mut query_builder: sqlx::QueryBuilder<sqlx::MySql> = sqlx::QueryBuilder::new("UPDATE user_roles SET ");
        let mut has_fields = false;
        let mut separated = query_builder.separated(", ");

        if let Some(val) = &patch.assigned_at {
            has_fields = true;
            separated.push("assigned_at = ");
            separated.push_bind_unseparated(val.clone());
        }

        if !has_fields {
            // Si le patch est vide, on économise un aller-retour réseau
            return Ok(0);
        }

        query_builder.push(" WHERE user_id = ");
        query_builder.push_bind(user_id.clone());
        query_builder.push(" AND role_name = ");
        query_builder.push_bind(role_name.clone());

        let result = query_builder.build().execute(executor).await?;
        Ok(result.rows_affected())
    }
}

