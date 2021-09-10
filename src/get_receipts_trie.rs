use crate::rlp_codec::get_rlp_encoded_receipts_and_nibble_tuples;
use crate::state::State;
use crate::trie::{put_in_trie_recursively, Trie};
use crate::types::{Receipt, Result};

fn get_receipts_trie_from_receipts(receipts: &[Receipt]) -> Result<Trie> {
    get_rlp_encoded_receipts_and_nibble_tuples(receipts).and_then(|key_value_tuples| {
        put_in_trie_recursively(Trie::get_new_trie()?, key_value_tuples, 0)
    })
}

pub fn get_receipts_trie_and_set_in_state(state: State) -> Result<State> {
    info!("✔ Building merkle-patricia trie from receipts...");
    get_receipts_trie_from_receipts(state.get_receipts_from_state()?)
        .and_then(|trie| state.set_receipts_trie_in_state(trie))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_sample_receipts, get_sample_tx_hashes_1, get_sample_tx_hashes_2,
        get_valid_initial_state, RECEIPTS_ROOT_1, RECEIPTS_ROOT_2, SAMPLE_RECEIPT_JSONS_1_PATH,
        SAMPLE_RECEIPT_JSONS_2_PATH,
    };
    use crate::utils::convert_h256_to_prefixed_hex;

    #[test]
    fn should_get_receipts_trie_1_from_receipts() {
        let receipts = get_sample_receipts(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1(),
        );
        let result = get_receipts_trie_from_receipts(&receipts).unwrap();
        let root_hex = convert_h256_to_prefixed_hex(result.root).unwrap();
        assert!(root_hex == RECEIPTS_ROOT_1);
    }

    #[test]
    fn should_get_receipts_trie_1_from_state() {
        let state = get_valid_initial_state().unwrap();
        let receipts = get_sample_receipts(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1(),
        );
        let state_with_receipts = state.set_receipts_in_state(receipts).unwrap();
        let result = get_receipts_trie_and_set_in_state(state_with_receipts).unwrap();
        let trie_from_state = result.get_receipts_trie_from_state().unwrap();
        let root_hex = convert_h256_to_prefixed_hex(trie_from_state.root).unwrap();
        assert!(root_hex == RECEIPTS_ROOT_1);
    }

    #[test]
    fn should_get_receipts_trie_2_from_receipts() {
        let receipts = get_sample_receipts(
            SAMPLE_RECEIPT_JSONS_2_PATH.to_string(),
            get_sample_tx_hashes_2(),
        );
        let result = get_receipts_trie_from_receipts(&receipts).unwrap();
        let root_hex = convert_h256_to_prefixed_hex(result.root).unwrap();
        assert!(root_hex == RECEIPTS_ROOT_2);
    }

    #[test]
    fn should_get_receipts_trie_2_from_state() {
        let state = get_valid_initial_state().unwrap();
        let receipts = get_sample_receipts(
            SAMPLE_RECEIPT_JSONS_2_PATH.to_string(),
            get_sample_tx_hashes_2(),
        );
        let state_with_receipts = state.set_receipts_in_state(receipts).unwrap();
        let result = get_receipts_trie_and_set_in_state(state_with_receipts).unwrap();
        let trie_from_state = result.get_receipts_trie_from_state().unwrap();
        let root_hex = convert_h256_to_prefixed_hex(trie_from_state.root).unwrap();
        assert!(root_hex == RECEIPTS_ROOT_2);
    }
}
