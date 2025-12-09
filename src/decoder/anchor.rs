use crate::output::DecodedError;

pub const ANCHOR_ERROR_OFFSET: u32 = 6000;

pub fn is_anchor_custom_error(code: u32) -> bool {
    code >= ANCHOR_ERROR_OFFSET
}

pub fn decode_anchor_custom(code: u32) -> DecodedError {
    let variant_index = code - ANCHOR_ERROR_OFFSET;
    DecodedError::new(code)
        .with_name(format!("CustomError[{}]", variant_index))
        .with_message(format!("Custom program error at variant index {}", variant_index))
}

pub fn parse_error_code(input: &str) -> anyhow::Result<u32> {
    let input = input.trim();

    // Try hex first
    if let Some(hex) = input.strip_prefix("0x") {
        return Ok(u32::from_str_radix(hex, 16)?);
    }

    // Try decimal
    Ok(input.parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        assert_eq!(parse_error_code("0x1771").unwrap(), 6001);
    }

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_error_code("6001").unwrap(), 6001);
    }

    #[test]
    fn test_is_anchor_custom() {
        assert!(is_anchor_custom_error(6000));
        assert!(is_anchor_custom_error(6001));
        assert!(!is_anchor_custom_error(2000));
    }
}
