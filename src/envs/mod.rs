use std::collections::HashMap;
use std::env;
use std::str::FromStr;

mod macros;

#[cfg(feature = "b32")]
mod b32;
#[cfg(feature = "b58")]
mod b58;
#[cfg(feature = "b64")]
mod b64u;

#[cfg(feature = "b32")]
pub use b32::*;
#[cfg(feature = "b58")]
pub use b58::*;
#[cfg(feature = "b64")]
pub use b64u::*;

/// Gets the value of an environment variable into a string.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A string representing the value of the environment variable.
pub fn get(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

/// Gets the value of an environment variable and parses it into a type.
///
/// # Parameters
/// - `name`: The name of the environment variable.
///
/// # Returns
/// A parsed value of the environment variable.
pub fn get_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

/// Reads key-value pairs from a specified environment variable.
///
/// The environment variable should contain pairs in the format `key:value`,
/// separated by commas. For example: `KEY1:VALUE1,KEY2:VALUE2`.
///
/// # Parameters
/// - `name`: The name of the environment variable containing the key-value pairs.
///
/// # Returns
/// A `Result<Vec<(String, String)>>` containing a vector of tuples,
/// where each tuple represents a key-value pair extracted from the
/// environment variable. If the environment variable is not set
/// or if the format is incorrect, an error is returned.
fn read_keys(name: &'static str) -> Result<Vec<(String, String)>> {
    // Read keys from environment
    let keys = env::var(name).map_err(|_| Error::MissingEnv(name))?;

    let mut result = Vec::new();

    for pair in keys.split(',') {
        let mut split = pair.split(':');
        if let (Some(key), Some(value)) = (split.next(), split.next()) {
            result.push((key.trim().to_string(), value.trim().to_string()));
        } else {
            return Err(Error::WrongFormat(name)); // Обработка ошибки, если пара некорректна
        }
    }

    Ok(result)
}

/// Retrieves a set of key-value pairs from an environment variable.
///
/// The environment variable should contain pairs in the format `key:value`,
/// separated by commas. For example: `KEY1:VALUE1,KEY2:VALUE2`.
///
/// # Parameters
/// - `name`: The name of the environment variable containing the key-value pairs.
///
/// # Returns
/// A `HashMap<String, String>` representing the key-value pairs extracted
/// from the environment variable. If the environment variable is not set
/// or if the format is incorrect, an error is returned.
pub fn get_keys(name: &'static str) -> Result<HashMap<String, String>> {
    Ok(read_keys(name)?.into_iter().collect())
}

/// Retrieves a set of key-value pairs from an environment variable and parses the values.
///
/// The environment variable should contain pairs in the format `key:value`,
/// separated by commas. For example: `KEY1:VALUE1,KEY2:VALUE2`.
///
/// # Parameters
/// - `name`: The name of the environment variable containing the key-value pairs.
///
/// # Type Parameters
/// - `T`: The type to which the values should be parsed. This type must implement
///   the `FromStr` trait.
///
/// # Returns
/// A `Result<HashMap<String, T>>` representing the key-value pairs extracted
/// from the environment variable, with values parsed into the specified type `T`.
/// If the environment variable is not set, if the format is incorrect, or if
/// the values cannot be parsed into the specified type, an error is returned.
pub fn get_keys_parse<T: FromStr>(name: &'static str) -> Result<HashMap<String, T>> {
    let mut hash_map = HashMap::new();

    for (key, value) in read_keys(name)? {
        let value = value.parse::<T>().map_err(|_| Error::WrongFormat(name))?;
        hash_map.insert(key, value);
    }

    Ok(hash_map)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get() {
        // Установим переменную окружения для теста
        env::set_var("TEST_VAR", "test_value");

        // Получаем значение переменной окружения
        let result = get("TEST_VAR");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_value");

        // Проверяем отсутствие переменной окружения
        let result = get("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_parse() {
        // Установим переменную окружения для теста
        env::set_var("TEST_INT", "42");

        // Получаем и парсим значение переменной окружения
        let result: Result<i32> = get_parse("TEST_INT");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        // Проверяем неправильный формат
        env::set_var("TEST_INVALID", "not_a_number");
        let result: Result<i32> = get_parse("TEST_INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys() {
        // Установим переменную окружения для теста
        env::set_var("TEST_KEYS", "KEY1:VALUE1,KEY2:VALUE2");

        // Получаем ключи из переменной окружения
        let result = get_keys("TEST_KEYS");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("KEY1"), Some(&"VALUE1".to_string()));
        assert_eq!(map.get("KEY2"), Some(&"VALUE2".to_string()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_parse() {
        // Установим переменную окружения для теста
        env::set_var("TEST_KEYS_PARSE", "KEY1:1,KEY2:2");

        // Получаем и парсим ключи из переменной окружения
        let result: Result<HashMap<String, i32>> = get_keys_parse("TEST_KEYS_PARSE");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("KEY1"), Some(&1));
        assert_eq!(map.get("KEY2"), Some(&2));

        // Проверяем неправильный формат
        env::set_var("TEST_INVALID_KEYS", "KEY1:not_a_number");
        let result: Result<HashMap<String, i32>> = get_keys_parse("TEST_INVALID_KEYS");
        assert!(result.is_err());
    }
}
