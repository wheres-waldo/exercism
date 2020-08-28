pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1..=19 => one_to_nineteen(n),
        20..=99 => {
            let ending = one_to_nineteen(n % 10);
            format!(
                "{}{}",
                tens(n),
                if !ending.is_empty() {
                    format!("-{}", ending)
                } else {
                    ending
                }
            )
        }
        100..=999 => {
            let ending = encode(n % 100);
            format!(
                "{} hundred{}",
                one_to_nineteen(n / 100),
                if n % 100 > 0 {
                    format!(" {}", ending)
                } else {
                    "".to_string()
                }
            )
        }
        _ => {
            // for borrow checker
            let tmp = n.to_string();
            let chunks = tmp
                .as_bytes()
                .rchunks(3)
                .flat_map(std::str::from_utf8)
                .rev()
                .collect::<Vec<&str>>();
            let mut count = chunks.len();
            let mut num_string = String::new();

            for chunk in chunks {
                let num = chunk.parse().unwrap();
                if num > 0 {
                    num_string.extend(format!("{}{}", encode(num), thousand_plus(count)).chars());
                }
                count -= 1;
            }

            num_string.trim().to_string()
        }
    }
}

fn one_to_nineteen(n: u64) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }
    .to_string()
}

fn tens(n: u64) -> String {
    match n / 10 {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    }
    .to_string()
}

fn thousand_plus(n: usize) -> String {
    match n {
        2 => " thousand ",
        3 => " million ",
        4 => " billion ",
        5 => " trillion ",
        6 => " quadrillion ",
        7 => " quintillion ",
        _ => "",
    }
    .to_string()
}
