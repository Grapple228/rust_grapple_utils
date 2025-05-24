use base58::{FromBase58, ToBase58};

/// Encodes data into a Base58 string.
///
/// # Parameters
/// - `content`: The data to be encoded. Can be any type that implements `AsRef<[u8]>`.
///
/// # Returns
/// A string representing the encoded data.
pub fn b58_encode(content: impl AsRef<[u8]>) -> String {
    let bytes: &[u8] = content.as_ref();
    bytes.to_base58()
}

/// Decodes a Base64 string into a vector of bytes.
///
/// # Parameters
/// - `b58`: A string containing the encoded Base58 data.
///
/// # Returns
/// A result containing a vector of bytes if decoding is successful, or an error.
pub fn b58_decode(b58: &str) -> Result<Vec<u8>> {
    b58.from_base58().map_err(|_| Error::FailToB58Decode)
}

/// Decodes a Base58 string into a string using the specified decoding method.
///
/// # Parameters
/// - `b58`: A string containing the encoded Base58 data.
///
/// # Returns
/// A result containing a string if decoding is successful and the data is valid UTF-8, or an error.
pub fn b58_decode_to_string(b58: &str) -> Result<String> {
    b58_decode(b58)
        .ok()
        .and_then(|r| String::from_utf8(r).ok())
        .ok_or(Error::FailToB58Decode)
}

// region:    --- Error

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    FailToB58Decode,
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
    fn test_b58_decode() -> Result<()> {
        let b58 = "3aump9mdueoaV87JMp3adSVWqNmpr9B43pnL";
        let decoded = b58_decode(b58)?;
        assert_eq!(decoded, TEXT.as_bytes());
        Ok(())
    }

    #[test]
    fn test_b58_decode_to_string() -> Result<()> {
        let b58 = "3aump9mdueoaV87JMp3adSVWqNmpr9B43pnL";
        let decoded = b58_decode_to_string(b58)?;
        assert_eq!(decoded, TEXT);
        Ok(())
    }

    #[test]
    fn test_b58_encode() -> Result<()> {
        let data = vec![0x12, 0x34, 0x56];
        let encoded = b58_encode(&data);
        assert_eq!(encoded, "77em");
        Ok(())
    }
}

// endregion: --- Tests
