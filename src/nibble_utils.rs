use std::fmt;
use crate::errors::AppError;
use crate::types::{
    Byte,
    Result,
};
use crate::constants::{
    BITS_IN_NIBBLE,
    NIBBLES_IN_BYTE,
    HIGHER_NIBBLE_BIT_MASK,
};

#[derive(Copy, Clone)]
pub struct NibbleSlice<'a> {
    data: &'a [u8],
    first_nibble_index: usize,
}

impl<'a> fmt::Debug for NibbleSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..get_length_in_nibbles(&self) {
            match get_nibble_at_index(&self, i) {
                Ok(nibble) => write!(f, "0x{:01x} ", nibble),
                Err(_) => write!(f, "Error getting nibble!"),
            };
        }
        Ok(())
    }
}

pub fn get_nibble_slice_from_slice(data: &[u8]) -> NibbleSlice {
    NibbleSlice { data, first_nibble_index: 0 }
}

pub fn get_nibble_slice_from_offset_slice(data: & [u8]) -> NibbleSlice {
    NibbleSlice { data, first_nibble_index: 1 }
}

pub fn get_length_in_nibbles(nibble_slice: &NibbleSlice) -> usize {
    nibble_slice.data.len() * 2 - nibble_slice.first_nibble_index
}

pub fn get_nibble_at_index(nibble_slice: &NibbleSlice, i: usize) -> Result<Byte> {
    match i > get_length_in_nibbles(&nibble_slice) {
        true => Err(AppError::Custom(
            format!("✘ Index {} is out-of-bounds in nibble slice!", i)
        )),
        _ => match nibble_slice.first_nibble_index {
            0 => match i % 2 {
                0 => get_high_nibble_from_byte(nibble_slice.data, &i),
                _ => get_low_nibble_from_byte(nibble_slice.data, &i),
            },
            _ => match i % 2 {
                0 => get_low_nibble_from_byte(nibble_slice.data, &i),
                _ => get_high_nibble_from_byte(nibble_slice.data, &(i + 1)),
            }
        }
    }
}

fn get_byte_from_slice_at_nibble_i(byte_slice: &[u8], i: &usize) -> Result<Byte> {
    Ok(byte_slice[i / NIBBLES_IN_BYTE])
}

fn mask_higher_nibble(byte: Byte) -> Byte {
    byte & HIGHER_NIBBLE_BIT_MASK
}

fn shift_nibble_right(byte: Byte) -> Byte {
    byte >> BITS_IN_NIBBLE
}

fn get_low_nibble_from_byte(byte_slice: &[u8], i: &usize) -> Result<Byte> {
    get_byte_from_slice_at_nibble_i(byte_slice, i)
        .map(mask_higher_nibble)
}

fn get_high_nibble_from_byte(byte_slice: &[u8], i: &usize) -> Result<Byte> {
    get_byte_from_slice_at_nibble_i(byte_slice, i)
        .map(shift_nibble_right)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_NIBBLES: [u8; 14] = [
        0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8,
        0x08u8, 0x09u8, 0x0au8, 0x0bu8, 0x0cu8, 0x0du8, 0x0eu8,
    ];

    fn get_slice_with_nibbles_from_index_zero() -> [u8; 7] {
        [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde]
    }

    fn get_slice_with_nibbles_from_index_one() -> [u8; 7] {
        [0x01u8, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd]
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_zero_correctly() {
        let expected_length = get_slice_with_nibbles_from_index_zero().len() * 2;
        let slice = &get_slice_with_nibbles_from_index_zero();
        let result = get_nibble_slice_from_slice(slice);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_one_correctly() {
        let expected_length = get_slice_with_nibbles_from_index_one().len() * 2 - 1;
        let slice = &get_slice_with_nibbles_from_index_one();
        let result = get_nibble_slice_from_offset_slice(slice);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_zero_correctly() {
        let slice = &get_slice_with_nibbles_from_index_zero();
        let nibble_slice = get_nibble_slice_from_slice(slice);
        for i in 0..get_length_in_nibbles(&nibble_slice) {
            let nibble = get_nibble_at_index(&nibble_slice, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_one_correctly() {
        let slice = &get_slice_with_nibbles_from_index_one();
        let nibble_slice = get_nibble_slice_from_offset_slice(slice);
        for i in 0..get_length_in_nibbles(&nibble_slice) {
            let nibble = get_nibble_at_index(&nibble_slice, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_err_if_attempting_to_get_out_of_bounds_nibble() {
        let slice = &get_slice_with_nibbles_from_index_zero();
        let nibble_slice = get_nibble_slice_from_slice(slice);
        let num_nibbles = get_length_in_nibbles(&nibble_slice);
        let out_of_bounds_index = num_nibbles + 1;
        assert!(out_of_bounds_index > num_nibbles);
        let expected_error = &format!(
            "✘ Index {} is out-of-bounds in nibble slice!",
            out_of_bounds_index
        );
        match get_nibble_at_index(&nibble_slice, out_of_bounds_index) {
            Err(AppError::Custom(e)) => assert!(e.contains(expected_error)),
            _ => panic!("Expected error not receieved!")
        }
    }

    #[test]
    fn should_display_nibble_starting_at_index_zero_string_correctly() {
        let slice = &get_slice_with_nibbles_from_index_zero();
        let nibble_slice = get_nibble_slice_from_slice(slice);
        println!("{:?}", nibble_slice);
    }

    #[test]
    fn should_display_nibble_starting_at_index_one_string_correctly() {
        let slice = &get_slice_with_nibbles_from_index_one();
        let nibble_slice = get_nibble_slice_from_offset_slice(slice);
        println!("{:?}", nibble_slice);
    }
}
