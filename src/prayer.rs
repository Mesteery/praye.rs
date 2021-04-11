use crate::astronomy::*;
use crate::dmath;
use chrono::{Date, Utc};

/// A calculation type
pub enum CalculationType {
	Angle(f64),
	Minutes(f64),
}
impl CalculationType {
	pub fn unwrap(&self) -> f64 {
		match self {
			CalculationType::Angle(v) | CalculationType::Minutes(v) => *v,
		}
	}
}

/// The mean time type
pub enum MeanTimeType {
	/// from Sunset to Sunrise
	Standard,
	/// from Sunset to Fajr
	Jafari,
}

/// Represents a calculation method (parameters)
pub struct CalculationMethod {
	imsak: CalculationType,
	fajr: f64,
	duhr: f64,
	asr: MeanTimeType,
	maghrib: CalculationType,
	isha: CalculationType,
	midnight: MeanTimeType,
}

impl CalculationMethod {
	pub fn new(
		imsak: Option<CalculationType>,
		fajr: f64,
		asr: Option<MeanTimeType>,
		maghrib: Option<CalculationType>,
		isha: CalculationType,
		midnight: Option<MeanTimeType>,
	) -> CalculationMethod {
		CalculationMethod {
			imsak: imsak.unwrap_or(CalculationType::Minutes(10.0)),
			fajr,
			duhr: 0.0,
			asr: asr.unwrap_or(MeanTimeType::Standard),
			maghrib: maghrib.unwrap_or(CalculationType::Minutes(0.0)),
			isha,
			midnight: midnight.unwrap_or(MeanTimeType::Standard),
		}
	}

	pub fn from(fajr: f64, isha: CalculationType) -> CalculationMethod {
		CalculationMethod::new(None, fajr, None, None, isha, None)
	}
}

/// The calculation methods
pub enum CalculationMethods {
	/// Muslim World Leage
	MWL,
	/// Islamic Society of North America
	ISNA,
	// Egyptian General Authority of Survey
	Egypt,
	/// Umm Al-Qura University, Makkah
	///
	/// # Example
	/// ~~~~
	/// CalculationMethods::Makkah(true); // in the ramadan period
	/// CalculationMethods::Makkah(false); // not in the ramadan period
	/// ~~~~
	Makkah(bool),
	/// University of Islamic Sciences, Karachi
	Karachi,
	/// Institute of Geophysics, University of Tehran
	Tehran,
	/// Shia Ithna-Ashari, Leva Institute, Qum
	Jafari,
	/// Muslims of France
	MF,
	/// *Custom parameters*
	///
	/// # Example
	/// ~~~~
	/// CalculationMethods::Custom(CalculationMethod::from(12.0, Calculation::Angle(13.0)));
	/// CalculationMethods::Custom(CalculationMethod::new(
	///		None,
	///		13.0,
	///		None,
	///		Some(CalculationType::Angle(6.0)),
	///		CalculationType::Angle(13.0),
	///		Some(MeanTimeType::Jafari),
	///	));
	/// ~~~~
	Custom(CalculationMethod),
}

/// Latitude, Longitude, Altitude (default to 0, in meters)
///
/// # Example
/// ~~~~
/// Coordinates(46, 69, None);
/// Coordinates(46, 69, 25);
/// ~~~~
pub struct Coordinates(pub f64, pub f64, pub f64);
impl Coordinates {
	/// altitude in meters
	pub fn new(latitude: f64, longitude: f64, altitude: Option<f64>) -> Coordinates {
		Coordinates(latitude, longitude, altitude.unwrap_or(0.0))
	}
}

#[derive(Debug)]
/// Represents prayer times
pub struct PrayerTimes {
	/// Imsak
	imsak: f64,
	/// Fajr
	fajr: f64,
	/// Sunrise
	sunrise: f64,
	/// Dhur
	dhuhr: f64,
	/// Asr
	asr: f64,
	/// Sunset
	sunset: f64,
	/// Maghrif
	maghrib: f64,
	/// Isha
	isha: f64,
	/// Middle of the night
	midnight: f64,
}

