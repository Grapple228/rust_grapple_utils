#![allow(unused)]

use super::{get, read_keys};
use super::{Error, Result};
use crate::b58::{decode, decode_to_string};
use std::collections::HashMap;

crate::generate_env_methods!(b58);

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_as_s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B58", "9Ajdvzr"); // "Hello" в base58

        // Получаем значение переменной окружения как строку
        let result = get_b58_as_s("TEST_B58");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");

        // Проверяем отсутствие переменной окружения
        let result = get_b58_as_s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B58", "9Ajdvzr"); // "Hello" в base58

        // Получаем значение переменной окружения как вектор байтов
        let result = get_b58_as_u8s("TEST_B58");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"Hello");

        // Проверяем отсутствие переменной окружения
        let result = get_b58_as_u8s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s() {
        // Установим переменную окружения для теста
        env::set_var(
            "TEST_KEYS_B58",
            "KEY1:9Ajdvzr,KEY2:9Ajdvzr", // "Hello" в base58
        );

        // Получаем ключи из переменной окружения
        let result = get_keys_b58_as_s("TEST_KEYS_B58");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("KEY1"), Some(&"Hello".to_string()));
        assert_eq!(map.get("KEY2"), Some(&"Hello".to_string()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b58_as_s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var(
            "TEST_KEYS_B58",
            "KEY1:9Ajdvzr,KEY2:9Ajdvzr", // "Hello" в base58
        );

        // Получаем ключи из переменной окружения
        let result = get_keys_b58_as_u8s("TEST_KEYS_B58");
        assert!(result.is_ok());
        let map = result.unwrap();

        assert_eq!(map.get("KEY1"), Some(&b"Hello".to_vec()));
        assert_eq!(map.get("KEY2"), Some(&b"Hello".to_vec()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b58_as_u8s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_B58_INVALID", "INVALID_BASE58");

        // Проверяем, что функция возвращает ошибку
        let result = get_b58_as_s("TEST_B58_INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_KEYS_B58_INVALID", "KEY1:INVALID_BASE58");

        // Проверяем, что функция возвращает ошибку
        let result = get_keys_b58_as_s("TEST_KEYS_B58_INVALID");
        assert!(result.is_err());
    }
}
