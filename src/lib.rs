mod astronomy;
mod dmath;
mod prayer;

pub use crate::astronomy::Coordinates;
pub use chrono::{Date, DateTime, Datelike, TimeZone, Timelike, Utc};
pub use prayer::{
	AsrJuristic, CalculationMethod, CalculationMethods, CalculationType, HightLatMethods,
	MidnightMethod, PrayerManager, PrayerTimes,
};

#[cfg(test)]
mod tests {
	use super::{CalculationMethods, Coordinates, HightLatMethods, PrayerManager, TimeZone, Utc};

	#[test]
	fn compute_prayer_times() {
		let prayer_manager =
			PrayerManager::new(CalculationMethods::MWL, Some(HightLatMethods::NightMiddle));

		let a_date = Utc.ymd(2021, 4, 12);
		let a_house = Coordinates(38.8976763, -77.036529, 18.0);
		let times = prayer_manager.get_times(a_date, a_house);

		assert_eq!(times.imsak, 8.860089038173626);
		assert_eq!(times.fajr, 9.026755704840292);
		assert_eq!(times.sunrise, 10.581941026910073);
		assert_eq!(times.dhuhr, 17.14708096904308);
		assert_eq!(times.asr, 20.831494870257075);
		assert_eq!(times.sunset, 23.72239613166242);
		assert_eq!(times.maghrib, 23.72239613166242);
		assert_eq!(times.isha, 25.1845331305664);
		assert_eq!(times.midnight, 29.152168579286247);
	}
}