/// The method to use for higher latitudes
///
/// http://praytimes.org/calculation#Higher_Latitudes
pub enum HightLatMethods {
	/// Middle of the Night
	///
	/// > *In this method, the period from sunset to sunrise is divided into two halves.
	/// The first half is considered to be the "night" and the other half as "day break".
	/// Fajr and Isha in this method are assumed to be at mid-night during the abnormal periods.*
	/// http://praytimes.org/calculation#Higher_Latitudes
	NightMiddle,
	/// Angle-Based Method
	///
	/// > *This is an intermediate solution, used by some recent prayer time calculators.
	/// Let α be the twilight angle for Isha, and let t = α/60.
	/// The period between sunset and sunrise is divided into t parts.
	/// Isha begins after the first part.
	/// For example, if the twilight angle for Isha is 15, then Isha begins at the end of the first quarter (15/60) of the night.
	/// Time for Fajr is calculated similarly.*
	/// http://praytimes.org/calculation#Higher_Latitudes
	AngleBased,
	/// One-Seventh of the Night
	///
	/// > *In this method, the period between sunset and sunrise is divided into seven parts.
	/// Isha begins after the first one-seventh part, and Fajr is at the beginning of the seventh part.*
	/// http://praytimes.org/calculation#Higher_Latitudes
	OneSeventh,
}

fn time_diff(time1: f64, time2: f64) -> f64 {
	dmath::fix_hour(time2 - time1)
}

pub struct PrayerManager {
	method: CalculationMethod,
	high_lats: Option<HightLatMethods>,
}
impl PrayerManager {
	/// Initialize a PrayerManager
	pub fn new(method: CalculationMethods, high_lats: Option<HightLatMethods>) -> PrayerManager {
		PrayerManager {
			method: PrayerManager::get_calculation_method(method),
			high_lats,
		}
	}

	/// Get calculation parameters from a `CalculationMethods`
	pub fn get_calculation_method(calculation_method: CalculationMethods) -> CalculationMethod {
		match calculation_method {
			CalculationMethods::MWL => CalculationMethod::from(18.0, CalculationType::Angle(17.0)),
			CalculationMethods::ISNA => CalculationMethod::from(15.0, CalculationType::Angle(15.0)),
			CalculationMethods::Egypt => CalculationMethod::from(19.5, CalculationType::Angle(17.5)),
			CalculationMethods::Makkah(is_ramadan) => CalculationMethod::from(
				18.5,
				CalculationType::Minutes(if is_ramadan { 120.0 } else { 90.0 }),
			),
			CalculationMethods::Karachi => CalculationMethod::from(18.0, CalculationType::Angle(18.0)),
			CalculationMethods::Tehran => CalculationMethod::new(
				None,
				17.7,
				None,
				Some(CalculationType::Angle(4.5)),
				CalculationType::Angle(14.0),
				Some(MeanTimeType::Jafari),
			),
			CalculationMethods::Jafari => CalculationMethod::new(
				None,
				16.0,
				None,
				Some(CalculationType::Angle(4.0)),
				CalculationType::Angle(14.0),
				Some(MeanTimeType::Jafari),
			),
			CalculationMethods::MF => CalculationMethod::from(12.0, CalculationType::Angle(12.0)),
			CalculationMethods::Custom(value) => value,
		}
	}

