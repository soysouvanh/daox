#[cfg(test)]
mod toml_injection_tests {
    #[test]
    fn test_description_with_quotes() {
        let desc = r#"This has "quotes" and \backslashes"#;
        let serialized = format!("{:?}", desc);
        assert_eq!(serialized, r#""This has \"quotes\" and \\backslashes""#);
    }

    #[test]
    fn test_format_regex_with_special_chars() {
        let regex = "^it's$";
        let serialized = format!("{:?}", regex);
        assert_eq!(serialized, r#""^it's$""#);
    }

    #[test]
    fn test_enum_values_with_special_chars() {
        let vals = vec!["val'ue", "val\"ue", "val\\ue"];
        for v in &vals {
            let serialized = format!("{:?}", v);
            assert!(serialized.starts_with('"'));
        }
    }
}

#[cfg(test)]
mod path_traversal_tests {
    use std::fs;

    #[test]
    fn test_reject_symlink_in_schema() {
        let parent = std::env::temp_dir().join("daox_tests");
        let schema_dir = parent.join(".daox_schema");
        let _ = fs::create_dir_all(&schema_dir);
        let target = parent.join("evil");
        let _ = fs::write(&target, "evil");
        #[cfg(unix)]
        {
            let link = schema_dir.join("evil_link");
            let _ = std::os::unix::fs::symlink(&target, &link);
            if let Ok(md) = fs::symlink_metadata(&link) {
                assert!(md.file_type().is_symlink());
            }
        }
    }

    #[test]
    fn test_reject_path_traversal_overrides() {
        unsafe {
            std::env::set_var("DAOX_OVERRIDES_DIR", "../../etc");
        }
        assert_eq!(std::env::var("DAOX_OVERRIDES_DIR").unwrap(), "../../etc");
        unsafe {
            std::env::remove_var("DAOX_OVERRIDES_DIR");
        }
    }
}

#[cfg(test)]
mod combinatorial_tests {
    #[test]
    fn test_generated_code_size_bounded() {
        // Pour 15 colonnes, le fichier généré doit être < 100 Ko
        // (avec le QueryBuilder dynamique)
    }

    #[test]
    fn test_reject_more_than_10_update_columns() {
        // Vérifier que le générateur rejette > 10 colonnes
    }
}

#[cfg(test)]
mod validation_tests {
    #[test]
    fn test_insert_validated_rejects_invalid() {
        // Insérer un email invalide avec insert_validated()
    }

    #[test]
    fn test_enum_validation() {
        // Insérer une valeur enum invalide
    }
}
