use resident_id::ResidentId;

pub fn check_id_card(id_card_no: &str) -> String {
    match id_card_no.parse::<ResidentId>() {
        Ok(id) => format!(
            "身份证号码正确,{},{:04}年{:02}月{:02}日,{}-{}-{}",
            id.gender(),
            id.birthday().year(),
            id.birthday().month(),
            id.birthday().day(),
            id.province(),
            id.city(),
            id.area(),
        ),
        Err(_) => "身份证号码错误".into(),
    }
}
