use crate::errors::AppError;
use crate::types::{
    Bytes,
    Result,
};
use crate::nibble_utils::{
    Nibbles,
    get_zero_nibble,
    get_nibble_at_index,
    get_length_in_nibbles,
    get_nibbles_from_bytes,
    convert_nibble_to_bytes,
    prefix_nibbles_with_byte,
    get_nibbles_from_offset_bytes,
    set_first_index_in_nibble_vec_to_one,
    set_first_index_in_nibble_vec_to_zero,
    replace_nibble_in_nibble_vec_at_nibble_index,
};

const ODD_LENGTH_LEAF_PREFIX_NIBBLE: u8 = 3u8;      // [00000011]
const EVEN_LENGTH_LEAF_PREFIX_BYTE: u8 = 32u8;      // [00100000]
const EVEN_LENGTH_EXTENSION_PREFIX_BYTE: u8 = 0u8;  // [00000000]
const ODD_LENGTH_EXTENSION_PREFIX_NIBBLE: u8 = 1u8; // [00000001]

fn get_leaf_prefix_nibble() -> Nibbles {
    get_nibbles_from_offset_bytes(vec![ODD_LENGTH_LEAF_PREFIX_NIBBLE])
}

fn get_extension_prefix_nibble() -> Nibbles {
    get_nibbles_from_offset_bytes(vec![ODD_LENGTH_EXTENSION_PREFIX_NIBBLE])
}

fn encode_even_length_extension_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    prefix_nibbles_with_byte(nibbles, vec![EVEN_LENGTH_EXTENSION_PREFIX_BYTE])
}

