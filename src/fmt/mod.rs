use std::error;
use std::fmt::{self, Write};

use decomposed::{Decompose, DecomposedTime};

const FIELD_DELIMITER: char = '%';

#[derive(Clone, Debug, PartialEq)]
pub enum FormatError {
    UnexpectedFieldDelimiter,
    UnknownField,
    FormatError(fmt::Error),
    DecomposeError,
}

impl From<fmt::Error> for FormatError {
    fn from(err: fmt::Error) -> FormatError {
        FormatError::FormatError(err)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DurationFormat<'a> {
    format: &'a str,
    time: DecomposedTime,
}

pub fn format_duration<D>(format: &str, time: D) -> Result<String, FormatError>
    where D: Decompose
{
    let fmt = make_format(format, time)?;
    Ok(format!("{}", fmt))
}

pub fn make_format<'a, D>(format_str: &'a str, time: D) -> Result<DurationFormat<'a>, FormatError>
    where D: Decompose
{
    let decomposed = time.decompose().map_err(|_| FormatError::DecomposeError)?;
    let fmt = DurationFormat {
        format: format_str,
        time: decomposed,
    };
    fmt.validate()?;
    Ok(fmt)
}

impl<'a> DurationFormat<'a> {
    pub fn format_string(&self) -> &'a str {
        self.format
    }
    pub fn time(&self) -> &DecomposedTime {
        &self.time
    }

    fn validate(&self) -> Result<(), FormatError> {
        let mut chars = self.format.chars();

        while let Some(ch) = chars.next() {
            if ch == FIELD_DELIMITER {
                if let Some(field) = chars.next() {
                    self.validate_field(field)?
                } else {
                    return Err(FormatError::UnexpectedFieldDelimiter);
                }
            }
        }
        Ok(())
    }

    pub fn format(&self, f: &mut fmt::Formatter) -> Result<(), FormatError> {
        let mut chars = self.format.chars();

        while let Some(ch) = chars.next() {
            if ch == FIELD_DELIMITER {
                if let Some(field) = chars.next() {
                    self.handle_format_field(f, field)?
                } else {
                    return Err(FormatError::UnexpectedFieldDelimiter);
                }
            } else {
                f.write_char(ch)?
            }
        }
        Ok(())
    }

    fn validate_field(&self, field: char) -> Result<(), FormatError> {
        match field {
            'S' | 'M' | 'H' | 'D' | 'Y' | 'F' | 'T' | 'U' | 's' | 'm' | 'h' | 'f' | 'x' | 'y' |
            'z' => Ok(()),
            _ => Err(FormatError::UnknownField),
        }
    }

    fn handle_format_field(&self, f: &mut fmt::Formatter, field: char) -> Result<(), FormatError> {
        match field {
            'x' => write!(f, "{:03}", self.time.milliseconds()).map_err(|e| e.into()),
            'y' => write!(f, "{:03}", self.time.microseconds()).map_err(|e| e.into()),
            'z' => write!(f, "{:03}", self.time.nanoseconds()).map_err(|e| e.into()),
            'f' => write!(f, "{}", self.time.fractional_seconds()).map_err(|e| e.into()),
            'F' => write!(f, "{:.5}", self.time.fractional_seconds()).map_err(|e| e.into()),
            's' => write!(f, "{}", self.time.seconds()).map_err(|e| e.into()),
            'm' => write!(f, "{}", self.time.minutes()).map_err(|e| e.into()),
            'h' => write!(f, "{}", self.time.hours()).map_err(|e| e.into()),
            'S' => write!(f, "{:02}", self.time.seconds()).map_err(|e| e.into()),
            'M' => write!(f, "{:02}", self.time.minutes()).map_err(|e| e.into()),
            'H' => write!(f, "{:02}", self.time.hours()).map_err(|e| e.into()),
            'D' => write!(f, "{}", self.time.days()).map_err(|e| e.into()),
            'Y' => write!(f, "{}", self.time.years()).map_err(|e| e.into()),
            'T' => write!(f, "{}", self.time.total_hours()).map_err(|e| e.into()),
            'U' => write!(f, "{}", self.time.total_days()).map_err(|e| e.into()),
            FIELD_DELIMITER => f.write_char(field).map_err(|e| e.into()),
            _ => Err(FormatError::UnknownField),
        }
    }
}

impl<'a> fmt::Display for DurationFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f).map_err(|_| fmt::Error::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use decomposed::{Decompose, DecomposedTime};
    use float_duration::FloatDuration;

    #[test]
    fn test_format() {
        assert_eq!(format_duration("%H hours", FloatDuration::hours(2.0)).unwrap(),
                   "02 hours");
        assert_eq!(format_duration("%H:%M", FloatDuration::hours(2.5)).unwrap(),
                   "02:30");
        assert_eq!(format_duration("%S.%x'%y'%z",
                                   FloatDuration::seconds(2.5) +
                                   FloatDuration::microseconds(100.0))
                           .unwrap(),
                   "02.500'100'000");
    }
}
