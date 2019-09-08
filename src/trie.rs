use ethereum_types::H256;
use crate::errors::AppError;
use crate::utils::convert_bytes_to_h256;
use crate::trie_nodes::{
    Node,
    rlp_decode_node,
    get_node_from_database,
};
use crate::nibble_utils::{
    Nibbles,
    get_length_in_nibbles,
    split_at_first_nibble,
    get_common_prefix_nibbles,
};
use crate::constants::{
    EMPTY_NIBBLES,
    HASHED_NULL_NODE,
    LEAF_NODE_STRING,
    EMPTY_NODE_STRING,
    BRANCH_NODE_STRING,
    EXTENSION_NODE_STRING,
};
use crate::get_database::{
    get_new_database,
    put_thing_in_database,
    remove_thing_from_database,
};
use crate::types::{
    Bytes,
    Result,
    Database
};

pub type NodeStack = Vec<Node>; // TODO: Move to types!

pub struct Trie {
    pub root: H256,
    pub database: Database,
    pub node_stack: NodeStack,
}

impl Trie {
    pub fn get_new_trie() -> Result<Trie> {
        Ok(
            Trie {
                node_stack: Vec::new(),
                root: HASHED_NULL_NODE,
                database: get_new_database()?
            }
        )
    }

    pub fn put(self, key: Nibbles, value: Bytes) -> Result<Self> {
        match self.root == HASHED_NULL_NODE { // NOTE: Tested!
            true => Node::get_new_leaf_node(key, value)
                .and_then(|node| Trie::put_node_in_stack(self, node))
                .and_then(Trie::update_root_hash_from_node_in_stack)
                .and_then(Trie::save_stack_to_database),
            false => Trie::find(self, key) // NOTE: Untested & unfinished
                .and_then(Trie::process_node_stack)
        }
    }

    fn find( // TODO Untested!
        self,
        target_key: Nibbles
    ) -> Result<(Self, NodeStack, Nibbles)> {
        get_node_from_database(&self.database, &self.root)
            .and_then(|maybe_node| match maybe_node {
                Some(node) => Trie::find_path(self, vec![node], target_key),
                None => Err(AppError::Custom(
                    "✘ Find Error: Could not find root node in db!".to_string()
                ))
            })
    }

