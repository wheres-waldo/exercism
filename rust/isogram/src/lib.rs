pub fn check(candidate: &str) -> bool {
    let mut chars = candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<char>>();
    let total_chars = chars.len();

    chars.sort();
    chars.dedup();

    total_chars == chars.len()
}
