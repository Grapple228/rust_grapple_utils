#[macro_export]
macro_rules! generate_env_methods {
    ($base:ident) => {
        paste::paste! {
        /// Gets the value of an environment variable in a specific encoded format.
        ///
        /// # Parameters
        /// - `name`: The name of the environment variable.
        ///
        /// # Returns
        /// A decoded string into UTF-8 string.
        pub fn [<get_ $base _as_s>] (name: &'static str) -> Result<String>  {
            decode_to_string(&get(name)?).map_err(|_| Error::WrongFormat(name))
        }}

        paste::paste! {
        /// Gets the value of an environment variable in a specific encoded format.
        ///
        /// # Parameters
        /// - `name`: The name of the environment variable.
        ///
        /// # Returns
        /// A decoded string into a vector of bytes.
        pub fn [<get_ $base _as_u8s>](name: &'static str) -> Result<Vec<u8>> {
            decode(&get(name)?).map_err(|_| Error::WrongFormat(name))
        }}

        paste::paste! {
        /// Retrieves a set of key-value pairs from an environment variable,
        /// decoding the values from a specific format into UTF-8 strings.
        ///
        /// The environment variable should contain pairs in the format `key:value`,
        /// separated by commas. For example: `KEY1:VALUE1,KEY2:VALUE2`.
        ///
        /// # Parameters
        /// - `name`: The name of the environment variable containing the key-value pairs.
        ///
        /// # Returns
        /// A `HashMap<String, String>` representing the key-value pairs extracted
        /// from the environment variable, with values decoded from the specific format.
        /// If the environment variable is not set, if the format is incorrect, or if
        /// the values cannot be decoded, an error is returned.
        pub fn [<get_keys_ $base _as_s>](name: &'static str) -> Result<HashMap<String, String>> {
            let mut hash_map = HashMap::new();

            for (key, value) in read_keys(name)? {
                let value = decode_to_string(&value).map_err(|_| Error::WrongFormat(name))?;
                hash_map.insert(key, value);
            }

            Ok(hash_map)
        }}

        paste::paste! {
        /// Retrieves a set of key-value pairs from an environment variable,
        /// decoding the values from a specific format into byte vectors.
        ///
        /// The environment variable should contain pairs in the format `key:value`,
        /// separated by commas. For example: `KEY1:VALUE1,KEY2:VALUE2`.
        ///
        /// # Parameters
        /// - `name`: The name of the environment variable containing the key-value pairs.
        ///
        /// # Returns
        /// A `HashMap<String, Vec<u8>>` representing the key-value pairs extracted
        /// from the environment variable, with values decoded from the specific format.
        /// If the environment variable is not set, if the format is incorrect, or if
        /// the values cannot be decoded, an error is returned.
        pub fn [<get_keys_ $base _as_u8s>](name: &'static str) -> Result<HashMap<String, Vec<u8>>> {
            let mut hash_map = HashMap::new();

            for (key, value) in read_keys(name)? {
                let value = decode(&value).map_err(|_| Error::WrongFormat(name))?;
                hash_map.insert(key, value);
            }

            Ok(hash_map)
        }}
    };
}
