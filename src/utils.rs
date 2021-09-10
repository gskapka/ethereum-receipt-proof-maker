use crate::constants::DOT_ENV_PATH;
use crate::constants::HASH_LENGTH;
use crate::errors::AppError;
use crate::types::{Bytes, NoneError, Result};
use ethereum_types::{Address, H256, U256};
use hex;
use serde_json::Value;
use std::path::Path;

pub fn convert_json_value_to_string(value: Value) -> Result<String> {
    // TODO: Test!
    Ok(value
        .as_str()
        .ok_or(NoneError(format!(
            "Could not unwrap {} as a string!",
            value
        )))?
        .to_string())
}

fn left_pad_with_zero(string: &str) -> Result<String> {
    Ok(format!("0{}", string))
}

pub fn dot_env_file_exists() -> bool {
    Path::new(&DOT_ENV_PATH).exists()
}

pub fn convert_num_string_to_usize(num_str: &str) -> Result<usize> {
    match num_str.parse::<usize>() {
        Ok(res) => Ok(res),
        Err(_) => Err(AppError::Custom(format!(
            "✘ Cannot convert {} to integer!",
            num_str
        ))),
    }
}

pub fn convert_num_to_prefixed_hex(num: usize) -> Result<String> {
    Ok(format!("0x{:x}", num))
}

pub fn convert_hex_to_bytes(hex: String) -> Result<Bytes> {
    Ok(hex::decode(strip_hex_prefix(&hex.to_string())?)?)
}

pub fn strip_hex_prefix(prefixed_hex: &str) -> Result<String> {
    let res = str::replace(prefixed_hex, "0x", "");
    match res.len() % 2 {
        0 => Ok(res),
        _ => left_pad_with_zero(&res),
    }
}

pub fn convert_hex_to_address(hex: String) -> Result<Address> {
    decode_prefixed_hex(hex).and_then(|x| Ok(Address::from_slice(&x)))
}

pub fn convert_hex_to_u256(hex: String) -> Result<U256> {
    decode_prefixed_hex(hex).and_then(|x| Ok(U256::from_big_endian(&x)))
}

pub fn convert_hex_to_h256(hex: String) -> Result<H256> {
    decode_prefixed_hex(hex).and_then(|bytes| match bytes.len() {
        HASH_LENGTH => Ok(H256::from_slice(&bytes)),
        _ => Err(AppError::Custom(
            "✘ Wrong number of bytes in hex to create H256 type!".into(),
        )),
    })
}

pub fn convert_hex_strings_to_h256s(hex_strings: Vec<String>) -> Result<Vec<H256>> {
    let hashes: Result<Vec<H256>> = hex_strings
        .into_iter()
        .map(|hex_string| convert_hex_to_h256(hex_string.to_string()))
        .collect();
    Ok(hashes?)
}

pub fn decode_hex(hex_to_decode: String) -> Result<Vec<u8>> {
    Ok(hex::decode(hex_to_decode)?)
}

pub fn decode_prefixed_hex(hex_to_decode: String) -> Result<Vec<u8>> {
    strip_hex_prefix(&hex_to_decode).and_then(decode_hex)
}

pub fn convert_h256_to_prefixed_hex(hash: H256) -> Result<String> {
    Ok(format!("0x{}", hex::encode(hash)))
}

pub fn get_not_in_state_err(substring: &str) -> String {
    format!("✘ No {} in state!", substring)
}

pub fn get_no_overwrite_state_err(substring: &str) -> String {
    format!("✘ Cannot overwrite {} in state!", substring)
}

pub fn convert_bytes_to_h256(bytes: &Bytes) -> Result<H256> {
    match bytes.len() {
        32 => Ok(H256::from_slice(&bytes[..])),
        _ => Err(AppError::Custom(
            "✘ Wrong number of bytes for hash!".to_string(),
        )),
    }
}

pub fn convert_bytes_to_hex(bytes: Bytes) -> String {
    hex::encode(bytes)
}

