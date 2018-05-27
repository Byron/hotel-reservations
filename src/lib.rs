#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use failure::{err_msg, Error, ResultExt};
use std::{io::{self, BufRead, BufReader, BufWriter, Write}, str::FromStr};

pub fn answers(db: &HotelDb, input: impl io::Read, output: impl io::Write) -> Result<(), Error> {
    let mut output = BufWriter::new(output);
    for line in BufReader::new(input).lines() {
        let line = line.with_context(|_err| "Could not read line with bookings")?;
        let (customer_type, dates) = parse_booking(&line)?;
        let hotel = db.0
            .iter()
            .map(|h| {
                (
                    h,
                    (h.rate(customer_type, &dates), ::std::u32::MAX - h.rating),
                )
            })
            .min_by_key(|&(_, rate_and_rating)| rate_and_rating)
            .map(|(h, _)| h)
            .ok_or_else(|| format_err!("Cannot find rating without any dates: '{}'", line))?;
        writeln!(output, "{}", hotel.name)?;
    }
    Ok(())
}

fn parse_booking(input: &str) -> Result<(CustomerKind, Vec<Date>), Error> {
    const CUSTOMER_TYPE_OFFSET: usize = 9;
    if input.len() < CUSTOMER_TYPE_OFFSET {
        bail!("Input '{}' does not even contain the customer type", input)
    }

    use self::CustomerKind::*;
    let customer_type = match &input[..CUSTOMER_TYPE_OFFSET] {
        "Rewards: " => Rewards,
        "Regular: " => Regular,
        invalid_input => bail!(
            "Could not understand customer type from input '{}'",
            invalid_input
        ),
    };

    Ok((
        customer_type,
        input[9..]
            .split(", ")
            .map(|t| t.parse())
            .collect::<Result<_, _>>()?,
    ))
}

struct Date {
    _day: u8,
    _month: u8,
    _year: u16,
    weekday: Weekday,
}

impl FromStr for Date {
    type Err = Error;

    fn from_str(s: &str) -> Result<Date, Error> {
        const EXPECTED_LEN: usize = 12;
        if s.len() < EXPECTED_LEN {
            bail!("Cannot parse date from '{}' - invalid length.", s)
        }
        const DAY_LEN: usize = 2;
        const MONTH_LEN: usize = 3;
        const YEAR_LEN: usize = 4;
        const MONTH_END: usize = DAY_LEN + MONTH_LEN;
        const YEAR_END: usize = MONTH_END + YEAR_LEN;

        Ok::<_, Error>(Date {
            _day: s[..DAY_LEN]
                .parse::<u8>()
                .with_context(|_e| "Could not parse day")?,
            _month: month_from(&s[DAY_LEN..MONTH_END])?,
            _year: s[MONTH_END..YEAR_END]
                .parse::<u16>()
                .with_context(|_e| "Could not parse year")?,
            weekday: s[YEAR_END + 1..s.find(')').ok_or_else(|| {
                           err_msg("Could not find closing bracket in weekday part")
                       })?].parse()?,
        }).with_context(|_err| format!("Could not parse date from '{}'", s))
            .map_err(Into::into)
    }
}

fn month_from(s: &str) -> Result<u8, Error> {
    Ok(match s {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        unknown => bail!("Unknown month '{}'", unknown),
    })
}

#[derive(Copy, Clone)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl FromStr for Weekday {
    type Err = Error;

    fn from_str(s: &str) -> Result<Weekday, Error> {
        use self::Weekday::*;
        Ok(match s {
            "mon" => Monday,
            "tues" => Tuesday,
            "wed" => Wednesday,
            "thur" => Thursday,
            "fri" => Friday,
            "sat" => Saturday,
            "sun" => Sunday,
            _ => bail!("Invalid day: '{}'", s),
        })
    }
}

#[derive(Clone, Copy)]
enum CustomerKind {
    Rewards,
    Regular,
}

#[derive(Deserialize)]
pub struct HotelDb(Vec<Hotel>);

#[derive(Deserialize)]
pub struct Hotel {
    name: String,
    rating: u32,
    rates: RatePerCustomer,
}

impl Hotel {
    fn rate(&self, customer: CustomerKind, dates: &[Date]) -> u32 {
        dates.iter().fold(0, |acc, d| {
            acc + self.rates.matching(customer).matching(d.weekday)
        })
    }
}

#[derive(Deserialize)]
pub struct RatePerCustomer {
    regular: Rate,
    rewards: Rate,
}

impl RatePerCustomer {
    fn matching(&self, customer: CustomerKind) -> &Rate {
        use self::CustomerKind::*;
        match customer {
            Regular => &self.regular,
            Rewards => &self.rewards,
        }
    }
}

#[derive(Deserialize)]
pub struct Rate {
    weekday: u32,
    weekend: u32,
}

impl Rate {
    fn matching(&self, day: Weekday) -> u32 {
        use self::Weekday::*;
        match day {
            Monday | Tuesday | Wednesday | Thursday | Friday => self.weekday,
            Saturday | Sunday => self.weekend,
        }
    }
}
