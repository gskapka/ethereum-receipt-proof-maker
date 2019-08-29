use ethereum_types::H256;
use crate::trie_nodes::Node;
use crate::nibble_utils::Nibbles;
use crate::constants::HASHED_NULL_NODE;
use crate::get_database::{
    get_new_database,
    put_thing_in_database,
};
use crate::types::{
    Bytes,
    Result,
    Database
};

pub type Stack = Vec<Node>; // TODO: Move to types!

pub struct Trie {
    pub root: H256,
    pub stack: Stack,
    pub database: Database,
}

impl Trie {
    pub fn get_new_trie() -> Result<Trie> {
        Ok(
            Trie {
                stack: Vec::new(),
                root: HASHED_NULL_NODE,
                database: get_new_database()?
            }
        )
    }

    pub fn put(self, key: Nibbles, value: Bytes) -> Result<Self> {
        match self.root == HASHED_NULL_NODE { // Note: Tested!
            true => Node::get_new_leaf_node(key, value)
                .and_then(|node| Trie::put_node_in_stack(self, node))
                .and_then(Trie::update_root_hash_from_node_in_stack)
                .and_then(Trie::save_stack_to_database),
            _ => Ok(self)
        }
    }

    //pub fn find(self, key: Nibbles)

    pub fn update_root_hash(mut self, new_hash: H256) -> Result<Self> {
        self.root = new_hash;
        Ok(self)
    }

    pub fn put_node_in_stack(mut self, node: Node) -> Result<Self> {
        self.stack.push(node);
        Ok(self)
    }

    pub fn update_root_hash_from_node_in_stack(self) -> Result<Self> {
        let hash = self.stack.last()?.get_hash()?;
        self.update_root_hash(hash)
    }

    fn save_stack_to_database(mut self) -> Result<Self> {
        match self.stack.len() > 0 {
            true => {
                let node = self.stack.pop()?;
                self.put_node_in_database(node)
                    .and_then(Trie::save_stack_to_database)
            },
            _ => Ok(self)
        }
    }

    fn put_node_in_database(self, node: Node) -> Result<Self> {
        Ok(
            Trie {
                root: self.root,
                stack: self.stack,
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
        assert!(trie.stack.len() == 0);
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
        assert!(result.stack.len() == 0);
        assert!(result.root == expected_node.get_hash().unwrap());
        let thing_from_db = get_thing_from_database(
            &result.database,
            expected_db_key
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
        assert!(trie.stack.len() == 0);
        let node = Node::get_new_leaf_node(key.clone(), value.clone())
            .unwrap();
        let result = trie.put_node_in_stack(node.clone())
            .unwrap();
        assert!(result.stack.len() == 1);
        assert!(result.stack.last() == Some(&node));
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
        assert!(result.stack.len() == 1);
        assert!(result.root == expected_root);
        assert!(result.stack.last().unwrap().get_hash().unwrap() == result.root);
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
        let result = get_thing_from_database(&updated_trie.database, hash)
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
        assert!(updated_trie.stack.len() == 1);
        let result = updated_trie.save_stack_to_database()
            .unwrap();
        assert!(result.stack.len() == 0);
        let thing_from_db = get_thing_from_database(
            &result.database,
            expected_db_key
        ).unwrap();
        assert!(thing_from_db == expected_thing_from_db)
    }
}
