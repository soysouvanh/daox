pub mod models_mysql;
pub mod models_pg;

use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Lecture du paramètre (ex: `cargo run -- pg` ou `cargo run -- mysql`)
    let args: Vec<String> = env::args().collect();
    let dialect = args.get(1).map(|s| s.as_str()).unwrap_or("mysql");

    println!("🚀 Démarrage des tests Daox...");

    if dialect == "postgres" || dialect == "pg" {
        run_postgres().await?;
    } else {
        run_mysql().await?;
    }

    Ok(())
}

async fn run_mysql() -> Result<(), sqlx::Error> {
    use models_mysql::users::{Users, UsersPatch}; 
    
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:root@localhost:3307/daox_test")
        .await?;

    println!("✅ Connecté à MariaDB !");

    // 1. On nettoie et on crée un utilisateur pour le test
    Users::delete_by_email(&pool, &"patch_test@daox.dev".to_string()).await?;
    
    let new_user = Users {
        id: 0,
        email: "patch_test@daox.dev".to_string(),
        first_name: Some("Initial".to_string()),
        last_name: "Original".to_string(),
        status: "active".to_string(),
        created_at: None,
    };
    let user_id = new_user.insert(&pool).await?;
    println!("👤 Utilisateur créé (ID: {})", user_id);

    // ==========================================
    // 2. LE TEST : L'UPDATE PARTIEL (PATCH)
    // ==========================================
    println!("\n🔧 Envoi du Patch léger (MariaDB)...");
    
    // Scénario REST / API Typique : L'utilisateur n'envoie que 2 modifications.
    let patch = UsersPatch {
        status: Some("banned".to_string()), 
        first_name: Some(None),
        ..Default::default()
    };

    let rows_affected = Users::update_partial_by_pk(&pool, &(user_id as i64), &patch).await?;
    println!("✨ Patch réussi ! {} ligne(s) affectée(s).", rows_affected);

    // 3. Vérification en base de données
    let user_verif = Users::get_by_pk(&pool, &(user_id as i64)).await?.unwrap();
    println!("\n🔍 État en base de données après Patch :");
    println!("   - Email (inchangé) : {}", user_verif.email);
    println!("   - Nom (inchangé) : {}", user_verif.last_name);
    println!("   - Prénom (mis à NULL) : {:?}", user_verif.first_name);
    println!("   - Statut (modifié) : {}", user_verif.status);

    Ok(())
}

async fn run_postgres() -> Result<(), sqlx::Error> {
    use models_pg::users::{Users, UsersPatch}; 
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost:5433/daox_test")
        .await?;

    println!("✅ Connecté à PostgreSQL !");

    // 1. On nettoie et on crée un utilisateur pour le test
    Users::delete_by_email(&pool, &"patch_test@daox.dev".to_string()).await?;
    
    let new_user = Users {
        id: 0, // En Postgres, cet ID sera ignoré par notre INSERT généré si la colonne est auto-incrémentée (BIGSERIAL/IDENTITY)
        email: "patch_test@daox.dev".to_string(),
        first_name: Some("Initial".to_string()),
        last_name: "Original".to_string(),
        status: "active".to_string(),
        created_at: None,
    };
    let user_id = new_user.insert(&pool).await?;
    println!("👤 Utilisateur créé (ID: {})", user_id);

    // ==========================================
    // 2. LE TEST : L'UPDATE PARTIEL (PATCH)
    // ==========================================
    println!("\n🔧 Envoi du Patch léger (PostgreSQL)...");
    
    let patch = UsersPatch {
        status: Some("banned".to_string()), 
        first_name: Some(None),
        ..Default::default()
    };

    let rows_affected = Users::update_partial_by_pk(&pool, &(user_id as i64), &patch).await?;
    println!("✨ Patch réussi ! {} ligne(s) affectée(s).", rows_affected);

    // 3. Vérification en base de données
    let user_verif = Users::get_by_pk(&pool, &(user_id as i64)).await?.unwrap();
    println!("\n🔍 État en base de données après Patch :");
    println!("   - Email (inchangé) : {}", user_verif.email);
    println!("   - Nom (inchangé) : {}", user_verif.last_name);
    println!("   - Prénom (mis à NULL) : {:?}", user_verif.first_name);
    println!("   - Statut (modifié) : {}", user_verif.status);

    Ok(())
}