use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut sum = hours * 60 + minutes;

        while sum < 0 {
            sum += 24 * 60;
        }

        Clock {
            hours: sum/60%24,
            minutes: sum%60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut sum = self.hours*60+self.minutes+minutes;

        while sum < 0 {
            sum += 24 * 60;
        }
        
        Clock {
            hours: sum/60%24,
            minutes: sum%60,
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = self.hours;
        let m = self.minutes;
        write!(f, "{h:02}:{m:02}")
    }
}