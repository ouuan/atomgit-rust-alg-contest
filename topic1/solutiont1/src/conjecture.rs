pub fn goldbach_conjecture() -> u64 {
    let mut cases = Vec::new();

    // only handle odd numbers
    let mut primes = Vec::new();
    let mut is_prime_odd = vec![false];

    for i in (3..).step_by(2) {
        let mut is_prime = true;
        for &p in &primes {
            if i % p == 0 {
                is_prime = false;
                break;
            }
            if i / p < p {
                break;
            }
        }
        is_prime_odd.push(is_prime);
        if is_prime {
            primes.push(i);
            continue;
        }
        for j in 1.. {
            let x = 2 * j * j;
            if x >= i {
                cases.push(i);
                break;
            }
            if is_prime_odd[(i - x) as usize / 2] {
                break;
            }
        }
        if cases.len() == 2 {
            return cases.iter().sum();
        }
    }

    unreachable!()
}
