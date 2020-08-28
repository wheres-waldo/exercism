fn eratosthenes(sieve: Vec<u32>) -> Vec<u32> {
    let mut answer = cross_out(sieve);
    answer.insert(0, 2);
    answer
}

fn cross_out(sieve: Vec<u32>) -> Vec<u32> {
    let end = (*sieve.last().unwrap() as f32).sqrt().trunc() as u32;

    let mut answer = sieve.clone();
    let mut current = *sieve.first().unwrap();
    let mut i = 1;

    while current <= end {
        let (mut l, r): (Vec<_>, Vec<_>) =
            answer.iter().copied().partition(|&x| x < current.pow(2));
        let sifted: Vec<_> = r.iter().copied().filter(|&x| x % current != 0).collect();

        l.extend_from_slice(&sifted);
        answer = l;
        current = *answer.get(i).unwrap();
        i += 1;
    }

    answer
}

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => {
            let mut s = 2 * n ;
            let mut primes = eratosthenes((3..=s).step_by(2).collect());
            let mut n_primes = primes.len() as u32;

            while n_primes <= n {
                s += n;
                primes = eratosthenes((3..=s).step_by(2).collect());
                n_primes = primes.len() as u32;
            }

            primes[n as usize]
        }
    }
}
