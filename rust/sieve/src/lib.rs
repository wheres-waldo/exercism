pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    match upper_bound {
        n if n < 2 => vec![],
        2 => vec![2],
        _ => eratosthenes((3..=upper_bound).step_by(2).collect()),
    }
}

fn eratosthenes(sieve: Vec<u64>) -> Vec<u64> {
    let mut answer = cross_out(sieve);
    answer.insert(0, 2);
    answer
}

fn cross_out(sieve: Vec<u64>) -> Vec<u64> {
    let end = (*sieve.last().unwrap() as f64).sqrt().trunc() as u64;

    let mut answer = sieve.clone();
    let mut current = *sieve.first().unwrap();
    let mut i = 1;

    while current <= end {
        let (mut l, r): (Vec<_>, Vec<_>) =
            answer.into_iter().partition(|&x| x < current.pow(2));
        let sifted: Vec<_> = r.into_iter().filter(|&x| x % current != 0).collect();

        l.extend_from_slice(&sifted);
        answer = l;
        current = *answer.get(i).unwrap();
        i += 1;
    }

    answer
}
