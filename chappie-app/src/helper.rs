use chrono::{NaiveDate, ParseError};

pub struct LocalDateTime(pub String);

impl TryFrom<LocalDateTime> for NaiveDate {
    type Error = ParseError;

    fn try_from(value: LocalDateTime) -> Result<Self, Self::Error> {
        NaiveDate::parse_from_str(&value.0, "%Y-%m-%d")
    }
}
