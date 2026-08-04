import os

target = "/mnt/project/framework/daox-workspace/daox/src/lib.rs"
with open(target, 'r') as f:
    lib_rs = f.read()

# F-01
target_f01 = """fn safe_write_if_changed<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P, 
    content: C
) -> Result<bool, String> {
    let p = path.as_ref();
    if p.exists() {
        let meta = fs::symlink_metadata(p).map_err(|e| e.to_string())?;
        if meta.file_type().is_symlink() {
            return Err(format!("Refusing to write to symlink: {:?}", p));
        }
    }
    if let Some(parent) = p.parent() {
        let parent_meta = fs::symlink_metadata(parent).map_err(|e| e.to_string())?;
        if parent_meta.file_type().is_symlink() {
            return Err(format!("Refusing to write into symlinked directory: {:?}", parent));
        }
    }
    write_if_changed(path, content).map_err(|e| e.to_string())
}"""
replace_f01 = """fn safe_write_if_changed<P: AsRef<std::path::Path>, C: AsRef<[u8]>>(
    path: P,
    content: C,
) -> Result<bool, String> {
    let p = path.as_ref();

    match fs::symlink_metadata(p) {
        Ok(meta) => {
            if meta.file_type().is_symlink() {
                return Err(format!("Refusing to write to symlink: {:?}", p));
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Le fichier n'existe pas encore
        }
        Err(e) => {
            return Err(format!("Cannot stat {:?}: {}", p, e));
        }
    }

    if let Some(parent) = p.parent() {
        match fs::symlink_metadata(parent) {
            Ok(parent_meta) => {
                if parent_meta.file_type().is_symlink() {
                    return Err(format!(
                        "Refusing to write into symlinked directory: {:?}",
                        parent
                    ));
                }
            }
            Err(e) => {
                return Err(format!("Cannot stat parent {:?}: {}", parent, e));
            }
        }
    }

    write_if_changed(path, content).map_err(|e| e.to_string())
}"""
lib_rs = lib_rs.replace(target_f01, replace_f01)

# F-02
target_f02 = """        let path = Path::new(&self.schema_dir).join("databases.toml");
        if let Ok(content) = fs::read_to_string(&path)
            && let Ok(val) = content.parse::<toml::Value>()"""
replace_f02 = """        let path = Path::new(&self.schema_dir).join("databases.toml");
        if let Ok(content) = read_toml_file_limited(&path)
            && let Ok(val) = content.parse::<toml::Value>()"""
lib_rs = lib_rs.replace(target_f02, replace_f02)

# F-06 Tables
target_f06_tables = """        let tables: Vec<(String, String)> = tables_rows
            .into_iter()
            .map(|row| {
                (
                    row.get::<String, _>("TABLE_NAME"),
                    row.get::<String, _>("TABLE_TYPE"),
                )
            })
            .collect();"""
replace_f06_tables = """        let mut tables: Vec<(String, String)> = Vec::new();
        for row in tables_rows {
            let table_name: String = match row.try_get("TABLE_NAME") {
                Ok(v) => v,
                Err(e) => {
                    println!("cargo:warning=Daox: skipping row with missing TABLE_NAME: {}", e);
                    continue;
                }
            };
            let table_type: String = match row.try_get("TABLE_TYPE") {
                Ok(v) => v,
                Err(e) => {
                    println!("cargo:warning=Daox: skipping table '{}' with missing TABLE_TYPE: {}", table_name, e);
                    continue;
                }
            };
            tables.push((table_name, table_type));
        }"""
lib_rs = lib_rs.replace(target_f06_tables, replace_f06_tables)

# F-06 Columns
target_f06_columns = """                let col_name: String = col.get("COLUMN_NAME");
                if !is_safe_identifier(&col_name) {
                    println!(
                        "cargo:warning=Daox: column '{}' in table '{}' skipped (invalid identifier characters)",
                        col_name, table_name
                    );
                    continue;
                }
                let data_type: String = col.get("DATA_TYPE");
                let is_nullable: String = col.get("IS_NULLABLE");
                let column_key: String = col.get("COLUMN_KEY");
                let extra: String = col.get("EXTRA");
                let column_type: String = col.get("COLUMN_TYPE");"""
