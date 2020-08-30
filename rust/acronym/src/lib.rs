pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c| "-_ ".contains(c))
        .flat_map(|s| {
            s.chars().take(1).chain(
                s.chars()
                    .skip_while(|c| c.is_ascii_uppercase())
                    .filter(|c| c.is_ascii_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}
