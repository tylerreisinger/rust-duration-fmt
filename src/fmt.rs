use std::time;
use std::fmt;

use decomposed::DecomposedTime;

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

#[cfg(test)]
mod tests {
    use super::*;
    use decomposed::{DecomposedTime, Decompose};
    use float_duration::FloatDuration;

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
