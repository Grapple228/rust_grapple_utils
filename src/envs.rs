use std::env;
use std::str::FromStr;

/// Gets the value of an environment variable into a string.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A string representing the value of the environment variable.
pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

/// Gets the value of an environment variable and parses it into a type.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A parsed value of the environment variable.
pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base64 URL-safe encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base64 string into UTF--8 string.
#[cfg(feature = "b64")]
pub fn get_env_b64u_as_s(name: &'static str) -> Result<String> {
    crate::b64::b64u_decode_to_string(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base64 URL-safe encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base64 string into a vector of bytes.
#[cfg(feature = "b64")]
pub fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    crate::b64::b64u_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base58 encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base58 string into UTF--8 string.
#[cfg(feature = "b58")]
pub fn get_env_b58_as_s(name: &'static str) -> Result<String> {
    crate::b58::b58_decode_to_string(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base58 encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base58 string into a vector of bytes.
#[cfg(feature = "b58")]
pub fn get_env_b58_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    crate::b58::b58_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base32 encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base32 string into UTF--8 string.
#[cfg(feature = "b32")]
pub fn get_env_b32_as_s(name: &'static str) -> Result<String> {
    crate::b32::b32_decode_to_string(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

/// Gets the value of an environment variable in Base32 encoded format.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A decoded Base32 string into a vector of bytes.
#[cfg(feature = "b32")]
pub fn get_env_b32_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    crate::b32::b32_decode(&get_env(name)?).map_err(|_| Error::WrongFormat(name))
}

// region:    --- Error
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
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
