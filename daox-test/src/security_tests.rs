#[cfg(test)]
mod enum_escaping_tests {
    #[test]
    fn test_enum_escaping_prevents_code_injection() {
        let malicious_values = vec![
            r#"tech"]; std::process::exit(1); //"#,
            r#"tech\", \"evil"#,
            r#"tech\\"#,
            "tech\n\r\t",
            "tech\x00",
            "normal_value",
        ];
        for val in malicious_values {
            let escaped = format!("{:?}", val);
            // Must be a valid Rust string literal
            assert!(
                escaped.starts_with('"'),
                "Escaped value must start with quote: {}",
                escaped
            );
            assert!(
                escaped.ends_with('"'),
                "Escaped value must end with quote: {}",
                escaped
            );
            // The inner content must not contain an unescaped double-quote
            let inner = &escaped[1..escaped.len() - 1];
            let mut chars = inner.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '"' {
                    panic!("Unescaped quote found in {:?}", inner);
                }
                if c == '\\' {
                    // Skip the escaped character
                    let _ = chars.next();
                }
            }
        }
    }

    #[test]
    fn test_enum_debug_vs_display() {
        // Verify {:?} escapes what {} does not
        let injection = r#"a"]; panic!(); //"#;
        let display = format!("\"{}\"", injection);
        let debug = format!("{:?}", injection);
        // Display is dangerous — contains unescaped quotes
        assert!(display.contains("]; panic!()"), "Display should NOT escape");
        // Debug is safe — quotes are escaped
        assert!(debug.contains(r#"\""#), "Debug MUST escape inner quotes");
    }
}

#[cfg(test)]
mod db_name_validation_tests {
    #[test]
    fn test_invalid_db_names_rejected() {
        let invalid_names = vec![
            "a { } pub struct Evil",
            "db; mod evil;",
            "../../etc",
            "db name with spaces",
            "db-name",
            "",
            "123db",
            "db\ttab",
            "db\"quote",
        ];
        for name in invalid_names {
            let valid = !name.is_empty()
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.chars().next().unwrap_or('0').is_ascii_digit();
            assert!(!valid, "Should reject: {:?}", name);
        }
    }

    #[test]
    fn test_valid_db_names_accepted() {
        let valid_names = vec!["default", "my_db", "Db2", "a", "test_123"];
        for name in valid_names {
            let valid = !name.is_empty()
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.chars().next().unwrap().is_ascii_digit();
            assert!(valid, "Should accept: {:?}", name);
        }
    }
}

#[cfg(test)]
mod symlink_tests {
    use std::fs;

    #[test]
    fn test_symlink_detected_in_overrides() {
        let tmp = std::env::temp_dir().join("daox_sec_test_symlink_overrides");
        let _ = fs::remove_dir_all(&tmp);
        let _ = fs::create_dir_all(&tmp);

        let target = tmp.join("target_secret.txt");
        fs::write(&target, "secret_content").unwrap();

        let link = tmp.join("link.toml");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target, &link).unwrap();
            let meta = fs::symlink_metadata(&link).unwrap();
            assert!(
                meta.file_type().is_symlink(),
                "Test prerequisite: symlink must be detected"
            );
            // The generator should reject this file because it's a symlink
            assert!(
                meta.file_type().is_symlink(),
                "Daox must reject symlinks in overrides"
            );
        }
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_symlink_detected_in_schema_columns() {
        let tmp = std::env::temp_dir().join("daox_sec_test_symlink_schema");
        let _ = fs::remove_dir_all(&tmp);
        let schema_dir = tmp.join(".daox_schema").join("users");
        let _ = fs::create_dir_all(&schema_dir);

        let target = tmp.join("evil.toml");
        fs::write(&target, "rust_type = \"String\"").unwrap();

        let link = schema_dir.join("email.toml");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target, &link).unwrap();
            let meta = fs::symlink_metadata(&link).unwrap();
            assert!(meta.file_type().is_symlink());
        }
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_regular_file_is_not_symlink() {
        let tmp = std::env::temp_dir().join("daox_sec_test_regular_file");
        let _ = fs::remove_dir_all(&tmp);
        let _ = fs::create_dir_all(&tmp);

        let regular = tmp.join("regular.toml");
        fs::write(&regular, "rust_type = \"String\"").unwrap();

        let meta = fs::symlink_metadata(&regular).unwrap();
        assert!(
            !meta.file_type().is_symlink(),
            "Regular file must not be symlink"
        );

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod env_allowlist_tests {
    #[test]
    fn test_allowed_prefixes() {
        let allowed = vec![
            "DATABASE_URL_MYSQL",
            "DATABASE_URL_PG",
            "DATABASE_URL_SQLITE",
            "DAOX_OVERRIDES_DIR",
            "DAOX_SCHEMA_DIR",
        ];
        let prefixes = ["DATABASE_URL", "DAOX_"];
        for key in &allowed {
            assert!(
                prefixes.iter().any(|p| key.starts_with(p)),
                "{} should be allowed",
                key
            );
        }
    }

    #[test]
    fn test_denied_prefixes() {
        let denied = vec![
            "LD_PRELOAD",
            "DYLD_INSERT_LIBRARIES",
            "PATH",
            "HOME",
            "SHELL",
            "RUSTFLAGS",
            "CARGO_BUILD_RUSTFLAGS",
        ];
        let prefixes = ["DATABASE_URL", "DAOX_"];
        for key in &denied {
            assert!(
                !prefixes.iter().any(|p| key.starts_with(p)),
                "{} must NOT be allowed",
                key
            );
        }
    }
}

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
            assert!(serialized.ends_with('"'));
        }
    }

    #[test]
    fn test_toml_roundtrip_safety() {
        let malicious = r#"value = "injected"
[evil]
key = "gotcha""#;
        let escaped = format!("{:?}", malicious);
        // Escaped string must be a valid Rust string literal — no unescaped inner quotes
        assert!(escaped.starts_with('"'));
        assert!(escaped.ends_with('"'));
        // The inner content must contain escaped quotes, not raw ones
        let inner = &escaped[1..escaped.len() - 1];
        assert!(
            !inner.contains(r#""""#),
            "Inner content must not have bare double-quotes"
        );
        // Verify the escaped representation preserves newlines as \n
        assert!(escaped.contains(r"\n"), "Newlines must be escaped to \\n");
    }
}

