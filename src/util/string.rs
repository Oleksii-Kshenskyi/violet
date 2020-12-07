pub fn clone_uppercased(original: &String) -> String {
    let cloned = original.clone();
    let mut chars = cloned.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().collect::<String>() + chars.as_str(),
    }
}