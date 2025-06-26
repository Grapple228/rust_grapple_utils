#![allow(unused)]

use super::{get, read_keys};
use super::{Error, Result};
use crate::b64u::{decode, decode_to_string};
use std::collections::HashMap;

crate::generate_env_methods!(b64u);

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_as_s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B64U", "SGVsbG8gd29ybGQ"); // "Hello world" в base64url

        // Получаем значение переменной окружения как строку
        let result = get_b64u_as_s("TEST_B64U");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello world");

        // Проверяем отсутствие переменной окружения
        let result = get_b64u_as_s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B64U", "SGVsbG8gd29ybGQ"); // "Hello world" в base64url

        // Получаем значение переменной окружения как вектор байтов
        let result = get_b64u_as_u8s("TEST_B64U");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"Hello world");

        // Проверяем отсутствие переменной окружения
        let result = get_b64u_as_u8s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s() {
        // Установим переменную окружения для теста
        env::set_var(
            "TEST_KEYS_B64U",
            "KEY1:SGVsbG8gd29ybGQ,KEY2:SGVsbG8gd29ybGQ", // "Hello world" в base64url
        );

        // Получаем ключи из переменной окружения
        let result = get_keys_b64u_as_s("TEST_KEYS_B64U");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("KEY1"), Some(&"Hello world".to_string()));
        assert_eq!(map.get("KEY2"), Some(&"Hello world".to_string()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b64u_as_s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var(
            "TEST_KEYS_B64U",
            "KEY1:SGVsbG8gd29ybGQ,KEY2:SGVsbG8gd29ybGQ", // "Hello world" в base64url
        );

        // Получаем ключи из переменной окружения
        let result = get_keys_b64u_as_u8s("TEST_KEYS_B64U");
        assert!(result.is_ok());
        let map = result.unwrap();

        assert_eq!(map.get("KEY1"), Some(&b"Hello world".to_vec()));
        assert_eq!(map.get("KEY2"), Some(&b"Hello world".to_vec()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b64u_as_u8s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_B64U_INVALID", "INVALID_BASE64URL");

        // Проверяем, что функция возвращает ошибку
        let result = get_b64u_as_s("TEST_B64U_INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_KEYS_B64U_INVALID", "KEY1:INVALID_BASE64URL");

        // Проверяем, что функция возвращает ошибку
        let result = get_keys_b64u_as_s("TEST_KEYS_B64U_INVALID");
        assert!(result.is_err());
    }
}
