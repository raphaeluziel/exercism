// THIS IS NOT GOING TO BE MY FINAL VERSION!!!!!!!

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
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury {
    pub orbital_period: f64,
}
pub struct Venus {
    pub orbital_period: f64,
}
pub struct Earth {
    pub orbital_period: f64,
}
pub struct Mars {
    pub orbital_period: f64,
}
pub struct Jupiter {
    pub orbital_period: f64,
}
pub struct Saturn {
    pub orbital_period: f64,
}
pub struct Uranus {
    pub orbital_period: f64,
}
pub struct Neptune {
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
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / EARTH_PERIOD
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / MARS_PERIOD
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / JUPITER_PERIOD
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / SATURN_PERIOD
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / URANUS_PERIOD
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.duration as f64 / EARTH_YEAR_IN_SECONDS / NEPTUNE_PERIOD
    }
}