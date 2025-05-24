use derive_more::derive::From;
use time::{Duration, OffsetDateTime};

pub use time::format_description::well_known::Rfc3339;

/// Returns the current time in UTC.
pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

/// Formats the time as a string in RFC3339 format.
pub fn format_time(time: OffsetDateTime) -> Result<String> {
    Ok(time.format(&Rfc3339)?)
}

/// Returns the current time in UTC plus the given seconds as a formatted RFC3339 string.
pub fn now_utc_plus_sec_str(sec: f64) -> Result<String> {
    let new_time = now_utc() + Duration::seconds_f64(sec);
    format_time(new_time)
}

/// Returns parsed time from string in RFC3339 format.
pub fn parse_utc(moment: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| Error::FailToDateParse(moment.to_string()))
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    FailToDateParse(String),
    #[from]
    FailToFormat(time::error::Format),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error
