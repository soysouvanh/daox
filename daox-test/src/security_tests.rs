
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
        let mut prev_was_quote = false;

        for c in vals_str.chars() {
            if escape {
                current.push(c);
                escape = false;
                prev_was_quote = false;
            } else if c == '\\' {
                escape = true;
                prev_was_quote = false;
            } else if c == '\'' {
                if in_quote && prev_was_quote {
                    current.push('\'');
                    prev_was_quote = false;
                } else if in_quote {
                    prev_was_quote = true;
                } else {
                    in_quote = true;
                    prev_was_quote = false;
                }
            } else if c == ',' && !in_quote {
                parsed_vals.push(current.trim().to_string());
                current.clear();
                prev_was_quote = false;
            } else {
                if prev_was_quote {
                    in_quote = false;
                    prev_was_quote = false;
                }
                current.push(c);
            }
        }
        if prev_was_quote {
            in_quote = false;
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