replace_f06_columns = """                let col_name: String = match col.try_get("COLUMN_NAME") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("cargo:warning=Daox: skipping column with missing COLUMN_NAME in table '{}': {}", table_name, e);
                        continue;
                    }
                };
                if !is_safe_identifier(&col_name) {
                    println!(
                        "cargo:warning=Daox: column '{}' in table '{}' skipped (invalid identifier characters)",
                        col_name, table_name
                    );
                    continue;
                }
                let data_type: String = match col.try_get("DATA_TYPE") {
                    Ok(v) => v,
                    Err(e) => {
                        println!("cargo:warning=Daox: skipping column '{}' with missing DATA_TYPE: {}", col_name, e);
                        continue;
                    }
                };
                let is_nullable: String = match col.try_get("IS_NULLABLE") { Ok(v) => v, Err(_) => "YES".to_string() };
                let column_key: String = match col.try_get("COLUMN_KEY") { Ok(v) => v, Err(_) => "".to_string() };
                let extra: String = match col.try_get("EXTRA") { Ok(v) => v, Err(_) => "".to_string() };
                let column_type: String = match col.try_get("COLUMN_TYPE") { Ok(v) => v, Err(_) => data_type.clone() };"""
lib_rs = lib_rs.replace(target_f06_columns, replace_f06_columns)

# F-06 Indexes
target_f06_indexes = """            for row in indexes_rows {
                let idx_name: String = row.get("INDEX_NAME");
                let col_name: String = row.get("COLUMN_NAME");
                let non_unique: i64 = row.get("NON_UNIQUE");"""
replace_f06_indexes = """            for row in indexes_rows {
                let idx_name: String = match row.try_get("INDEX_NAME") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let col_name: String = match row.try_get("COLUMN_NAME") {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let non_unique: i64 = match row.try_get("NON_UNIQUE") {
                    Ok(v) => v,
                    Err(_) => continue,
                };"""
lib_rs = lib_rs.replace(target_f06_indexes, replace_f06_indexes)

# F-07 Enum
target_f07 = """                    let parsed_vals = parse_mysql_enum(vals_str)
                        .unwrap_or_else(|_| vec![]);

                    let rx_inner = parsed_vals.iter()
                        .map(|s| std::borrow::Cow::Owned(regex::escape(s)))
                        .collect::<Vec<_>>()
                        .join("|");
                    let format_regex = format!("^({})$", rx_inner);
                    // Valider la regex avant de l'écrire dans le TOML
                    if let Err(e) = regex::Regex::new(&format_regex) {
                        println!("cargo:warning=Daox: invalid regex for enum in table '{}': {}", table_name, e);
                        "^.*$".to_string()
                    } else {
                        format_regex
                    }"""
replace_f07 = """                    let parsed_vals = match parse_mysql_enum(vals_str) {
                        Ok(v) if !v.is_empty() => v,
                        Ok(_) => {
                            println!("cargo:warning=Daox: ENUM in table '{}' column '{}' parsed to empty values, using permissive regex", table_name, col_name);
                            Vec::new()
                        }
                        Err(e) => {
                            println!("cargo:warning=Daox: failed to parse ENUM in table '{}' column '{}': {}, using permissive regex", table_name, col_name, e);
                            Vec::new()
                        }
                    };

                    let format_regex = if parsed_vals.is_empty() {
                        "^.*$".to_string()
                    } else {
                        let rx_inner = parsed_vals.iter()
                            .map(|s| std::borrow::Cow::Owned(regex::escape(s)))
                            .collect::<Vec<_>>()
                            .join("|");
                        let format_regex = format!("^({})$", rx_inner);
                        if let Err(e) = regex::Regex::new(&format_regex) {
                            println!("cargo:warning=Daox: invalid regex for enum in table '{}': {}", table_name, e);
                            "^.*$".to_string()
                        } else {
                            format_regex
                        }
                    }"""
lib_rs = lib_rs.replace(target_f07, replace_f07)

# F-10 is_finite
target_f10 = """                    if let Some(prop) = &col_meta.min_value {
                        let min = prop.value();
                        if ty == "f32" || ty == "f64" {"""
