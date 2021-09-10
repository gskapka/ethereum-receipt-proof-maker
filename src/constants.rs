use crate::nibble_utils::Nibbles;
use crate::types::Byte;
use ethereum_types::H256;

pub const ZERO_BYTE: u8 = 0u8;
pub const HASH_LENGTH: usize = 32;
pub const HASH_HEX_CHARS: usize = 64;
pub const HEX_PREFIX_LENGTH: usize = 2;
pub const NUM_BITS_IN_NIBBLE: usize = 4;
pub const REQWEST_TIMEOUT_TIME: u64 = 5;
pub const NUM_NIBBLES_IN_BYTE: usize = 2;
pub const HIGH_NIBBLE_MASK: Byte = 15u8; // NOTE: 15u8 == [0,0,0,0,1,1,1,1]
pub static DOT_ENV_PATH: &'static str = "./.env";
pub static LOG_FILE_PATH: &'static str = "logs/";
pub static LEAF_NODE_STRING: &'static str = "leaf";
pub static BRANCH_NODE_STRING: &'static str = "branch";
pub static EXTENSION_NODE_STRING: &'static str = "extension";
pub const HASHED_NULL_NODE: H256 = H256(HASHED_NULL_NODE_BYTES);
pub static DEFAULT_ENDPOINT: &'static str = "http://localhost:8545/";
pub const EMPTY_NIBBLES: Nibbles = Nibbles {
    data: Vec::new(),
    offset: 0,
};

const HASHED_NULL_NODE_BYTES: [u8; 32] = [
    // NOTE: keccak hash of the RLP of null
    0x56, 0xe8, 0x1f, 0x17, 0x1b, 0xcc, 0x55, 0xa6, 0xff, 0x83, 0x45, 0xe6, 0x92, 0xc0, 0xf8, 0x6e,
    0x5b, 0x48, 0xe0, 0x1b, 0x99, 0x6c, 0xad, 0xc0, 0x01, 0x62, 0x2f, 0xb5, 0xe3, 0x63, 0xb4, 0x21,
];
