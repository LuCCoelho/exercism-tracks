extern crate time;
use time::PrimitiveDateTime as DateTime;
use time::Duration;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1000000000)
}
