use std::fmt;
use std::time;

#[cfg(feature = "float_duration")]
use float_duration::{duration, FloatDuration};

#[cfg(feature = "chrono")]
use chrono;

const SECS_PER_YEAR: f64 = SECS_PER_DAY * 365.0;
const SECS_PER_DAY: f64 = SECS_PER_HOUR * 24.0;
const SECS_PER_HOUR: f64 = SECS_PER_MINUTE * 60.0;
const SECS_PER_MINUTE: f64 = 60.0;
const MILLIS_PER_SEC: f64 = 1000.0;
const MICROS_PER_SEC: f64 = 1.0e6;
const NANOS_PER_SEC: f64 = 1.0e9;

pub trait Decompose {
    type Error;
    fn decompose(self) -> Result<DecomposedTime, Self::Error>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DecomposedTime {
    sign_num: i8,
    years: u64,
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    milliseconds: u32,
    microseconds: u32,
    nanoseconds: u32,
    fractional_seconds: f64,
}

impl DecomposedTime {
    pub fn new(years: u64,
               days: u32,
               hours: u32,
               minutes: u32,
               seconds: u32,
               fractional_seconds: f64,
               is_positive: bool)
               -> DecomposedTime {
        let sign_num = if is_positive { 1 } else { 0 };
        let (milliseconds, microseconds, nanoseconds) =
            decompose_fractional_seconds(fractional_seconds);
        DecomposedTime {
            sign_num,
            years,
            days,
            hours,
            minutes,
            seconds,
            milliseconds,
            microseconds,
            nanoseconds,
            fractional_seconds,
        }
    }
    pub fn years(&self) -> u64 {
        self.years
    }
    pub fn days(&self) -> u32 {
        self.days
    }
    pub fn hours(&self) -> u32 {
        self.hours
    }
    pub fn minutes(&self) -> u32 {
        self.minutes
    }
    pub fn seconds(&self) -> u32 {
        self.seconds
    }
    pub fn milliseconds(&self) -> u32 {
        self.milliseconds
    }
    pub fn microseconds(&self) -> u32 {
        self.microseconds
    }
    pub fn nanoseconds(&self) -> u32 {
        self.nanoseconds
    }
    pub fn fractional_seconds(&self) -> f64 {
        self.fractional_seconds
    }
    pub fn with_years(mut self, years: u64) -> DecomposedTime {
        self.years = years;
        self
    }
    pub fn with_days(mut self, days: u32) -> DecomposedTime {
        assert!(days < 365, "days out of bounds");
        self.days = days;
        self
    }
    pub fn with_hours(mut self, hours: u32) -> DecomposedTime {
        assert!(hours < 24, "hours out of bounds");
        self.hours = hours;
        self
    }
    pub fn with_minutes(mut self, mins: u32) -> DecomposedTime {
        assert!(mins < 60, "minutes out of bounds");
        self.minutes = mins;
        self
    }
    pub fn with_seconds(mut self, secs: u32) -> DecomposedTime {
        assert!(secs < 60, "seconds out of bounds");
        self.seconds = secs;
        self
    }
    pub fn with_fractional_seconds(mut self, frac: f64) -> DecomposedTime {
        assert!(frac < 1.0 && frac >= 0.0,
                "fractional_seconds out of bounds");

        let (milliseconds, microseconds, nanoseconds) = decompose_fractional_seconds(frac);
        self.milliseconds = milliseconds;
        self.microseconds = microseconds;
        self.nanoseconds = nanoseconds;
        self.fractional_seconds = frac;
        self
    }

    pub fn zero() -> DecomposedTime {
        DecomposedTime {
            sign_num: 1,
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            fractional_seconds: 0.0,
        }
    }

