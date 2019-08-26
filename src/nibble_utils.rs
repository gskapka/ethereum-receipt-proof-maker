use std::fmt;
use crate::errors::AppError;
use crate::types::{
    Byte,
    Bytes,
    Result,
};
use crate::constants::{
    ZERO_BYTE,
    BITS_IN_NIBBLE,
    NIBBLES_IN_BYTE,
    HIGH_NIBBLE_MASK,
};

#[derive(Clone)]
pub struct Nibbles {
    pub data: Bytes,
    pub first_nibble_index: usize,
}

impl fmt::Debug for Nibbles {
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

fn remove_first_nibble(nibbles: Nibbles) -> Result<Nibbles> {
    match get_length_in_nibbles(&nibbles) > 1 {
        true => match nibbles.first_nibble_index {
            1 => remove_first_byte_from_nibble_vec(nibbles),
            _ => replace_nibble_in_nibble_vec_at_nibble_index(
                nibbles,
                get_zero_nibble(),
                0
            ).map(set_first_index_in_nibble_vec_to_one),
        },
        false => replace_nibble_in_nibble_vec_at_nibble_index(
            nibbles,
            get_zero_nibble(),
            0
        )
    }
}

pub fn get_zero_nibble() -> Nibbles {
    Nibbles { data: vec![ZERO_BYTE], first_nibble_index: 1 }
}

fn remove_first_byte_from_nibble_vec(nibbles: Nibbles) -> Result<Nibbles> {
    match nibbles.data.len() > 1 {
        true => Ok(
            Nibbles {
                first_nibble_index: 0,
                data: nibbles.data[1..].to_vec()
            }
        ),
        false => Err(AppError::Custom(
            "✘ Cannot remove byte, there's only 1 in the nibble vec!".to_string()
        ))
    }

}

pub fn set_first_index_in_nibble_vec_to_zero(nibbles: Nibbles) -> Nibbles {
    Nibbles { data: nibbles.data, first_nibble_index: 0 }
}

pub fn set_first_index_in_nibble_vec_to_one(nibbles: Nibbles) -> Nibbles {
    Nibbles { data: nibbles.data, first_nibble_index: 1 }
}

pub fn get_nibbles_from_bytes(nibbles: Bytes) -> Nibbles {
    Nibbles { data: nibbles, first_nibble_index: 0 }
}

pub fn get_nibbles_from_offset_bytes(nibbles: Bytes) -> Nibbles {
    Nibbles { data: nibbles, first_nibble_index: 1 }
}

pub fn replace_nibble_in_nibble_vec_at_nibble_index(
    nibbles: Nibbles,
    replacement_nibble: Nibbles,
    nibble_index: usize
) -> Result<Nibbles> {
    get_byte_containing_nibble_at_nibble_index(&nibbles, &nibble_index)
        .map(|byte|
            match (nibble_index + nibbles.first_nibble_index) % 2 {
                0 => replace_high_nibble_in_byte(byte, replacement_nibble),
                _ => replace_low_nibble_in_byte(byte, replacement_nibble)
            }
        )
        .map(|byte| {
            replace_byte_in_nibble_vec_at_byte_index(
                convert_nibble_index_to_byte_index(&nibbles, &nibble_index),
                nibbles,
                byte
            )
        })
}

fn convert_nibble_index_to_byte_index(
    nibbles: &Nibbles,
    nibble_index: &usize
) -> usize {
    (nibbles.first_nibble_index + nibble_index) / NIBBLES_IN_BYTE
}

fn replace_byte_in_nibble_vec_at_byte_index(
    index: usize,
    nibbles: Nibbles,
    byte: Byte,
) -> Nibbles {
    let byte_length = nibbles.data.len();
    let mut vec = nibbles.data.clone();
    for i in 0..byte_length {
        match i == index {
            false => vec[i] = nibbles.data[i],
            _ => vec[i] = byte
        }
    };
    match nibbles.first_nibble_index {
        0 => get_nibbles_from_bytes(vec),
        _ => get_nibbles_from_offset_bytes(vec)
    }
}

pub fn replace_high_nibble_in_byte(
    byte: Byte,
    replacement_nibble: Nibbles,
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
    replacement_nibble: Nibbles,
) -> Byte {
    match replacement_nibble.first_nibble_index {
        1 => merge_nibbles_from_bytes(replacement_nibble.data[0], byte),
        _ => merge_nibbles_from_bytes(
            shift_nibble_right(replacement_nibble.data[0]),
            byte
        )
    }
}

fn merge_nibbles_from_bytes(
    low_nibble_byte: Byte,
    high_nibble_byte: Byte,
) -> Byte {
    high_nibble_byte ^ ((high_nibble_byte ^ low_nibble_byte) & HIGH_NIBBLE_MASK)
}

pub fn get_length_in_nibbles(nibbles: &Nibbles) -> usize {
    nibbles.data.len() * 2 - nibbles.first_nibble_index
}

pub fn get_nibble_at_index(
    nibbles: &Nibbles,
    nibble_index: usize
) -> Result<Byte> {
    match nibble_index > get_length_in_nibbles(&nibbles) {
        true => Err(AppError::Custom(
            format!(
                "✘ Index {} is out-of-bounds in nibble vector!",
                nibble_index
            )
        )),
        _ => match nibbles.first_nibble_index {
            0 => match nibble_index % 2 {
                0 => get_high_nibble_from_byte(&nibbles, &nibble_index),
                _ => get_low_nibble_from_byte(&nibbles, &nibble_index),
            }
            _ => match nibble_index % 2 {
                0 => get_low_nibble_from_byte(&nibbles, &nibble_index),
                _ => get_high_nibble_from_byte(&nibbles, &(nibble_index + 1)),
            }
        }
    }
}

fn get_byte_containing_nibble_at_nibble_index(
    nibbles: &Nibbles,
    nibble_index: &usize
) -> Result<Byte> {
    Ok(nibbles.data[convert_nibble_index_to_byte_index(nibbles, nibble_index)])
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

fn get_low_nibble_from_byte(nibbles: &Nibbles, nibble_index: &usize) -> Result<Byte> {
    get_byte_containing_nibble_at_nibble_index(nibbles, nibble_index)
        .map(mask_higher_nibble)
}

fn get_high_nibble_from_byte(nibbles: &Nibbles, nibble_index: &usize) -> Result<Byte> {
    get_byte_containing_nibble_at_nibble_index(nibbles, nibble_index)
        .map(shift_nibble_right)
}

pub fn prefix_nibbles_with_byte(
    nibbles: Nibbles,
    mut vec_including_prefix_byte: Vec<u8>
) -> Result<Bytes> {
    convert_nibble_to_bytes(nibbles)
        .and_then(|bytes| {
            vec_including_prefix_byte.append(& mut bytes.clone());
            Ok(vec_including_prefix_byte)
        })
}

pub fn convert_nibble_to_bytes(nibbles: Nibbles) -> Result<Bytes> {
    Ok(nibbles.data)
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

    fn get_sample_nibble_vec() -> Nibbles {
        get_nibbles_from_bytes(get_bytes_with_nibbles_from_index_zero())
    }

    fn get_sample_offset_nibble_vec() -> Nibbles {
        get_nibbles_from_offset_bytes(get_bytes_with_nibbles_from_index_one())
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_zero_correctly() {
        let expected_length = get_bytes_with_nibbles_from_index_zero().len() * 2;
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let result = get_nibbles_from_bytes(bytes);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_convert_slice_with_nibble_at_index_one_correctly() {
        let expected_length = get_bytes_with_nibbles_from_index_one().len() * 2 - 1;
        let bytes = get_bytes_with_nibbles_from_index_one();
        let result = get_nibbles_from_offset_bytes(bytes);
        assert!(get_length_in_nibbles(&result) == expected_length)
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_zero_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let nibbles = get_nibbles_from_bytes(bytes);
        for i in 0..get_length_in_nibbles(&nibbles) {
            let nibble = get_nibble_at_index(&nibbles, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_get_all_nibbles_with_first_nibble_at_index_one_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_one();
        let nibbles = get_nibbles_from_offset_bytes(bytes);
        for i in 0..get_length_in_nibbles(&nibbles) {
            let nibble = get_nibble_at_index(&nibbles, i)
                .unwrap();
            assert!(nibble == EXPECTED_NIBBLES[i]);
        }
    }

    #[test]
    fn should_err_if_attempting_to_get_out_of_bounds_nibble() {
        let bytes = get_bytes_with_nibbles_from_index_zero();
        let nibbles = get_nibbles_from_bytes(bytes);
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
        let nibbles = get_nibbles_from_bytes(bytes);
        println!("{:?}", nibbles);
    }

    #[test]
    fn should_display_nibble_starting_at_index_one_string_correctly() {
        let bytes = get_bytes_with_nibbles_from_index_one();
        let nibbles = get_nibbles_from_offset_bytes(bytes);
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
        let nibbles = get_nibbles_from_bytes(vec![174u8]); // [10101110]
        let expected_result = 14u8;  // [00001110]
        let result = get_low_nibble_from_byte(
            &nibbles,
            &index_of_byte
        ).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_high_nibble_from_byte_correctly() {
        let index_of_byte = 0;
        let nibbles = get_nibbles_from_bytes(vec![174u8]); // [10101110]
        let expected_result = 10u8;  // [00001010]
        let result = get_high_nibble_from_byte(
            &nibbles,
            &index_of_byte
        ).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_byte_containing_nibble_at_i_correctly() {
        let index_of_nibble = 2;
        let nibbles = get_nibbles_from_bytes(vec![0u8, 1u8, 2u8, 3u8, 4u8, 5u8]);
        let expected_result = 1u8;
        let result = get_byte_containing_nibble_at_nibble_index(
            &nibbles,
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
            get_nibbles_from_bytes(vec![replacement_nibble])
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
            get_nibbles_from_offset_bytes(vec![replacement_nibble])
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
            get_nibbles_from_bytes(vec![replacement_nibble])
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
            get_nibbles_from_offset_bytes(vec![replacement_nibble])
        );
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_byte_in_nibble_vec_correctly() {
        let byte_index = 3;
        let replacement_byte = 170u8;
        let original_bytes = get_bytes_with_nibbles_from_index_zero();
        let original_byte = original_bytes[byte_index];
        let nibbles = get_sample_nibble_vec();
        assert!(original_byte != replacement_byte);
        let updated_nibbles= replace_byte_in_nibble_vec_at_byte_index(
            byte_index,
            nibbles,
            replacement_byte
        );
        let result = updated_nibbles.data[byte_index];
        assert!(result != original_byte);
        assert!(result == replacement_byte);
        for i in 0..updated_nibbles.data.len() {
            match i == byte_index {
                false => assert!(updated_nibbles.data[i] == original_bytes[i]),
                 _ => assert!(updated_nibbles.data[i] == replacement_byte)
            }
        };
    }

    #[test]
    fn should_replace_byte_in_offset_nibble_vec_correctly() {
        let byte_index = 3;
        let replacement_byte = 170u8;
        let original_bytes = get_bytes_with_nibbles_from_index_one();
        let original_byte = original_bytes[byte_index];
        let nibbles = get_sample_offset_nibble_vec();
        assert!(original_byte != replacement_byte);
        let updated_nibbles= replace_byte_in_nibble_vec_at_byte_index(
            byte_index,
            nibbles,
            replacement_byte
        );
        let result = updated_nibbles.data[byte_index];
        assert!(result != original_byte);
        assert!(result == replacement_byte);
        for i in 0..updated_nibbles.data.len() {
            match i == byte_index {
                false => assert!(updated_nibbles.data[i] == original_bytes[i]),
                 _ => assert!(updated_nibbles.data[i] == replacement_byte)
            }
        };
    }

    #[test]
    fn should_convert_nibble_i_to_byte_i_in_nibble_vec_correctly() {
        let nibble_index = 3;
        let expected_result = 1;
        let nibbles = get_sample_nibble_vec();
        let result = convert_nibble_index_to_byte_index(&nibbles, &nibble_index);
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_nibble_i_to_byte_i_in_offset_nibble_vec_correctly() {
        let nibble_index = 3;
        let expected_result = 2;
        let nibbles = get_sample_offset_nibble_vec();
        let result = convert_nibble_index_to_byte_index(&nibbles, &nibble_index);
        assert!(result == expected_result);
    }

    #[test]
    fn should_replace_offset_nibble_at_nibble_index_in_nibble_vec_correctly() {
        for nibble_index in 0..get_length_in_nibbles(&get_sample_nibble_vec()) {
            let nibbles_before = get_sample_nibble_vec();
            let byte_index = convert_nibble_index_to_byte_index(
                &nibbles_before,
                &nibble_index
            );
            let byte_before = get_byte_containing_nibble_at_nibble_index(
                &nibbles_before,
                &nibble_index,
            ).unwrap();
            let replacement_nibble = get_nibbles_from_offset_bytes(
                vec![15u8] // [00001111]
            );
            let expected_byte = match nibble_index % 2 {
                0 => replace_high_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                ),
                _ => replace_low_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                )
            };
            let nibbles_after = replace_nibble_in_nibble_vec_at_nibble_index(
                nibbles_before.clone(),
                replacement_nibble.clone(),
                nibble_index
            ).unwrap();
            let target_nibble_after = get_nibble_at_index(
                &nibbles_after,
                nibble_index,
            ).unwrap();
            let byte_after = get_byte_containing_nibble_at_nibble_index(
                &nibbles_after,
                &nibble_index
            ).unwrap();
            assert!(byte_before != byte_after);
            assert!(target_nibble_after == replacement_nibble.data[0]);
            assert!(nibbles_before.data.len() == nibbles_after.data.len());
            assert!(byte_after == expected_byte);
            for i in 0..nibbles_after.data.len() {
                match i == byte_index {
                    true => assert!(nibbles_after.data[i] == expected_byte),
                    _ => assert!(nibbles_after.data[i] == nibbles_before.data[i])
                }
            }
        }
    }

    #[test]
    fn should_replace_offset_nibble_at_nibble_index_in_offset_nibble_vec_correctly() {
        for nibble_index in 0..get_length_in_nibbles(&get_sample_offset_nibble_vec()) {
            let nibbles_before = get_sample_offset_nibble_vec();
            let byte_index = convert_nibble_index_to_byte_index(
                &nibbles_before,
                &nibble_index
            );
            let byte_before = get_byte_containing_nibble_at_nibble_index(
                &nibbles_before,
                &nibble_index,
            ).unwrap();
            let replacement_nibble = get_nibbles_from_offset_bytes(
                vec![15u8] // [00001111]
            );
            let expected_byte = match nibble_index % 2 {
                0 => replace_low_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                ),
                _ => replace_high_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                )
            };
            let nibbles_after = replace_nibble_in_nibble_vec_at_nibble_index(
                nibbles_before.clone(),
                replacement_nibble.clone(),
                nibble_index
            ).unwrap();
            let target_nibble_after = get_nibble_at_index(
                &nibbles_after,
                nibble_index,
            ).unwrap();
            let byte_after = get_byte_containing_nibble_at_nibble_index(
                &nibbles_after,
                &nibble_index
            ).unwrap();
            assert!(byte_before != byte_after);
            assert!(target_nibble_after == replacement_nibble.data[0]);
            assert!(nibbles_before.data.len() == nibbles_after.data.len());
            assert!(byte_after == expected_byte);
            for i in 0..nibbles_after.data.len() {
                match i == byte_index {
                    true => assert!(nibbles_after.data[i] == expected_byte),
                    _ => assert!(nibbles_after.data[i] == nibbles_before.data[i])
                }
            }
        }
    }

    #[test]
    fn should_replace_nibble_at_nibble_index_in_offset_nibble_vec_correctly() {
        for nibble_index in 0..get_length_in_nibbles(&get_sample_offset_nibble_vec()) {
            let nibbles_before = get_sample_offset_nibble_vec();
            let byte_index = convert_nibble_index_to_byte_index(
                &nibbles_before,
                &nibble_index
            );
            let byte_before = get_byte_containing_nibble_at_nibble_index(
                &nibbles_before,
                &nibble_index,
            ).unwrap();
            let replacement_nibble = get_nibbles_from_bytes(
                vec![240u8] // [11110000]
            );
            let expected_byte = match nibble_index % 2 {
                0 => replace_low_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                ),
                _ => replace_high_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                )
            };
            let nibbles_after = replace_nibble_in_nibble_vec_at_nibble_index(
                nibbles_before.clone(),
                replacement_nibble.clone(),
                nibble_index
            ).unwrap();
            let target_nibble_after = get_nibble_at_index(
                &nibbles_after,
                nibble_index,
            ).unwrap();
            let byte_after = get_byte_containing_nibble_at_nibble_index(
                &nibbles_after,
                &nibble_index
            ).unwrap();
            assert!(byte_before != byte_after);
            // NOTE: Shift left ∵ we're replacing w/ a non-offset nibble!
            assert!(shift_nibble_left(target_nibble_after) == replacement_nibble.data[0]);
            assert!(nibbles_before.data.len() == nibbles_after.data.len());
            assert!(byte_after == expected_byte);
            for i in 0..nibbles_after.data.len() {
                match i == byte_index {
                    true => assert!(nibbles_after.data[i] == expected_byte),
                    _ => assert!(nibbles_after.data[i] == nibbles_before.data[i])
                }
            }
        }
    }

    #[test]
    fn should_replace_nibble_at_nibble_index_in_nibble_vec_correctly() {
        for nibble_index in 0..get_length_in_nibbles(&get_sample_nibble_vec()) {
            let nibbles_before = get_sample_nibble_vec();
            let byte_index = convert_nibble_index_to_byte_index(
                &nibbles_before,
                &nibble_index
            );
            let byte_before = get_byte_containing_nibble_at_nibble_index(
                &nibbles_before,
                &nibble_index,
            ).unwrap();
            let replacement_nibble = get_nibbles_from_bytes(
                vec![240u8] // [11110000]
            );
            let expected_byte = match nibble_index % 2 {
                0 => replace_high_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                ),
                _ => replace_low_nibble_in_byte(
                    byte_before.clone(),
                    replacement_nibble.clone(),
                )
            };
            let nibbles_after = replace_nibble_in_nibble_vec_at_nibble_index(
                nibbles_before.clone(),
                replacement_nibble.clone(),
                nibble_index
            ).unwrap();
            let target_nibble_after = get_nibble_at_index(
                &nibbles_after,
                nibble_index,
            ).unwrap();
            let byte_after = get_byte_containing_nibble_at_nibble_index(
                &nibbles_after,
                &nibble_index
            ).unwrap();
            assert!(byte_before != byte_after);
            // NOTE: Shift left ∵ we're replacing w/ a non-offset nibble!
            assert!(shift_nibble_left(target_nibble_after) == replacement_nibble.data[0]);
            assert!(nibbles_before.data.len() == nibbles_after.data.len());
            assert!(byte_after == expected_byte);
            for i in 0..nibbles_after.data.len() {
                match i == byte_index {
                    true => assert!(nibbles_after.data[i] == expected_byte),
                    _ => assert!(nibbles_after.data[i] == nibbles_before.data[i])
                }
            }
        }
    }

    #[test]
    fn should_set_first_nibble_flag_in_nibble_vec_to_zero_correctly() {
        let expected_result = 0;
        let nibbles = get_sample_offset_nibble_vec();
        let nibble_flag_before = nibbles.first_nibble_index;
        assert!(nibble_flag_before != expected_result);
        let updated_nibbles= set_first_index_in_nibble_vec_to_zero(nibbles);
        let result = updated_nibbles.first_nibble_index;
        assert!(result == expected_result);
    }

    #[test]
    fn should_set_first_nibble_flag_in_nibble_vec_to_one_correctly() {
        let expected_result = 1;
        let nibbles = get_sample_nibble_vec();
        let nibble_flag_before = nibbles.first_nibble_index;
        assert!(nibble_flag_before != expected_result);
        let updated_nibbles= set_first_index_in_nibble_vec_to_one(nibbles);
        let result = updated_nibbles.first_nibble_index;
        assert!(result == expected_result);
    }

    #[test]
    fn should_remove_first_byte_from_nibble_vec() {
        let nibbles_before = get_sample_nibble_vec();
        let number_of_nibbles_before = get_length_in_nibbles(&nibbles_before);
        let nibbles_after = remove_first_byte_from_nibble_vec(nibbles_before.clone())
            .unwrap();
        let number_of_nibbles_after = get_length_in_nibbles(&nibbles_after);
        assert!(number_of_nibbles_after == number_of_nibbles_before - 2);
        assert!(nibbles_after.data.len() == nibbles_before.data.len() - 1);
    }

    #[test]
    fn should_remove_first_byte_from_offest_nibble_vec() {
        let nibbles_before = get_sample_offset_nibble_vec();
        let number_of_nibbles_before = get_length_in_nibbles(&nibbles_before);
        let nibbles_after = remove_first_byte_from_nibble_vec(nibbles_before.clone())
            .unwrap();
        let number_of_nibbles_after = get_length_in_nibbles(&nibbles_after);
        assert!(number_of_nibbles_after == number_of_nibbles_before - 1);
        assert!(nibbles_after.data.len() == nibbles_before.data.len() - 1);

    }

    #[test]
    fn should_err_when_trying_to_remove_first_byte_of_one_byte_nibble() {
        let expected_err = "✘ Cannot remove byte, there's only 1 in the nibble vec!".to_string();
        let vec = vec![8u8];
        assert!(vec.len() == 1);
        let nibble = get_nibbles_from_bytes(vec);
        match remove_first_byte_from_nibble_vec(nibble) {
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            _ => panic!("Should be able to slice byte off 1 byte nibble!")
        }
    }

    #[test]
    fn should_get_zero_nibble() {
        let expected_byte = 0u8;
        let expected_length = 1;
        let expected_num_nibbles = 1;
        let expected_first_nibble_index = 1;
        let result = get_zero_nibble();
        let num_nibbles = get_length_in_nibbles(&result);
        assert!(result.data[0] == expected_byte);
        assert!(num_nibbles == expected_num_nibbles);
        assert!(result.data.len() == expected_length);
        assert!(result.first_nibble_index == expected_first_nibble_index);
    }

    #[test]
    fn should_remove_first_nibble_from_nibble_vec() {
        let nibbles = get_sample_nibble_vec();
        let first_nibble_before = get_nibble_at_index(&nibbles, 0)
            .unwrap();
        let expected_first_nibble_after = get_nibble_at_index(&nibbles, 1)
            .unwrap();
        let nibble_len_before = get_length_in_nibbles(&nibbles);
        let last_nibble_before = get_nibble_at_index(
            &nibbles,
            nibble_len_before - 1
        ).unwrap();
        let result = remove_first_nibble(nibbles)
            .unwrap();
        let nibble_len_after = get_length_in_nibbles(&result);
        let first_nibble_after = get_nibble_at_index(&result, 0)
            .unwrap();
        let last_nibble_after = get_nibble_at_index(
            &result,
            nibble_len_after - 1
        ).unwrap();
        let nibble_len_after = get_length_in_nibbles(&result);
        assert!(last_nibble_before == last_nibble_after);
        assert!(nibble_len_after == nibble_len_before - 1);
        assert!(first_nibble_before != first_nibble_after);
        assert!(first_nibble_after == expected_first_nibble_after);
    }

    #[test]
    fn should_remove_first_nibble_from_offset_nibble_vec() {
        let nibbles = get_sample_offset_nibble_vec();
        let first_nibble_before = get_nibble_at_index(&nibbles, 0)
            .unwrap();
        let expected_first_nibble_after = get_nibble_at_index(&nibbles, 1)
            .unwrap();
        let nibble_len_before = get_length_in_nibbles(&nibbles);
        let last_nibble_before = get_nibble_at_index(
            &nibbles,
            nibble_len_before - 1
        ).unwrap();
        let result = remove_first_nibble(nibbles)
            .unwrap();
        let nibble_len_after = get_length_in_nibbles(&result);
        let first_nibble_after = get_nibble_at_index(&result, 0)
            .unwrap();
        let last_nibble_after = get_nibble_at_index(
            &result,
            nibble_len_after - 1
        ).unwrap();
        let nibble_len_after = get_length_in_nibbles(&result);
        assert!(last_nibble_before == last_nibble_after);
        assert!(nibble_len_after == nibble_len_before - 1);
        assert!(first_nibble_before != first_nibble_after);
        assert!(first_nibble_after == expected_first_nibble_after);
    }

    #[test]
    fn should_remove_first_nibble_if_only_one_nibble() {
        let byte = 5u8;
        let expected_length = 1;
        let expected_byte = 0u8;
        let expected_nibble_length = 1;
        let nibble = get_nibbles_from_offset_bytes(vec![byte]);
        let result = remove_first_nibble(nibble)
            .unwrap();
        let nibble_length = get_length_in_nibbles(&result);
        assert!(result.data[0] == expected_byte);
        assert!(result.data.len() == expected_length);
        assert!(nibble_length == expected_nibble_length);
    }

    #[test]
    fn should_prefix_nibble_with_byte_correctly() {
        let nibbles = get_sample_nibble_vec();
        let mut prefix = vec![0xff];
        let mut expected_result = prefix.clone();
        let result = prefix_nibbles_with_byte(nibbles, prefix)
            .unwrap();
        let bytes = get_bytes_with_nibbles_from_index_zero();
        expected_result.append(&mut bytes.clone());
        assert!(result == expected_result);
    }

    #[test]
    fn should_prefix_offset_nibble_with_byte_correctly() {
        let nibbles = get_sample_offset_nibble_vec();
        let mut prefix = vec![0xff];
        let mut expected_result = prefix.clone();
        let result = prefix_nibbles_with_byte(nibbles, prefix)
            .unwrap();
        let bytes = get_bytes_with_nibbles_from_index_one();
        expected_result.append(&mut bytes.clone());
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_nibbles_to_bytes_correctly() {
        let nibbles = get_sample_nibble_vec();
        let expected_result = get_bytes_with_nibbles_from_index_zero();
        let result = convert_nibble_to_bytes(nibbles).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_offset_nibbles_to_bytes_correctly() {
        let nibbles = get_sample_offset_nibble_vec();
        let expected_result = get_bytes_with_nibbles_from_index_one();
        let result = convert_nibble_to_bytes(nibbles).unwrap();
        assert!(result == expected_result);
    }
}
