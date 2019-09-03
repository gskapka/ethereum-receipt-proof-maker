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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::convert_hex_to_h256;
    use crate::get_database::get_thing_from_database;
    use crate::nibble_utils::{
        get_nibbles_from_bytes,
        convert_hex_string_to_nibbles,
    };

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
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        assert!(trie.node_stack.len() == 0);
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
            .unwrap();
        let result = trie.put_node_in_stack(node.clone())
            .unwrap();
        assert!(result.node_stack.len() == 1);
        assert!(result.node_stack.last() == Some(&node));
    }

    #[test]
    fn should_update_root_hash_from_node_in_stack() {
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let old_root = trie.root;
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
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
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
            .unwrap();
        let expected_result = node.get_rlp_encoding()
            .unwrap();
        let hash = node
            .get_hash()
            .unwrap();
        let updated_trie = trie
            .put_node_in_database(node.clone())
            .unwrap();
        let result = get_thing_from_database(&updated_trie.database, &hash)
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_save_stack_of_length_1_to_database() {
        let key = convert_hex_string_to_nibbles("c0ffe".to_string())
            .unwrap();
        let value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
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
        let value = vec![0xde, 0xca, 0xff];
        let trie = Trie::get_new_trie()
            .unwrap();
        assert!(trie.node_stack.len() == 0);
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
            .unwrap();
        let trie_with_stack = trie.put_node_in_stack(node.clone())
            .unwrap();
        assert!(trie_with_stack.node_stack.len() == 1);
        assert!(trie_with_stack.node_stack.last() == Some(&node));
        let result = trie_with_stack.reset_stack()
            .unwrap();
        assert!(result.node_stack.len() == 0);
    }
}
