#![allow(unused)]

use super::{get, read_keys};
use super::{Error, Result};
use crate::b32::{decode, decode_to_string};
use std::collections::HashMap;

crate::generate_env_methods!(b32);

#[cfg(test)]
mod tests {
    use crate::b32;

    use super::*;
    use std::{env, io::Read};

    #[test]
    fn test_get_as_s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B32", "NBSWY3DP"); // "hello" в base32

        // Получаем значение переменной окружения как строку
        let result = get_b32_as_s("TEST_B32");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");

        // Проверяем отсутствие переменной окружения
        let result = get_b32_as_s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_B32", "NBSWY3DP"); // "hello" в base32

        // Получаем значение переменной окружения как вектор байтов
        let result = get_b32_as_u8s("TEST_B32");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"hello");

        // Проверяем отсутствие переменной окружения
        let result = get_b32_as_u8s("NON_EXISTENT_VAR");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_KEYS_B32", "KEY1:JNCVSMI,KEY2:JNCVSMQ"); // "hello" в base32

        // Получаем ключи из переменной окружения
        let result = get_keys_b32_as_s("TEST_KEYS_B32");
        assert!(result.is_ok());
        let map = result.unwrap();
        assert_eq!(map.get("KEY1"), Some(&"KEY1".to_string()));
        assert_eq!(map.get("KEY2"), Some(&"KEY2".to_string()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b32_as_s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_u8s() {
        // Установим переменную окружения для теста
        env::set_var("TEST_KEYS_B32", "KEY1:NBSWY3DP,KEY2:NBSWY3DP"); // "hello" в base32

        // Получаем ключи из переменной окружения
        let result = get_keys_b32_as_u8s("TEST_KEYS_B32");
        assert!(result.is_ok());
        let map = result.unwrap();

        assert_eq!(map.get("KEY1"), Some(&b"hello".to_vec()));
        assert_eq!(map.get("KEY2"), Some(&b"hello".to_vec()));

        // Проверяем отсутствие переменной окружения
        let result = get_keys_b32_as_u8s("NON_EXISTENT_KEYS");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_B32_INVALID", "INVALID_BASE32");

        // Проверяем, что функция возвращает ошибку
        let result = get_b32_as_s("TEST_B32_INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_keys_as_s_invalid() {
        // Установим переменную окружения с некорректным значением
        env::set_var("TEST_KEYS_B32_INVALID", "KEY1:INVALID_BASE32");

        // Проверяем, что функция возвращает ошибку
        let result = get_keys_b32_as_s("TEST_KEYS_B32_INVALID");
        assert!(result.is_err());
    }
}
