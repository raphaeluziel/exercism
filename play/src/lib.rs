// Third iteration

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

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[derive(Debug)]
pub struct Duration;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        todo!("s, measured in seconds: {s}")
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}

////////////////////////////////////////////////////////////////

fn assert_in_delta(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Your result of {actual} should be within {delta} of the expected result {expected}")
    }
}

#[test]
fn age_on_earth() {
    let seconds = 1_000_000_000;
    let duration = Duration::from(seconds);
    let output = Earth::years_during(&duration);
    let expected = 31.69;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_mercury() {
    let seconds = 2_134_835_688;
    let duration = Duration::from(seconds);
    let output = Mercury::years_during(&duration);
    let expected = 280.88;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_venus() {
    let seconds = 189_839_836;
    let duration = Duration::from(seconds);
    let output = Venus::years_during(&duration);
    let expected = 9.78;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_mars() {
    let seconds = 2_129_871_239;
    let duration = Duration::from(seconds);
    let output = Mars::years_during(&duration);
    let expected = 35.88;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_jupiter() {
    let seconds = 901_876_382;
    let duration = Duration::from(seconds);
    let output = Jupiter::years_during(&duration);
    let expected = 2.41;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_saturn() {
    let seconds = 2_000_000_000;
    let duration = Duration::from(seconds);
    let output = Saturn::years_during(&duration);
    let expected = 2.15;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_uranus() {
    let seconds = 1_210_123_456;
    let duration = Duration::from(seconds);
    let output = Uranus::years_during(&duration);
    let expected = 0.46;
    assert_in_delta(expected, output);
}

#[test]
#[ignore]
fn age_on_neptune() {
    let seconds = 1_821_023_456;
    let duration = Duration::from(seconds);
    let output = Neptune::years_during(&duration);
    let expected = 0.35;
    assert_in_delta(expected, output);
}
