pub mod models;

use sqlx::mysql::MySqlPoolOptions;
// On importe le Modèle ET son Patch
use models::users::{Users, UsersPatch}; 

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
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
    println!("\n🔧 Envoi du Patch léger...");
    
    // Scénario REST / API Typique : L'utilisateur n'envoie que 2 modifications.
    let patch = UsersPatch {
        // Colonne NOT NULL : Un simple Option
        status: Some("banned".to_string()), 
        
        // Colonne NULLABLE : Double Option ! 
        // On veut modifier le champ (Some extérieur) en lui donnant la valeur NULL (None intérieur)
        first_name: Some(None),
        
        // La magie de Rust : tout le reste (email, last_name, etc.) est mis à `None`
        // et sera donc IGNORÉ par la requête SQL !
        ..Default::default()
    };

    // Le QueryBuilder de `daox` va générer EXACTEMENT :
    // UPDATE users SET status = ?, first_name = ? WHERE id = ?
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