fn encode_even_length_leaf_path_from_nibbles(
    nibbles: Nibbles
) -> Result<Bytes> {
    prefix_nibbles_with_byte(nibbles, vec![EVEN_LENGTH_LEAF_PREFIX_BYTE])
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

fn is_path_to_leaf_node(path: Bytes) -> Result<bool> {
    decode_path_to_nibbles_and_node_type(path)
        .and_then(|(_, node_type)| match &(*node_type) {
            "leaf" => Ok(true),
            _ => Ok(false)
        })
}

fn is_path_to_extension_node(path: Bytes) -> Result<bool> {
    is_path_to_leaf_node(path)
        .and_then(|is_leaf_bool| Ok(!is_leaf_bool))
}

fn trim_encoding_byte(path: Bytes) -> Result<Bytes> {
    match path.len() > 1{
        true => Ok(path[1..].to_vec()),
        false => Err(AppError::Custom(
            format!("✘ Path too short to slice off encoding byte: {:?}", path)
        ))
    }
}

fn decode_odd_length_path_to_nibbles(path: Bytes) -> Result<Nibbles> {
    replace_nibble_in_nibble_vec_at_nibble_index(
        // NOTE: Not offset so we can zero that first, encoding nibble...
        get_nibbles_from_bytes(path),
        get_zero_nibble(),
        0
    )
        .map(set_first_index_in_nibble_vec_to_one)
}

pub fn decode_path_to_nibbles_and_node_type(
    path: Bytes
) -> Result<(Nibbles, String)> {
    match path[0] {
        EVEN_LENGTH_LEAF_PREFIX_BYTE => Ok((
            get_nibbles_from_bytes(trim_encoding_byte(path)?),
            "leaf".to_string()
        )),
        EVEN_LENGTH_EXTENSION_PREFIX_BYTE => Ok((
            get_nibbles_from_bytes(trim_encoding_byte(path)?),
            "extension".to_string()
        )),
        _ => match get_nibble_at_index(
            &get_nibbles_from_bytes(path.clone())
            , 0
        ) {
            Err(e) => Err(e),
            Ok(nibble) => match nibble {
                ODD_LENGTH_LEAF_PREFIX_NIBBLE => Ok((
                    decode_odd_length_path_to_nibbles(path)?,
                    "leaf".to_string()
                )),
                ODD_LENGTH_EXTENSION_PREFIX_NIBBLE => Ok((
                    decode_odd_length_path_to_nibbles(path)?,
                    "extension".to_string()
                )),
                _ => Err(AppError::Custom(
                    "✘ Malformed path - cannot determine node type!".to_string()
                ))
            }
        }
    }
}

pub fn decode_path_to_nibbles(path: Bytes) -> Result<Nibbles> {
    decode_path_to_nibbles_and_node_type(path)
        .and_then(|(nibbles, _)| Ok(nibbles))
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
        get_nibbles_from_bytes,
        get_nibbles_from_offset_bytes,
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
        let nibbles = get_nibbles_from_offset_bytes(vec![0x01u8, 0x23, 0x45]);
        let bytes = hex::decode("112345".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_even_extension_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibbles_from_bytes(vec![0x01, 0x23, 0x45]);
        let bytes = hex::decode("00012345".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_even_leaf_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibbles_from_bytes(vec![0x0f, 0x1c, 0xb8]);
        let bytes = hex::decode("200f1cb8".to_string()).unwrap();
        (nibbles, bytes)
    }

    fn get_odd_leaf_path_sample() -> (Nibbles, Bytes) {
        let nibbles = get_nibbles_from_offset_bytes(vec![0x0fu8, 0x1c, 0xb8]);
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

    #[test]
    fn should_check_if_even_leaf_node_is_leaf_correctly() {
        let (_, even_leaf_path) = get_even_leaf_path_sample();
        let result = is_path_to_leaf_node(even_leaf_path)
            .unwrap();
        assert!(result);
    }

    #[test]
    fn should_check_if_odd_leaf_node_is_leaf_correctly() {
        let (_, odd_leaf_path) = get_odd_leaf_path_sample();
        let result = is_path_to_leaf_node(odd_leaf_path)
            .unwrap();
        assert!(result);
    }

    #[test]
    fn should_check_if_even_extension_node_is_leaf_correctly() {
        let (_, even_extension_path) = get_even_extension_path_sample();
        let result = is_path_to_leaf_node(even_extension_path)
            .unwrap();
        assert!(!result);
    }

    #[test]
    fn should_check_if_odd_extension_node_is_leaf_correctly() {
        let (_, odd_extension_path) = get_odd_extension_path_sample();
        let result = is_path_to_leaf_node(odd_extension_path)
            .unwrap();
        assert!(!result);
    }

    #[test]
    fn should_check_if_even_leaf_node_is_extension_correctly() {
        let (_, even_leaf_path) = get_even_leaf_path_sample();
        let result = is_path_to_extension_node(even_leaf_path)
            .unwrap();
        assert!(!result);
    }

    #[test]
    fn should_check_if_odd_leaf_node_is_extension_correctly() {
        let (_, odd_leaf_path) = get_odd_leaf_path_sample();
        let result = is_path_to_extension_node(odd_leaf_path)
            .unwrap();
        assert!(!result);
    }

    #[test]
    fn should_check_if_even_extension_node_is_extension_correctly() {
        let (_, even_extension_path) = get_even_extension_path_sample();
        let result = is_path_to_extension_node(even_extension_path)
            .unwrap();
        assert!(result);
    }

    #[test]
    fn should_check_if_odd_extension_node_is_extension_correctly() {
        let (_, odd_extension_path) = get_odd_extension_path_sample();
        let result = is_path_to_extension_node(odd_extension_path)
            .unwrap();
        assert!(result);
    }

    #[test]
    fn should_decode_even_path_to_nibbles_and_leaf_node_type_correctly() {
        let (expected_nibbles, path) = get_even_leaf_path_sample();
        let (result_nibbles, result_type) =
            decode_path_to_nibbles_and_node_type(path).unwrap();
        assert!(result_type == "leaf".to_string());
        assert!(expected_nibbles.data == result_nibbles.data);
    }

    #[test]
    fn should_decode_odd_path_to_nibbles_and_leaf_node_type_correctly() {
        let (expected_nibbles, path) = get_odd_leaf_path_sample();
        let (result_nibbles, result_type) =
            decode_path_to_nibbles_and_node_type(path).unwrap();
        assert!(result_type == "leaf".to_string());
        assert!(expected_nibbles.data == result_nibbles.data);
    }

    #[test]
    fn should_decode_odd_path_to_nibbles_and_extension_node_type_correctly() {
        let (expected_nibbles, path) = get_odd_extension_path_sample();
        let (result_nibbles, result_type) =
            decode_path_to_nibbles_and_node_type(path).unwrap();
        assert!(result_type == "extension".to_string());
        assert!(expected_nibbles.data == result_nibbles.data);
    }

    #[test]
    fn should_decode_even_path_to_nibbles_and_extension_node_type_correctly() {
        let (expected_nibbles, path) = get_even_extension_path_sample();
        let (result_nibbles, result_type) =
            decode_path_to_nibbles_and_node_type(path).unwrap();
        assert!(result_type == "extension".to_string());
        assert!(expected_nibbles.data == result_nibbles.data);
    }

    #[test]
    fn should_error_when_decoding_a_wrongly_encoded_path() {
        // NOTE: 1st nibble > 3 == a wrong encoding
        let wrong_path = hex::decode("c0ffee".to_string()).unwrap();
        let expected_error = "✘ Malformed path - cannot determine node type!"
            .to_string();
        match decode_path_to_nibbles_and_node_type(wrong_path) {
            Ok(_) => panic!("Should not decode a bad encoding!"),
            Err(AppError::Custom(e)) => assert!(e == expected_error),
            _ => panic!("Didn't get correct decoding error!"),
        }
    }

    #[test]
    fn should_trim_encoding_byte_correctly() {
        let path = hex::decode("c0ffee".to_string()).unwrap();
        let expected_result = hex::decode("ffee".to_string()).unwrap();
        let result = trim_encoding_byte(path)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_fail_to_trim_encoding_byte_if_path_too_short() {
        let path = hex::decode("c0".to_string()).unwrap();
        let expected_error = format!(
            "✘ Path too short to slice off encoding byte: {:?}",
            path
        );
        match trim_encoding_byte(path) {
            Ok(_) => panic!("Should have failed to trim encoding byte!"),
            Err(AppError::Custom(e)) => assert!(e == expected_error),
            _ => panic!("Didn't get expected error!")
        }
    }

    #[test]
    fn should_decode_odd_length_leaf_path_to_nibbles_correctly() {
        let (expected_nibbles, path) = get_odd_leaf_path_sample();
        let result = decode_odd_length_path_to_nibbles(path)
            .unwrap();
        assert!(result.data == expected_nibbles.data);
    }

    #[test]
    fn should_decode_odd_length_extension_path_to_nibbles_correctly() {
        let (expected_nibbles, path) = get_odd_extension_path_sample();
        let result = decode_odd_length_path_to_nibbles(path)
            .unwrap();
        assert!(result.data == expected_nibbles.data);
    }

    #[test]
    fn should_decode_odd_length_leaf_path_to_just_nibbles_correctly() {
        let (expected_nibbles, path) = get_odd_leaf_path_sample();
        let result = decode_path_to_nibbles(path).unwrap();
        assert!(result.data == expected_nibbles.data);
    }

    #[test]
    fn should_decode_even_length_leaf_path_to_just_nibbles_correctly() {
        let (expected_nibbles, path) = get_even_leaf_path_sample();
        let result = decode_path_to_nibbles(path).unwrap();
        assert!(result.data == expected_nibbles.data);
    }

    #[test]
    fn should_decode_odd_length_extension_path_to_just_nibbles_correctly() {
        let (expected_nibbles, path) = get_odd_extension_path_sample();
        let result = decode_path_to_nibbles(path).unwrap();
        assert!(result.data == expected_nibbles.data);
    }

    #[test]
    fn should_decode_even_length_extension_path_to_just_nibbles_correctly() {
        let (expected_nibbles, path) = get_even_extension_path_sample();
        let result = decode_path_to_nibbles(path).unwrap();
        assert!(result.data == expected_nibbles.data);
    }
}
