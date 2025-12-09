pub mod anchor;
pub mod registry;

use crate::output::DecodedError;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn decode_error(code: u32, program_id: Option<&str>) -> DecodedError {
    // 1. Check built-in Anchor errors (< 6000)
    if code < 6000 {
        if let Some(anchor_err) = registry::lookup_anchor_error(code) {
            return anchor_err;
        }
    }

    // 2. Check program-specific database
    if let Some(pid) = program_id {
        if let Some(known_err) = registry::lookup_program_error(pid, code) {
            return known_err;
        }
    }

    // 3. For custom Anchor errors (>= 6000), provide generic info
    if anchor::is_anchor_custom_error(code) {
        let mut error = anchor::decode_anchor_custom(code);
        if let Some(pid) = program_id {
            error = error.with_program(pid.to_string());
        }
        return error;
    }

    // 4. Return raw code for unknown errors
    let mut error = DecodedError::new(code)
        .with_name("Unknown".to_string())
        .with_message("Unknown error code".to_string());

    if let Some(pid) = program_id {
        error = error.with_program(pid.to_string());
    }

    error
}

pub fn parse_program_id(input: &str) -> anyhow::Result<Pubkey> {
    Pubkey::from_str(input).map_err(|e| anyhow::anyhow!("Invalid program ID: {}", e))
}