pub fn convert_h256_to_bytes(hash: H256) -> Bytes {
    hash.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{HASH_HEX_CHARS, HEX_PREFIX_LENGTH};
    use crate::test_utils::{delete_env_file, read_env_file, restore_env_file, write_env_file};

    fn get_sample_block_hash() -> &'static str {
        "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10"
    }

    fn get_sample_h256() -> H256 {
        convert_hex_to_h256(get_sample_block_hash().to_string()).unwrap()
    }

    #[test]
    fn should_convert_hash_to_bytes() {
        let hash = get_sample_h256();
        let result = convert_h256_to_bytes(hash);
        let expected_hex = &get_sample_block_hash()[2..];
        let expected_result = hex::decode(expected_hex).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_h256_to_prefixed_hex_correctly() {
        let expected_result = get_sample_block_hash();
        let hash = get_sample_h256();
        let result = convert_h256_to_prefixed_hex(hash).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_unprefixed_hex_to_bytes_correctly() {
        let hex = "c0ffee".to_string();
        let expected_result = [192, 255, 238];
        let result = convert_hex_to_bytes(hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_prefixed_hex_to_bytes_correctly() {
        let hex = "0xc0ffee".to_string();
        let expected_result = [192, 255, 238];
        let result = convert_hex_to_bytes(hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_decode_none_prefixed_hex_correctly() {
        let none_prefixed_hex = "c0ffee";
        assert!(!none_prefixed_hex.contains("x"));
        let expected_result = [192, 255, 238];
        let result = decode_hex(none_prefixed_hex.to_string()).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_decode_prefixed_hex_correctly() {
        let prefixed_hex = "0xc0ffee";
        let mut chars = prefixed_hex.chars();
        assert!("0" == chars.next().unwrap().to_string());
        assert!("x" == chars.next().unwrap().to_string());
        let expected_result = [192, 255, 238];
        let result = decode_prefixed_hex(prefixed_hex.to_string()).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_left_pad_string_with_zero_correctly() {
        let dummy_hex = "0xc0ffee";
        let expected_result = "00xc0ffee".to_string();
        let result = left_pad_with_zero(dummy_hex).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_strip_hex_prefix_correctly() {
        let dummy_hex = "0xc0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummy_hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_not_strip_missing_hex_prefix_correctly() {
        let dummy_hex = "c0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummy_hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_hex_to_address_correcty() {
        let address_hex = "0xb2930b35844a230f00e51431acae96fe543a0347";
        let result = convert_hex_to_address(address_hex.to_string()).unwrap();
        let expected_result = decode_prefixed_hex(address_hex.to_string()).unwrap();
        let expected_result_bytes = &expected_result[..];
        assert!(result.as_bytes() == expected_result_bytes);
    }

    #[test]
    fn should_convert_hex_to_h256_correctly() {
        let dummy_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cec8b";
        assert!(dummy_hash.len() == HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        let result = convert_hex_to_h256(dummy_hash.to_string()).unwrap();
        let expected_result = decode_prefixed_hex(dummy_hash.to_string()).unwrap();
        let expected_result_bytes = &expected_result[..];
        assert!(result.as_bytes() == expected_result_bytes);
    }

    #[test]
    fn should_fail_to_convert_short_hex_to_h256_correctly() {
        let short_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cec";
        assert!(short_hash.len() < HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        match convert_hex_to_h256(short_hash.to_string()) {
            Err(AppError::Custom(e)) => {
                assert!(e == "✘ Wrong number of bytes in hex to create H256 type!")
            }
            _ => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_fail_to_convert_long_hex_to_h256_correctly() {
        let long_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cecffff";
        assert!(long_hash.len() > HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        match convert_hex_to_h256(long_hash.to_string()) {
            Err(AppError::Custom(e)) => {
                assert!(e == "✘ Wrong number of bytes in hex to create H256 type!")
            }
            _ => panic!("Wrong error received!"),
        }
    }

    #[test]
    fn should_fail_to_convert_invalid_hex_to_h256_correctly() {
        let long_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cecffzz";
        assert!(long_hash.len() > HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        assert!(long_hash.contains("z"));
        match convert_hex_to_h256(long_hash.to_string()) {
            Err(AppError::HexError(e)) => assert!(e.to_string().contains("Invalid")),
            Err(AppError::Custom(_)) => panic!("Should be hex error!"),
            _ => panic!("Should have errored ∵ of invalid hash!"),
        }
    }

    #[test]
    fn should_convert_hex_to_u256_correctly() {
        let hex = "0xc0ffee";
        let expected_result: u128 = 12648430;
        let result = convert_hex_to_u256(hex.to_string()).unwrap();
        assert!(result.as_u128() == expected_result)
    }

    #[test]
    fn should_get_no_state_err_string() {
        let thing = "thing".to_string();
        let expected_result = "✘ No thing in state!";
        let result = get_not_in_state_err(&thing);
        assert!(result == expected_result)
    }

    #[test]
    fn should_get_no_overwrite_err_string() {
        let thing = "thing".to_string();
        let expected_result = "✘ Cannot overwrite thing in state!";
        let result = get_no_overwrite_state_err(&thing);
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_hex_strings_to_h256s() {
        let str1 = "0xebfa2e7610ea186fa3fa97bbaa5db80cce033dfff7e546c6ee05493dbcbfda7a".to_string();
        let str2 = "0x08075826de57b85238fe1728a37b366ab755b95c65c59faec7b0f1054fca1654".to_string();
        let expected_result1 = convert_hex_to_h256(str1.clone()).unwrap();
        let expected_result2 = convert_hex_to_h256(str2.clone()).unwrap();
        let hex_strings: Vec<String> = vec![str1, str2];
        let results: Vec<H256> = convert_hex_strings_to_h256s(hex_strings).unwrap();
        assert!(results[0] == expected_result1);
        assert!(results[1] == expected_result2);
    }

    #[test]
    fn should_convert_number_to_hex_correctly() {
        let num = 1337;
        let result = convert_num_to_prefixed_hex(num).unwrap();
        let expected_result = "0x539";
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_num_string_to_usize_correctly() {
        let num_string = "1337";
        let expected_result: usize = 1337;
        let result = convert_num_string_to_usize(num_string).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_err_when_converting_invalid_num_string_to_usize() {
        let expected_err = "✘ Cannot convert";
        let invalid_num_str = "invalid num string";
        match convert_num_string_to_usize(invalid_num_str) {
            Ok(_) => panic!("Should fail to convert to int!"),
            Err(AppError::Custom(e)) => assert!(e.contains(expected_err)),
            Err(_) => panic!("Wrong error type received!"),
        }
    }

    #[test]
    #[serial]
    fn should_return_true_if_dot_env_file_exists() {
        if Path::new(&DOT_ENV_PATH).exists() {
            assert!(dot_env_file_exists());
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_return_false_if_dot_env_file_does_not_exist() {
        if Path::new(&DOT_ENV_PATH).exists() {
            let file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == file);
        } else {
            assert!(!dot_env_file_exists())
        }
    }

    #[test]
    fn should_get_hash_from_bytes() {
        let bytes = vec![
            0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff,
            0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0,
            0xff, 0xee, 0xc0, 0xff,
        ];
        assert!(bytes.len() == 32);
        match convert_bytes_to_h256(&bytes) {
            Ok(hash) => assert!(hash.to_fixed_bytes().to_vec() == bytes),
            Err(_) => panic!("Should have created hash!"),
        }
    }

    #[test]
    fn should_fail_to_get_hash_from_wrong_sized_bytes() {
        let expected_error = "✘ Wrong number of bytes for hash!";
        let bytes = vec![
            0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff,
            0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0, 0xff, 0xee, 0xc0,
            0xff, 0xee, 0xc0,
        ];
        assert!(bytes.len() != 32);
        match convert_bytes_to_h256(&bytes) {
            Err(AppError::Custom(e)) => assert!(e == expected_error),
            _ => panic!("did not get expected error!"),
        }
    }

    #[test]
    fn should_convert_h256_to_prefixed_hex() {
        let h256 = H256::zero();
        let expected_result =
            "0x0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let result = convert_h256_to_prefixed_hex(h256).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_convert_bytes_to_hex() {
        let bytes = vec![0xc0, 0xff, 0xee];
        let expected_result: String = "c0ffee".to_string();
        let result = convert_bytes_to_hex(bytes);
        assert!(result == expected_result);
    }
}
