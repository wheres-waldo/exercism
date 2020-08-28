use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut set = HashSet::new();

    sentence
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|c| c.to_ascii_lowercase())
        .map(|c| set.insert(c))
        .filter(|b| *b)
        .count()
        == 26
}
