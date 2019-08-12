use ethereum_types::H256;
use tiny_keccak::keccak256;
use crate::utils::convert_hex_to_h256;
use crate::types::{
    Bytes,
    Result,
};

pub fn keccak_hash_bytes(bytes: &Bytes) -> Result<H256> {
    Ok(keccak256(bytes).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_expected_hash() -> H256 {
        let hash = "0x28a564315acde65743f71672cda538275f2118d98b485a30f2af6679bfc510c8";
        convert_hex_to_h256(hash.to_string())
            .unwrap()
    }

    #[test]
    fn should_get_keccak_hash_correctly() {
        let bytes = vec![1 ,3 ,3, 7];
        let result = keccak_hash_bytes(&bytes)
            .unwrap();
        assert!(result == get_expected_hash());
    }
}
