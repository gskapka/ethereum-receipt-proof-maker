use crate::state::State;
use crate::types::Result;
use ethereum_types::H256;
use crate::utils::convert_hex_to_h256;

pub fn get_valid_dummy_hash_hex() -> String { // FIXME: Test!
     "0x8aa208025cf2b43ac4b1fada62f707f82a6e2159ebd2e3aad3c94f4907e92c94".to_string()
}

pub fn get_valid_dummy_hash_h256() -> Result<H256> { // FIXME: Test!
     convert_hex_to_h256(get_valid_dummy_hash_hex())
}

pub fn get_dummy_initial_state() -> Result<State> { // FIXME: Test!
    State::get_initial_state(
        get_valid_dummy_hash_h256()?,
        true
    )
}

#[cfg(test)]
mod tests {
    use hex;
    use super::*;
    use crate::errors::AppError;
    use crate::utils::get_not_in_state_err;
    use crate::validate_tx_hash::validate_tx_hash;

    #[test]
    fn should_get_valid_dummy_hash_as_hex() {
        let result = get_valid_dummy_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Hex tx hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_dummy_hash_as_h256() {
        let result = get_valid_dummy_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_dummy_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_dummy_intial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_dummy_hash_h256()
            .unwrap();
        let result = get_dummy_initial_state()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        assert!(result.verbose == expected_verbosity);
        match State::get_endpoint_from_state(result.clone()) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("endpoint")),
            _ => panic!("Intial state should not have endpoint set!")
        }
        match State::get_block_from_state(result.clone()) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }

    }
}