replace_f10 = """                    if ty == "f32" || ty == "f64" {
                        write!(&mut code,
                            "        if let Some(v) = {val_ref} {{\n\
                                         if !v.is_finite() {{\n\
                                             errors.push(\"{field}: value must be finite (NaN/Infinity rejected)\".into());\n\
                                         }}\n\
                                     }}\n",
                            val_ref = val_ref, field = field
                        ).unwrap();
                    }
                    if let Some(prop) = &col_meta.min_value {
                        let min = prop.value();
                        if ty == "f32" || ty == "f64" {"""
lib_rs = lib_rs.replace(target_f10, replace_f10)

# F-03 Postgres insert_batch
target_f03_a = """                             pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 let mut copy_in = executor.copy_in_raw(r#"COPY {tbl} ({cols}) FROM STDIN WITH (FORMAT csv)"#).await?;\n\\"""
replace_f03_a = """                             pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 for (idx, item) in items.iter().enumerate() {{\n\
                                     if let Err(e) = item.validate() {{\n\
                                         return Err(sqlx::Error::Protocol(format!("insert_batch: item {{}} failed validation: {{}}", idx, e.join(", ")).into()));\n\
                                     }}\n\
                                 }}\n\
                                 let mut copy_in = executor.copy_in_raw(r#"COPY {tbl} ({cols}) FROM STDIN WITH (FORMAT csv)"#).await?;\n\\"""
lib_rs = lib_rs.replace(target_f03_a, replace_f03_a)

# F-03 Generic insert_batch
target_f03_b = """                             pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\\"""
replace_f03_b = """                             pub async fn insert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 for (idx, item) in items.iter().enumerate() {{\n\
                                     if let Err(e) = item.validate() {{\n\
                                         return Err(sqlx::Error::Protocol(format!("insert_batch: item {{}} failed validation: {{}}", idx, e.join(", ")).into()));\n\
                                     }}\n\
                                 }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\\"""
lib_rs = lib_rs.replace(target_f03_b, replace_f03_b)

# F-03 Generic upsert_batch
target_f03_upsert = """                             pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\\"""
replace_f03_upsert = """                             pub async fn upsert_batch<'e>(executor: &mut sqlx::Transaction<'e, {db}>, items: &[Self]) -> sqlx::Result<u64> {{\n\
                                 if items.is_empty() {{ return Ok(0); }}\n\
                                 for (idx, item) in items.iter().enumerate() {{\n\
                                     if let Err(e) = item.validate() {{\n\
                                         return Err(sqlx::Error::Protocol(format!("upsert_batch: item {{}} failed validation: {{}}", idx, e.join(", ")).into()));\n\
                                     }}\n\
                                 }}\n\
                                 let chunk_size = {max_params} / {col_count};\n\\"""
lib_rs = lib_rs.replace(target_f03_upsert, replace_f03_upsert)


# F-04 update_partial_by_{pk}
# Wait, F-04 implies generating code for each column based on constraints. But instead of generating this code, I can just write a short validation loop over update_cols. Wait! There's no easy way to get the metadata at runtime. The validation rules are ONLY known at build-time. We must generate the rust code.
# The `generate_dao` loop for patch already iterates over `update_cols`. We can insert validation statements before we push to the QueryBuilder!

# I will replace the generation of `update_partial_by_{sfx}`.
target_f04_start = """                write!(&mut code, 
                    "    #[allow(unused_assignments)]\n\
                         pub async fn update_partial_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}, patch: &{patch}) -> sqlx::Result<u64> {{\n\
                             let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"UPDATE {tbl} SET \");\n\
                             let mut first = true;\n",
                    sfx = pk_suffix, db = db_type,
                    args = pk_args.join(", "), patch = patch_name,
                    tbl = tbl_escaped
                ).unwrap();"""
