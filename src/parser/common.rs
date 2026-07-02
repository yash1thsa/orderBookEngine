/// Parses a 6-byte timestamp into a u64
/// Converts bytes at positions [0..6] into a single u64 with bit shifts
pub fn parse_timestamp(b: &[u8]) -> u64 {
    ((b[0] as u64) << 40)
        | ((b[1] as u64) << 32)
        | ((b[2] as u64) << 24)
        | ((b[3] as u64) << 16)
        | ((b[4] as u64) << 8)
        | (b[5] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp() {
        // Test with known values
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x01];
        assert_eq!(parse_timestamp(&bytes), 1);

        let bytes = [0x00, 0x00, 0x00, 0x00, 0x01, 0x00];
        assert_eq!(parse_timestamp(&bytes), 256);

        let bytes = [0x00, 0x00, 0x00, 0x01, 0x00, 0x00];
        assert_eq!(parse_timestamp(&bytes), 65536);

        let bytes = [0x00, 0x00, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(parse_timestamp(&bytes), 16777216);

        let bytes = [0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(parse_timestamp(&bytes), 4294967296);

        let bytes = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(parse_timestamp(&bytes), 1099511627776);
    }

    #[test]
    fn test_parse_timestamp_max_value() {
        // Test with all bytes set to 0xFF
        let bytes = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(parse_timestamp(&bytes), 281474976710655);
    }

    #[test]
    fn test_parse_timestamp_zero() {
        let bytes = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(parse_timestamp(&bytes), 0);
    }
}
