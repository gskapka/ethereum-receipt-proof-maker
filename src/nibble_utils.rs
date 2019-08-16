use std::fmt;
use crate::errors::AppError;
use crate::types::{
    Byte,
    Bytes,
    Result,
};
use crate::constants::{
    BITS_IN_NIBBLE,
    NIBBLES_IN_BYTE,
    HIGH_NIBBLE_MASK,
};

#[derive(Clone)]
pub struct NibbleVec {
    data: Bytes,
    first_nibble_index: usize,
}

impl fmt::Debug for NibbleVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..get_length_in_nibbles(&self) {
            write!(
                f,
                "0x{:01x} ",
                get_nibble_at_index(&self, i).unwrap()
            )?;
        }
        Ok(())
    }
}

pub fn get_nibble_vec_from_bytes(nibbles: Bytes) -> NibbleVec {
    NibbleVec { data: nibbles, first_nibble_index: 0 }
}

pub fn get_nibble_vec_from_offset_bytes(nibbles: Bytes) -> NibbleVec {
    NibbleVec { data: nibbles, first_nibble_index: 1 }
}
}

pub fn replace_high_nibble_in_byte(
    byte: Byte,
    replacement_nibble: NibbleVec,
) -> Byte {
    match replacement_nibble.first_nibble_index {
        0 => merge_nibbles_from_bytes(byte, replacement_nibble.data[0]),
        _ => merge_nibbles_from_bytes(
            byte,
            shift_nibble_left(replacement_nibble.data[0])
        )
    }
}

pub fn replace_low_nibble_in_byte(
    byte: Byte,
    replacement_nibble: NibbleVec,
) -> Byte {
    match replacement_nibble.first_nibble_index {
        1 => merge_nibbles_from_bytes(replacement_nibble.data[0], byte),
        _ => merge_nibbles_from_bytes(
            shift_nibble_right(replacement_nibble.data[0]),
            byte
        )
    }
}

pub fn merge_nibbles_from_bytes(
    low_nibble_byte: Byte,
    high_nibble_byte: Byte,
) -> Byte {
    high_nibble_byte ^ ((high_nibble_byte ^ low_nibble_byte) & HIGH_NIBBLE_MASK)
}

pub fn get_length_in_nibbles(nibbles: &NibbleVec) -> usize {
    nibbles.data.len() * 2 - nibbles.first_nibble_index
}

pub fn get_nibble_at_index(nibbles: &NibbleVec, i: usize) -> Result<Byte> {
    match i > get_length_in_nibbles(&nibbles) {
        true => Err(AppError::Custom(
            format!("✘ Index {} is out-of-bounds in nibble vector!", i)
        )),
        _ => match nibbles.first_nibble_index {
            0 => match i % 2 {
                0 => get_high_nibble_from_byte(&nibbles.data, &i),
                _ => get_low_nibble_from_byte(&nibbles.data, &i),
            },
            _ => match i % 2 {
                0 => get_low_nibble_from_byte(&nibbles.data, &i),
                _ => get_high_nibble_from_byte(&nibbles.data, &(i + 1)),
            }
        }
    }
}

fn get_byte_containing_nibble_at_i(
    bytes: &Bytes,
    i: &usize
) -> Result<Byte> {
    Ok(bytes[i / NIBBLES_IN_BYTE])
}

fn mask_higher_nibble(byte: Byte) -> Byte {
    byte & HIGH_NIBBLE_MASK
}

fn shift_nibble_right(byte: Byte) -> Byte {
    byte >> BITS_IN_NIBBLE
}

fn shift_nibble_left(byte: Byte) -> Byte {
    byte << BITS_IN_NIBBLE
}

fn get_low_nibble_from_byte(bytes: &Bytes, i: &usize) -> Result<Byte> {
    get_byte_containing_nibble_at_i(bytes, i)
        .map(mask_higher_nibble)
}

