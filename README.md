# Praye.rs

_Lightweight and highly accurate low-level library for calculating (Islamic) prayer times._

## Usage

```rust
use prayers::{PrayerManager, CalculationMethods, Coordinates, Utc};


let prayer_manager = PrayerManager::new(CalculationMethods::MWL);

let a_date = Utc.ymd(2021, 4, 12);
let a_house = Coordinates(38.8976763, -77.036529, 18);
let prayers = prayer_manager.get_times(a_date, a_house);
```
