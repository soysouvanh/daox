fn main() {
    let r = r#"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$"#.to_string();
    println!("Res Debug: {:?}", r);
}
