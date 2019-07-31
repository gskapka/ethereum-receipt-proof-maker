use crate::state::State;
use crate::types::Result;
use crate::errors::AppError;
use crate::utils::convert_hex_to_h256;

use crate::constants::{
    HEX_PREFIX_LENGTH,
    HASH_HEX_CHARS
};

fn check_tx_hash_prefix(tx_hash: String) -> Result<String> {
    match tx_hash.starts_with("0x") {
        true => Ok(tx_hash),
        _ => Err(
            AppError::Custom(
                "✘ Passed in transaction hash has no hex prefix!".to_string()
            )
        )
    }
}

fn check_tx_hash_length(tx_hash: String) -> Result<String> {
    let expected_len = HEX_PREFIX_LENGTH + HASH_HEX_CHARS;
    match tx_hash.len() == expected_len {
        true => Ok(tx_hash),
        _ => Err(
            AppError::Custom(
                "✘ Passed in transaction hash is wrong length!".to_string()
            )
        )
    }
}

pub fn check_tx_hash_add_set_in_state(state: State, tx_hash: String) -> Result<State> {
    check_tx_hash_prefix(tx_hash)
        .and_then(check_tx_hash_length)
        .and_then(convert_hex_to_h256)
        .and_then(|hash| State::set_tx_hash_in_state(state, hash))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_hash_when_checking_prefixed_hash() {
        let prefixed_hex = "0xc0ffee".to_string();
        let result = check_tx_hash_prefix(prefixed_hex.clone())
            .unwrap();
        assert!(result == prefixed_hex);
    }

    #[test]
    fn should_error_when_checking_unprefixed_hash() {
        let expected_err = "✘ Passed in transaction hash has no hex prefix!";
        let unprefixed_hex = "c0ffee".to_string();
        match check_tx_hash_prefix(unprefixed_hex.clone()) {
            Ok(_) => panic!("Should error when checking unprefixed hex!"),
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            Err(e) => panic!(
                format!(
                    "Expected this error:\n{}\nBut got:\n{}",
                    expected_err,
                    e
                )
            )
        }
    }

    #[test]
    fn should_return_hash_if_correct_length() {
        let valid_hash = "0x8aa208025cf2b43ac4b1fada62f707f82a6e2159ebd2e3aad3c94f4907e92c94".to_string();
        assert!(valid_hash.len() == HEX_PREFIX_LENGTH + HASH_HEX_CHARS);
        let result = check_tx_hash_length(valid_hash.clone())
            .unwrap();
        assert!(result == valid_hash)
    }

    #[test]
    fn should_error_when_checking_short_hash() {
        let short_hash = "0xc0ffee".to_string();
        let expected_len = HEX_PREFIX_LENGTH + HASH_HEX_CHARS;
        assert!(short_hash.len() < expected_len);
        let expected_err = "✘ Passed in transaction hash is wrong length!".to_string();
        match check_tx_hash_length(short_hash.clone()) {
            Ok(_) => panic!("Should error when checking unprefixed hex!"),
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            Err(e) => panic!(
                format!(
                    "Expected this error:\n{}\nBut got:\n{}",
                    expected_err,
                    e
                )
            )
        }
    }

    #[test]
    fn should_error_when_checking_long_hash() {
        let long_hash = "0x8aa208025cf2b43ac4b1fada62f707f82a6e2159ebd2e3aad3c94f4907e92c94c0ffee".to_string();
        let expected_len = HEX_PREFIX_LENGTH + HASH_HEX_CHARS;
        assert!(long_hash.len() > expected_len);
        let expected_err = "✘ Passed in transaction hash is wrong length!".to_string();
        match check_tx_hash_length(long_hash.clone()) {
            Ok(_) => panic!("Should error when checking unprefixed hex!"),
            Err(AppError::Custom(e)) => assert!(e == expected_err),
            Err(e) => panic!(
                format!(
                    "Expected this error:\n{}\nBut got:\n{}",
                    expected_err,
                    e
                )
            )
        }
    }

    #[test]
    fn should_check_tx_hash_and_set_in_state() {
        let valid_hash = "0x8aa208025cf2b43ac4b1fada62f707f82a6e2159ebd2e3aad3c94f4907e92c94".to_string();
        let valid_hash_h256 = convert_hex_to_h256(valid_hash.clone())
            .unwrap();
        let state = State::get_initial_state()
            .unwrap();
        match State::get_tx_hash_from_state(state.clone()) {
            Err(AppError::Custom(e)) =>
                assert!( e == "✘ No transaction hash in state!"),
            Err(e) =>
                panic!("Wrong error when accessing state!"),
            Ok(_) =>
                panic!("State should not have tx hash set yet!")
        }
        let resultant_state = check_tx_hash_add_set_in_state(state, valid_hash)
            .unwrap();
        let tx_hash_from_state = State::get_tx_hash_from_state(resultant_state)
            .unwrap();
        assert!(tx_hash_from_state == valid_hash_h256);

    }
}
