mod zuc_encryption;

fn main() {
    let input = String::from("love");
    let result = zuc_encryption::encryption(input);
    println!("{result}");
}
