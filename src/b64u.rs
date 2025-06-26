use super::base_x::{self, Result};
const ENCODING: data_encoding::Encoding = data_encoding::BASE64URL_NOPAD;

/// Encodes data into a Base64 URL-safe string without padding.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A string representing the encoded data.
pub fn encode(content: impl AsRef<[u8]>) -> String {
    base_x::encode(content, ENCODING)
}

/// Decodes a Base64 URL-safe string without padding into a vector of bytes.
///
/// # Parameters
/// - `b64u`: A string containing the encoded Base64 data.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn decode(b64u: &str) -> Result<Vec<u8>> {
    base_x::decode(b64u, ENCODING)
}

/// Decodes a Base64 URL-safe string without padding into a string.
///
/// # Parameters
/// - `b64u`: A string containing the encoded Base64 data.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn decode_to_string(b64u: &str) -> Result<String> {
    base_x::decode_to_string(b64u, ENCODING)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const TEXT: &str = "This is not just a string!";
    const RESULT: &str = "VGhpcyBpcyBub3QganVzdCBhIHN0cmluZyE";

    #[test]
    fn test_decode() -> Result<()> {
        let decoded = decode(RESULT)?;
        assert_eq!(decoded, TEXT.as_bytes());
        Ok(())
    }

    #[test]
    fn test_decode_to_string() -> Result<()> {
        let decoded = decode_to_string(RESULT)?;
        assert_eq!(decoded, TEXT);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let encoded = encode(&TEXT);
        assert_eq!(encoded, RESULT);

        Ok(())
    }
}

// endregion: --- Tests
