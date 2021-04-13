use crate::dmath;
use chrono::{Date, Datelike, Utc};

/// Latitude, Longitude, Altitude (default to 0, in meters)
///
/// # Example
/// ~~~~
/// use prayers::*;
///
/// Coordinates(46.0, 69.0, 0.0);
/// Coordinates(46.0, 69.0, 25.0);
/// ~~~~
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Coordinates(pub f64, pub f64, pub f64);

pub fn get_julian_day(date: &Date<Utc>) -> f64 {
	let mut year = date.year() as f64;
	let mut month = date.month() as f64;
	let day = date.day() as f64;

	if month < 3.0 {
		year -= 1.0;
		month += 12.0;
	}

	(365.2425 * year + 30.6001 * month).floor() + day + 1721027.5
}

pub fn mid_day(julian_date: f64, time: f64) -> f64 {
	let eqt = sun_position(julian_date + time).1;
	dmath::fix_hour(12.0 - eqt)
}

pub fn sun_angle_time(julian_date: f64, latitude: f64, angle: f64, time: f64, ccw: bool) -> f64 {
	let (decl, eqt) = sun_position(julian_date + time);
	let t = 1.0 / 15.0
		* dmath::arccos(
			&((-dmath::sin(&angle) - dmath::sin(&decl) * dmath::sin(&latitude))
				/ (dmath::cos(&decl) * dmath::cos(&latitude))),
		);

	dmath::fix_hour(12.0 - eqt) + if ccw { -t } else { t }
}

/// (decl, eqt)
#[allow(non_snake_case)]
pub fn sun_position(jd: f64) -> (f64, f64) {
	let D = jd - 2451545.0;

	let q = dmath::fix_angle(280.46061837 + 0.98564736 * D);
	let g = dmath::fix_angle(357.528 + 0.98560028 * D);
	let L = dmath::fix_angle(q + 1.915 * dmath::sin(&g) + 0.020 * dmath::sin(&(2.0 * g)));
	let e = 23.439 - 0.00000036 * D;

	let decl = dmath::arcsin(&(dmath::sin(&e) * dmath::sin(&L)));
	let eqt = q / 15.0
		- dmath::fix_hour(dmath::arctan2(&(dmath::cos(&e) * dmath::sin(&L)), &dmath::cos(&L)) / 15.0);

	return (decl, eqt);
}

pub fn rise_set_angle(elevation: f64) -> f64 {
	let earth_radius = 6371008.7714; // in meters
	let angle = dmath::arccos(&(earth_radius / (earth_radius + elevation)));
	return 0.833 + angle;
}
