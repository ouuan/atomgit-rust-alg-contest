mod conjecture;

fn main() {
    let sum = conjecture::goldbach_conjecture();
    println!("top 10 goldbach's conjecture on primes: {sum}");
}
