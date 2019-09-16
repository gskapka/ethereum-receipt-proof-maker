use rlp::RlpStream;
use crate::state::State;
use crate::utils::convert_bytes_to_hex;
use crate::types::{
    Bytes,
    Result,
    HexProof,
    NodeStack,
};

fn rlp_encode_node_stack(node_stack: &NodeStack) -> Result<Bytes> {
    let mut rlp_stream = RlpStream::new();
    rlp_stream.begin_list(node_stack.len());
    for i in 0..node_stack.len() {
        rlp_stream.append_raw(
            &node_stack[i].get_rlp_encoding()?,
            1
        );
    }
    Ok(rlp_stream.out())
}

fn get_hex_proof_from_branch(branch: &NodeStack) -> Result<HexProof> {
    rlp_encode_node_stack(branch)
        .map(convert_bytes_to_hex)
}

pub fn get_hex_proof_from_branch_in_state(state: State) -> Result<HexProof> {
    info!("✔ Hex encoding proof from nodes in branch...");
    state
        .get_branch_from_state()
        .and_then(get_hex_proof_from_branch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_branch_from_trie::get_branch_from_trie;
    use crate::test_utils::{
        PROOF_1_INDEX,
        PROOF_2_INDEX,
        get_sample_proof_1,
        get_sample_proof_2,
        get_sample_tx_hashes_1,
        get_sample_tx_hashes_2,
        SAMPLE_RECEIPT_JSONS_1_PATH,
        SAMPLE_RECEIPT_JSONS_2_PATH,
        get_sample_trie_with_sample_receipts,
        get_valid_state_with_receipts_trie_index_and_branch,
    };

    #[test]
    fn should_get_hex_proof_1_from_branch() {
        let expected_result = get_sample_proof_1();
        let trie = get_sample_trie_with_sample_receipts(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1()
        );
        let branch = get_branch_from_trie(trie, PROOF_1_INDEX)
            .unwrap();
        let result = get_hex_proof_from_branch(&branch)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_hex_proof_2_from_branch() {
        let expected_result = get_sample_proof_2();
        let trie = get_sample_trie_with_sample_receipts(
            SAMPLE_RECEIPT_JSONS_2_PATH.to_string(),
            get_sample_tx_hashes_2()
        );
        let branch = get_branch_from_trie(trie, PROOF_2_INDEX)
            .unwrap();
        let result = get_hex_proof_from_branch(&branch)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_hex_proof_from_branch_in_state() {
        let expected_result = get_sample_proof_1();
        let state = get_valid_state_with_receipts_trie_index_and_branch(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1()
        ).unwrap();
        let result = get_hex_proof_from_branch_in_state(state)
            .unwrap();
        assert!(result == expected_result);
    }
}
