mod scheme;

use derive_more::derive::From;
use scheme::{get_scheme, Scheme};
use uuid::Uuid;

#[derive(Debug)]
pub enum CUuid {
    #[cfg(feature = "b58")]
    B58,
    #[cfg(feature = "b64")]
    B64,
    #[cfg(feature = "b32")]
    B32,
}

impl CUuid {
    #[inline]
    fn get_scheme(&self) -> impl Scheme {
        get_scheme(self)
    }

    /// Converts a given id into a BaseX encoded UUID.
    ///
    /// # Arguments
    /// * `id` - An object that can be converted into a `uuid::Uuid`.
    ///
    /// # Returns
    /// A BaseX encoded string representation of the UUID.
    #[inline]
    pub fn from(&self, id: impl Into<Uuid>) -> String {
        self.encode(id.into())
    }

    /// Try to convert a given id into a BaseX encoded UUID.
    ///
    /// # Arguments
    /// * `id` - An object that can be tried to convert into a `uuid::Uuid`.
    ///
    /// # Returns
    /// * `Ok(String)` if the conversion is successful.
    /// * `Err(Error::InvalidFormat)` if the conversion fails.
    #[inline]
    pub fn try_from(&self, id: impl TryInto<Uuid>) -> Result<String> {
        let id = id.try_into().map_err(|_| Error::InvalidFormat)?;
        Ok(self.encode(id))
    }

    /// Generates a new time-based UUID and encodes it to BaseX.
    #[inline]
    pub fn now_v7(&self) -> String {
        self.encode(Uuid::now_v7())
    }

    /// Generates a new random UUID and encodes it to BaseX.
    #[inline]
    pub fn new_v4(&self) -> String {
        self.encode(Uuid::new_v4())
    }

    /// Attempts to decode a BaseX encoded string into a UUID.
    ///
    /// # Arguments
    /// * `value` - An BaseX encoded string.
    ///
    /// # Returns
    /// * `Ok(uuid::Uuid)` if the decoding is successful.
    /// * `Err(Error::FailToDecode)` if decoding fails.
    /// * `Err(Error::InvalidFormat)` if the length is not 16 bytes.
    pub fn try_decode(&self, value: &str) -> Result<Uuid> {
        if value.is_empty() {
            return Err(Error::InvalidFormat);
        }

        let bytes = self
            .get_scheme()
            .try_decode(value)
            .map_err(|_| Error::FailToDecode)?;

        match bytes.len() {
            16 => Uuid::from_slice(&bytes).map_err(|_| Error::InvalidFormat),
            _ => Err(Error::InvalidFormat),
        }
    }

    /// Decode an encoded string into a UUID.
    ///
    /// # Arguments
    /// * `value` - An BaseX encoded string.
    ///
    /// # Panics
    /// This will panic if decoding fails.
    #[inline]
    pub fn decode(&self, value: &str) -> Uuid {
        self.try_decode(value).expect("Decode failed")
    }

    #[inline]
    /// Encode Uuid into string in specified format
    fn encode(&self, id: Uuid) -> String {
        self.get_scheme().encode(id)
    }
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    FailToEncode,
    FailToDecode,
    InvalidFormat,

    #[from]
    Scheme(scheme::Error),
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
