/// This function implements a linear time complexity prime sieve that identifies all prime numbers
/// up to the given limit.
///
/// # Arguments
///
/// * `until` - The upper bound (exclusive) up to which to generate the prime sieve
///
/// # Returns
///
/// A tuple containing:
/// * A vector of booleans where `true` indicates the index is prime
/// * A vector of prime numbers found up to the given limit
///
/// # Example
///
/// ```
/// use prime::prime_sieve;
///
/// let (is_prime, primes) = prime_sieve(10);
/// assert_eq!(primes, vec![2, 3, 5, 7]);
/// assert_eq!(is_prime[2], true);
/// assert_eq!(is_prime[4], false);
/// ```
pub fn prime_sieve(until: u32) -> (Vec<bool>, Vec<u32>) {
    let mut primes = Vec::new();
    let mut is_prime = vec![true; until as usize];
    is_prime.iter_mut().take(2).for_each(|p| *p = false);

    for i in 2..until {
        if is_prime[i as usize] {
            primes.push(i);
        }
        for p in &primes {
            let x = i * p;
            if x >= until {
                break;
            }
            is_prime[x as usize] = false;
            if i % p == 0 {
                break;
            }
        }
    }

    (is_prime, primes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force_prime_sieve(until: u32) -> (Vec<bool>, Vec<u32>) {
        let is_prime = (0..until)
            .map(|n| n >= 2 && (2..n).all(|d| n % d != 0))
            .collect::<Vec<_>>();
        let primes = is_prime
            .iter()
            .enumerate()
            .filter(|(_, &is)| is)
            .map(|(i, _)| i as u32)
            .collect();
        (is_prime, primes)
    }

    #[test]
    fn test_prime_sieve() {
        for i in 0..100 {
            assert_eq!(prime_sieve(i), brute_force_prime_sieve(i));
        }
        for i in (100..1000).step_by(100) {
            assert_eq!(prime_sieve(i), brute_force_prime_sieve(i));
        }
    }
}
