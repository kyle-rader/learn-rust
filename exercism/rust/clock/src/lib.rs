use std::{f32::MIN, fmt::Display};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

const MINUTES: i32 = 60;
const HOURS: i32 = 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours + minutes / MINUTES;

        let minutes = match minutes % MINUTES {
            n if n < 0 => {
                hours -= 1;
                n + MINUTES
            }
            n => n,
        };

        let hours = match hours % HOURS {
            n if n < 0 => n + HOURS,
            n => n,
        };
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
