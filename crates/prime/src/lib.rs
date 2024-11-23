use rand::prelude::*;
use std::iter;

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

const fn qmul(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut ans = 0;
    while b > 0 {
        if b & 1 == 1 {
            ans = (ans + a) % m;
        }
        a = (a + a) % m;
        b >>= 1;
    }
    ans
}

const fn qpow_128(mut a: u128, mut b: u128, m: u128) -> u128 {
    let mut ans = 1;
    while b > 0 {
        if b & 1 == 1 {
            ans = qmul(ans, a, m);
        }
        a = qmul(a, a, m);
        b >>= 1;
    }
    ans
}

/// Test if `n` is a prime number using the Miller-Rabin primality test.
///
/// # Example
///
/// ```
/// use prime::miller_rabin_128;
///
/// assert_eq!(miller_rabin_128(1), false);
/// assert_eq!(miller_rabin_128(2), true);
/// assert_eq!(miller_rabin_128(91), false);
/// assert_eq!(miller_rabin_128(998244353), true);
/// ```
pub fn miller_rabin_128(n: u128) -> bool {
    const TEST_TIMES: usize = 16;
    match n {
        0 | 1 => return false,
        2 | 3 => return true,
        _ => {}
    }
    let d = n - 1;
    let t = d.trailing_zeros();
    let u = d >> t;
    let mut rng = thread_rng();
    (0..TEST_TIMES).all(|_| {
        let a = rng.gen_range(2..=n - 2);
        let mut v = qpow_128(a, u, n);
        if v == 1 || v == n - 1 {
            return true;
        }
        (1..t).any(|_| {
            v = qmul(v, v, n);
            v == n - 1
        })
    })
}

const fn qpow_64(a: u64, mut b: u64, m: u64) -> u64 {
    let mut a = a as u128;
    let m = m as u128;
    let mut ans = 1;
    while b > 0 {
        if b & 1 == 1 {
            ans = ans * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    ans as u64
}

/// Much faster version of `miller_rabin_128` for `u64`.
pub fn miller_rabin_64(n: u64) -> bool {
    const TESTERS: [u64; 7] = [2, 325, 9375, 28178, 450775, 9780504, 1795265022];
    match n {
        0 | 1 => return false,
        2 | 3 => return true,
        _ => {}
    }
    let d = n - 1;
    let t = d.trailing_zeros();
    let u = d >> t;
    TESTERS.iter().all(|&a| {
        let a = a % n;
        if a == 0 || a == 1 || a == n - 1 {
            return true;
        }
        let mut v = qpow_64(a, u, n) as u128;
        if v == 1 || v as u64 == n - 1 {
            return true;
        }
        (1..t).any(|_| {
            v = v * v % n as u128;
            v as u64 == n - 1
        })
    })
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn pollard_rho<R: Rng>(n: u128, rng: &mut R) -> u128 {
    let c = rng.gen_range(1..n);
    let mut t = 0;
    for len in iter::successors(Some(2), |x| Some(x * 2)) {
        let s = t;
        let mut prod = 1;
        for step in 1..=len {
            t = (qmul(t, t, n) + c) % n;
            prod = qmul(prod, t.abs_diff(s), n);
            if step % 127 == 0 || step == len {
                let d = gcd(prod, n);
                if d > 1 {
                    return d;
                }
            }
        }
    }
    unreachable!()
}

/// Find the maximum prime factor of `n`. Returns `1` for 0 and 1.
///
/// # Example
///
/// ```
/// use prime::max_factor;
///
/// assert_eq!(max_factor(0), 1);
/// assert_eq!(max_factor(1), 1);
/// assert_eq!(max_factor(6), 3);
/// assert_eq!(max_factor(37), 37);
/// assert_eq!(max_factor(91), 13);
/// ```
pub fn max_factor(n: u128) -> u128 {
    let mut max = 1;
    factor_with_max(n, &mut max);
    max
}

fn factor_with_max(n: u128, max: &mut u128) {
    if n <= *max {
        return;
    }
    if miller_rabin_128(n) {
        *max = n;
        return;
    }
    let mut rng = thread_rng();
    let d = iter::repeat_with(|| pollard_rho(n, &mut rng))
        .find(|&d| d != n)
        .unwrap();
    let e = iter::successors(
        Some(n / d),
        |&x| if x % d == 0 { Some(x / d) } else { None },
    )
    .last()
    .unwrap();
    factor_with_max(d, max);
    factor_with_max(e, max);
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
        for i in (100..10000).step_by(100) {
            assert_eq!(prime_sieve(i), brute_force_prime_sieve(i));
        }
    }

    #[test]
    fn test_miller_rabin() {
        let (is_prime, _) = prime_sieve(200000);
        for (i, &is_prime) in is_prime.iter().enumerate() {
            assert_eq!(miller_rabin_128(i as u128), is_prime);
        }
    }

    #[test]
    fn miller_rabin_128_perf() {
        let mut rng = thread_rng();
        for _ in 0..2000 {
            let n = rng.gen_range(0..=u128::MAX);
            miller_rabin_128(n);
        }
    }

    #[test]
    fn miller_rabin_64_perf() {
        let mut rng = thread_rng();
        for _ in 0..1000000 {
            let n = rng.gen_range(0..=u64::MAX);
            miller_rabin_64(n);
        }
    }

    #[test]
    fn pollard_rho_perf() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let n = rng.gen_range(2..1 << 80);
            if !miller_rabin_128(n) {
                pollard_rho(n, &mut rng);
            }
        }
    }

    fn brute_force_max_factor(n: u32, is_prime: &[bool], primes: &[u32]) -> u32 {
        let mut x = n;
        for &p in primes {
            while x % p == 0 {
                x /= p;
            }
            if x == 1 {
                return p;
            }
            if is_prime[x as usize] {
                return x;
            }
        }
        unreachable!()
    }

    #[test]
    fn test_max_factor() {
        const N: u32 = 50000;
        let (is_prime, primes) = prime_sieve(N);
        for i in 2..N {
            assert_eq!(
                max_factor(i as u128) as u32,
                brute_force_max_factor(i, &is_prime, &primes),
                "failed for {i}"
            );
        }
    }

    #[test]
    fn max_factor_perf() {
        assert_eq!(max_factor(680207505711764507977), 33941217091);
        assert_eq!(max_factor(739551621235312646143), 30756477229);
        assert_eq!(max_factor(3183627274266828089483), 57996878441);
    }
}
