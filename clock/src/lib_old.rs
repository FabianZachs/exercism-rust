// view solution https://exercism.io/tracks/rust/exercises/clock/solutions/f0876d740f0346aaaaf872bc05269f15
const MINUTES_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut new_clock = Clock { hours, minutes };
        normalise(&mut new_clock);

        new_clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_clock = Clock {
            minutes: self.minutes + minutes,
            ..*self
        };
        normalise(&mut new_clock);

        new_clock
    }
}

fn normalise(clock: &mut Clock) {
    // normalise negative
    if clock.minutes < 0 {
        clock.hours += clock.minutes / MINUTES_IN_HOUR;
        clock.minutes = ((clock.minutes % MINUTES_IN_HOUR) + MINUTES_IN_HOUR) % MINUTES_IN_HOUR; // modulo to remainder
        clock.hours = if clock.minutes == 0 {
            clock.hours
        } else {
            clock.hours - 1
        };
    }
    if clock.hours < 0 {
        clock.hours = ((clock.hours % HOURS_IN_DAY) + HOURS_IN_DAY) % HOURS_IN_DAY;
    }

    // normalise positive
    if clock.minutes > 0 {
        clock.hours += clock.minutes / MINUTES_IN_HOUR;
        clock.minutes = clock.minutes % MINUTES_IN_HOUR;
    }
    clock.hours = clock.hours % HOURS_IN_DAY;
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl std::fmt::Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
