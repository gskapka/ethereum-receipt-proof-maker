use crate::state::State;
use crate::rlp_codec::get_rlp_encoded_receipts_and_nibble_tuples;
use crate::types::{
    Result,
    Receipt,
};
use crate::trie::{
    Trie,
    put_in_trie_recursively,
};

fn get_receipts_trie_from_receipts(receipts: &Vec<Receipt>) -> Result<Trie> {
    get_rlp_encoded_receipts_and_nibble_tuples(receipts)
        .and_then(|key_value_tuples|
            put_in_trie_recursively(Trie::get_new_trie()?, key_value_tuples, 0)
        )
}

pub fn get_receipts_trie_and_set_in_state(state: State) -> Result<State> {
    get_receipts_trie_from_receipts(state.get_receipts_from_state()?)
        .and_then(|trie| state.set_receipts_trie_in_state(trie))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::convert_h256_to_prefixed_hex;
    use crate::test_utils::{
        RECEIPTS_ROOT,
        get_sample_receipts,
        get_valid_initial_state,
    };

    #[test]
    fn should_get_receipts_trie_from_receipts() {
        let receipts = get_sample_receipts();
        let result = get_receipts_trie_from_receipts(&receipts)
            .unwrap();
        let root_hex = convert_h256_to_prefixed_hex(result.root)
            .unwrap();
        assert!(root_hex == RECEIPTS_ROOT);
    }

    #[test]
    fn should_get_receipts_trie_from_state() {
        let state = get_valid_initial_state()
            .unwrap();
        let receipts = get_sample_receipts();
        let state_with_receipts = state.set_receipts_in_state(receipts)
            .unwrap();
        let result = get_receipts_trie_and_set_in_state(state_with_receipts)
            .unwrap();
        let trie_from_state = result.get_receipts_trie_from_state()
            .unwrap();
        let root_hex = convert_h256_to_prefixed_hex(trie_from_state.root)
            .unwrap();
        assert!(root_hex == RECEIPTS_ROOT);
    }
}
