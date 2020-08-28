/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|c| c.to_ascii_lowercase())
        .fold(0, |mut acc, c| {
            acc += match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
                'd' | 'g' => 2,
                'b' | 'c' | 'm' | 'p' => 3,
                'f' | 'h' | 'v' | 'w' | 'y' => 4,
                'k' => 5,
                'j' | 'x' => 8,
                'q' | 'z' => 10,
                _ => 0,
            };

            acc
        })
}
