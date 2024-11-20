use prime::prime_sieve;

pub fn goldbach_conjecture() -> u64 {
    let mut cases = Vec::new();

    let mut l = 2;

    // Solve by binary lifting.
    // Time complexity: O(n * sqrt(n)), where n is the second target number.
    while cases.len() < 2 {
        let r = l * 2;
        cases.extend(solve(l, r));
        l = r;
    }

    (cases[0] + cases[1]) as u64
}

/// Find all numbers in [l, r) that cannot be expressed as the sum of a prime and twice a square.
/// Time complexity: O(r + (r-l) * sqrt(r))
fn solve(l: u32, r: u32) -> impl Iterator<Item = u32> {
    let (is_prime, _) = prime_sieve(r);
    (l..r).filter(move |&i| {
        if i % 2 == 0 {
            return false;
        }
        for j in 0.. {
            let x = 2 * j * j;
            if x >= i {
                break;
            }
            if is_prime[(i - x) as usize] {
                return false;
            }
        }
        true
    })
}
