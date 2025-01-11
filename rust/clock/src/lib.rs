use std::fmt;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        convert_minutes_to_hours(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        convert_minutes_to_hours(self.hours * 60 + self.minutes + minutes)
    }
}

fn convert_minutes_to_hours(tm: i32) -> Clock {
    let mut h = (tm / 60) % 24;
    let mut m = tm % 60;

    if tm < 0 {
        h += 23;
        m += 60;
    }
    
    if m == 60 {
        m = 0;
        h += 1;
    }

    Clock {
        hours: h,
        minutes: m,
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
