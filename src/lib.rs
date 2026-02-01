#![doc = include_str!("../README.md")]

use chrono::NaiveDateTime;
use core::str::FromStr;

pub mod diesel_impls;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature="diesel", diesel(sql_type = crate::diesel_impls::TimestampUTC))]
/// A wrapper around the `chrono` crate's `DateTime<Utc>` type.
pub struct TimestampUTC(chrono::DateTime<chrono::Utc>);

impl TimestampUTC {
    #[must_use]
    /// Returns the current time in UTC.
    ///
    /// # Examples
    ///
    /// ```
    /// use rosetta_utc::TimestampUTC;
    ///
    /// let now = TimestampUTC::now();
    /// ```
    pub fn now() -> Self {
        Self(chrono::Utc::now())
    }
}

impl Default for TimestampUTC {
    fn default() -> Self {
        Self::now()
    }
}

impl From<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl From<TimestampUTC> for chrono::DateTime<chrono::Utc> {
    fn from(value: TimestampUTC) -> Self {
        value.0
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl AsMut<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn as_mut(&mut self) -> &mut chrono::DateTime<chrono::Utc> {
        &mut self.0
    }
}

impl From<NaiveDateTime> for TimestampUTC {
    fn from(value: NaiveDateTime) -> Self {
        use chrono::TimeZone;
        Self(chrono::Utc.from_utc_datetime(&value))
    }
}

impl core::ops::Deref for TimestampUTC {
    type Target = chrono::DateTime<chrono::Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for TimestampUTC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl core::fmt::Display for TimestampUTC {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for TimestampUTC {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            chrono::DateTime::parse_from_rfc3339(s)?.with_timezone(&chrono::Utc),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;
    use std::collections::HashSet;

    #[test]
    fn test_default() {
        let t = TimestampUTC::default();
        // Just checking it runs, harder to check equality with time moving
        assert!(t.timestamp() > 0);
    }

    #[test]
    fn test_from_str() {
        let s = "2023-10-27T10:00:00+00:00";
        let t = TimestampUTC::from_str(s).unwrap();
        assert_eq!(t.to_rfc3339(), s);
    }

    #[test]
    fn test_from_conversions() {
        let inner = chrono::Utc::now();
        let wrapper: TimestampUTC = inner.into();
        assert_eq!(wrapper.0, inner);

        let back: chrono::DateTime<chrono::Utc> = wrapper.into();
        assert_eq!(back, inner);
    }

    #[test]
    fn test_naive_conversion() {
        let naive =
            NaiveDateTime::parse_from_str("2023-10-27 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let t: TimestampUTC = naive.into();
        assert_eq!(
            t.format("%Y-%m-%d %H:%M:%S").to_string(),
            "2023-10-27 10:00:00"
        );
    }

    #[test]
    fn test_as_ref_as_mut() {
        let mut t = TimestampUTC::now();

        let r: &chrono::DateTime<chrono::Utc> = t.as_ref();
        assert_eq!(*r, t.0);

        let m: &mut chrono::DateTime<chrono::Utc> = t.as_mut();
        *m = chrono::Utc::now(); // Modify
    }

    #[test]
    fn test_deref_deref_mut() {
        let mut t = TimestampUTC::now();

        // Deref
        assert!(t.timestamp() > 0);

        // DerefMut (modifying internally)
        // using a specific date
        let old_time = t;
        *t = old_time.0 - chrono::Duration::days(1);
        assert!(*t < *old_time);
    }

    #[test]
    fn test_display() {
        let s = "2023-10-27T10:00:00+00:00";
        let t = TimestampUTC::from_str(s).unwrap();
        assert_eq!(format!("{t}"), "2023-10-27 10:00:00 UTC");
    }

    #[test]
    fn test_standard_traits() {
        let t1 = TimestampUTC::now();
        let t2 = Clone::clone(&t1); // Clone
        let t3 = t1; // Copy

        assert_eq!(t1, t2); // PartialEq
        assert_eq!(t1, t3); // PartialEq
        assert!(t1 == t2); // Eq check implicitly

        let mut set = HashSet::new();
        set.insert(t1); // Hash
        assert!(set.contains(&t2));
    }

    #[test]
    fn test_ord() {
        let t1 = TimestampUTC::from_str("2023-01-01T00:00:00+00:00").unwrap();
        let t2 = TimestampUTC::from_str("2023-01-02T00:00:00+00:00").unwrap();

        assert_eq!(t1.cmp(&t2), Ordering::Less); // Ord
        assert!(t1 < t2); // PartialOrd
    }
}
