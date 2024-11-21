use date::{lunar::date_of_chinese_new_year, Date};

pub fn time_info(time: &str) -> String {
    let date: Date = time.parse().expect("invalid date");
    let week_of_year = date.week_of_year();
    let day_of_year = date.day_of_year();
    let days_remain = date.days_remaining_in_year();
    let this_new_year = date_of_chinese_new_year(date.year()).unwrap();
    let days_to_new_year = if date < this_new_year {
        this_new_year.day_of_year() - day_of_year
    } else {
        let next_new_year = date_of_chinese_new_year(date.year() + 1).unwrap();
        next_new_year.day_of_year() + days_remain
    } - 1;
    format!("{},{},{}", week_of_year, days_remain, days_to_new_year)
}
