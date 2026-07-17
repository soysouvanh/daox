use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Demander à Cargo de recompiler si ce fichier de configuration change
    println!("cargo:rerun-if-changed=build.rs");
    // Relancer la compilation si le code source de la librairie daox est modifié
    println!("cargo:rerun-if-changed=../daox/src");
    
    // URL de la base de données (via le service docker compose)
    let mysql_url = "mysql://root:root@localhost:3307/daox_test";
    let pg_url = "postgres://root:root@localhost:5433/daox_test";

    let mysql_out = "src/models_mysql";
    let pg_out = "src/models_pg";

    // On crée les dossiers s'ils n'existent pas
    if !Path::new(mysql_out).exists() { fs::create_dir_all(mysql_out).unwrap(); }
    if !Path::new(pg_out).exists() { fs::create_dir_all(pg_out).unwrap(); }

    // On lance le générateur pour MySQL
    let generator_mysql = daox::DaoxGenerator::new(mysql_url, mysql_out);
    generator_mysql.generate().await.expect("Erreur lors de la génération DAO MySQL");

    // On lance le générateur pour Postgres
    let generator_pg = daox::DaoxGenerator::new(pg_url, pg_out);
    generator_pg.generate().await.expect("Erreur lors de la génération DAO Postgres");
}