extern crate csv;
#[macro_use]
extern crate serde_derive;
extern crate time;

mod record;
use record::{Position, Star, PolarPosition, DateTime};

use std::error::Error;
use std::f64::consts::PI;

/// Returns the hour angle in radians, according to 
/// ha = GST - longitude - ra,
/// where GST is the greenwhich sidereal time and ra
/// is the right ascension of the observation.
fn get_hour_angle(date: &DateTime, longitude: f64, right_ascension: f64) -> f64 {

    let mut year = date.year;
    let mut month = date.month;
    let mut day = date.day as f64;

    day += (((date.second as f64) / 60.0 + (date.minute as f64)) / 60.0_f64 + (date.hour as f64)) / 24.0_f64;

    if month <= 2 {
        year -= 1;
        month += 12;
    }

    let a = (year as f64) / 100.0_f64;
    let b = 2.0 - a + (a/4.0);

    let y = year as f64;
    let m = month as f64;

    let jd : f64 = ((((365.25 * (y + 4716.0)) as i64) + ((30.6 * (m + 1.0)) as i64)) as f64) + day + b - 1524.5;
    let t : f64 = (jd - 2451545.0) / 36525.0;
    let theta: f64 = 280.46061837 + 360.98564736629 * (jd - 2451545.0) + 0.000387933 * t.powf(2.0) - t.powf(3.0) / 38710000.0;

    (theta + longitude - right_ascension) % (2.0 * PI)
}

/// Turn right ascension and declination
/// into polar stereographic projection
fn get_position(star: &Star, position: &Position, date: &DateTime) -> Option<PolarPosition> {

    // Ensure all values are in radians
    let lat = position.latitude * PI / 180.0;
    let lon = position.longitude * PI / 180.0;
    let ra = star.right_ascension_rad.unwrap(); // We can unwrap these since we already checked
    let dec = star.declination_rad.unwrap();

    // Calculate the hour angle
    let ha = get_hour_angle(date, lon, ra);

    // Calculate the azimuthal angle
    let x = -lat.sin()*dec.cos()*ha.cos() + lat.cos()*dec.sin();
    let y = dec.cos()*ha.sin();
    let mut azimuth = -y.atan2(x);
    if azimuth < 0.0 {
        azimuth += 2.0 * PI;
    }

    // Calculate the altitude (zenith) angle
    let sin_alt = lat.sin()*dec.sin() + lat.cos()*dec.cos()*ha.cos();
    let altitude = sin_alt.asin();
    if altitude <= 0.0 {
        return None; // Restrict data to hemisphere above (lat, long)
    }
    let zenith = PI / 2.0 - altitude;

    // Project stereographically
    let radius = zenith.sin() / (1.0_f64 - zenith.cos());

    Some(PolarPosition{ r: radius, theta: azimuth })
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

    // Testing values
    let position = Position { latitude: -90.0, longitude: 0.0 };
    let date = DateTime {
        year: 2018,
        month: 5,
        day: 28,
        hour: 13,
        minute: 24,
        second: 30
    };

    let result = parse_data();

    match result {
        Ok(mut stars) => {

            // Temporary fileter to Ursa Major stars
            let bear = ["Dubhe", "Merak", "Phad", "Megrez", "Alioth", "Mizar", "Alkaid", "Polaris"];
            stars.retain(|s| {
                s.name.as_ref().map_or(false, |name| {
                    let name = name.as_str();
                    bear.contains(&name)
                })
            });

            // Filter stars to have all the variables we need
            stars.retain(|ref s| s.right_ascension_rad.is_some());
            stars.retain(|ref s| s.declination_rad.is_some());
            stars.retain(|s| {
                s.magnitude.as_ref().map_or(false, |magnitude| {
                    magnitude < &5.0_f64 // Arbitrary cut-off for magnitude
                })
            });

            for s in &stars {
                // println!("{}", s);
                let pos = get_position(s, &position, &date);

                match pos {
                    Some(polar_position) => println!("{}", polar_position),
                    None => {},
                }
            }
        },
        Err(_) => {
            println!("Unable to parse data.");
        },
    }
}
