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

type ChildNodes = [Option<Bytes>; 16]; // TODO: Move to types?

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BranchNode {
    //pub raw: Bytes,
    pub value: Option<Bytes>,
    pub branches: ChildNodes,
    //pub encoded_path: Bytes,
    //pub path_nibbles: Nibbles,
}

impl Node {
    pub fn new_leaf(path_nibbles: Nibbles, value: Bytes) -> Result<Node> {
        let encoded_path = encode_leaf_path_from_nibbles(path_nibbles.clone())?;
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

    pub fn new_extension(path_nibbles: Nibbles, value: Bytes) -> Result<Node> {
        let encoded_path = encode_extension_path_from_nibbles(path_nibbles.clone())?;
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

    pub fn new_branch(value: Option<Bytes>) -> Result<Node> {
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

    pub fn update_branch_at_index(self, new_value: Option<Bytes>, index: usize) -> Result<Self> {
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

    pub fn rlp_encode(&self) -> Result<Bytes> {
        if let Some(leaf) = &self.leaf {
            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(2);
            rlp_stream.append(&leaf.encoded_path);
            rlp_stream.append(&leaf.value);
            Ok(rlp_stream.out())
        } else if let Some(extension) = &self.extension {
            let mut rlp_stream = RlpStream::new();
            rlp_stream.begin_list(2);
            rlp_stream.append(&extension.encoded_path);
            rlp_stream.append(&extension.value);
            Ok(rlp_stream.out())
        } else {
            Err(AppError::Custom(NO_NODE_IN_STRUCT_ERR.to_string()))
        }
    }

    pub fn hash(&self) -> Result<H256> {
        self.rlp_encode()
            .and_then(|encoded| keccak_hash_bytes(&encoded))
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
    use crate::nibble_utils::{
        Nibbles,
        get_length_in_nibbles,
    };
    use crate::utils::convert_hex_to_h256;

    fn get_sample_leaf_node() -> Node {
        let path_bytes = vec![0x12, 0x34, 0x56];
        let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
        let value = hex::decode("c0ffee".to_string()).unwrap();
        Node::new_leaf(path_nibbles, value)
            .unwrap()
    }

    fn get_sample_extension_node() -> Node {
        let path_bytes = vec![0xc0, 0xff, 0xee];
        let path_nibbles = get_nibbles_from_bytes(path_bytes);
        let value = hex::decode(
            "1d237c84432c78d82886cb7d6549c179ca51ebf3b324d2a3fa01af6a563a9377".to_string()
        ).unwrap();
        Node::new_extension(path_nibbles, value)
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
        let result = Node::new_leaf(path_nibbles.clone(), value.clone())
            .unwrap();
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
            .rlp_encode()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_leaf_node_hash_correctly() {
        let leaf_node = get_sample_leaf_node();
        let expected_result = get_sample_leaf_node_expected_hash();
        let result = leaf_node
            .hash()
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
        let result = Node::new_extension(path_nibbles.clone(), value.clone())
            .unwrap();
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
            .rlp_encode()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_extension_node_hash_correctly() {
        let extension_node = get_sample_extension_node();
        let expected_result = get_sample_extension_node_expected_hash();
        let result = extension_node
            .hash()
            .unwrap();
        assert!(result == expected_result);
    }

    #[test]
    fn should_get_new_branch_with_no_value_correctly() {
        let panic_str = "Node should be a branch node";
        let result = Node::new_branch(None).unwrap();
        if let Some(_) = result.extension {
            panic!(panic_str)
        } else if let Some(_) = result.leaf {
            panic!(panic_str)
        }
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
        let result = Node::new_branch(Some(value.clone())).unwrap();
        if let Some(_) = result.extension {
            panic!(panic_str)
        } else if let Some(_) = result.leaf {
            panic!(panic_str)
        }
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
        let branch_node = Node::new_branch(value).unwrap();
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


}
