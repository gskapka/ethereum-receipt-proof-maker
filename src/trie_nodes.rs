use rlp::RlpStream;
use ethereum_types::H256;
use crate::errors::AppError;
use crate::get_keccak_hash::keccak_hash_bytes;
use crate::path_codec::{
    encode_leaf_path_from_nibbles,
    encode_extension_path_from_nibbles,
};
use crate::nibble_utils::{
    Nibbles,
    get_nibbles_from_bytes
};
use crate::types::{
    Bytes,
    Result,
    ChildNodes,
};
use crate::constants::{
    EMPTY_NIBBLES,
    LEAF_NODE_STRING,
    EMPTY_NODE_STRING,
    BRANCH_NODE_STRING,
    EXTENSION_NODE_STRING,
};

static NO_NODE_IN_STRUCT_ERR: &'static str = "✘ No node present in struct to rlp-encode!";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub leaf: Option<LeafNode>,
    pub branch: Option<BranchNode>,
    pub extension: Option<ExtensionNode>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LeafNode {
    pub raw: Bytes,
    pub value: Bytes,
    pub encoded_path: Bytes,
    pub path_nibbles: Nibbles,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ExtensionNode {
    pub raw: Bytes,
    pub value: Bytes,
    pub encoded_path: Bytes,
    pub path_nibbles: Nibbles,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BranchNode {
    pub value: Option<Bytes>,
    pub branches: ChildNodes,
}

impl Node {
    pub fn get_new_empty_node() -> Result<Node> {
        Ok(
            Node {
                leaf: None,
                branch: None,
                extension: None
            }
        )
    }

    pub fn get_new_leaf_node(
        path_nibbles: Nibbles,
        value: Bytes
    ) -> Result<Node> {
        let encoded_path = encode_leaf_path_from_nibbles(
            path_nibbles.clone()
        )?;
        let mut raw = encoded_path.clone();
        raw.append(&mut value.clone());
        Ok(
            Node {
                branch: None,
                extension: None,
                leaf: Some(
                    LeafNode {
                        raw,
                        value,
                        path_nibbles,
                        encoded_path,
                    }
                )
            }
        )
    }

    pub fn get_new_extension_node(
        path_nibbles: Nibbles,
        value: Bytes
    ) -> Result<Node> {
        let encoded_path = encode_extension_path_from_nibbles(
            path_nibbles.clone()
        )?;
        let mut raw = encoded_path.clone();
        raw.append(&mut value.clone());
        Ok(
            Node {
                leaf: None,
                branch: None,
                extension: Some(
                    ExtensionNode {
                        raw,
                        value,
                        path_nibbles,
                        encoded_path,
                    }
                )
            }
        )
    }

    pub fn get_new_branch_node(value: Option<Bytes>) -> Result<Node> {
        Ok(
            Node {
                leaf: None,
                extension: None,
                branch: Some(
                    BranchNode {
                        value,
                        branches: get_empty_child_nodes(),
                    }
                )
            }
        )
    }

    pub fn update_branch_at_index(
        self,
        new_value: Option<Bytes>,
        index: usize
    ) -> Result<Self> {
        if let Some(branch) = self.branch {
            Ok(
                Node {
                    leaf: None,
                    extension: None,
                    branch: Some(
                        BranchNode {
                            value: branch.value,
                            branches: update_child_nodes(
                                branch.branches,
                                new_value,
                                index,
                            )?
                        }
                    )
                }
            )
        } else {
            Err(AppError::Custom(
                "✘ Cannot update branches - not a branch node!".to_string()
            ))
        }
    }

    pub fn get_rlp_encoding(&self) -> Result<Bytes> {
        let mut rlp_stream = RlpStream::new();
        if let Some(leaf) = &self.leaf {
            rlp_stream.begin_list(2);
            rlp_stream.append(&leaf.encoded_path);
            rlp_stream.append(&leaf.value);
            Ok(rlp_stream.out())
        } else if let Some(extension) = &self.extension {
            rlp_stream.begin_list(2);
            rlp_stream.append(&extension.encoded_path);
            rlp_stream.append(&extension.value);
            Ok(rlp_stream.out())
        } else if let Some(branch) = &self.branch {
            rlp_stream.begin_list(17);
            for i in 0..branch.branches.len() {
                match &branch.branches[i] {
                    None => rlp_stream.append_empty_data(),
                    Some(thing) => rlp_stream.append(&thing.clone())
                };
            }
            match &branch.value {
                None => rlp_stream.append_empty_data(),
                Some(value) => rlp_stream.append(&value.clone())
            };
            Ok(rlp_stream.out())
        } else {
            Err(AppError::Custom(NO_NODE_IN_STRUCT_ERR.to_string()))
        }
    }

    pub fn get_hash(&self) -> Result<H256> {
        self.get_rlp_encoding()
            .and_then(|encoded| keccak_hash_bytes(&encoded))
    }

    pub fn get_key(&self) -> Nibbles {
        if let Some(leaf_node) = &self.leaf {
            leaf_node.path_nibbles.clone()
        } else if let Some(extension_node) = &self.extension {
            extension_node.path_nibbles.clone()
        } else {
            EMPTY_NIBBLES
        }
    }

    pub fn get_type(&self) -> &'static str {
        if let Some(_) = self.leaf {
            LEAF_NODE_STRING
        } else if let Some(_) = self.branch {
            BRANCH_NODE_STRING
        } else if let Some(_) = self.extension {
            EXTENSION_NODE_STRING
        } else {
            EMPTY_NODE_STRING
        }
    }
}

fn get_empty_child_nodes() -> ChildNodes {
    [
        None, None, None, None,
        None, None, None, None,
        None, None, None, None,
        None, None, None, None,
    ]
}

fn update_child_nodes(
    mut child_nodes: ChildNodes,
    new_value: Option<Bytes>,
    index: usize,
) -> Result<ChildNodes> {
    child_nodes[index] = new_value;
    Ok(child_nodes)
}

#[cfg(test)]
mod tests {
    use hex;
    use super::*;
    use crate::nibble_utils::get_length_in_nibbles;
    use crate::utils::convert_hex_to_h256;

    fn get_sample_leaf_node() -> Node {
        let path_bytes = vec![0x12, 0x34, 0x56];
        let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
        let value = hex::decode("c0ffee".to_string()).unwrap();
        Node::get_new_leaf_node(path_nibbles, value)
            .unwrap()
    }

    fn get_sample_extension_node() -> Node {
        let path_bytes = vec![0xc0, 0xff, 0xee];
        let path_nibbles = get_nibbles_from_bytes(path_bytes);
        let value = hex::decode(
            "1d237c84432c78d82886cb7d6549c179ca51ebf3b324d2a3fa01af6a563a9377".to_string()
        ).unwrap();
        Node::get_new_extension_node(path_nibbles, value)
            .unwrap()
    }

    fn get_sample_branch_node() -> Node {
        let branch_value_1 = hex::decode(
            "4f81663d4c7aeb115e49625430e3fa114445dc0a9ed73a7598a31cd60808a758"
        ).unwrap();
        let branch_value_2 = hex::decode(
            "d55a192f93e0576f46019553e2b4c0ff4b8de57cd73020f751aed18958e9ecdb"
        ).unwrap();
        let index_1 = 1;
        let index_2 = 2;
        let value = None;
        Node::get_new_branch_node(value)
            .and_then(|node| node.update_branch_at_index(Some(branch_value_1), index_1))
            .and_then(|node| node.update_branch_at_index(Some(branch_value_2), index_2))
            .unwrap()
    }

    fn get_sample_leaf_node_expected_encoding() -> Bytes {
        hex::decode("c9842012345683c0ffee")
            .unwrap()
    }

    fn get_sample_extension_node_expected_encoding() -> Bytes {
        hex::decode("e68400c0ffeea01d237c84432c78d82886cb7d6549c179ca51ebf3b324d2a3fa01af6a563a9377")
            .unwrap()
    }

    fn get_sample_branch_node_expected_encoding() -> Bytes {
        hex::decode("f85180a04f81663d4c7aeb115e49625430e3fa114445dc0a9ed73a7598a31cd60808a758a0d55a192f93e0576f46019553e2b4c0ff4b8de57cd73020f751aed18958e9ecdb8080808080808080808080808080")
            .unwrap()
    }

    fn get_sample_leaf_node_expected_hash() -> H256 {
        let hex = "c9161ce49c6a3362f5d20db4b6e36c259c9624eac5f99e64a052f45035d14c5d"
            .to_string();
        convert_hex_to_h256(hex)
            .unwrap()
    }

    fn get_sample_extension_node_expected_hash() -> H256 {
        let hex = "d1425391446456311990cdf61e1dbe92b14cb53caad0539a15564b9efac0f733"
            .to_string();
        convert_hex_to_h256(hex)
            .unwrap()
    }

    fn get_sample_branch_node_expected_hash() -> H256 {
        let hex = "9b88bb3372fcfde94cfbfd784ffcf64490a75bb2adedc128e67c887ce3d78535"
            .to_string();
        convert_hex_to_h256(hex)
            .unwrap()
    }

    #[test]
    fn should_get_new_leaf_node_correctly() {
        let panic_str = "Node should be a leaf node";
        let path_bytes = vec![0x12, 0x34, 0x56];
        let expected_nibble_length = path_bytes.clone().len() * 2;
        let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
        let value = hex::decode("c0ffee".to_string()).unwrap();
        let expected_encoded_path = encode_leaf_path_from_nibbles(path_nibbles.clone())
            .unwrap();
        let mut expected_raw = expected_encoded_path.clone();
        expected_raw.append(&mut value.clone());
        let result = Node::get_new_leaf_node(path_nibbles.clone(), value.clone())
            .unwrap();
        let node_type = result
            .clone()
            .get_type();
        assert!(node_type == LEAF_NODE_STRING);
        if let Some(_) = result.extension {
            panic!(panic_str)
        } else if let Some(_) = result.branch {
            panic!(panic_str)
        }
        match result.leaf {
            None => panic!(panic_str),
            Some(leaf) => {
                let nibble_length = get_length_in_nibbles(&leaf.path_nibbles.clone());
                assert!(leaf.value == value);
                assert!(leaf.raw == expected_raw);
                assert!(leaf.path_nibbles == path_nibbles);
                assert!(leaf.encoded_path == expected_encoded_path);
                assert!(nibble_length == expected_nibble_length)
            }
        }
    }

    #[test]
    fn should_rlp_encode_leaf_node_correctly() {
        let leaf_node = get_sample_leaf_node();
        let expected_result = get_sample_leaf_node_expected_encoding();
        let result = leaf_node
            .get_rlp_encoding()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_leaf_node_hash_correctly() {
        let leaf_node = get_sample_leaf_node();
        let expected_result = get_sample_leaf_node_expected_hash();
        let result = leaf_node
            .get_hash()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_extension_node_correctly() {
        let panic_str = "Node should be an extension node";
        let path_bytes = vec![0xc0, 0xff, 0xee];
        let expected_nibble_length = path_bytes.clone().len() * 2;
        let path_nibbles = get_nibbles_from_bytes(path_bytes);
        let value = hex::decode(
            "4aad98246efabf243441508dc0f328d80e83e9522e43709abab1c0c9cf4416dc".to_string()
        ).unwrap();
        let expected_encoded_path = encode_extension_path_from_nibbles(path_nibbles.clone())
            .unwrap();
        let result = Node::get_new_extension_node(path_nibbles.clone(), value.clone())
            .unwrap();
        let node_type = result
            .clone()
            .get_type();
        assert!(node_type == EXTENSION_NODE_STRING);
        let mut expected_raw = expected_encoded_path.clone();
        expected_raw.append(&mut value.clone());
        if let Some(_) = result.leaf {
            panic!(panic_str)
        } else if let Some(_) = result.branch {
            panic!(panic_str)
        }
        match result.extension {
            None => panic!(panic_str),
            Some(extension) => {
                let nibble_length = get_length_in_nibbles(&extension.path_nibbles.clone());
                assert!(extension.value == value);
                assert!(extension.raw == expected_raw);
                assert!(extension.path_nibbles == path_nibbles);
                assert!(extension.encoded_path == expected_encoded_path);
                assert!(nibble_length == expected_nibble_length)
            }
        }
    }

    #[test]
    fn should_rlp_encode_extension_node_correctly() {
        let extension_node = get_sample_extension_node();
        let expected_result = get_sample_extension_node_expected_encoding();
        let result = extension_node
            .get_rlp_encoding()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_extension_node_hash_correctly() {
        let extension_node = get_sample_extension_node();
        let expected_result = get_sample_extension_node_expected_hash();
        let result = extension_node
            .get_hash()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_new_branch_with_no_value_correctly() {
        let panic_str = "Node should be a branch node";
        let result = Node::get_new_branch_node(None).unwrap();
        if let Some(_) = result.extension {
            panic!(panic_str)
        } else if let Some(_) = result.leaf {
            panic!(panic_str)
        }
        let node_type = result
            .clone()
            .get_type();
        assert!(node_type == BRANCH_NODE_STRING);
        match result.branch {
            None => panic!(panic_str),
            Some(branch) => {
                if let Some(_) = branch.value {
                    panic!("Branch should not have a value!")
                };
                assert!(branch.branches == get_empty_child_nodes());
            }
        }
    }

    #[test]
    fn should_get_new_branch_with_value_correctly() {
        let panic_str = "Node should be a branch node";
        let value = hex::decode("c0ffee")
            .unwrap();
        let result = Node::get_new_branch_node(Some(value.clone())).unwrap();
        if let Some(_) = result.extension {
            panic!(panic_str)
        } else if let Some(_) = result.leaf {
            panic!(panic_str)
        }
        let node_type = result
            .clone()
            .get_type();
        assert!(node_type == "branch".to_string());
        match result.branch {
            None => panic!(panic_str),
            Some(branch) => {
                match branch.value {
                    Some(_value) => assert!(_value == value),
                    None => panic!("Branch should have a value!")
                }
                assert!(branch.branches == get_empty_child_nodes());
            }
        }
    }

    #[test]
    fn should_update_branch_at_index_correctly() {
        let index = 5;
        let value = None;
        let branch_value = hex::decode("c0ffee").unwrap();
        let branch_node = Node::get_new_branch_node(value).unwrap();
        assert!(
            branch_node
                .clone()
                .branch
                .unwrap()
                .branches[index] == None
        );
        let result = branch_node.update_branch_at_index(
            Some(branch_value.clone()),
            index,
        ).unwrap();
        assert!(
            result
                .clone()
                .branch
                .unwrap()
                .branches[index] == Some(branch_value)
        );
    }

    #[test]
    fn should_fail_to_update_branch_of_non_branch_node_correctly() {
        let expected_error = "✘ Cannot update branches - not a branch node!";
        let non_branch_node = get_sample_leaf_node();
        match non_branch_node.update_branch_at_index(None, 4) {
            Err(AppError::Custom(e)) => assert!(e == expected_error),
            _ => panic!("Did not receive expected error!")
        }
    }

    #[test]
    fn should_rlp_encode_branch_node_correctly() {
        let branch_node = get_sample_branch_node();
        let expected_result = get_sample_branch_node_expected_encoding();
        let result = branch_node
            .get_rlp_encoding()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_branch_node_hash_correctly() {
        let branch_node = get_sample_branch_node();
        let expected_result = get_sample_branch_node_expected_hash();
        let result = branch_node
            .get_hash()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_new_empty_node() {
        let empty = Node::get_new_empty_node().unwrap();
        let result = empty.get_type();
        assert!(result == EMPTY_NODE_STRING);
    }

    #[test]
    fn should_get_key_from_leaf_node() {
        let path_bytes = vec![0x12, 0x34, 0x56];
        let expected_result = get_nibbles_from_bytes(path_bytes.clone());
        let node = get_sample_leaf_node();
        let result = node.get_key();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_key_from_extension_node() {
        let node = get_sample_extension_node();
        let path_bytes = vec![0xc0, 0xff, 0xee];
        let expected_result = get_nibbles_from_bytes(path_bytes);
        let result = node.get_key();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_no_key_from_branch_node() {
        let node = get_sample_branch_node();
        let result = node.get_key();
        assert!(result == EMPTY_NIBBLES);
    }
}
