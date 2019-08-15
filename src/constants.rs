use crate::types::Byte;

pub const HASH_LENGTH: usize  = 32;
pub const BITS_IN_NIBBLE: usize = 4;
pub const NIBBLES_IN_BYTE: usize = 2;
pub const HASH_HEX_CHARS: usize  = 64;
pub const HEX_PREFIX_LENGTH: usize = 2;
pub const REQWEST_TIMEOUT_TIME: u64 = 5;
pub const ADDRESS_LENGTH_CHARS: usize = 40;
pub const HIGHER_NIBBLE_BIT_MASK: Byte = 15u8; // NOTE: 15u8 == [0,0,0,0,1,1,1,1]
pub static DOT_ENV_PATH: &'static str = "./.env";
pub static EMPTY_NODE: crate::types::Node<'static> = (&[], None);
pub static DEFAULT_ENDPOINT: &'static str = "http://localhost:8545/";
