use data_encoding::Encoding;

/// Encodes data into a BaseX string using the specified encoding method.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
/// - `encoding`: The BaseX encoding method to be used.
///
/// # Returns
/// A string representing the encoded data.
pub fn encode(content: impl AsRef<[u8]>, encoding: Encoding) -> String {
    encoding.encode(content.as_ref())
}

/// Decodes a BaseX string into a vector of bytes using the specified decoding method.
///
/// # Parameters
/// - `value`: A string containing the encoded BaseX data.
/// - `encoding`: The BaseX decoding method to be used.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn decode(value: &str, encoding: Encoding) -> Result<Vec<u8>> {
    encoding
        .decode(value.as_bytes())
        .map_err(|_| Error::DecodeError(value.to_string()))
}

/// Decodes a BaseX string into a string using the specified decoding method.
///
/// # Parameters
/// - `value`: A string containing the encoded BaseX data.
/// - `encoding`: The BaseX decoding method to be used.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn decode_to_string(value: &str, encoding: Encoding) -> Result<String> {
    let decoded = decode(value, encoding)?;
    String::from_utf8(decoded).map_err(|_| Error::InvalidUtf8)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    DecodeError(String),
    InvalidUtf8,
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Error::DecodeError(b64) => write!(fmt, "Failed to decode string: {}", b64),
            Error::InvalidUtf8 => write!(fmt, "Decoded bytes are not valid UTF-8"),
        }
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error
