use data_encoding::Encoding;

/// Encodes data into a Base64 string using the specified encoding method.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
/// - `encoding`: The Base64 encoding method to be used.
///
/// # Returns
/// A string representing the encoded data.
pub fn b64_encode(content: impl AsRef<[u8]>, encoding: Encoding) -> String {
    encoding.encode(content.as_ref())
}

/// Decodes a Base64 string into a vector of bytes using the specified decoding method.
///
/// # Parameters
/// - `b64`: A string containing the encoded Base64 data.
/// - `encoding`: The Base64 decoding method to be used.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn b64_decode(b64: &str, encoding: Encoding) -> Result<Vec<u8>> {
    encoding
        .decode(b64.as_bytes())
        .map_err(|_| Error::DecodeError(b64.to_string()))
}

/// Decodes a Base64 string into a string using the specified decoding method.
///
/// # Parameters
/// - `b64u`: A string containing the encoded Base64 data.
/// - `encoding`: The Base64 decoding method to be used.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn b64_decode_to_string(b64u: &str, encoding: Encoding) -> Result<String> {
    let decoded = b64_decode(b64u, encoding)?;
    String::from_utf8(decoded).map_err(|_| Error::InvalidUtf8)
}

/// Encodes data into a Base64 URL-safe string without padding.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A string representing the encoded data.
pub fn b64u_encode(content: impl AsRef<[u8]>) -> String {
    b64_encode(content, data_encoding::BASE64URL_NOPAD)
}

/// Decodes a Base64 URL-safe string without padding into a vector of bytes.
///
/// # Parameters
/// - `b64u`: A string containing the encoded Base64 data.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn b64u_decode(b64u: &str) -> Result<Vec<u8>> {
    b64_decode(b64u, data_encoding::BASE64URL_NOPAD)
}

/// Decodes a Base64 URL-safe string without padding into a string.
///
/// # Parameters
/// - `b64u`: A string containing the encoded Base64 data.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn b64u_decode_to_string(b64u: &str) -> Result<String> {
    b64_decode_to_string(b64u, data_encoding::BASE64URL_NOPAD)
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
            Error::DecodeError(b64) => write!(fmt, "Failed to decode Base64 string: {}", b64),
            Error::InvalidUtf8 => write!(fmt, "Decoded bytes are not valid UTF-8"),
        }
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// endregion: --- Error

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const TEXT: &str = "This is not just a string!";

    #[test]
    fn test_b64u_decode() -> Result<()> {
        let b64u = "VGhpcyBpcyBub3QganVzdCBhIHN0cmluZyE";
        let decoded = b64u_decode(b64u)?;
        assert_eq!(decoded, TEXT.as_bytes());
        Ok(())
    }

    #[test]
    fn test_b64u_decode_to_string() -> Result<()> {
        let b64u = "VGhpcyBpcyBub3QganVzdCBhIHN0cmluZyE";
        let decoded = b64u_decode_to_string(b64u)?;
        assert_eq!(decoded, TEXT);
        Ok(())
    }

    #[test]
    fn test_b58_encode() -> Result<()> {
        let data = vec![0x12, 0x34, 0x56];
        let encoded = b64u_encode(&data);
        assert_eq!(encoded, "EjRW");
        Ok(())
    }
}

// endregion: --- Tests
