pub fn clone_uppercased(original: &str) -> String {
    let cloned = original.to_string();
    let mut chars = cloned.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
