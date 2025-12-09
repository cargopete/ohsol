pub mod fetch;
pub mod parse;

pub use fetch::{fetch_idl, fetch_idl_with_url, get_idl_address};
pub use parse::{parse_idl, Idl, IdlError};
