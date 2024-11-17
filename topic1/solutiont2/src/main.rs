mod district;

fn main() {
    let provinces = district::count_provinces();
    println!("provinces: {provinces}");
}