	/// Get prayer times for a specific UTC date and coordinates
	///
	/// # Example
	/// ~~~~
	/// let prayer_manager = PrayerManager::new(CalculationMethods::MWL);
	///
	/// let a_date = Utc.ymd(2021, 4, 12);
	/// let a_house = Coordinates(38.8976763, -77.036529, 18);
	/// let prayers = prayer_manager.get_times(a_date, a_house);
	/// ~~~~
	pub fn get_times(&self, date: Date<Utc>, coords: Coordinates) -> PrayerTimes {
		let julian_day = get_julian_day(&date) - coords.1 / (15.0 * 24.0);
		let method = &self.method;
		let adjust = coords.1 / 15.0;

		let mut imsak = sun_angle_time(
			julian_day,
			coords.0,
			method.imsak.unwrap(),
			5.0 / 24.0,
			true,
		) - adjust;

		let mut fajr = sun_angle_time(julian_day, coords.0, method.fajr, 5.0 / 24.0, true);

		let sunrise = sun_angle_time(
			julian_day,
			coords.0,
			rise_set_angle(coords.2),
			6.0 / 24.0,
			true,
		) - adjust;

		let dhuhr = mid_day(julian_day, 12.0 / 24.0) - adjust + method.duhr;

		let asr = PrayerManager::asr_time(julian_day, coords.0, &method.asr, 13.0 / 24.0) - adjust;

		let sunset = sun_angle_time(
			julian_day,
			coords.0,
			rise_set_angle(coords.2),
			18.0 / 24.0,
			false,
		) - adjust;

		let mut maghrib = sun_angle_time(
			julian_day,
			coords.0,
			method.maghrib.unwrap(),
			18.0 / 24.0,
			true,
		) - adjust;

		let mut isha = sun_angle_time(
			julian_day,
			coords.0,
			method.isha.unwrap(),
			18.0 / 24.0,
			false,
		) - adjust;

		if self.high_lats.is_some() {
			let night_time = time_diff(sunset, sunrise);

			imsak = self.adjust_highlat_time(imsak, sunrise, method.imsak.unwrap(), night_time, true);
			fajr = self.adjust_highlat_time(fajr, sunrise, method.fajr, night_time, true);
			isha = self.adjust_highlat_time(isha, sunset, method.isha.unwrap(), night_time, false);
			maghrib =
				self.adjust_highlat_time(maghrib, sunset, method.maghrib.unwrap(), night_time, false);
		}

		if let CalculationType::Minutes(minutes) = method.imsak {
			imsak = fajr - minutes / 60.0;
		}
		if let CalculationType::Minutes(minutes) = method.maghrib {
			maghrib = sunset - minutes / 60.0;
		}
		if let CalculationType::Minutes(minutes) = method.isha {
			isha = maghrib - minutes / 60.0;
		}

		let midnight = match method.midnight {
			MeanTimeType::Standard => time_diff(sunset, fajr),
			MeanTimeType::Jafari => time_diff(sunset, sunrise),
		} / 2.0;

		PrayerTimes {
			imsak,
			fajr,
			sunrise,
			dhuhr,
			asr,
			sunset,
			maghrib,
			isha,
			midnight,
		}
	}

	fn adjust_highlat_time(&self, time: f64, base: f64, angle: f64, night: f64, ccw: bool) -> f64 {
		let portion = self.night_portion(angle, night);
		let diff = if ccw {
			time_diff(time, base)
		} else {
			time_diff(base, time)
		};

		if portion > diff {
			return time;
		}

		base + if ccw { -portion } else { portion }
	}

	fn asr_time(julian_day: f64, latitude: f64, factor_type: &MeanTimeType, time: f64) -> f64 {
		let decl = sun_position(julian_day + time).0;
		let factor = match factor_type {
			MeanTimeType::Standard => 0.0,
			MeanTimeType::Jafari => 1.0,
		};

		let angle = -dmath::arccot(&(factor + dmath::tan(&(latitude - decl).abs())));
		sun_angle_time(julian_day, latitude, angle, time, false)
	}

	fn night_portion(&self, angle: f64, night: f64) -> f64 {
		(match self.high_lats.as_ref().unwrap() {
			HightLatMethods::NightMiddle => 1.0 / 2.0,
			HightLatMethods::AngleBased => 1.0 / 60.0 * angle,
			HightLatMethods::OneSeventh => 1.0 / 7.0,
		}) * night
	}
}
