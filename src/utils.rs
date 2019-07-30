use hex;
use std::result;
use crate::errors::AppError;
use ethereum_types::{U256, H256, Address};

type Bytes = Vec<u8>; // TODO: Types module?
type Result<T> = result::Result<T, AppError>;

use crate::constants::{
    HASH_LENGTH,
    HASH_HEX_CHARS,
    HEX_PREFIX_LENGTH,
};

fn left_pad_with_zero(string: &str) -> Result<String> {
    Ok(format!("0{}", string))
}

pub fn hex_to_bytes(hex: String) -> Result<Bytes> {
    Ok(hex::decode(strip_hex_prefix(&hex.to_string())?)?)
}

pub fn strip_hex_prefix(prefixed_hex : &str) -> Result<String> {
    let res = str::replace(prefixed_hex, "0x", "");
    match res.len() % 2 {
        0 => Ok(res),
        _ => left_pad_with_zero(&res),
    }
}

pub fn convert_hex_to_address(hex: String) -> Result<Address> {
    decode_prefixed_hex(hex)
        .and_then(|x| Ok(Address::from_slice(&x)))
}

pub fn convert_hex_to_u256(hex: String) -> Result<U256> {
    decode_prefixed_hex(hex)
        .and_then(|x| Ok(U256::from_big_endian(&x)))
}

pub fn convert_hex_to_h256(hex: String) -> Result<H256> {
    decode_prefixed_hex(hex)
        .and_then(|bytes| match bytes.len() {
            HASH_LENGTH => Ok(H256::from_slice(&bytes)),
            0..HASH_LENGTH => Err(
                AppError::Custom(
                    format!("✘ Too few bytes in hex to create H256 type!")
                )
            ),
            _ => Err(
                AppError::Custom(
                    format!("✘ Too many bytes in hex to create H256 type!")
                )
            )
        })
}

pub fn decode_hex(hex_to_decode: String) -> Result<Vec<u8>> {
    Ok(hex::decode(hex_to_decode)?)
}

pub fn decode_prefixed_hex(hex_to_decode: String) -> Result<Vec<u8>> {
    strip_hex_prefix(&hex_to_decode)
        .and_then(decode_hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_convert_unprefixed_hex_to_bytes_correctly() {
        let hex = "c0ffee".to_string();
        let expected_result = [ 192, 255, 238 ];
        let result = hex_to_bytes(hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_prefixed_hex_to_bytes_correctly() {
        let hex = "0xc0ffee".to_string();
        let expected_result = [ 192, 255, 238 ];
        let result = hex_to_bytes(hex).unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_decode_none_fixed_hex_correctly() {
        let none_prefixed_hex = "c0ffee";
        assert!(!none_prefixed_hex.contains("x"));
        let expected_result = [192, 255, 238];
        let result = decode_hex(none_prefixed_hex.to_string())
            .unwrap();
        assert!(result == expected_result)
    }
    #[test]
    fn should_decode_prefixed_hex_correctly() {
        let prefixed_hex = "0xc0ffee";
        let mut chars = prefixed_hex.chars();
        assert!("0" == chars.next().unwrap().to_string());
        assert!("x" == chars.next().unwrap().to_string());
        let expected_result = [192, 255, 238];
        let result = decode_prefixed_hex(prefixed_hex.to_string())
            .unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_left_pad_string_with_zero_correctly() {
        let dummy_hex = "0xc0ffee";
        let expected_result = "00xc0ffee".to_string();
        let result = left_pad_with_zero(dummy_hex)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_strip_hex_prefix_correctly() {
        let dummy_hex = "0xc0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummy_hex)
            .unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_not_strip_missing_hex_prefix_correctly() {
        let dummy_hex = "c0ffee";
        let expected_result = "c0ffee".to_string();
        let result = strip_hex_prefix(dummy_hex)
            .unwrap();
        assert!(result == expected_result)
    }

    #[test]
    fn should_convert_hex_to_address_correcty() {
        let address_hex = "0xb2930b35844a230f00e51431acae96fe543a0347";
        let result = convert_hex_to_address(address_hex.to_string())
            .unwrap();
        let expected_result = decode_prefixed_hex(address_hex.to_string())
            .unwrap();
        let expected_result_bytes = &expected_result[..];
        assert!(result.as_bytes() == expected_result_bytes);
    }

    #[test]
    fn should_convert_hex_to_h256_correctly() {
        let dummy_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cec8b";
        assert!(dummy_hash.len() == HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        let result = convert_hex_to_h256(dummy_hash.to_string())
            .unwrap();
        let expected_result = decode_prefixed_hex(dummy_hash.to_string())
            .unwrap();
        let expected_result_bytes = &expected_result[..];
        assert!(result.as_bytes() == expected_result_bytes);
    }

    #[test]
    fn should_fail_to_convert_short_hex_to_h256_correctly() {
        let short_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cec";
        assert!(short_hash.len() < HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        match convert_hex_to_h256(short_hash.to_string()) {
            Err(AppError::Custom(e)) => assert!(e == "✘ Too few bytes in hex to create H256 type!"),
            _ => panic!("Should have errored ∵ of short hash!")
        }
    }

    #[test]
    fn should_fail_to_convert_long_hex_to_h256_correctly() {
        let long_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cecffff";
        assert!(long_hash.len() > HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        match convert_hex_to_h256(long_hash.to_string()) {
            Err(AppError::Custom(e)) => assert!(
                e == "✘ Too many bytes in hex to create H256 type!"
            ),
            Err(e) => println!("weird err{:?}", e),
            _ => panic!("Should have errored ∵ of short hash!")
        }
    }

    #[test]
    fn should_fail_to_convert_invalid_hex_to_h256_correctly() {
        let long_hash = "0xc5acf860fa849b72fc78855dcbc4e9b968a8af5cdaf79f03beeca78e6a9cecffzz";
        assert!(long_hash.len() > HASH_HEX_CHARS + HEX_PREFIX_LENGTH);
        assert!(long_hash.contains("z"));
        match convert_hex_to_h256(long_hash.to_string()) {
            Err(AppError::HexError(e)) => assert!(
                e.to_string().contains("Invalid")
            ),
            Err(AppError::Custom(_)) => panic!("Should be hex error!"),
            _ => panic!("Should have errored ∵ of invalid hash!")
        }
    }

    #[test]
    fn should_convert_hex_to_u256_correctly() {
        let hex = "0xc0ffee";
        let expected_result: u128 = 12648430;
        let result = convert_hex_to_u256(hex.to_string())
            .unwrap();
        assert!(result.as_u128() == expected_result)
    }

}
