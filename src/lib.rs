mod astronomy;
mod dmath;
pub mod prayer;
pub use chrono;

pub mod prelude {
	pub use crate::prayer::{
		CalculationMethod, CalculationMethods, CalculationType, Coordinates, HightLatMethods,
		MeanTimeType, PrayerManager, PrayerTimes,
	};
	pub use chrono;
}
