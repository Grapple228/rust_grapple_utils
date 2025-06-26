// This is config example, than uses the `env` feature.

use std::collections::HashMap;

#[cfg(feature = "envs")]
use grapple_utils::envs;

#[cfg(feature = "envs")]
pub fn config() -> &'static Config {
    use std::sync::OnceLock;
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Config {
    pub STRING: String,
    pub NUMBER: f64,
    pub KEYS: HashMap<String, String>,
    #[cfg(feature = "b64")]
    pub B64: Vec<u8>, // Useful for secret keys
    #[cfg(feature = "b64")]
    pub B64D: String,
    #[cfg(feature = "b58")]
    pub B58: Vec<u8>,
    #[cfg(feature = "b58")]
    pub B58D: String,
    #[cfg(feature = "b32")]
    pub B32: Vec<u8>,
    #[cfg(feature = "b32")]
    pub B32D: String,
}

impl Config {
    #[cfg(feature = "envs")]
    fn load_from_env() -> grapple_utils::envs::Result<Config> {
        Ok(Config {
            STRING: envs::get("STRING")?,
            NUMBER: envs::get_parse("NUMBER")?,
            KEYS: envs::get_keys("KEYS")?,

            #[cfg(feature = "b64")]
            B64: envs::get_b64u_as_u8s("B64")?,
            #[cfg(feature = "b58")]
            B58: envs::get_b58_as_u8s("B58")?,
            #[cfg(feature = "b32")]
            B32: envs::get_b32_as_u8s("B32")?,
            #[cfg(feature = "b64")]
            B64D: envs::get_b64u_as_s("B64")?,
            #[cfg(feature = "b58")]
            B58D: envs::get_b58_as_s("B58")?,
            #[cfg(feature = "b32")]
            B32D: envs::get_b32_as_s("B32")?,
        })
    }
}

#[cfg(not(feature = "envs"))]
fn main() {
    panic!("This example requires 'envs' feature to be enabled.");
}

#[cfg(feature = "envs")]
fn main() {
    println!("STRING: {}", config().STRING);

    println!("NUMBER: {}", config().NUMBER);

    #[cfg(feature = "b64")]
    println!("B64: {}", config().B64.len());
    #[cfg(feature = "b64")]
    println!("B64 decoded: {}", config().B64D);

    #[cfg(feature = "b58")]
    println!("B58: {}", config().B58.len());
    #[cfg(feature = "b58")]
    println!("B58 decoded: {}", config().B58D);

    #[cfg(feature = "b32")]
    println!("B32: {}", config().B32.len());
    #[cfg(feature = "b32")]
    println!("B32 decoded: {}", config().B32D);
}
