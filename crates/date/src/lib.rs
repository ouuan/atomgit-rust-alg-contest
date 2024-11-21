pub mod lunar;

use std::borrow::Cow;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u32,
    month: u32,
    day: u32,
}

impl Date {
    pub const fn try_new(year: u32, month: u32, day: u32) -> Result<Self, &'static str> {
        if year < 1583 {
            return Err("year out of range");
        }
        if month < 1 || month > 12 {
            return Err("month out of range");
        }
        if day < 1 || day > days_in_month(year, month) {
            return Err("day out of range");
        }
        Ok(Date { year, month, day })
    }

    /// # Safety
    ///
    /// The caller should ensure that the date is in range.
    pub const unsafe fn new_unchecked(year: u32, month: u32, day: u32) -> Self {
        Date { year, month, day }
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn day_of_year(&self) -> u32 {
        const fn accumulated_days_before_months() -> [u32; 12] {
            let mut days = [0; 12];
            let mut month = 1;
            while month <= 11 {
                days[month as usize] = days[month as usize - 1] + days_in_month(2001, month);
                month += 1;
            }
            days
        }
        const TABLE: [u32; 12] = accumulated_days_before_months();

        let leap = if is_leap_year(self.year) && self.month > 2 {
            1
        } else {
            0
        };
        TABLE[self.month as usize - 1] + self.day + leap
    }

    pub fn days_remaining_in_year(&self) -> u32 {
        days_in_year(self.year) - self.day_of_year()
    }

    pub fn day_of_week(&self) -> u32 {
        let mut year = self.year;
        let mut month = self.month;
        if month < 3 {
            year -= 1;
            month += 12;
        }
        (self.day + 2 * month + (3 * (month + 1) / 5) + year + year / 4 - year / 100 + year / 400)
            % 7
    }

    /// ISO week number
    pub fn week_of_year(&self) -> u32 {
        // use Thursday of the current week to determine the year
        let current_day_of_year = self.day_of_year() as i32 + 3 - self.day_of_week() as i32;
        let target_year;
        let target_day_of_year;
        if current_day_of_year <= 0 {
            target_year = self.year - 1;
            target_day_of_year = (current_day_of_year + days_in_year(target_year) as i32) as u32;
        } else if current_day_of_year as u32 > days_in_year(self.year) {
            target_year = self.year + 1;
            target_day_of_year = current_day_of_year as u32 - days_in_year(self.year);
        } else {
            target_year = self.year;
            target_day_of_year = current_day_of_year as u32;
        }
        let first_day_of_week = unsafe { Date::new_unchecked(target_year, 1, 1) }.day_of_week();
        let delta = if first_day_of_week <= 3 {
            first_day_of_week + 6
        } else {
            first_day_of_week - 1
        };
        (target_day_of_year + delta) / 7
    }
}

const fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

const fn is_leap_year(year: u32) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

const fn days_in_year(year: u32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

impl FromStr for Date {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let year = parts
            .next()
            .ok_or("missing year part")?
            .parse()
            .map_err(|e| format!("{e}"))?;
        let month = parts
            .next()
            .ok_or("missing month part")?
            .parse()
            .map_err(|e| format!("{e}"))?;
        let day = parts
            .next()
            .ok_or("missing day part")?
            .parse()
            .map_err(|e| format!("{e}"))?;
        if parts.next().is_some() {
            return Err("extra parts".into());
        }
        Date::try_new(year, month, day).map_err(Cow::Borrowed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc() {
        let date = Date::from_str("2024-11-21").unwrap();
        assert_eq!(date.day_of_year(), 326);
        assert_eq!(date.days_remaining_in_year(), 40);
        assert_eq!(date.day_of_week(), 3);
        let date = Date::try_new(2000, 12, 25).unwrap();
        assert_eq!(date.day_of_year(), 360);
        assert_eq!(date.days_remaining_in_year(), 6);
        assert_eq!(date.day_of_week(), 0);
        let date = Date::try_new(2077, 1, 24).unwrap();
        assert_eq!(date.day_of_year(), 24);
        assert_eq!(date.days_remaining_in_year(), 341);
        assert_eq!(date.day_of_week(), 6);
        let date = Date::try_new(2100, 4, 17).unwrap();
        assert_eq!(date.day_of_year(), 107);
        assert_eq!(date.days_remaining_in_year(), 258);
        assert_eq!(date.day_of_week(), 5);
    }

    #[test]
    fn test_week_of_year() {
        let date = Date::try_new(2025, 1, 1).unwrap();
        assert_eq!(date.week_of_year(), 1);
        let date = Date::try_new(2024, 12, 30).unwrap();
        assert_eq!(date.week_of_year(), 1);
        let date = Date::try_new(2023, 1, 1).unwrap();
        assert_eq!(date.week_of_year(), 52);
        let date = Date::try_new(2021, 1, 3).unwrap();
        assert_eq!(date.week_of_year(), 53);
        let date = Date::try_new(2021, 1, 4).unwrap();
        assert_eq!(date.week_of_year(), 1);
    }

    #[test]
    fn test_err() {
        assert!(Date::try_new(1582, 1, 1).is_err());
        assert!(Date::try_new(2000, 2, 29).is_ok());
        assert!(Date::try_new(1900, 2, 29).is_err());
        assert!(Date::from_str("2024-1-2").is_ok());
        assert!(Date::from_str("2024-1-00").is_err());
        assert!(Date::from_str("2024-11-21-").is_err());
        assert!(Date::from_str("2024/11/21").is_err());
    }
}
