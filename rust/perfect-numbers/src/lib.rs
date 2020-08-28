use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => Some(match factors(num).iter().sum::<u64>().cmp(&num) {
            Ordering::Less => Classification::Deficient,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Abundant,
        }),
    }
}

fn factors(n: u64) -> HashSet<u64> {
    (1..(n as f64).sqrt() as u64 + 1)
        .filter(|i| n % i == 0)
        .fold(HashSet::new(), |mut factors, i| {
            factors.insert(i);
            if i != 1 {
                factors.insert(n / i);
            }
            factors
        })
}
