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

macro_rules! implement_Planet {
    ( $( ($p:ty, $o:expr) ),+ ) => {
        $(impl Planet for $p {
            fn years_during(d: &Duration) -> f64 {
                d.duration as f64 / EARTH_YEAR_IN_SECONDS / $o
            }
        })*
    }
}

#[derive(Debug)]
pub struct Duration {
    duration: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { duration: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

implement_Planet!(
    (Mercury, MERCURY_PERIOD),
    (Venus, VENUS_PERIOD),
    (Earth, EARTH_PERIOD),
    (Mars, MARS_PERIOD),
    (Jupiter, JUPITER_PERIOD),
    (Saturn, SATURN_PERIOD),
    (Uranus, URANUS_PERIOD),
    (Neptune, NEPTUNE_PERIOD)
);