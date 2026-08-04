fn main() {
    let t = "ENUM('a', 'b, c', 'd''e')";
    let re = regex::Regex::new(r"(?i)^enum\((.*)\)$").unwrap();
    if let Some(caps) = re.captures(t) {
        println!("{}", caps.get(1).unwrap().as_str());
    } else {
        println!("No match");
    }
}