fn get_high_nibble_from_byte(bytes: &Bytes, i: &usize) -> Result<Byte> {
    get_byte_containing_nibble_at_i(bytes, i)
        .map(shift_nibble_right)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_NIBBLES: [u8; 14] = [
        0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
        0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x0du8, 0x0eu8,
    ];

    fn get_bytes_with_nibbles_from_index_zero() -> Bytes {
        vec![0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]
    }

    fn get_bytes_with_nibbles_from_index_one() -> Bytes {
        vec![0x01u8, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd]
    }

    fn get_sample_nibble_vec() -> NibbleVec {
        get_nibble_vec_from_bytes(get_bytes_with_nibbles_from_index_zero())
    }

    fn get_sample_offset_nibble_vec() -> NibbleVec {
        get_nibble_vec_from_bytes(get_bytes_with_nibbles_from_index_one())
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_zero_correctly() {
        let expected_length = get_bytes_with_nibbles_from_index_zero().len() * 2;
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let result = get_nibble_vec_from_bytes(bytes);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_one_correctly() {
        let expected_length = get_bytes_with_nibbles_from_index_one().len() * 2 - 1;
        let bytes = get_bytes_with_nibbles_from_index_one();
        let result = get_nibble_vec_from_offset_bytes(bytes);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_zero_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let nibbles = get_nibble_vec_from_bytes(bytes);
        for i in 0..get_length_in_nibbles(&nibbles) {
            let nibble = get_nibble_at_index(&nibbles, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_one_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_one();
        let nibbles = get_nibble_vec_from_offset_bytes(bytes);
        for i in 0..get_length_in_nibbles(&nibbles) {
            let nibble = get_nibble_at_index(&nibbles, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_err_if_attempting_to_get_out_of_bounds_nibble() {
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let nibbles = get_nibble_vec_from_bytes(bytes);
        let num_nibbles = get_length_in_nibbles(&nibbles);
        let out_of_bounds_index = num_nibbles + 1;
        assert!(out_of_bounds_index > num_nibbles);
        let expected_error = &format!(
            "✘ Index {} is out-of-bounds in nibble vector!",
            out_of_bounds_index
        );
        match get_nibble_at_index(&nibbles, out_of_bounds_index) {
            Err(AppError::Custom(e)) => assert!(e.contains(expected_error)),
            _ => panic!("Expected error not receieved!")
        }
    }

    #[test]
    fn should_display_nibble_starting_at_index_zero_string_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let nibbles = get_nibble_vec_from_bytes(bytes);
        println!("{:?}", nibbles);
    }

    #[test]
    fn should_display_nibble_starting_at_index_one_string_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_one();
        let nibbles = get_nibble_vec_from_offset_bytes(bytes);
        println!("{:?}", nibbles);
    }

    #[test]
    fn should_merge_nibbles_from_bytes_correctly() {
        let low_nibble_byte = 14u8;   // [00001110]
        let high_nibble_byte = 160u8; // [10100000]
        let expected_result = 174u8;  // [10101110]
        let result = merge_nibbles_from_bytes(
            low_nibble_byte,
            high_nibble_byte,
        );
        assert!(result == expected_result);
    }

    #[test]
    fn should_shift_nibble_right_correctly() {
        let test_byte = 160u8;      // [10100000]
        let expected_result = 10u8; // [00001010]
        let result = shift_nibble_right(test_byte);
        assert!(result == expected_result);
    }

    #[test]
    fn should_shift_nibble_left_correctly() {
        let test_byte = 10u8;        // [00001010]
        let expected_result = 160u8; // [10100000]
        let result = shift_nibble_left(test_byte);
        assert!(result == expected_result);
    }

    #[test]
    fn should_mask_higher_nibble_correctly() {
        let test_byte = 174u8;      // [10101110]
        let expected_result = 14u8; // [00001110]
        let result = mask_higher_nibble(test_byte);
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_low_nibble_from_byte_correctly() {
        let index_of_byte = 0;
        let test_byte = vec![174u8]; // [10101110]
        let expected_result = 14u8;  // [00001110]
        let result = get_low_nibble_from_byte(
            &test_byte,
            &index_of_byte
        ).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_high_nibble_from_byte_correctly() {
        let index_of_byte = 0;
        let test_byte = vec![174u8]; // [10101110]
        let expected_result = 10u8;  // [00001010]
        let result = get_high_nibble_from_byte(
            &test_byte,
            &index_of_byte
        ).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_byte_containing_nibble_at_i_correctly() {
        let index_of_nibble = 2;
        let bytes = vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8];
        let expected_result = 1u8;
        let result = get_byte_containing_nibble_at_i(
            &bytes,
            &index_of_nibble,
        ).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_high_nibble_in_byte_correctly() {
        let test_byte = 170u8;          // [10101010]
        let replacement_nibble = 240u8; // [11110000]
        let expected_result = 250u8;    // [11111010]
        let result = replace_high_nibble_in_byte(
            test_byte,
            get_nibble_vec_from_bytes(vec![replacement_nibble])
        );
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_high_offset_nibble_in_byte_correctly() {
        let test_byte = 170u8;         // [10101010]
        let replacement_nibble = 15u8; // [00001111]
        let expected_result = 250u8;   // [11111010]
        let result = replace_high_nibble_in_byte(
            test_byte,
            get_nibble_vec_from_offset_bytes(vec![replacement_nibble])
        );
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_low_nibble_in_byte_correctly() {
        let test_byte = 170u8;          // [10101010]
        let replacement_nibble = 240u8; // [11110000]
        let expected_result = 175u8;    // [10101111]
        let result = replace_low_nibble_in_byte(
            test_byte,
            get_nibble_vec_from_bytes(vec![replacement_nibble])
        );
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_low_offset_nibble_in_byte_correctly() {
        let test_byte = 170u8;         // [10101010]
        let replacement_nibble = 15u8; // [00001111]
        let expected_result = 175u8;   // [10101111]
        let result = replace_low_nibble_in_byte(
            test_byte,
            get_nibble_vec_from_offset_bytes(vec![replacement_nibble])
        );
        assert!(result == expected_result);
    }
}
