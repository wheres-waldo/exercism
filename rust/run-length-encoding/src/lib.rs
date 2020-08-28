pub fn encode(source: &str) -> String {
    match source {
        "" => "".to_string(),
        _ => source
            .chars()
            .map(Some)
            .chain(std::iter::once(None))
            .scan((0, '\0'), |(n, c), ch| match ch {
                Some(ch) if *n == 0 || *c == ch => {
                    *n += 1;
                    *c = ch;

                    Some(String::new())
                }
                Some(ch) => {
                    let run = if *n > 1 {
                        format!("{}{}", n, c)
                    } else {
                        format!("{}", c)
                    };

                    *n = 1;
                    *c = ch;

                    Some(run)
                }
                None if *n > 1 => Some(format!("{}{}", n, c)),
                None => Some(format!("{}", c)),
            })
            .collect::<String>(),
    }
}

pub fn decode(source: &str) -> String {
    source
        .chars()
        .fold((0usize, String::new()), |(n, text), c| {
            if c.is_digit(10) {
                (n * 10 + c.to_digit(10).unwrap() as usize, text)
            } else {
                let decode = if n > 1 {
                    format!("{}", c.to_string().repeat(n))
                } else {
                    format!("{}", c.to_string())
                };

                (0, text + &decode)
            }
        })
        .1
}
