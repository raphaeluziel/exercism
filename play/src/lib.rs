// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 365.25 * 24.0 * 60.0 * 60.0;

// Orbital periods in terms of Earth years
const MERCURY_PERIOD: f64 = 0.2408467;
const VENUS_PERIOD: f64 = 0.61519726;
const EARTH_PERIOD: f64 = 1.0;
const MARS_PERIOD: f64 = 1.8808158;
const JUPITER_PERIOD: f64 = 11.862615;
const SATURN_PERIOD: f64 = 29.447498;
const URANUS_PERIOD: f64 = 84.016846;
const NEPTUNE_PERIOD: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration {
    duration: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // todo!("s, measured in seconds: {s}")
        Duration { duration: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury {
    pub orbital_period: f64,
}
pub struct Venus {
    pub orbital_period: f64,
}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / MERCURY_PERIOD
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / VENUS_PERIOD
    }
}

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

#[test]
fn age_on_mercury() {
    let seconds = 2_134_835_688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    assert_in_delta(expected, output);
}

#[test]
fn age_on_venus() {
    let seconds = 189_839_836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);
    let expected = 9.78;
    assert_in_delta(expected, output);
}