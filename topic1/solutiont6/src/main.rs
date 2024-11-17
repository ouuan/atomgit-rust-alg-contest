mod simple2traditional;

fn main() {
    let input = "老板";
    let tp = "s2t";
    let res = simple2traditional::converter(input, tp);
    println!("{res}");
}
