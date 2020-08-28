use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set_of_multiples = HashSet::new();

    for &factor in factors.iter().filter(|x| **x != 0) {
        set_of_multiples.extend(
            (1..)
                .map(|x| x * factor)
                .take_while(|&x| x < limit)
                .collect::<HashSet<u32>>(),
        );
    }

    set_of_multiples.iter().sum()
}
