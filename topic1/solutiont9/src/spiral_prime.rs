use prime::miller_rabin_64;

pub fn min_edge_prime_num(number: u32) -> String {
    let mut prime_count = 0;
    let mut total = 1;
    let mut n = 1;

    for step in (2..).step_by(2) {
        for _ in 0..3 {
            n += step;
            if miller_rabin_64(n) {
                prime_count += 1;
            }
        }
        n += step; // now n is a square number, skipping primality test
        total += 4;
        if prime_count * 100 < total * number {
            return format!("{},{}", step + 1, prime_count);
        }
    }

    unreachable!()
}