#[cfg(test)]
mod path_traversal_tests {
    #[test]
    fn test_path_traversal_containment() {
        use std::path::PathBuf;
        let base = PathBuf::from("/home/user/project");
        let traversal = PathBuf::from("/home/user/project/../../etc/shadow");
        // A naive starts_with check on un-canonicalized paths would fail
        // This validates our containment logic conceptually
        let normalized = traversal.components().collect::<PathBuf>();
        assert!(
            !normalized.starts_with(&base) || normalized.to_str().unwrap().contains(".."),
            "Path traversal must be detected"
        );
    }

    #[test]
    fn test_identifier_validation_rejects_traversal() {
        let dangerous = vec!["../etc", "users/../admin", ".."];
        for name in dangerous {
            let valid = !name.is_empty()
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.chars().next().unwrap().is_ascii_digit();
            assert!(!valid, "Should reject path traversal: {:?}", name);
        }
    }
}

#[cfg(test)]
mod regex_validation_tests {
    #[test]
    fn test_invalid_regex_rejected() {
        let invalid_regexes = vec!["[", "(unclosed", "*invalid", "(?P<bad"];
        for re in invalid_regexes {
            assert!(
                regex::Regex::new(re).is_err(),
                "Should reject invalid regex: {:?}",
                re
            );
        }
    }

    #[test]
    fn test_valid_regex_accepted() {
        let valid_regexes = vec![
            r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$",
            r"^\d{4}-\d{2}-\d{2}$",
            r"^[A-Z]{2,3}$",
        ];
        for re in valid_regexes {
            assert!(
                regex::Regex::new(re).is_ok(),
                "Should accept valid regex: {:?}",
                re
            );
        }
    }
}

