use date::Date;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

const DIVISION_DATA: &str = include_str!("../data/administrative-divisions.txt");

static DIVISION_MAP: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
    DIVISION_DATA
        .lines()
        .map(|line| {
            let (code, name) = line.split_at(6);
            (code.parse().unwrap(), name)
        })
        .collect()
});

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IdType {
    V1,
    V2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Male => write!(f, "男"),
            Self::Female => write!(f, "女"),
        }
    }
}

#[derive(Debug)]
pub struct ResidentId {
    id_type: IdType,
    province: &'static str,
    city: &'static str,
    area: &'static str,
    birthday: Date,
    serial: u32,
    gender: Gender,
}

impl ResidentId {
    pub fn id_type(&self) -> IdType {
        self.id_type
    }

    pub fn province(&self) -> &'static str {
        self.province
    }

    pub fn city(&self) -> &'static str {
        self.city
    }

    pub fn area(&self) -> &'static str {
        self.area
    }

    pub fn birthday(&self) -> Date {
        self.birthday
    }

    pub fn serial(&self) -> u32 {
        self.serial
    }

    pub fn gender(&self) -> Gender {
        self.gender
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdError {
    InvalidLength,
    InvalidCharacter,
    AreaNotFound,
    InvalidDate,
    ChecksumMismatch,
}

// caller should ensure that the input length is 18
fn validate_checksum(s: &str) -> Result<(), IdError> {
    const WEIGHTS: [u32; 17] = [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
    let mut chars = s.chars();
    let sum = (&mut chars)
        .take(WEIGHTS.len()) // don't consume the last character
        .zip(WEIGHTS)
        .map(|(c, w)| {
            c.to_digit(10)
                .ok_or(IdError::InvalidCharacter)
                .map(|x| x * w)
        })
        .sum::<Result<u32, _>>()?;
    let checksum = match sum % 11 {
        0 => '1',
        1 => '0',
        2 => 'X',
        x => std::char::from_digit(12 - x, 10).unwrap(),
    };
    if chars.next().unwrap() == checksum {
        Ok(())
    } else {
        Err(IdError::ChecksumMismatch)
    }
}

impl FromStr for ResidentId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id_type, birth_year, birth_month, birth_day, serial) = match s.len() {
            15 => {
                if !s.chars().all(|c| c.is_ascii_digit()) {
                    return Err(IdError::InvalidCharacter);
                }
                let partial_year = s[6..8].parse::<u32>().unwrap();
                let birth_month = s[8..10].parse().unwrap();
                let birth_day = s[10..12].parse().unwrap();
                let serial = s[12..15].parse().unwrap();
                let birth_year = if serial >= 996 {
                    partial_year + 1800
                } else {
                    partial_year + 1900
                };
                (IdType::V1, birth_year, birth_month, birth_day, serial)
            }
            18 => {
                validate_checksum(s)?;
                // invalid characters are already checked by `validate_checksum`
                let birth_year = s[6..10].parse().unwrap();
                let birth_month = s[10..12].parse().unwrap();
                let birth_day = s[12..14].parse().unwrap();
                let serial = s[14..17].parse().unwrap();
                (IdType::V2, birth_year, birth_month, birth_day, serial)
            }
            _ => return Err(IdError::InvalidLength),
        };
        let code = s[0..6].parse().map_err(|_| IdError::InvalidCharacter)?;
        let province = DIVISION_MAP
            .get(&(code - code % 10000))
            .ok_or(IdError::AreaNotFound)?;
        let city = if let Some(city) = DIVISION_MAP.get(&(code - code % 100)) {
            city
        } else {
            match code / 100 % 100 {
                1 => "市辖区",
                2 => "县",
                90 if province.ends_with("省") => "省直辖县级行政区划",
                90 if province.ends_with("自治区") => "自治区直辖县级行政区划",
                _ => return Err(IdError::AreaNotFound),
            }
        };
        let area = DIVISION_MAP.get(&code).ok_or(IdError::AreaNotFound)?;
        let birthday =
            Date::try_new(birth_year, birth_month, birth_day).map_err(|_| IdError::InvalidDate)?;
        let gender = if serial % 2 == 0 {
            Gender::Female
        } else {
            Gender::Male
        };
        Ok(Self {
            id_type,
            province,
            city,
            area,
            birthday,
            serial,
            gender,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_hierarchy() {
        for (&code, area) in DIVISION_MAP.iter() {
            let province_code = code - code % 10000;
            let province = DIVISION_MAP
                .get(&province_code)
                .expect("province not found");
            let city_code = code - code % 100;
            if DIVISION_MAP.get(&city_code).is_none() {
                match code / 100 % 100 {
                    1 => assert!(
                        area.ends_with("区") || area.ends_with("城"),
                        "{code} {area}"
                    ),
                    2 => assert!(area.ends_with("县"), "{code} {area}"),
                    90 => assert!(
                        province.ends_with("省") || province.ends_with("自治区"),
                        "{code} {area} ({province})"
                    ),
                    _ => panic!("unknown city: {code} {area}"),
                }
            }
        }
    }

    #[test]
    fn test_area() {
        let id: ResidentId = "11010119900101004X".parse().unwrap();
        assert_eq!(id.province(), "北京市");
        assert_eq!(id.city(), "市辖区");
        assert_eq!(id.area(), "东城区");
        let id: ResidentId = "130102199001010015".parse().unwrap();
        assert_eq!(id.province(), "河北省");
        assert_eq!(id.city(), "石家庄市");
        assert_eq!(id.area(), "长安区");
        let id: ResidentId = "50024219000101001X".parse().unwrap();
        assert_eq!(id.province(), "重庆市");
        assert_eq!(id.city(), "县");
        assert_eq!(id.area(), "酉阳土家族苗族自治县");
        let id: ResidentId = "419001190102030042".parse().unwrap();
        assert_eq!(id.province(), "河南省");
        assert_eq!(id.city(), "省直辖县级行政区划");
        assert_eq!(id.area(), "济源市");
        let id: ResidentId = "659011191111111110".parse().unwrap();
        assert_eq!(id.province(), "新疆维吾尔自治区");
        assert_eq!(id.city(), "自治区直辖县级行政区划");
        assert_eq!(id.area(), "新星市");
    }

    #[test]
    fn test_birth_year() {
        let id: ResidentId = "11010119900101004X".parse().unwrap();
        assert_eq!(id.birthday().year(), 1990);
        let id: ResidentId = "110101900101001".parse().unwrap();
        assert_eq!(id.birthday().year(), 1990);
        let id: ResidentId = "110101900101996".parse().unwrap();
        assert_eq!(id.birthday().year(), 1890);
        let id: ResidentId = "110101000101995".parse().unwrap();
        assert_eq!(id.birthday().year(), 1900);
    }

    #[test]
    fn test_error() {
        let err: IdError = "110101199001010040".parse::<ResidentId>().unwrap_err();
        assert_eq!(err, IdError::ChecksumMismatch);
        let err: IdError = "1101011990010100410".parse::<ResidentId>().unwrap_err();
        assert_eq!(err, IdError::InvalidLength);
        let err: IdError = "123测试一下".parse::<ResidentId>().unwrap_err();
        assert_eq!(err, IdError::InvalidCharacter);
        let err: IdError = "110101000229001".parse::<ResidentId>().unwrap_err();
        assert_eq!(err, IdError::InvalidDate);
        let err: IdError = "114514010101001".parse::<ResidentId>().unwrap_err();
        assert_eq!(err, IdError::AreaNotFound);
    }
}
