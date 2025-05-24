#[cfg(feature = "b32")]
mod b32;
#[cfg(feature = "b58")]
mod b58;
#[cfg(feature = "b64")]
mod b64;

mod error;

pub use error::{Error, Result};

use enum_dispatch::enum_dispatch;

use super::CUuid;

#[enum_dispatch]
pub trait Scheme {
    fn encode(&self, content: impl AsRef<[u8]>) -> String;
    #[inline]
    fn decode(&self, content: &str) -> Vec<u8> {
        self.try_decode(content).expect("Decode failed")
    }
    fn try_decode(&self, content: &str) -> Result<Vec<u8>>;
}

#[enum_dispatch(Scheme)]
pub enum SchemeDispatcher {
    #[cfg(feature = "b32")]
    B32(b32::SchemeB32),
    #[cfg(feature = "b58")]
    B58(b58::SchemeB58),
    #[cfg(feature = "b64")]
    B64(b64::SchemeB64),
}

pub fn get_scheme(cuuid: &CUuid) -> impl Scheme {
    match cuuid {
        #[cfg(feature = "b32")]
        CUuid::B32 => SchemeDispatcher::B32(b32::SchemeB32),
        #[cfg(feature = "b58")]
        CUuid::B58 => SchemeDispatcher::B58(b58::SchemeB58),
        #[cfg(feature = "b64")]
        CUuid::B64 => SchemeDispatcher::B64(b64::SchemeB64),
    }
}
