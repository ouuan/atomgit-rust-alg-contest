mod identity_card;

fn main() {
    let id_no = "xx";
    let res = identity_card::check_id_card(id_no);
    println!("{res}");
}
