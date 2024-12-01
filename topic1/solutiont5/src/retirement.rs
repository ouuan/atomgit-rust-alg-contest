use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::sync::LazyLock;

static RULE_MAP: LazyLock<HashMap<&'static str, Box<dyn Rule + Send + Sync>>> =
    LazyLock::new(|| {
        [
            (
                "男职工",
                Box::new(LinearRule {
                    original: Time::from_years(60),
                    birth_since: Time::new(1965, 1),
                    birth_until: Time::new(1976, 12),
                    rate: 0.25,
                }) as Box<dyn Rule + Send + Sync>,
            ),
            (
                "原法定退休年龄50周岁女职工",
                Box::new(LinearRule {
                    original: Time::from_years(50),
                    birth_since: Time::new(1975, 1),
                    birth_until: Time::new(1984, 12),
                    rate: 0.5,
                }),
            ),
            (
                "原法定退休年龄55周岁女职工",
                Box::new(LinearRule {
                    original: Time::from_years(55),
                    birth_since: Time::new(1970, 1),
                    birth_until: Time::new(1981, 12),
                    rate: 0.25,
                }),
            ),
        ]
        .into_iter()
        .collect()
    });

pub fn retire_time(time: &str, tp: &str) -> String {
    let birth = time.parse().expect("invalid birth time");
    let rule = RULE_MAP.get(tp).expect("invalid type");
    let delay = rule.delay(birth);
    let retire_age = rule.original() + delay;
    let retire = birth + retire_age;
    format!(
        "{},{},{}",
        retire,
        format!("{:.2}", retire_age.years())
            .trim_end_matches('0')
            .trim_end_matches('.'),
        delay.months
    )
}

const MONTHS_PER_YEAR: i32 = 12;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Time {
    months: i32,
}

impl Time {
    const fn new(years: i32, months: i32) -> Self {
        Time {
            months: years * MONTHS_PER_YEAR + months,
        }
    }

    const fn from_years(years: i32) -> Self {
        Time {
            months: years * MONTHS_PER_YEAR,
        }
    }

    const fn from_months(months: i32) -> Self {
        Time { months }
    }

    const fn zero() -> Self {
        Time { months: 0 }
    }

    const fn years(&self) -> f32 {
        self.months as f32 / MONTHS_PER_YEAR as f32
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_months(self.months - rhs.months)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_months(self.months + rhs.months)
    }
}

impl FromStr for Time {
    type Err = Cow<'static, str>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let year = parts
            .next()
            .ok_or("invalid format")?
            .parse()
            .map_err(|e| format!("{e}"))?;
        let month = parts
            .next()
            .ok_or("invalid format")?
            .parse()
            .map_err(|e| format!("{e}"))?;
        Ok(Time::new(year, month))
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let months_minus_one = self.months - 1;
        write!(
            f,
            "{:04}-{:02}",
            months_minus_one / MONTHS_PER_YEAR,
            months_minus_one % MONTHS_PER_YEAR + 1
        )
    }
}

trait Rule {
    fn original(&self) -> Time;

    fn delay(&self, birth: Time) -> Time;
}

struct LinearRule {
    original: Time,
    birth_since: Time,
    birth_until: Time,
    rate: f32,
}

impl Rule for LinearRule {
    fn original(&self) -> Time {
        self.original
    }

    fn delay(&self, birth: Time) -> Time {
        if birth < self.birth_since {
            return Time::zero();
        }
        let birth_diff = birth.min(self.birth_until) - self.birth_since;
        Time::from_months((birth_diff.months as f32 * self.rate) as i32 + 1)
    }
}
