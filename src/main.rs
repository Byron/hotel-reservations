#[macro_use]
extern crate failure;
extern crate failure_tools;
extern crate hotel_reservations;
extern crate serde_yaml;

use failure_tools::ok_or_exit;
use failure::{Error, ResultExt};
use std::{env, fs::File, io::stdout};
use hotel_reservations::HotelDb;

fn run() -> Result<(), Error> {
    let args: Vec<_> = env::args().skip(1).collect();
    match args.as_slice() {
        [hotel_db_filename, input_filename] => {
            let db: HotelDb = serde_yaml::from_reader(File::open(&hotel_db_filename)
                .with_context(|_| {
                    format_err!(
                        "Could not open hotel database at '{}' for reading",
                        hotel_db_filename
                    )
                })?).with_context(|_err| "Failed to deserialize hotel database")?;
            let input = File::open(&input_filename).with_context(|_| {
                format_err!(
                    "Could not open input file at '{}' for reading",
                    input_filename
                )
            })?;

            let stdout = stdout();
            let lock = stdout.lock();
            hotel_reservations::answers(&db, input, lock)
        }
        _ => Err(usage()),
    }
}

fn usage() -> Error {
    format_err!(
        "USAGE: {} <db.yml> <input>\n\n\
         Where <db.yml> is a yaml file with the hotel database and\n\
         where <input> is the input file with customer statements",
        env::args().next().expect("program name")
    )
}

fn main() {
    ok_or_exit(run())
}
