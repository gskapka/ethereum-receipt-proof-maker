use rlp::RlpStream;
use ethereum_types::H256;
use crate::get_keccak_hash::keccak_hash_bytes;
use crate::utils::convert_hex_to_h256;
use crate::types::{
    Bytes,
    Result,
    Receipt
};

fn rlp_encode_receipt(receipt: &Receipt) -> Result<Bytes> {
    let mut rlp_stream = RlpStream::new();
    rlp_stream.append(receipt);
    Ok(rlp_stream.out())
}

fn keccak_hash_rlp_encoded_receipt(rlp_encoded_receipt: &Bytes) -> Result<H256> {
    keccak_hash_bytes(rlp_encoded_receipt)
}

pub fn get_rlp_encoded_receipt_and_hash_tuple(
    receipt: &Receipt
) -> Result<(H256, Bytes)> {
    rlp_encode_receipt(&receipt)
        .and_then(|rlp_encoded_receipt|
            Ok(
                (
                    keccak_hash_rlp_encoded_receipt(&rlp_encoded_receipt)?,
                    rlp_encoded_receipt,
                )
            )
        )
}

fn get_rlp_encoded_receipts_and_hash_tuples(
    receipts: &Vec<Receipt>
) -> Result<Vec<(H256, Bytes)>> {
    receipts
        .iter()
        .map(|receipt| get_rlp_encoded_receipt_and_hash_tuple(&receipt))
        .collect::<Result<Vec<(H256, Bytes)>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_expected_receipt
    };

    fn get_expected_receipt_hash() -> H256 {
        let hash = "0x514201561c8302dca9beed96d1af3c02d4bfff1fc7f1593797dcf948126eee61";
        convert_hex_to_h256(hash.to_string())
            .unwrap()
    }

    fn get_rlp_encoded_receipt() -> Bytes {
        vec![249, 1, 197, 1, 131, 120, 240, 40, 185, 1, 0, 0, 0, 0, 0, 0, 0,0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 248, 187, 248, 185, 148, 6, 1, 44, 140, 249, 123, 234, 213, 222, 174, 35, 112, 112, 249, 88, 127, 142, 122, 38, 109, 225, 160, 36, 30, 160, 60, 162, 2, 81, 128, 80, 132, 210, 125, 68, 64, 55, 28, 52, 160, 184, 95, 241, 8, 246, 187, 86, 17, 36, 143, 115, 129, 139, 128, 184, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 209, 163, 42, 11, 229, 31, 113, 112, 47, 143, 100, 197, 110, 81, 199, 86, 11, 47, 76, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 88, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 86, 226, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 165, 53]
    }

    #[test]
    fn should_rlp_encode_receipt() {
        let result = rlp_encode_receipt(&get_expected_receipt())
            .unwrap();
        assert!(result == get_rlp_encoded_receipt())
    }

    #[test]
    fn should_hash_receipt_correctly() {
        let expected_result = keccak_hash_bytes(&get_rlp_encoded_receipt())
            .unwrap();
        let result = rlp_encode_receipt(&get_expected_receipt())
            .and_then(|rlp_encoded_receipt| keccak_hash_rlp_encoded_receipt(&rlp_encoded_receipt))
            .unwrap();
        assert!(expected_result == result);
    }

    #[test]
    fn should_get_encoded_receipt_and_hash_tuple() {
        let result = get_rlp_encoded_receipt_and_hash_tuple(&get_expected_receipt())
            .unwrap();
        assert!(result.0 == get_expected_receipt_hash());
        assert!(result.1 == get_rlp_encoded_receipt());
    }

    #[test]
    fn should_get_encoded_receipts_and_hash_tuples() {
        let receipts = vec![
            get_expected_receipt(),
            get_expected_receipt(),
        ];
        let results = get_rlp_encoded_receipts_and_hash_tuples(&receipts)
            .unwrap();
        results
            .iter()
            .map(|result| {
                assert!(result.0 == get_expected_receipt_hash());
                assert!(result.1 == get_rlp_encoded_receipt());
            })
            .for_each(drop);
    }
}