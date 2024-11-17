mod prime_factor;

fn main() {
    let number = 100;
    let res = prime_factor::find_max_prime_factor(number);
    println!("{number}'s max prime factor: {res}");
}
