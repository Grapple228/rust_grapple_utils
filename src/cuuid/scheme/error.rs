use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    SchemeNotFound(String),

    #[from]
    #[cfg(feature = "b32")]
    B32(crate::b32::Error),
    #[from]
    #[cfg(feature = "b58")]
    B58(crate::b58::Error),
    #[from]
    #[cfg(feature = "b64")]
    B64(crate::b64::Error),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate
