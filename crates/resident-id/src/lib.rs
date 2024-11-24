use date::Date;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

const DIVISION_DATA: &str = include_str!("../data/administrative-divisions.txt");

static DIVISION_MAP: LazyLock<HashMap<u32, &str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    for line in DIVISION_DATA.lines() {
        let (code, name) = line.split_at(6);
        let code = code.parse().unwrap();
        map.insert(code, name);
    }
    map
});

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IdType {
    V1,
    V2,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
        .zip(WEIGHTS.iter())
        .take(WEIGHTS.len()) // don't consume the last character
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
                let birth_year = s[6..8].parse::<u32>().or(Err(IdError::InvalidCharacter))? + 1900;
                let birth_month = s[8..10].parse().or(Err(IdError::InvalidCharacter))?;
                let birth_day = s[10..12].parse().or(Err(IdError::InvalidCharacter))?;
                let serial = s[12..15].parse().or(Err(IdError::InvalidCharacter))?;
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
        let code = s[0..6].parse().or(Err(IdError::InvalidCharacter))?;
        let province = DIVISION_MAP
            .get(&(code - code % 10000))
            .ok_or(IdError::AreaNotFound)?;
        let city = DIVISION_MAP
            .get(&(code - code % 100))
            .copied()
            .or(match code / 100 % 100 {
                1 => Some("市辖区"),
                2 => Some("县"),
                90 => Some("省直辖县级行政区划"),
                _ => None,
            })
            .ok_or(IdError::AreaNotFound)?;
        let area = DIVISION_MAP.get(&code).ok_or(IdError::AreaNotFound)?;
        let birthday =
            Date::try_new(birth_year, birth_month, birth_day).or(Err(IdError::InvalidDate))?;
        let gender = match serial % 2 {
            0 => Gender::Female,
            1 => Gender::Male,
            _ => unreachable!(),
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
            assert!(DIVISION_MAP.get(&province_code).is_some());
            let city_code = code - code % 100;
            if DIVISION_MAP.get(&city_code).is_none() {
                match code / 100 % 100 {
                    1 => assert!(
                        area.ends_with("区") || area.ends_with("城"),
                        "{code} {area}"
                    ),
                    2 => assert!(area.ends_with("县"), "{code} {area}"),
                    90 => (),
                    _ => panic!("unknown city: {code} {area}"),
                }
            }
        }
    }
}
