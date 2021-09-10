use crate::{
    state::State,
    trie_nodes::Node,
    types::{Bytes, HexProof, Result},
    utils::convert_bytes_to_hex,
};
use rlp::RlpStream;

fn rlp_encode_node_stack(node_stack: &[Node]) -> Result<Bytes> {
    let mut rlp_stream = RlpStream::new();
    rlp_stream.begin_list(node_stack.len());
    node_stack
        .iter()
        .map(|node| node.get_rlp_encoding())
        .collect::<Result<Vec<_>>>()?
        .iter()
        .for_each(|rlp_encoded_node| {
            rlp_stream.append_raw(rlp_encoded_node, 1);
        });
    Ok(rlp_stream.out())
}

fn get_hex_proof_from_branch(branch: &[Node]) -> Result<HexProof> {
    rlp_encode_node_stack(branch).map(convert_bytes_to_hex)
}

pub fn get_hex_proof_from_branch_in_state(state: State) -> Result<HexProof> {
    info!("✔ Hex encoding proof from nodes in branch...");
    state
        .get_branch_from_state()
        .and_then(|node_stack| get_hex_proof_from_branch(node_stack))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get_branch_from_trie::get_branch_from_trie;
    use crate::test_utils::{
        get_sample_proof_1, get_sample_proof_3, get_sample_trie_with_sample_receipts,
        get_sample_tx_hashes_1, get_sample_tx_hashes_3,
        get_valid_state_with_receipts_trie_index_and_branch, PROOF_1_INDEX, PROOF_3_INDEX,
        SAMPLE_RECEIPT_JSONS_1_PATH, SAMPLE_RECEIPT_JSONS_3_PATH,
    };

    #[test]
    fn should_get_hex_proof_1_from_branch() {
        let expected_result = get_sample_proof_1();
        let trie = get_sample_trie_with_sample_receipts(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1(),
        );
        let branch = get_branch_from_trie(trie, PROOF_1_INDEX).unwrap();
        let result = get_hex_proof_from_branch(&branch).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_hex_proof_3_from_branch() {
        let expected_result = get_sample_proof_3();
        let trie = get_sample_trie_with_sample_receipts(
            SAMPLE_RECEIPT_JSONS_3_PATH.to_string(),
            get_sample_tx_hashes_3(),
        );
        let branch = get_branch_from_trie(trie, PROOF_3_INDEX).unwrap();
        let result = get_hex_proof_from_branch(&branch).unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_hex_proof_from_branch_in_state() {
        let expected_result = get_sample_proof_1();
        let state = get_valid_state_with_receipts_trie_index_and_branch(
            SAMPLE_RECEIPT_JSONS_1_PATH.to_string(),
            get_sample_tx_hashes_1(),
        )
        .unwrap();
        let result = get_hex_proof_from_branch_in_state(state).unwrap();
        assert!(result == expected_result);
    }
}
