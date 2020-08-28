pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => "".to_string(),
        _ => {
            let proverb = (0..list.len() - 1).fold(String::new(), |s, x| {
                s + &format!("For want of a {} the {} was lost.\n", list[x], list[x + 1])
            });
            proverb + &format!("And all for the want of a {}.", list[0])
        }
    }
}
