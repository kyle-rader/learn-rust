use std::ops::Add;

use time::Duration;
use time::PrimitiveDateTime as DateTime;

const GIGA_SECOND: f64 = 1_000_000_000.0;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds_f64(GIGA_SECOND)
}
