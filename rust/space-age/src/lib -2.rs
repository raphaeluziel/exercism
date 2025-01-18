// Second iteration
// It works, but not sure what the advantage is of doing it this way

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
        Duration { duration: s }
    }
}

pub trait Planet {
    fn get_orbital_period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / Self::get_orbital_period()
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

impl Planet for Mercury {
    fn get_orbital_period() -> f64 {
        MERCURY_PERIOD
    }
}
impl Planet for Venus {
    fn get_orbital_period() -> f64 {
        VENUS_PERIOD
    }
}
impl Planet for Earth {
    fn get_orbital_period() -> f64 {
        EARTH_PERIOD
    }
}
impl Planet for Mars {
    fn get_orbital_period() -> f64 {
        MARS_PERIOD
    }
}
impl Planet for Jupiter {
    fn get_orbital_period() -> f64 {
        JUPITER_PERIOD
    }
}
impl Planet for Saturn {
    fn get_orbital_period() -> f64 {
        SATURN_PERIOD
    }
}
impl Planet for Uranus {
    fn get_orbital_period() -> f64 {
        URANUS_PERIOD
    }
}
impl Planet for Neptune {
    fn get_orbital_period() -> f64 {
        NEPTUNE_PERIOD
    }
}