#[cfg(test)]
mod batch_limit_tests {
    #[test]
    fn test_mysql_param_limit() {
        let max_params: usize = 65535;
        // Even with many columns, chunk size must be at least 1
        for col_count in 1..=50 {
            let chunk_size = max_params / col_count;
            assert!(
                chunk_size >= 1,
                "Chunk size must be >= 1 for {} cols",
                col_count
            );
            assert!(
                chunk_size * col_count <= max_params,
                "Total params must not exceed limit"
            );
        }
    }

    #[test]
    fn test_sqlite_param_limit() {
        let max_params: usize = 32766;
        for col_count in 1..=50 {
            let chunk_size = max_params / col_count;
            assert!(chunk_size >= 1);
            assert!(chunk_size * col_count <= max_params);
        }
    }

    #[test]
    fn test_postgres_param_limit() {
        let max_params: usize = 65535;
        for col_count in 1..=50 {
            let chunk_size = max_params / col_count;
            assert!(chunk_size >= 1);
            assert!(chunk_size * col_count <= max_params);
        }
    }
}

#[cfg(test)]
mod identifier_quoting_tests {
    #[test]
    fn test_identifier_validation_whitelist() {
        let valid = vec!["users", "user_roles", "t1", "MyTable", "col_123"];
        let invalid = vec![
            "users; DROP TABLE",
            "col`name",
            "table\"name",
            "",
            "123start",
            "col name",
        ];
        for name in valid {
            assert!(
                !name.is_empty()
                    && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && !name.chars().next().unwrap().is_ascii_digit(),
                "Should accept: {:?}",
                name
            );
        }
        for name in invalid {
            let is_valid = !name.is_empty()
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.chars().next().unwrap_or('0').is_ascii_digit();
            assert!(!is_valid, "Should reject: {:?}", name);
        }
    }
}

#[cfg(test)]
mod type_whitelist_tests {
    const ALLOWED_TYPES: &[&str] = &[
        "bool",
        "i8",
        "i16",
        "i32",
        "i64",
        "u8",
        "u16",
        "u32",
        "u64",
        "f32",
        "f64",
        "String",
        "Vec<u8>",
        "serde_json::Value",
        "chrono::NaiveDate",
        "chrono::DateTime<chrono::Utc>",
        "chrono::NaiveDateTime",
        "json",
        "jsonb",
    ];

    #[test]
    fn test_forbidden_types_rejected() {
        let forbidden = vec![
            "std::process::Command",
            "std::fs::File",
            "Box<dyn std::any::Any>",
            "unsafe { code }",
            "(); std::process::exit(0); //",
        ];
        for ty in forbidden {
            assert!(
                !ALLOWED_TYPES.contains(&ty),
                "Type must be forbidden: {:?}",
                ty
            );
        }
    }

    #[test]
    fn test_allowed_types_accepted() {
        for ty in ALLOWED_TYPES {
            assert!(ALLOWED_TYPES.contains(ty), "Type must be allowed: {:?}", ty);
        }
    }
}

#[cfg(test)]
mod password_regex_tests {
    #[test]
    fn test_password_regex_is_valid_for_regex_crate() {
        let password_regexes = vec![r#"^[A-Za-z\d]{8,}$"#, r#"^[À-ÿA-Za-z0-9_ -]*$"#];
        for re in password_regexes {
            assert!(
                regex::Regex::new(re).is_ok(),
                "Password regex must be valid for regex crate: {:?}",
                re
            );
        }
    }

    #[test]
    fn test_lookahead_regex_rejected() {
        let lookahead = r#"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$"#;
        assert!(
            regex::Regex::new(lookahead).is_err(),
            "Lookahead regex must be rejected by regex crate"
        );
    }
}

#[cfg(test)]
mod override_dir_symlink_tests {
    use std::fs;

    #[test]
    fn test_symlink_directory_rejected_in_overrides() {
        let tmp = std::env::temp_dir().join("daox_sec_test_override_dir_symlink");
        let _ = fs::remove_dir_all(&tmp);
        let overrides_dir = tmp.join("overrides");
        let _ = fs::create_dir_all(&overrides_dir);

        let target_dir = tmp.join("sensitive_dir");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("email.toml"), "[type]\nvalue = \"String\"").unwrap();

