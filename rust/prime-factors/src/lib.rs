pub fn factors(n: u64) -> Vec<u64> {
    match n {
        1 => vec![],
        _ => {
            let end = (n as f64).sqrt() as u64;
            let prime = (2..=end).filter(|x| n % x == 0).next().unwrap_or(n);
            let mut vec = vec![prime];
            vec.extend(factors(n / prime));
            vec
        }
    }
}
