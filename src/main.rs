extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod record;

use record::{Position, Star, StellarPosition};
use std::error::Error;
use std::process;
use time::Tm;

fn get_position(star: Star, position: Position, time: Tm) -> StellarPosition {
    let right_ascension = star.right_ascension;
    let declination = star.declination;

    let p = StellarPosition { altitude: 1.0, azimuth: 1.0 };

    p
}

fn parse_data() -> Result<(), Box<Error>> {
    let data_file_path = "data/hygdata_v3.csv";
    let mut rdr = csv::Reader::from_path(data_file_path)?;
    for result in rdr.deserialize() {
        let record: Star = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = parse_data() {
        println!("{}", err);
        process::exit(1);
    }
}
