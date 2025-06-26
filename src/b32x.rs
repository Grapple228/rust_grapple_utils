use crate::base_x::{self, Result};
const ENCODING: data_encoding::Encoding = data_encoding::BASE32HEX_NOPAD;

/// Encodes data into a Base32 HEX string.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A string representing the encoded data.
pub fn encode(content: impl AsRef<[u8]>) -> String {
    base_x::encode(content, ENCODING)
}

/// Decodes a Base32 HEX string into a vector of bytes.
///
/// # Parameters
/// - `b32`: A string containing the encoded Base32 HEX data.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn decode(b32: &str) -> Result<Vec<u8>> {
    base_x::decode(b32, ENCODING)
}

/// Decodes a Base32 HEX string into a string using the specified decoding method.
///
/// # Parameters
/// - `b32`: A string containing the encoded Base32 HEX data.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn decode_to_string(b32: &str) -> Result<String> {
    base_x::decode_to_string(b32, ENCODING)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const TEXT: &str = "This is not just a string!";
    const RESULT: &str = "AHK6ISP0D5PI0RJFEGG6KTBJEGG6283JEHP6IRJ744";

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