    pub fn is_positive(&self) -> bool {
        self.sign_num.is_positive()
    }
    pub fn is_negative(&self) -> bool {
        self.sign_num.is_negative()
    }
    pub fn signum(&self) -> i8 {
        self.sign_num
    }
}

impl Default for DecomposedTime {
    fn default() -> DecomposedTime {
        DecomposedTime {
            sign_num: 1,
            years: 0,
            days: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
            microseconds: 0,
            nanoseconds: 0,
            fractional_seconds: 0.0,
        }
    }
}

#[cfg(feature = "float_duration")]
impl From<DecomposedTime> for FloatDuration {
    fn from(time: DecomposedTime) -> FloatDuration {
        FloatDuration::seconds(time.signum() as f64 *
                               (duration::SECS_PER_YEAR * time.years() as f64 +
                                duration::SECS_PER_DAY * time.days() as f64 +
                                duration::SECS_PER_HOUR * time.hours() as f64 +
                                duration::SECS_PER_MINUTE * time.minutes() as f64 +
                                time.seconds() as f64 +
                                time.fractional_seconds))
    }
}

#[cfg(feature = "float_duration")]
impl Decompose for FloatDuration {
    //TODO: Handle: NAN, INF
    type Error = ();
    fn decompose(self) -> Result<DecomposedTime, ()> {
        Ok(decomposed_from_float_seconds(self.as_seconds()))
    }
}
#[cfg(feature = "chrono")]
impl Decompose for chrono::Duration {
    type Error = ();
    fn decompose(self) -> Result<DecomposedTime, ()> {
        if let Some(nanos) = self.num_nanoseconds() {
            Ok(decomposed_from_float_seconds((nanos as f64) * NANOS_PER_SEC))
        } else if let Some(micros) = self.num_microseconds() {
            Ok(decomposed_from_float_seconds((micros as f64) * MICROS_PER_SEC))
        } else {
            Ok(decomposed_from_float_seconds((self.num_milliseconds() as f64) * MILLIS_PER_SEC))
        }
    }
}
impl Decompose for time::Duration {
    type Error = ();
    fn decompose(self) -> Result<DecomposedTime, ()> {
        Ok(decomposed_from_float_seconds(self.as_secs() as f64 +
                                         (self.subsec_nanos() as f64) * NANOS_PER_SEC))
    }
}

impl fmt::Display for DecomposedTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.years() > 0 {
            write!(f, "{}yr ", self.years())?;
        }
        if self.days() > 0 {
            write!(f, "{}d ", self.days())?;
        }
        if self.hours() > 0 || self.days() > 0 || self.years() > 0 {
            write!(f, "{:02}:", self.hours())?;
        }
        write!(f, "{:02}:{:02}", self.minutes(), self.seconds())?;

        if self.nanoseconds() > 0 {
            write!(f,
                   ".{:03}'{:03}'{:03}",
                   self.milliseconds(),
                   self.microseconds(),
                   self.nanoseconds())?;
        } else if self.microseconds() > 0 {
            write!(f, ".{:03}'{:03}", self.milliseconds(), self.microseconds())?;
        } else if self.milliseconds() > 0 {
            write!(f, ".{:03}", self.milliseconds())?;
        }
        Ok(())
    }
}


fn decompose_fractional_seconds(fractional_seconds: f64) -> (u32, u32, u32) {
    let mut rem_frac = fractional_seconds;

    let milliseconds = (rem_frac * MILLIS_PER_SEC).trunc();
    rem_frac -= milliseconds / MILLIS_PER_SEC;
    let microseconds = (rem_frac * MICROS_PER_SEC).trunc();
    rem_frac -= microseconds / MICROS_PER_SEC;
    let nanoseconds = (rem_frac * NANOS_PER_SEC).trunc();

    (milliseconds as u32, microseconds as u32, nanoseconds as u32)
}