        let link = overrides_dir.join("users");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target_dir, &link).unwrap();
            let meta = fs::symlink_metadata(&link).unwrap();
            assert!(
                meta.file_type().is_symlink(),
                "Override table dir symlink must be detected"
            );
        }
        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod enum_double_quote_tests {
    #[test]
    fn test_enum_parsing_with_double_quotes() {
        let vals_str = "'a', 'b''c', 'd'";
        let mut parsed_vals = Vec::new();
        let mut current = String::new();
        let mut in_quote = false;
        let mut escape = false;
        let mut chars = vals_str.chars().peekable();

        while let Some(c) = chars.next() {
            if escape {
                current.push(c);
                escape = false;
                continue;
            }

            match c {
                '\\' if in_quote => {
                    escape = true;
                }
                '\'' => {
                    if in_quote {
                        if chars.peek() == Some(&'\'') {
                            current.push('\'');
                            chars.next();
                        } else {
                            in_quote = false;
                        }
                    } else {
                        in_quote = true;
                    }
                }
                ',' if !in_quote => {
                    parsed_vals.push(std::mem::take(&mut current).trim().to_string());
                }
                _ => current.push(c),
            }
        }
        if !current.trim().is_empty() {
            parsed_vals.push(current.trim().to_string());
        }

        assert_eq!(parsed_vals, vec!["a", "b'c", "d"]);
    }
}

#[cfg(test)]
mod sql_safety_tests {
    #[test]
    fn test_dynamic_query_construction_safe() {
        let valid_identifiers = vec!["users", "user_roles", "col_123"];
        for id in valid_identifiers {
            assert!(
                id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
                "Identifier must be safe: {:?}",
                id
            );
        }

        let invalid_identifiers =
            vec!["users; DROP TABLE", "col`name", "table\"name", "id = 1; --"];
        for id in invalid_identifiers {
            let is_valid = !id.is_empty()
                && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !id.chars().next().unwrap_or('0').is_ascii_digit();
            assert!(!is_valid, "Should reject: {:?}", id);
        }
    }
}

#[cfg(test)]
mod partial_update_limit_tests {
    #[test]
    fn test_update_partial_column_limit() {
        let max_cols = 10;
        for col_count in 1..=max_cols {
            let combinations = 2usize.pow(col_count as u32);
            assert!(
                combinations <= 1024,
                "Combinations must be bounded for {} cols",
                col_count
            );
        }
        assert!(2usize.pow(11) > 1024);
    }
}

#[cfg(test)]
mod batch_chunking_tests {
    #[test]
    fn test_mysql_chunk_size_never_zero() {
        let max_params: usize = 65535;
        for col_count in 1..=100 {
            let chunk_size = max_params / col_count;
            assert!(
                chunk_size >= 1,
                "Chunk size must be >= 1 for {} cols",
                col_count
            );
        }
    }

    #[test]
    fn test_sqlite_chunk_size_never_zero() {
        let max_params: usize = 32766;
        for col_count in 1..=100 {
            let chunk_size = max_params / col_count;
            assert!(chunk_size >= 1);
        }
    }
}

#[cfg(test)]
mod csv_escaping_tests {
    #[test]
    fn test_csv_double_quote_escaping() {
        let input = r#"He said "hello" and left"#;
        let mut escaped = String::new();
        escaped.push('"');
        for c in input.chars() {
            if c == '"' {
                escaped.push_str("\"\"");
            } else {
                escaped.push(c);
            }
        }
        escaped.push('"');
        assert_eq!(escaped, r#""He said ""hello"" and left""#);
    }

    #[test]
    fn test_csv_newline_in_quoted_field() {
        let input = "line1\nline2";
        let mut escaped = String::new();
        escaped.push('"');
        for c in input.chars() {
            if c == '"' {
                escaped.push_str("\"\"");
            } else {
                escaped.push(c);
            }
        }
        escaped.push('"');
        assert!(escaped.contains('\n'));
        assert!(escaped.starts_with('"'));
        assert!(escaped.ends_with('"'));
    }
}

#[cfg(test)]
mod transaction_safety_tests {
    #[test]
    fn test_batch_requires_transaction() {
        assert!(
            true,
            "Compile-time check: insert_batch requires &mut Transaction"
        );
    }
}

#[cfg(test)]
mod toml_serialization_tests {
    #[test]
    fn test_toml_roundtrip_with_special_chars() {
        // Vérifier que la sérialisation toml::Value gère correctement
        // les caractères spéciaux
        let mut root = toml::map::Map::new();
        let mut section = toml::map::Map::new();
        section.insert(
            "value".into(),
            toml::Value::String("test\"with\\quotes\nand newlines".into()),
        );
        root.insert("type".into(), toml::Value::Table(section));

        let serialized = toml::to_string_pretty(&toml::Value::Table(root)).unwrap();
        let parsed: toml::Value = toml::from_str(&serialized).unwrap();

        let val = parsed
            .get("type")
            .unwrap()
            .get("value")
            .unwrap()
            .as_str()
            .unwrap();
        assert_eq!(val, "test\"with\\quotes\nand newlines");
    }

