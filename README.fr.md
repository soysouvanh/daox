[English](README.md) | [Français](README.fr.md)

# Écosystème du Workspace Daox

[![Crates.io](https://img.shields.io/crates/v/daox.svg)](https://crates.io/crates/daox)
[![Documentation](https://docs.rs/daox/badge.svg)](https://docs.rs/daox)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Daox** est un moteur de pointe qui lit vos bases de données et génère automatiquement le code Rust exact, sûr et ultra-rapide nécessaire pour communiquer avec elles.

Ce dépôt est l'espace de travail central (workspace) contenant tout ce qui concerne Daox (son code source principal, ses tests de validations et sa documentation).

---

## 📖 Débutants : Par où commencer ?

Si votre but est **d'apprendre à utiliser Daox pour votre propre projet**, ne restez pas sur cette page !
Rendez-vous exactement ici pour lire le guide pas-à-pas spécialement conçu pour les débutants absolus :

👉 **[Lisez le guide pas-à-pas complet ici](./daox/README.fr.md)** 👈

---

## 🏗️ Structure du Workspace

Pour les utilisateurs avancés et contributeurs, voici l'organisation technique de l'écosystème Daox :

```text
daox-workspace/
├── docker-compose.yml      # Déploiement automatisé des BDD (PostgreSQL, MySQL)
├── init_*.sql              # Schémas de base auto-chargés au lancement de Docker
├── daox/                   # Le moteur principal du framework (publié sur crates.io)
│   ├── src/                # Builders de dialecte, parser AOP, logique des macros
│   └── assets/             # Diagrammes SVG de l'architecture
│
└── daox-test/              # Suite exhaustive de tests d'intégration
    ├── src/models/         # Code 100% auto-généré par Daox (via le build.rs)
    └── src/main.rs         # Les workflows exécutés (Streams, Pagination, Upserts, etc.)
```

---

## 🧪 Comment exécuter localement les Tests d'Intégration (Guide Pédagogique)

Si vous avez cloné ce dépôt pour vérifier que Daox fonctionne, ou pour contribuer au code, suivez exactement ces étapes sans aucune ambiguïté :

### Prérequis indispensables

Vous devez absolument posséder :

1. **Rust** installé sur votre machine.
2. **Docker** et son greffon **Docker Compose** installés (permettant de lancer nos bases de test en toute sécurité sans polluer votre architecture locale).

### Étape 1 : Lancer les bases de test

Daox a besoin de bases de données vivantes pour tester sa logique fondamentale. Un fichier `docker-compose.yml` est pré-configuré. Ouvrez votre terminal à la racine `daox-workspace` et saisissez :

```bash
docker compose up -d
```

_Que se passe-t-il ?_ Docker va télécharger et exécuter silencieusement une base PostgreSQL (port 5433) et MySQL/MariaDB (port 3307). Ces ports spécifiques garantissent de n'entrer en conflit avec aucun de vos projets locaux actuels.

### Étape 2 : Exécuter la suite de test

Naviguez dans le dossier de test.
Ici, le script `build.rs` ira se connecter aux bases fraîchement lancées, générera les modèles Rust correspondants, puis le fichier `main.rs` validera en profondeur chaque fonctionnalité (Upserts, Patching, Streaming...).

```bash
# 1. Entrez dans le répertoire de test
cd daox-test

# 2. Tester le moteur SQLite (Totalement isolé en local, Docker non requis)
DAOX_TEST_SQLITE=1 cargo run

# 3. Tester le moteur MySQL / MariaDB (Connexion active au conteneur Docker)
cargo run

# 4. Tester le moteur PostgreSQL (Connexion active au conteneur Docker)
DAOX_TEST_PG=1 cargo run
```

Si le terminal n'affiche aucun crash et que les tests vont au bout, vous venez de vérifier et de prouver mathématiquement le comportement parfait du framework sur votre machine !

---

## 👨‍💻 Auteur

**Vincent SOYSOUVANH**  
_[Skillwaker](https://app.skillwaker.com)_

- [LinkedIn](https://www.linkedin.com/in/vincentsoysouvanh/)
- [Twitter / X](https://x.com/skillwaker)

## 📄 Licence

Ce projet est gracieusement mis à disposition sous licence MIT.