fn decomposed_from_float_seconds(secs: f64) -> DecomposedTime {
    let mut rem_seconds = secs.trunc().abs();
    let fractional_seconds = secs.fract().abs();
    let sign_num = secs.signum();

    let years = (rem_seconds / SECS_PER_YEAR).trunc();
    rem_seconds -= years * SECS_PER_YEAR;
    let days = (rem_seconds / SECS_PER_DAY).trunc();
    rem_seconds -= days * SECS_PER_DAY;
    let hours = (rem_seconds / SECS_PER_HOUR).trunc();
    rem_seconds -= hours * SECS_PER_HOUR;
    let minutes = (rem_seconds / SECS_PER_MINUTE).trunc();
    rem_seconds -= minutes * SECS_PER_MINUTE;
    let seconds = rem_seconds.trunc();

    let (milliseconds, microseconds, nanoseconds) =
        decompose_fractional_seconds(fractional_seconds);

    DecomposedTime {
        years: years as u64,
        days: days as u32,
        hours: hours as u32,
        minutes: minutes as u32,
        seconds: seconds as u32,
        milliseconds: milliseconds,
        microseconds: microseconds,
        nanoseconds: nanoseconds,
        fractional_seconds: fractional_seconds,
        sign_num: sign_num as i8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_duration::FloatDuration;

    #[test]
    fn test_with_builders() {
        let time = DecomposedTime::default().with_years(5).with_days(10);
        assert_eq!(time,
                   (FloatDuration::years(5.0) + FloatDuration::days(10.0))
                       .decompose()
                       .unwrap());

        assert_eq!(DecomposedTime::zero(),
                   FloatDuration::zero().decompose().unwrap());
        assert_eq!(DecomposedTime::default().with_days(10).with_minutes(30),
                   (FloatDuration::days(10.0) + FloatDuration::minutes(30.0))
                       .decompose()
                       .unwrap());
        assert_eq!(DecomposedTime::default()
                       .with_seconds(30)
                       .with_fractional_seconds(0.5),
                   (FloatDuration::seconds(30.5)).decompose().unwrap());

        assert_eq!(DecomposedTime::default()
                       .with_fractional_seconds(0.2)
                       .milliseconds(),
                   200);
        assert_eq!(DecomposedTime::default()
                       .with_fractional_seconds(0.00005)
                       .microseconds(),
                   50);
        assert_eq!(DecomposedTime::default()
                       .with_fractional_seconds(0.00000005)
                       .nanoseconds(),
                   50);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", FloatDuration::years(2.5).decompose().unwrap()),
                   "2yr 182d 12:00:00");
        assert_eq!(format!("{}", FloatDuration::years(1.0).decompose().unwrap()),
                   "1yr 00:00:00");
        assert_eq!(format!("{}", FloatDuration::days(2.0).decompose().unwrap()),
                   "2d 00:00:00");
        assert_eq!(format!("{}", FloatDuration::minutes(10.0).decompose().unwrap()),
                   "10:00");
        assert_eq!(format!("{}", FloatDuration::microseconds(50.0).decompose().unwrap()),
                   "00:00.000'050");
        assert_eq!(format!("{}", FloatDuration::seconds(12.5).decompose().unwrap()),
                   "00:12.500");
        assert_eq!(format!("{}",
                           FloatDuration::milliseconds(100.0).decompose().unwrap()),
                   "00:00.100");
        assert_eq!(format!("{}", FloatDuration::nanoseconds(10.0).decompose().unwrap()),
                   "00:00.000'000'010");

        assert_eq!(format!("{}",
                           (FloatDuration::days(10.0) + FloatDuration::minutes(20.0) +
                            FloatDuration::seconds(2.0))
                                   .decompose()
                                   .unwrap()),
                   "10d 00:20:02");
        assert_eq!(format!("{}",
                           (FloatDuration::seconds(30.5) + FloatDuration::nanoseconds(100.0))
                               .decompose()
                               .unwrap()),
                   "00:30.500'000'100");
        assert_eq!(format!("{}",
                           (FloatDuration::seconds(90.0) + FloatDuration::microseconds(500.0))
                               .decompose()
                               .unwrap()),
                   "01:30.000'500");

    }
}
