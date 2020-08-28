pub fn verse(n: u32) -> String {
    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} {2} of beer on the wall.\n", n, n - 1, if n - 1 > 1 { "bottles" } else { "bottle" })
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().fold(String::new(), |s, x| {
        s + &verse(x) + if x != end { "\n" } else { "" }
    })
}
