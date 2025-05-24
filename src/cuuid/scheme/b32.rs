use crate::b32::b32_decode;
use crate::b32::b32_encode;

use super::Result;
use super::Scheme;

pub struct SchemeB32;

impl Scheme for SchemeB32 {
    #[inline]
    fn encode(&self, content: impl AsRef<[u8]>) -> String {
        b32_encode(content)
    }

    #[inline]
    fn try_decode(&self, content: &str) -> Result<Vec<u8>> {
        Ok(b32_decode(content)?)
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use crate::cuuid::Result;

    use uuid::Uuid;

    use crate::cuuid::CUuid;

    const V4_ENCODED: &str = "3RH05RCGI11J3BO6O9GN2LFG98";
    const V7_ENCODED: &str = "06BG6S1E5LRE73397EQUIGTIK8";
    const C: CUuid = CUuid::B32;

    fn get_v7() -> Uuid {
        Uuid::parse_str("01970370-2e2d-76e3-8c69-3bb5e943b2a2").unwrap()
    }

    fn get_v4() -> Uuid {
        Uuid::parse_str("1ee202ed-9090-4331-af06-c2617155f04a").unwrap()
    }

    #[test]
    fn test_uuid_v7_b32_encode() -> Result<()> {
        let fx_id = get_v7();

        let encoded = C.encode(fx_id);

        assert_eq!(encoded, V7_ENCODED);

        Ok(())
    }

    #[test]
    fn test_uuid_v7_b32_decode() -> Result<()> {
        let fx_id = get_v7();

        let decoded = C.decode(V7_ENCODED);

        assert_eq!(decoded, fx_id);

        Ok(())
    }

    #[test]
    fn test_uuid_v4_b32_encode() -> Result<()> {
        let fx_id = get_v4();

        let encoded = C.encode(fx_id);

        assert_eq!(encoded, V4_ENCODED);

        Ok(())
    }

    #[test]
    fn test_uuid_v4_b32_decode() -> Result<()> {
        let fx_id = get_v4();

        let decoded = C.decode(V4_ENCODED);

        assert_eq!(decoded, fx_id);

        Ok(())
    }
}
// endregion: --- Tests