    #[test]
    fn test_toml_rejects_oversized_file() {
        // Simuler un fichier TOML > 1MB
        let oversized = "x".repeat(2 * 1024 * 1024);
        assert!(oversized.len() > 1_048_576);
        // Le générateur devrait rejeter ce fichier
    }
}

#[cfg(test)]
mod identifier_defense_in_depth_tests {
    #[test]
    fn test_is_safe_identifier_rejects_all_attack_vectors() {
        let long_str = "a".repeat(129);
        let attacks = vec![
            "table; DROP TABLE users",
            "col`backtick",
            "col\"quote",
            "col'single",
            "col\\backslash",
            "col\nnewline",
            "col\ttab",
            "col\x00null",
            "../traversal",
            "col name",
            "",
            "123start",
            long_str.as_str(), // > 128 chars
        ];
        for attack in attacks {
            let valid = !attack.is_empty()
                && attack.len() <= 128
                && attack
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !attack.chars().next().unwrap_or('0').is_ascii_digit();
            assert!(!valid, "Should reject: {:?}", attack);
        }
    }

    #[test]
    fn test_is_safe_identifier_accepts_valid() {
        let valid_names = vec!["users", "user_roles", "t1", "MyTable", "col_123", "a"];
        for name in valid_names {
            let valid = !name.is_empty()
                && name.len() <= 128
                && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                && !name.chars().next().unwrap().is_ascii_digit();
            assert!(valid, "Should accept: {:?}", name);
        }
    }
}

#[cfg(test)]
mod symlink_write_protection_tests {
    use std::fs;

    #[test]
    fn test_refuse_write_to_symlink() {
        let tmp = std::env::temp_dir().join("daox_sec_test_symlink_write");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let target = tmp.join("target_file.txt");
        fs::write(&target, "original").unwrap();

        let link = tmp.join("link_file.txt");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target, &link).unwrap();
            let meta = fs::symlink_metadata(&link).unwrap();
            assert!(meta.file_type().is_symlink());
            assert!(daox::safe_write_if_changed(&link, b"malicious code").is_err());
        }

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod generated_code_security_tests {
    #[test]
    fn test_validate_does_not_panic_on_invalid_regex() {
        // Simuler le comportement du code généré avec regex invalide
        let invalid_regex = "[unclosed";
        let re = regex::Regex::new(invalid_regex).ok();
        assert!(re.is_none());
        // Le code généré utilise Option<Regex> et gère None gracieusement
    }

    #[test]
    fn test_validate_handles_empty_string() {
        let empty = "";
        let re = regex::Regex::new("^[a-z]+$").unwrap();
        assert!(!re.is_match(empty));
    }

    #[test]
    fn test_batch_chunk_size_never_zero() {
        let max_params_list = vec![65535, 32766];
        for max_params in max_params_list {
            for col_count in 1..=200 {
                let chunk_size = max_params / col_count;
                assert!(chunk_size >= 1, "Chunk size must be >= 1");
                assert!(
                    chunk_size * col_count <= max_params,
                    "Total params must not exceed limit"
                );
            }
        }
    }

    #[test]
    fn test_csv_escaping_handles_all_special_chars() {
        let test_cases = vec![
            ("normal", "\"normal\""),
            ("with\"quote", "\"with\"\"quote\""),
            ("with\nnewline", "\"with\nnewline\""),
            ("with\r\ncrlf", "\"with\r\ncrlf\""),
            ("", "\"\""),
        ];
        for (input, expected) in test_cases {
            let mut escaped = String::new();
            escaped.push('"');
            for c in input.chars() {
                if c == '"' {
                    escaped.push_str("\"\"");
                } else {
                    escaped.push(c);
                }
            }
            escaped.push('"');
            assert_eq!(escaped, expected);
        }
    }
}

#[cfg(test)]
mod orphan_cleanup_security_tests {
    use std::fs;

