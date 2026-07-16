use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Demander à Cargo de recompiler si ce fichier de configuration change
    println!("cargo:rerun-if-changed=build.rs");
    // Relancer la compilation si le code source de la librairie daox est modifié
    println!("cargo:rerun-if-changed=../daox/src");
    
    // URL de la base de données (via le service docker compose, port 3307)
    let db_url = "mysql://root:root@localhost:3307/daox_test";
    let out_dir = "src/models";

    // On crée le dossier s'il n'existe pas
    if !Path::new(out_dir).exists() {
        fs::create_dir_all(out_dir).unwrap();
    }

    // On lance le générateur
    let generator = daox::DaoxGenerator::new(db_url, out_dir);
    generator.generate().await.expect("Erreur lors de la génération DAO");
}