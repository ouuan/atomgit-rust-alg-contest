mod calc_time;

fn main() {
    let time_infos = calc_time::time_info("2024-11-18");
    println!("time info: {time_infos}");
    // println!("Week {}, left {} day(s) this year, {} day(s) after is spring festival", );
}