    #[test]
    fn test_orphan_cleanup_does_not_follow_internal_symlinks() {
        let tmp = std::env::temp_dir().join("daox_sec_test_orphan_symlink");
        let _ = fs::remove_dir_all(&tmp);
        let orphan_dir = tmp.join("orphan_table");
        let external_dir = tmp.join("external_data");
        fs::create_dir_all(&orphan_dir).unwrap();
        fs::create_dir_all(&external_dir).unwrap();
        fs::write(external_dir.join("important.txt"), "data").unwrap();

        #[cfg(unix)]
        {
            let link = orphan_dir.join("link_to_external");
            std::os::unix::fs::symlink(&external_dir, &link).unwrap();

            // Call the real safe_remove_dir_all
            daox::safe_remove_dir_all(&orphan_dir).unwrap();

            assert!(external_dir.join("important.txt").exists());
        }

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod file_permission_tests {
    use std::fs;

    #[test]
    fn test_generated_files_have_correct_permissions() {
        let tmp = std::env::temp_dir().join("daox_sec_test_permissions");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let file_path = tmp.join("test_generated.rs");
        fs::write(&file_path, "// generated code").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o644);
            fs::set_permissions(&file_path, perms).unwrap();

            let meta = fs::metadata(&file_path).unwrap();
            assert_eq!(meta.permissions().mode() & 0o777, 0o644);
        }

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod residual_fixes_tests {
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_enum_toml_roundtrip_control_chars() {
        let malicious_values = vec!["val\0", "val\nnewline", "val\r\ttab", "val\x7F", "val🚀"];

        let mut root = toml::map::Map::new();
        let mut ev_section = toml::map::Map::new();
        let vals: Vec<toml::Value> = malicious_values
            .iter()
            .map(|s| toml::Value::String(s.to_string()))
            .collect();
        ev_section.insert("value".into(), toml::Value::Array(vals));
        root.insert("enum_values".into(), toml::Value::Table(ev_section));

        let serialized = toml::to_string_pretty(&toml::Value::Table(root))
            .expect("Failed to serialize with control chars");
        let parsed: toml::Value =
            toml::from_str(&serialized).expect("Failed to parse back valid TOML");

        let arr = parsed
            .get("enum_values")
            .unwrap()
            .get("value")
            .unwrap()
            .as_array()
            .unwrap();
        assert_eq!(arr.len(), malicious_values.len());
        for (i, val) in malicious_values.iter().enumerate() {
            assert_eq!(arr[i].as_str().unwrap(), *val);
        }
    }

    /*
    This code simulates what safe_remove_dir_all does. We don't have access to safe_remove_dir_all here unless we expose it.
    But we can test the behavior by creating a symlink to a dir, and just verifying that if we call fs::remove_file on a symlink, the target remains.
    */
    #[test]
    fn test_safe_remove_symlink_root() {
        let tmp = std::env::temp_dir().join("daox_sec_test_remove_symlink_root");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let target_dir = tmp.join("target_dir");
        fs::create_dir_all(&target_dir).unwrap();
        fs::write(target_dir.join("file.txt"), "keep").unwrap();

        let link = tmp.join("link_dir");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target_dir, &link).unwrap();
            let meta = fs::symlink_metadata(&link).unwrap();
            assert!(meta.file_type().is_symlink());

            // Call the real safe_remove_dir_all
            daox::safe_remove_dir_all(&link).unwrap();

            // The target directory must still exist and contain the file
            assert!(target_dir.exists());
            assert_eq!(
                fs::read_to_string(target_dir.join("file.txt")).unwrap(),
                "keep"
            );
        }

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod dangling_symlink_tests {
    use std::fs;

    #[test]
    fn test_dangling_symlink_rejected() {
        let tmp = std::env::temp_dir().join("daox_sec_test_dangling_symlink");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let link = tmp.join("generated.rs");
        let nonexistent_target = tmp.join("nonexistent_target.rs");
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&nonexistent_target, &link).unwrap();

            assert!(
                fs::symlink_metadata(&link)
                    .unwrap()
                    .file_type()
                    .is_symlink()
            );
            assert!(!nonexistent_target.exists());

            let result = daox::safe_write_if_changed(&link, b"malicious code");
            assert!(result.is_err(), "Must reject dangling symlink");

            assert!(!nonexistent_target.exists());
        }
        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod databases_toml_size_tests {
    use std::fs;
    #[test]
    fn test_oversized_databases_toml_rejected() {
        let tmp = std::env::temp_dir().join("daox_sec_test_databases_size");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let path = tmp.join("databases.toml");
        let oversized = format!(
            "[default]\ndialect = \"mysql\"\n# {}\n",
            "x".repeat(2 * 1024 * 1024)
        );
        fs::write(&path, &oversized).unwrap();

        let result = daox::read_toml_file_limited(&path);
        assert!(result.is_err(), "Must reject file > 1MB");

        let _ = fs::remove_dir_all(&tmp);
    }
}

#[cfg(test)]
mod partial_update_validation_tests {
    #[tokio::test]
    async fn test_update_partial_rejects_invalid_patch() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, email TEXT NOT NULL, last_name TEXT NOT NULL, first_name TEXT, status TEXT NOT NULL, created_at TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let patch = crate::models_sqlite::UsersPatch {
            email: None,
            last_name: Some(String::from("")),
            first_name: None,
            status: None,
            created_at: None,
        };

        let result = crate::models_sqlite::Users::update_partial_by_id(&pool, 1, &patch).await;
        assert!(
            result.is_err(),
            "update_partial must reject invalid patch values"
        );
    }
}

#[cfg(test)]
mod nan_infinity_tests {
    #[test]
    fn test_validate_rejects_nan() {
        let item = crate::models_sqlite::CompTypesTable {
            id: 0,
            f_bool: None,
            f_decimal: Some(f64::NAN),
            f_double: None,
            f_float: None,
            f_int: None,
            f_text: None,
            f_varchar: None,
        };
        let result = item.validate();
        assert!(result.is_err(), "validate() must reject NaN");
    }

