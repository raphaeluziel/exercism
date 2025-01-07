use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASEC: time::Duration = Duration::new(1_000_000_000, 0);

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");

    // Method 1
    // start.checked_add(GIGASEC).unwrap()

    // Method 2
    // match start.checked_add(GIGASEC) {
    //     Some(x) => x,
    //     None => start
    // }

    // Method 3
    // if let Some(x) = start.checked_add(GIGASEC) {
    //     x
    // }
    // else {
    //     start
    // }

    // Method 4
    // Best, I think
    start.checked_add(GIGASEC).unwrap_or(start)
}
