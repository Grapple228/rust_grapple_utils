/// Encodes data into a Base58 string.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A string representing the encoded data.
pub fn b32_encode(content: impl AsRef<[u8]>) -> String {
    data_encoding::BASE32HEX_NOPAD.encode(content.as_ref())
}

/// Decodes a Base64 string into a vector of bytes.
///
/// # Parameters
/// - `b32`: A string containing the encoded Base58 data.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn b32_decode(b32: &str) -> Result<Vec<u8>> {
    data_encoding::BASE32HEX_NOPAD
        .decode(b32.as_bytes())
        .map_err(|_| Error::FailTob32Decode)
}

/// Decodes a Base58 string into a string using the specified decoding method.
///
/// # Parameters
/// - `b32`: A string containing the encoded Base58 data.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn b32_decode_to_string(b32: &str) -> Result<String> {
    b32_decode(b32)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailTob32Decode)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailTob32Decode,
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>; // For tests.

    use super::*;

    const TEXT: &str = "This is not just a string!";

    #[test]
    fn test_b32_decode() -> Result<()> {
        let b32 = "AHK6ISP0D5PI0RJFEGG6KTBJEGG6283JEHP6IRJ744";
        let decoded = b32_decode(b32)?;
        assert_eq!(decoded, TEXT.as_bytes());
        Ok(())
    }

    #[test]
    fn test_b32_decode_to_string() -> Result<()> {
        let b32 = "AHK6ISP0D5PI0RJFEGG6KTBJEGG6283JEHP6IRJ744";
        let decoded = b32_decode_to_string(b32)?;
        assert_eq!(decoded, TEXT);
        Ok(())
    }

    #[test]
    fn test_b32_encode() -> Result<()> {
        let data = vec![0x12, 0x34, 0x56];
        let encoded = b32_encode(&data);
        assert_eq!(encoded, "28Q5C");

        Ok(())
    }
}

// endregion: --- Tests
