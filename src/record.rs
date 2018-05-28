use std;

#[derive(Debug, Deserialize)]
pub struct Star {
	pub id: u64,
	#[serde(rename = "hip")]
	pub hipparcos_id: Option<u64>,
	#[serde(rename = "hd")]
	pub henry_draper_id: Option<u64>,
	#[serde(rename = "hr")]
	pub yale_bsc_id: Option<u64>,
	#[serde(rename = "gl")]
	pub gliese_id: Option<String>,
	#[serde(rename = "bf")]
	pub bayer_flamsteed: Option<String>,
	#[serde(rename = "proper")]
	pub proper_name: Option<String>,
	#[serde(rename = "ra")]
	pub right_ascension: Option<f64>,
	#[serde(rename = "dec")]
	pub declination: Option<f64>,
	#[serde(rename = "dist")]
	pub distance: Option<f64>,
	#[serde(rename = "pmra")]
	pub proper_motion_right_ascension: Option<f64>,
	#[serde(rename = "pmdec")]
	pub proper_motion_declination: Option<f64>,
	#[serde(rename = "rv")]
	pub radial_velocity: Option<f64>,
	#[serde(rename = "mag")]
	pub magnitude: Option<f64>,
	#[serde(rename = "absmag")]
	pub absolute_magnitude: Option<f64>,
	#[serde(rename = "spect")]
	pub spectral_type: Option<String>,
	#[serde(rename = "ci")]
	pub color_index: Option<f64>,
	#[serde(rename = "x")]
	pub position_x: Option<f64>,
	#[serde(rename = "y")]
	pub position_y: Option<f64>,
	#[serde(rename = "z")]
	pub position_z: Option<f64>,
	#[serde(rename = "vx")]
	pub velocity_x: Option<f64>,
	#[serde(rename = "vy")]
	pub velocity_y: Option<f64>,
	#[serde(rename = "vz")]
	pub velocity_z: Option<f64>,
	#[serde(rename = "rarad")]
	pub right_ascension_rad: Option<f64>,
	#[serde(rename = "decrad")]
	pub declination_rad: Option<f64>,
	#[serde(rename = "pmrarad")]
	pub proper_motion_right_ascension_rad: Option<f64>,
	#[serde(rename = "pmdecrad")]
	pub proper_motion_declination_rad: Option<f64>,
	#[serde(rename = "bayer")]
	pub bayer: Option<String>,
	#[serde(rename = "flam")]
	pub flamsteed: Option<String>,
	#[serde(rename = "con")]
	pub constellation_abr: Option<String>,
	#[serde(rename = "comp")]
	pub companion: Option<u64>,
	#[serde(rename = "comp_primary")]
	pub companion_primary: Option<u64>,
	#[serde(rename = "base")]
	pub base: Option<String>,
	#[serde(rename = "lum")]
	pub luminosity: Option<f64>,
	#[serde(rename = "var")]
	pub variable_designation: Option<String>,
	#[serde(rename = "var_min")]
	pub var_min: Option<f64>,
	#[serde(rename = "car_max")]
	pub var_max: Option<f64>,
}

pub struct PolarPosition {
	pub r: f64,
	pub theta: f64
}

impl std::fmt::Display for PolarPosition {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}\t{}", self.r, self.theta)
	}
}

#[derive(Debug)]
pub struct Position {
	pub latitude: f64, // + is North, - is South
	pub longitude: f64 // + West, - is East
}

pub struct DateTime {
	year: i32,
	month: i32,
	day: i32,
	hour: i32,
	minute: i32,
	second: i32
}