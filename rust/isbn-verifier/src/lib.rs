/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let numbers = isbn
        .chars()
        .enumerate()
        .filter(|(i, c)| *c != '-' && (c.is_ascii_digit() || (*c == 'X' && *i == isbn.len() - 1)))
        .map(|(_, c)| c.to_digit(10).unwrap_or(10))
        .collect::<Vec<u32>>();

    match &numbers[..] {
        [x1, x2, x3, x4, x5, x6, x7, x8, x9, x10] => {
            (x1 * 10
                + x2 * 9
                + x3 * 8
                + x4 * 7
                + x5 * 6
                + x6 * 5
                + x7 * 4
                + x8 * 3
                + x9 * 2
                + x10 * 1)
                % 11
                == 0
        }
        _ => false,
    }
}