replace_f04_start = """                write!(&mut code, 
                    "    #[allow(unused_assignments, unused_comparisons)]\n\
                         pub async fn update_partial_by_{sfx}<'e, E: sqlx::Executor<'e, Database = {db}>>(executor: E, {args}, patch: &{patch}) -> sqlx::Result<u64> {{\n\
                             let mut errors: Vec<String> = Vec::new();\n",
                    sfx = pk_suffix, db = db_type,
                    args = pk_args.join(", "), patch = patch_name
                ).unwrap();

                // F-04 Validation block
                for (n, col_meta) in update_cols.iter() {
                    let field = Self::escape_rust_keyword(n);
                    let ty = col_meta.rust_type_resolved();
                    let val_ref = format!("v");
                    write!(&mut code, "                             if let Some({val_ref}) = &patch.{field} {{\n").unwrap();
                    
                    if ty == "String" || ty == "Vec<u8>" {
                        if let Some(prop) = &col_meta.min_length {
                            let min = prop.value();
                            if min > 0 {
                                write!(&mut code, "                                 if {val_ref}.len() < {min} {{ errors.push(\"{field}: min_length {min} not met\".into()); }}\n").unwrap();
                            }
                        }
                        if let Some(prop) = &col_meta.max_length {
                            let max = prop.value();
                            write!(&mut code, "                                 if {val_ref}.len() > {max} {{ errors.push(\"{field}: exceeds max_length {max}\".into()); }}\n").unwrap();
                        }
                    }

                    if ty == "String" {
                        if let Some(prop) = &col_meta.enum_values {
                            let evs = prop.value();
                            if !evs.is_empty() {
                                let evs_str = evs.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(", ");
                                write!(&mut code, "                                 let valid_enums = [{evs_str}];\n                                 if !valid_enums.contains(&{val_ref}.as_str()) {{ errors.push(\"{field}: invalid enum value\".into()); }}\n").unwrap();
                            }
                        }
                        if let Some(prop) = &col_meta.format {
                            let re_str = prop.value();
                            if !re_str.is_empty() {
                                let re_escaped = format!("{:?}", re_str);
                                write!(&mut code, "                                 #[cfg(feature = \"validation\")]\n                                 {{\n                                     static RE: std::sync::OnceLock<Option<regex::Regex>> = std::sync::OnceLock::new();\n                                     let re = RE.get_or_init(|| regex::Regex::new({re_escaped}).ok());\n                                     if let Some(re) = re {{ if !re.is_match({val_ref}) {{ errors.push(\"{field}: format constraint not met\".into()); }} }}\n                                 }}\n").unwrap();
                            }
                        }
                    }

                    if ty != "String" && ty != "Vec<u8>" && ty != "bool" && ty != "serde_json::Value" && !ty.starts_with("chrono::") {
                        if ty == "f32" || ty == "f64" {
                            write!(&mut code, "                                 if !{val_ref}.is_finite() {{ errors.push(\"{field}: value must be finite (NaN/Infinity rejected)\".into()); }}\n").unwrap();
                        }
                        if let Some(prop) = &col_meta.min_value {
                            let min = prop.value();
                            let cast = if ty == "f32" || ty == "f64" { "f64" } else { "i128" };
                            write!(&mut code, "                                 if (*{val_ref} as {cast}) < ({min} as {cast}) {{ errors.push(\"{field}: minimum value '{min}' not met\".into()); }}\n").unwrap();
                        }
                        if let Some(prop) = &col_meta.max_value {
                            let max = prop.value();
                            let cast = if ty == "f32" || ty == "f64" { "f64" } else { "i128" };
                            write!(&mut code, "                                 if (*{val_ref} as {cast}) > ({max} as {cast}) {{ errors.push(\"{field}: exceeds max_value '{max}'\".into()); }}\n").unwrap();
                        }
                    }
                    write!(&mut code, "                             }}\n").unwrap();
                }

                write!(&mut code, 
                    "                             if !errors.is_empty() {{\n\
                                                      return Err(sqlx::Error::Protocol(errors.join(\", \").into()));\n\
                                                  }}\n\
                                                  let mut qb: sqlx::QueryBuilder<{db}> = sqlx::QueryBuilder::new(\"UPDATE {tbl} SET \");\n\
                                                  let mut first = true;\n",
                    db = db_type, tbl = tbl_escaped
                ).unwrap();"""
lib_rs = lib_rs.replace(target_f04_start, replace_f04_start)

# Save lib.rs
with open(target, 'w') as f:
    f.write(lib_rs)

