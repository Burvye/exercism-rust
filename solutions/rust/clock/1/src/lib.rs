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
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut mins = minutes;
        let mut heures = 0;
        let mut clock = self.clone();

        while mins >= 60 {
            mins -= 60;
            heures += 1;
        }
        clock.hours += heures;
        clock.minutes += mins;
        clock
    }
}