    fn find_path( // TODO: Untested!
        self,
        mut node_stack: NodeStack,
        key: Nibbles
    ) -> Result<(Self, NodeStack, Nibbles)> {
        match node_stack.pop() {
            None => Ok((self, node_stack, key)),
            Some(current_node) => {
                match current_node.get_type() {
                    "leaf" => Self::continue_finding_from_leaf(
                        self,
                        current_node,
                        node_stack,
                        key,
                    ),
                    "branch" => Self::continue_finding_from_branch(
                        self,
                        current_node,
                        node_stack,
                        key
                    ),
                    "extension" => Self::continue_finding_from_extension(
                        self,
                        current_node,
                        node_stack,
                        key,
                    ),
                    _ => Err(AppError::Custom(
                        "✘ Find Error: Node type not recognized!".to_string()
                    ))
                }
            }
        }
    }
    /**
     *
     * Finding onwards from a leaf node:
     *
     * Once at a leaf node we first check for any common prefix between our
     * target key and the leaf key. Once determined, we consider the two cases
     * of what remains of the target key:
     *
     * 1) No key remains.
     * 2) Some or all the key remains.
     *
     * In the first case, we have a full match and so return stack including
     * this leaf node along with an empty key.
     *
     * In case 2) we have no match but this is the closest node we got to. The
     * curent node is pushed back on the stack, which latter is returned along
     * with the key that remains to be found that was passed in.
     *
     */
    fn continue_finding_from_leaf( // TODO: Untested!
        self,
        current_node: Node,
        mut node_stack: NodeStack,
        key: Nibbles
    ) -> Result<(Self, NodeStack, Nibbles)> {
        get_common_prefix_nibbles(key.clone(), current_node.get_key())
            .and_then(|(_, remaining_key, _)| {
                node_stack.push(current_node);
                match remaining_key.len() {
                    0 => Ok((self, node_stack, EMPTY_NIBBLES)), // Full match
                    _ => Ok((self, node_stack, key)) // Some|no match
                }
            })
    }
    /**
     *
     * Finding onwards from an extension node:
     *
     * Once at an extension either we have three cases to consider:
     *
     * 1) No common prefix between target key and extension key.
     * 2) A common prefix that partially consumes the extension key.
     * 3) A common prefix that entirely consumes the extension key.
     *
     * In all three case we require the current node returned for further work.
     * In cases 1) & 2) we have reached the end of our search and so simply
     * return the stack of nodes and the key passed in.
     *
     * In case 3) we have fully consumed the extension node and so must get the
     * next node that the extension points to and add that to the stack. Then
     * pass that stack along with what remains of our target key for continued
     * searching.
     *
     */
    fn continue_finding_from_extension( // TODO Untested!
        self,
        current_node: Node,
        mut node_stack: NodeStack,
        key: Nibbles
    ) -> Result<(Self, NodeStack, Nibbles)> {
        get_common_prefix_nibbles(key.clone(), current_node.get_key())
            .and_then(|(common_prefix, remaining_key, remaining_node_key)| {
                let next_node_hash = &convert_bytes_to_h256(
                    &current_node.get_value()?
                )?;
                node_stack.push(current_node);
                match common_prefix.len() {
                    0 => Ok((self, node_stack, key)),
                    _ => match remaining_node_key.len() > 0 {
                        true => Ok((self, node_stack, key)),
                        false => { // Fully matched the extension ∴ continue.
                            match get_node_from_database(
                                &self.database,
                                next_node_hash
                            )? {
                                Some(next_node) => {
                                    node_stack.push(next_node);
                                    Self::find_path(
                                        self,
                                        node_stack,
                                        remaining_key
                                    )
                                },
                                None => Err(AppError::Custom(
                                    "✘ Find Error: Extension child not in db!"
                                        .to_string()
                                ))
                            }
                        }
                    }
                }
            })
    }
    /**
     *
     * Finding onwards from a branch node:
     *
     * When arriving at a branch node, we take our target key and slice off the
     * first nibble. This is then used as the index for inspecting the branches
     * children, at which point there are two cases:
     *
     * 1) The child is empty.
     * 2) The child is not empty.
     *
     * In the first case, we have reached the end of our search. The branch node
     * is placed back in the stack which is then returned along with the target
     * key passed in.
     *
     * In the second case, we have two more cases:
     *
     * 1) The child is a hash.
     * 2) The child is an inline node.
     *
     * In the first case we search the database for the node pointed to by that
     * hash and then add it to the stack after first adding the branch node
     * we're currently looking at back to the stack. We then recurse back into
     * the `find_path` function with our updated stack and the target key.
     *
     * The second case is not yet currently handled. // TODO!
     *
     */
    fn continue_finding_from_branch( // TODO Untested!
        self,
        current_node: Node,
        mut node_stack: NodeStack,
        key: Nibbles
    ) -> Result<(Self, NodeStack, Nibbles)> {
        node_stack.push(current_node.clone());
        split_at_first_nibble(&key)
            .and_then(|(first_nibble, remaining_nibbles)| {
                let index = first_nibble.data[0] as usize;
                match &current_node.branch?.branches[index] {
                    None => Ok((self, node_stack, key)),
                    Some(bytes) => {
                        let maybe_next_node = get_node_from_database(
                            &self.database,
                            &convert_bytes_to_h256(&bytes)?
                        )?;
                        match maybe_next_node {
                            Some(next_node) => {
                                node_stack.push(next_node);
                                Self::find_path(
                                    self,
                                    node_stack,
                                    remaining_nibbles
                                )
                            },
                            None => Err(AppError::Custom(
                                "✘ Find Error: Branch child not in db!"
                                    .to_string()
                            )),
                        }
                    }
                }
        })
    }

    pub fn update_root_hash(mut self, new_hash: H256) -> Result<Self> {
        self.root = new_hash;
        Ok(self)
    }

    pub fn put_node_in_stack(mut self, node: Node) -> Result<Self> {
        self.node_stack.push(node);
        Ok(self)
    }

    pub fn update_root_hash_from_node_in_stack(self) -> Result<Self> {
        let hash = self.node_stack.last()?.get_hash()?;
        self.update_root_hash(hash)
    }

    fn save_stack_to_database(mut self) -> Result<Self> {
        match self.node_stack.len() > 0 {
            true => {
                let node = self.node_stack.pop()?;
                self.put_node_in_database(node)
                    .and_then(Trie::save_stack_to_database)
            },
            _ => Ok(self)
        }
    }

    fn reset_stack(self) -> Result<Self> {
        Ok(
            Trie {
                root: self.root,
                node_stack: Vec::new(),
                database: self.database
            }
        )
    }

    fn put_node_in_database(self, node: Node) -> Result<Self> {
        Ok(
            Trie {
                root: self.root,
                node_stack: self.node_stack,
                database: put_thing_in_database(
                    self.database,
                    node.get_hash()?,
                    node.get_rlp_encoding()?
                )?
            }
        )
    }

    fn remove_node_from_database(self, node: Node) -> Result<Self> {
        Ok(
            Trie {
                root: self.root,
                node_stack: self.node_stack,
                database: remove_thing_from_database(
                    self.database,
                    &node.get_hash()?,
                )?
            }
        )
    }
}

