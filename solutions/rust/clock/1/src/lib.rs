use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours, self.minutes))
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self {
            hours: self.hours,
            minutes: self.minutes + minutes,
        }
    }
}
