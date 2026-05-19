use std::fmt::Display;

#[derive(Clone, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = hours;
        let mut minutes = minutes;
        while minutes <= 60 {
            minutes += 60;
            hours -= 1;
        }
        while minutes >= 60 {
            minutes -= 60;
            hours += 1;
        }
        while hours <= 0 {
            hours = 24 + hours;
        }
        while hours >= 24 {
            hours -= 24
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
/*
let clock = Clock::new(23, 59+2=61).add_minutes(2);

pub fn new(hours: 23, minutes: 61) -> Self {
    let mut hours = 23;
    let mut minutes = 61;
    while 61 >= 60 {
        minutes -= 60 == 1;
        hours += 1;
    }
    minutes = 1
    hours = 24
    while 24 >= 24 {
        hours -= 24
    }
    hours = 0
    Clock { hours, minutes }
}
 */