fn get_key_length_accounted_for_in_stack(node_stack: &NodeStack) -> usize {
    node_stack
        .iter()
        .map(|node| node.get_key_length())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use crate::types::Receipt;
    use crate::utils::convert_hex_to_h256;
    use crate::get_receipt::get_receipt_from_tx_hash;
    use crate::get_database::get_thing_from_database;
    use crate::make_rpc_call::deserialize_to_receipt_rpc_response;
    use crate::rlp_codec::get_rlp_encoded_receipts_and_hash_tuples;
    use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
    use crate::nibble_utils::{
        get_nibbles_from_bytes,
        convert_hex_string_to_nibbles,
    };
    use crate::test_utils::{
        get_sample_leaf_node,
        get_sample_branch_node,
        SAMPLE_RECEIPT_JSONS_PATH,
        SAMPLE_RECECIPT_TX_HASHES,
        get_sample_extension_node,
    };

    fn get_sample_receipts() -> Vec<Receipt> {
        SAMPLE_RECECIPT_TX_HASHES
            .iter()
            .map(|hash_string|
                 format!("{}{}", SAMPLE_RECEIPT_JSONS_PATH, hash_string)
            )
            .map(|path| fs::read_to_string(path).unwrap())
            .map(|rpc_string|
                 deserialize_to_receipt_rpc_response(rpc_string)
                    .unwrap()
            )
            .map(|receipt_json|
                 deserialize_receipt_json_to_receipt_struct(receipt_json.result)
                    .unwrap()
            )
            .collect::<Vec<Receipt>>()
    }

    fn put_in_trie_recursively(
        trie: Trie,
        mut keys: Vec<Nibbles>,
        mut values: Vec<Bytes>,
        i: usize
    ) -> Result<Trie> {
        match i == keys.len() - 1  {
            true => Ok(trie),
            false => trie
                .put(keys[i].clone(), values[i].clone())
                .and_then(|new_trie|
                    put_in_trie_recursively(
                        new_trie,
                        keys,
                        values,
                        i + 1
                    )
                )
        }
    }

    #[test]
    fn should_get_empty_trie() {
        let trie = Trie::get_new_trie()
            .unwrap();
        assert!(trie.database.is_empty());
        assert!(trie.root == HASHED_NULL_NODE);
        assert!(trie.node_stack.len() == 0);
    }

    #[test]
    fn should_put_thing_in_empty_trie() {
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let value = vec![0xde, 0xca, 0xff];
        let expected_node = Node::get_new_leaf_node(key.clone(), value.clone())
            .unwrap();
        let expected_db_key = expected_node
            .get_hash()
            .unwrap();
        let expected_thing_from_db = expected_node
            .get_rlp_encoding()
            .unwrap();
        let trie = Trie::get_new_trie()
            .unwrap();
        let result = trie.put(key, value)
            .unwrap();
        assert!(result.node_stack.len() == 0);
        assert!(result.root == expected_node.get_hash().unwrap());
        let thing_from_db = get_thing_from_database(
            &result.database,
            &expected_db_key
        ).unwrap();
        assert!(thing_from_db == expected_thing_from_db)
    }

    #[test]
    fn should_update_root_hash() {
        let trie = Trie::get_new_trie()
            .unwrap();
        let old_hash = trie.root;
        let new_hash = convert_hex_to_h256(
            "a8780134f4add652b6e22e16a45b3436d3ecc293840fe8433f6fbcdc9ea8f16e".to_string()
        ).unwrap();
        assert!(old_hash != new_hash);
        let result = trie.update_root_hash(new_hash)
            .unwrap();
        assert!(result.root == new_hash);
        assert!(result.root != old_hash);
    }

    #[test]
    fn should_put_node_in_stack() {
        let node_key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        assert!(trie.node_stack.len() == 0);
        let node = Node::get_new_leaf_node(node_key.clone(), node_value.clone())
            .unwrap();
        let result = trie.put_node_in_stack(node.clone())
            .unwrap();
        assert!(result.node_stack.len() == 1);
        assert!(result.node_stack.last() == Some(&node));
    }

    #[test]
    fn should_update_root_hash_from_node_in_stack() {
        let node_key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let old_root = trie.root;
        let node = Node::get_new_leaf_node(node_key.clone(), node_value.clone())
            .unwrap();
        let expected_root = node.get_hash()
            .unwrap();
        let updated_trie = trie.put_node_in_stack(node.clone())
            .unwrap();
        let result = updated_trie.update_root_hash_from_node_in_stack()
            .unwrap();
        assert!(result.root != old_root);
        assert!(result.node_stack.len() == 1);
        assert!(result.root == expected_root);
        assert!(result.node_stack.last().unwrap().get_hash().unwrap() == result.root);
    }

    #[test]
    fn should_put_node_in_database_in_trie() {
        let node_key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node = Node::get_new_leaf_node(node_key.clone(), node_value.clone())
            .unwrap();
        let expected_result = node.get_rlp_encoding()
            .unwrap();
        let node_hash = node
            .get_hash()
            .unwrap();
        let updated_trie = trie
            .put_node_in_database(node.clone())
            .unwrap();
        let result = get_thing_from_database(&updated_trie.database, &node_hash)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_remove_node_from_database() {
        let node_key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node = Node::get_new_leaf_node(node_key.clone(), node_value.clone())
            .unwrap();
        let expected_result = node.get_rlp_encoding()
            .unwrap();
        let node_hash = node
            .get_hash()
            .unwrap();
        let updated_trie = trie
            .put_node_in_database(node.clone())
            .unwrap();
        assert!(updated_trie.database.contains_key(&node_hash));
        let resulting_trie = updated_trie.remove_node_from_database(node)
            .unwrap();
        assert!(!resulting_trie.database.contains_key(&node_hash));
    }

    #[test]
    fn should_save_stack_of_length_1_to_database() {
        let node_key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node = Node::get_new_leaf_node(node_key.clone(), node_value.clone())
            .unwrap();
        let expected_db_key = node
            .get_hash()
            .unwrap();
        let expected_thing_from_db = node
            .get_rlp_encoding()
            .unwrap();
        let updated_trie = trie.put_node_in_stack(node)
            .unwrap();
        assert!(updated_trie.node_stack.len() == 1);
        let result = updated_trie.save_stack_to_database()
            .unwrap();
        assert!(result.node_stack.len() == 0);
        let thing_from_db = get_thing_from_database(
            &result.database,
            &expected_db_key
        ).unwrap();
        assert!(thing_from_db == expected_thing_from_db)
    }

    #[test]
    #[ignore]
    fn should_save_stack_of_length_gt_one() {
        let key_1 = convert_hex_string_to_nibbles("c0ffee".to_string())
            .unwrap();
        let key_2 = convert_hex_string_to_nibbles("decaf".to_string())
            .unwrap();
        let value_1 = vec![0xde, 0xca, 0xff];
        let value_2 = vec![0xc0, 0xff, 0xee];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node_1 = Node::get_new_leaf_node(key_1.clone(), value_1.clone())
            .unwrap();
        let node_2 = Node::get_new_leaf_node(key_2.clone(), value_2.clone())
            .unwrap();
        let expected_db_key_1 = node_1
            .get_hash()
            .unwrap();
        let expected_db_key_2 = node_2
            .get_hash()
            .unwrap();
        let expected_thing_from_db_1 = node_1
            .get_rlp_encoding()
            .unwrap();
        let expected_thing_from_db_2 = node_2
            .get_rlp_encoding()
            .unwrap();
        let updated_trie_1 = trie.put_node_in_stack(node_1)
            .unwrap();
        let updated_trie_2 = updated_trie_1.put_node_in_stack(node_2)
            .unwrap();
        assert!(updated_trie_2.node_stack.len() == 2);
        let result = updated_trie_2.save_stack_to_database()
            .unwrap();
        assert!(result.node_stack.len() == 0);
        let thing_from_db_1 = get_thing_from_database(
            &result.database,
            &expected_db_key_1
        ).unwrap();
        let thing_from_db_2 = get_thing_from_database(
            &result.database,
            &expected_db_key_2
        ).unwrap();
        assert!(thing_from_db_1 == expected_thing_from_db_1);
        assert!(thing_from_db_2 == expected_thing_from_db_2);
    }

    #[test]
    fn should_reset_stack() {
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let node_value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        assert!(trie.node_stack.len() == 0);
        let node = Node::get_new_leaf_node(key.clone(), node_value.clone())
            .unwrap();
        let trie_with_stack = trie.put_node_in_stack(node.clone())
            .unwrap();
        assert!(trie_with_stack.node_stack.len() == 1);
        assert!(trie_with_stack.node_stack.last() == Some(&node));
        let result = trie_with_stack.reset_stack()
            .unwrap();
        assert!(result.node_stack.len() == 0);
    }

    #[test]
    fn should_sum_length_of_key_so_far_in_node_stack() {
        let mut node_stack: NodeStack = Vec::new();
        let leaf_node = get_sample_leaf_node();
        let branch_node = get_sample_branch_node();
        let extension_node = get_sample_extension_node();
        node_stack.push(leaf_node);
        node_stack.push(extension_node);
        node_stack.push(branch_node);
        let expected_result = 13;
        let result = get_key_length_accounted_for_in_stack(&node_stack);
        assert!(result == expected_result);
    }
}