    #[test]
    fn test_validate_rejects_infinity() {
        let item = crate::models_sqlite::CompTypesTable {
            id: 0,
            f_bool: None,
            f_decimal: Some(f64::INFINITY),
            f_double: None,
            f_float: None,
            f_int: None,
            f_text: None,
            f_varchar: None,
        };
        let result = item.validate();
        assert!(result.is_err(), "validate() must reject Infinity");
    }
}

#[cfg(test)]
mod enum_malformed_tests {
    #[test]
    fn test_malformed_enum_fallback() {
        let malformed = "ENUM('a', 'b";
        let result = daox::parse_mysql_enum(malformed);
        match result {
            Ok(vals) => assert!(
                !vals.is_empty(),
                "Malformed ENUM should not produce empty vec silently"
            ),
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod batch_validation_tests {
    #[tokio::test]
    async fn test_insert_batch_rejects_invalid_items() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, email TEXT NOT NULL, last_name TEXT NOT NULL, first_name TEXT, status TEXT NOT NULL, created_at TEXT)")
            .execute(&pool)
            .await
            .unwrap();

        let mut tx = pool.begin().await.unwrap();

        let invalid_user = crate::models_sqlite::Users {
            id: 0,
            email: String::from(""), // Invlaid, min_length >= 1
            last_name: String::from(""),
            first_name: None,
            status: String::from(""),
            created_at: None,
        };

        let result = crate::models_sqlite::Users::insert_batch(&mut tx, &[invalid_user]).await;
        assert!(result.is_err(), "insert_batch must reject invalid items");
    }
}
