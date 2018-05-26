extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use failure::Error;
use std::io::{self, BufReader};

pub fn answers(
    _db: &HotelDb,
    input: impl io::Read,
    mut _output: impl io::Write,
) -> Result<(), Error> {
    let _input = BufReader::new(input);
    unimplemented!();
}

#[derive(Deserialize)]
pub struct HotelDb(Vec<Hotel>);

#[derive(Deserialize)]
pub struct Hotel {
    name: String,
    rating: u32,
    rates: RatePerCustomer,
}

#[derive(Deserialize)]
pub struct RatePerCustomer {
    regular: Rate,
    rewards: Rate,
}

#[derive(Deserialize)]
pub struct Rate {
    weekday: usize,
    weekend: usize,
}
