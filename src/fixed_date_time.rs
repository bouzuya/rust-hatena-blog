use chrono::{DateTime, FixedOffset, SecondsFormat};
use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FixedDateTime(DateTime<FixedOffset>);

impl From<DateTime<FixedOffset>> for FixedDateTime {
    fn from(value: DateTime<FixedOffset>) -> Self {
        Self(value)
    }
}

impl From<FixedDateTime> for DateTime<FixedOffset> {
    fn from(fixed_date_time: FixedDateTime) -> Self {
        fixed_date_time.0
    }
}

impl std::fmt::Display for FixedDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

#[derive(Debug, Eq, Error, PartialEq)]
#[error("fixed date time parse error")]
pub struct FixedDateTimeParseError;

impl std::str::FromStr for FixedDateTime {
    type Err = FixedDateTimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::<FixedOffset>::parse_from_rfc3339(s)
            .map(FixedDateTime)
            .map_err(|_| FixedDateTimeParseError)
    }
}

impl serde::Serialize for FixedDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn string_conversion_test() -> anyhow::Result<()> {
        let s1 = "2021-02-03T16:17:18+09:00";
        let s2 = "2021-02-03T16:17:18+00:00";
        let s3 = "2021-02-03T16:17:18Z";
        assert_eq!(FixedDateTime::from_str(s1)?.to_string(), s1);
        assert_eq!(FixedDateTime::from_str(s2)?.to_string(), s3);
        assert_eq!(FixedDateTime::from_str(s3)?.to_string(), s3);
        Ok(())
    }

    #[test]
    fn date_time_fixed_conversion_test() -> anyhow::Result<()> {
        let dt1 = DateTime::<FixedOffset>::parse_from_rfc3339("2021-02-03T16:17:18+09:00")?;
        let dt2 = DateTime::<FixedOffset>::parse_from_rfc3339("2021-02-03T16:17:18+00:00")?;
        assert_eq!(DateTime::<FixedOffset>::from(FixedDateTime::from(dt1)), dt1);
        assert_eq!(DateTime::<FixedOffset>::from(FixedDateTime::from(dt2)), dt2);
        Ok(())
    }

    #[test]
    fn serialize_test() -> anyhow::Result<()> {
        let f = serde_json::to_string;
        let d1 = FixedDateTime::from_str("2021-02-03T16:17:18+09:00")?;
        let d2 = FixedDateTime::from_str("2021-02-03T16:17:18+00:00")?;
        assert_eq!(f(&d1)?, r#""2021-02-03T16:17:18+09:00""#);
        assert_eq!(f(&d2)?, r#""2021-02-03T16:17:18Z""#);
        Ok(())
    }
}
