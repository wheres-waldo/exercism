pub fn is_armstrong_number(num: u32) -> bool {
    let num_string = num.to_string();
    let len = num_string.len() as u32;

    num == num_string
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|n| n.pow(len))
        .sum()
}
