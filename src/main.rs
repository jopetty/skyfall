extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod record;

use record::{Position, Star, PolarPosition};
use std::error::Error;
// use std::process;
// use time::Tm;

/// Turn right ascension and declination
/// into azimuth and altitude.
fn get_position(star: &Star, position: &Position) -> Option<PolarPosition> {

    match star.right_ascension {
        Some(ra) => {
            match star.declination {
                Some(dec) => {
                    // Borrow position
                    let mut lat = position.latitude;
                    let mut lon = position.longitude;

                    // Calculate the hour angle
                    let mut ha = 1.0_f64;


                    // Calculate the azimuthal angle
                    let x = -lat.sin()*dec.cos()*ha.cos() + lat.cos()*dec.sin();
                    let y = dec.cos()*ha.sin();
                    let mut azimuth = -y.atan2(x);

                    // Calculate the altitude (zenith) angle
                    let sin_alt = lat.sin()*dec.sin() + lat.cos()*dec.cos()*ha.cos();
                    let altitude = sin_alt.asin();

                    // Project stereographically
                    let radius = altitude.sin() / (1.0_f64 - altitude.cos());

                    Some(PolarPosition{
                        r: radius,
                        theta: azimuth
                    })
                },
                None => {
                    println!("No DEC was found.");
                    return None;
                },
            }
        },
        None => {
            println!("No RA was found.");
            return None;
        },
    }
}

/// Parses stellar catalogue to deserialize
/// data into Star structs.
fn parse_data() -> Result<Vec<Star>, Box<Error>> {

    let mut stars = Vec::new();

    let data_file_path = "data/hygdata_v3.csv";
    let mut reader = csv::Reader::from_path(data_file_path)?;
    for result in reader.deserialize() {
        let record: Star = result?;
        stars.push(record);
    }

    Ok(stars)
}

fn main() {
    let result = parse_data();
    let position = Position {latitude: 90.0, longitude: 0.0};

    match result {
        Ok(stars) => {
            for s in &stars {
                let pos = get_position(s, &position);
                match pos {
                    Some(val) => println!("{}", val),
                    None => {},
                }
            }
        },
        Err(_) => {
            println!("Unable to parse data.");
        },
    }
}
