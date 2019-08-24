use crate::types::{
    Bytes,
    Result,
};
use crate::nibble_utils::{
    Nibbles,
    get_length_in_nibbles,
    convert_nibble_to_bytes,
    prefix_nibbles_with_byte,
    get_nibble_vec_from_offset_bytes,
    set_first_index_in_nibble_vec_to_zero,
    replace_nibble_in_nibble_vec_at_nibble_index,
};

// TODO: [x] Need utils to encode nibbles w/ starting nibbles
// TODO: [ ] Need utils to decode nibbles w/ starting nibbles

fn get_leaf_prefix_nibble() -> Nibbles {
    get_nibble_vec_from_offset_bytes(vec![3u8]) // [00000001]
}

fn get_extension_prefix_nibble() -> Nibbles {
    get_nibble_vec_from_offset_bytes(vec![1u8]) // [00000011]
}

fn encode_even_length_extension_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    prefix_nibbles_with_byte(nibbles, vec![0u8]) // [00000000]
}

fn encode_even_length_leaf_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    prefix_nibbles_with_byte(nibbles, vec![32u8]) // [00100000]
}

fn encode_odd_length_path_from_nibbles(
    nibbles: Nibbles,
    prefix_nibble: Nibbles
) -> Result<Bytes> {
    replace_nibble_in_nibble_vec_at_nibble_index(
        set_first_index_in_nibble_vec_to_zero(nibbles),
        prefix_nibble,
        0
    )
        .and_then(convert_nibble_to_bytes)
}

fn encode_odd_length_extension_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    encode_odd_length_path_from_nibbles(nibbles, get_extension_prefix_nibble())
}

fn encode_odd_length_leaf_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    encode_odd_length_path_from_nibbles(nibbles, get_leaf_prefix_nibble())
}

pub fn encode_extension_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    match get_length_in_nibbles(&nibbles) % 2 == 0 {
        true => encode_even_length_extension_path_from_nibbles(nibbles),
        false => encode_odd_length_extension_path_from_nibbles(nibbles),
    }
}

pub fn encode_leaf_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    match get_length_in_nibbles(&nibbles) % 2 == 0 {
        true => encode_even_length_leaf_path_from_nibbles(nibbles),
        false => encode_odd_length_leaf_path_from_nibbles(nibbles),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use crate::nibble_utils::{
        get_nibble_vec_from_bytes,
        get_nibble_vec_from_offset_bytes,
    };

/*
 * Test vectors are from the spec @:
 * https://github.com/ethereum/wiki/wiki/Patricia-Tree
 *
 * > [ 1, 2, 3, 4, 5, ...]
 * '11 23 45'
 * > [ 0, 1, 2, 3, 4, 5, ...]
 * '00 01 23 45'
 * > [ 0, f, 1, c, b, 8, 10]
 * '20 0f 1c b8'
 * > [ f, 1, c, b, 8, 10]
 * '3f 1c b8'
 *
 */

    fn get_odd_extension_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibble_vec_from_offset_bytes(vec![0x01u8, 0x23, 0x45]);
        let bytes = hex::decode("112345".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_even_extension_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibble_vec_from_bytes(vec![0x01, 0x23, 0x45]);
        let bytes = hex::decode("00012345".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_even_leaf_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibble_vec_from_bytes(vec![0x0f, 0x1c, 0xb8]);
        let bytes = hex::decode("200f1cb8".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_odd_leaf_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibble_vec_from_offset_bytes(vec![0x0fu8, 0x1c, 0xb8]);
        let bytes = hex::decode("3f1cb8".to_string()).unwrap();
        (nibbles, bytes)
    }

    #[test]
    fn should_encode_odd_length_extension_path_correctly() {
        let (sample, expected_result) = get_odd_extension_path_sample();
        let result = encode_odd_length_extension_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_even_length_extension_path_correctly() {
        let (sample, expected_result) = get_even_extension_path_sample();
        let result = encode_even_length_extension_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_odd_length_leaf_path_correctly() {
        let (sample, expected_result) = get_odd_leaf_path_sample();
        let result = encode_odd_length_leaf_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_even_length_leaf_path_correctly() {
        let (sample, expected_result) = get_even_leaf_path_sample();
        let result = encode_even_length_leaf_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_extension_path_from_offset_nibbles_correctly() {
        let (sample, expected_result) = get_odd_extension_path_sample();
        let result = encode_extension_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_extension_path_from_nibbles_correctly() {
        let (sample, expected_result) = get_even_extension_path_sample();
        let result = encode_extension_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_leaf_path_from_offset_nibbles_correctly() {
        let (sample, expected_result) = get_odd_leaf_path_sample();
        let result = encode_leaf_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_encode_leaf_path_from_nibbles_correctly() {
        let (sample, expected_result) = get_even_leaf_path_sample();
        let result = encode_leaf_path_from_nibbles(sample)
            .unwrap();
        assert!(result == expected_result);
    }
}
