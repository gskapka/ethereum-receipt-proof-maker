#![cfg(test)]
#![allow(unused_imports)]

use std::fs;
use crate::state::State;
use ethereum_types::H256;
use crate::trie_nodes::Node;
use crate::nibble_utils::get_nibbles_from_bytes;
use crate::get_block::deserialize_block_json_to_block_struct;
use crate::rlp_codec::get_rlp_encoded_receipts_and_nibble_tuples;
use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
use crate::get_branch_from_trie::get_branch_from_trie_and_put_in_state;
use crate::trie::{
    Trie,
    put_in_trie_recursively
};
use crate::utils::{
    convert_hex_to_h256,
    convert_h256_to_prefixed_hex,
};
use crate::make_rpc_call::{
    deserialize_to_block_rpc_response,
    deserialize_to_receipt_rpc_response,
};
use crate::constants::{
    DOT_ENV_PATH,
    DEFAULT_ENDPOINT,
};
use crate::get_database::{
    put_thing_in_database,
};
use crate::types::{
    Log,
    Block,
    Bytes,
    Result,
    Receipt,
    Database,
};

pub const TX_INDEX: usize = 96;

pub const PROOF_INDEX: usize = 14;

pub const WORKING_ENDPOINT: &str = "http://localhost:8545";

pub const SAMPLE_BLOCK_JSON_PATH: &str = "./test_utils/sample_block_json";

pub const SAMPLE_RECEIPT_JSON_PATH: &str = "./test_utils/sample_receipt_json";

pub const SAMPLE_RECEIPT_JSONS_1_PATH: &str = "./test_utils/sample_receipt_jsons_1/";

pub const SAMPLE_RECEIPT_JSONS_2_PATH: &str = "./test_utils/sample_receipt_jsons_2/";

pub fn get_sample_tx_hashes_1() -> Vec<String> {
        vec![
        "0xee6b2afff6a61686199965dd64d56ec613213b48bb4620e71e0176a881d3b0dc"
            .to_string(),
        "0xf2df2d51c0b5187e32363ec5dbcfe2e0bb8b8cb70a6708ffc0095d9db53ffda9"
            .to_string(),
        "0xab8078c9aa8720c5f9206bd2673f25f359d8a01b62212da99ff3b53c1ca3d440"
            .to_string(),
        "0x0ab2a8d425c3a55855717ce37b0831f644ae8afe496b269b347690ab4f393e3e"
            .to_string(),
        "0x5af4923b95627fdc57c6573d16e6fa0df716a98063a1027d9733e3eed2cbc24b"
            .to_string(),
        "0x93c8c513ad5a3eed0150166861c76010254efedbe4951ccb4d02f81cc0f85369"
            .to_string(),
        "0xe35e3b404ccd568df46ed52ce421998b83063ee1ee1420b36a90288121d5dcc1"
            .to_string(),
        "0xcdc5a5c943c62a489a04045dbe0e10eda34e3a7162ca6fb0e618b6590ca72ae1"
            .to_string(),
        "0xe805f3c56e99d3dbbf3bc0fd93f440fd8c9dae1f7876153f96449da523ea21f0"
            .to_string(),
        "0x4250ff983d0907f560003873c6a916e319a85a111f26127fb2ad459a296e0ce8"
            .to_string(),
        "0x8cedbb955a7c090ea993591ea541adfe1383f3b2391b74526ef481729b32aa7f"
            .to_string(),
        "0x8bbcf4950d5924a739114ca0c2bc6f2be118651ccd0dc9028f74f500198ecc06"
            .to_string(),
        "0x5f023c49e60c14763f5fe72cf6df2666aa4d311e6897ce408301a7246dc17bda"
            .to_string(),
        "0xbbebd7bbb8797b8790e4f91a0ee49080c4456b8f95c27af8562f70dda40be67a"
            .to_string(),
        "0x640cb533d56a7e215c6a81aa1cf988c1e7ba479e70a571b974fa811ab2d41796"
            .to_string(),
        "0xa067162103a794e23234844ff4c8951853488cbafb3e138df2a8ce24968fd394"
            .to_string(),
        "0xf9ca12a74c3454fcf7e23f5287a057c3605e2aec13fee03a3e03b4774b5faf38"
            .to_string(),
        "0x20d2a35a89b01589489f142f4881acf8e419308f99c30c791a1bb1f3035b949e"
            .to_string(),
        "0x40a07797beb2b5247a832e62deff7b631f415a5e6c559eae621d40bc7c33e8bd"
            .to_string(),
        "0x852cce56dcd2d00c22fab9143d59e5e2a547f0d3390e500f351124b922e7903d"
            .to_string(),
        "0x164207a34902693be57ccc4b6c2860eb781db2aba1a6e2ed93473a9dd516a542"
            .to_string(),
        "0x9b8063fe52a38566d5279e8ee9fa3c23c17557b339ea55a7ea1100b44f436434"
            .to_string(),
        "0x5272da6bc5a763d93e2023a1cd80ad97a112d4a8af0e8e0629c5e7d6e5eddb9d"
            .to_string(),
        "0x4d2c712ffbc54f8970a4377c03cc7ca8b6d58f8af2181282954b9b16f860cda2"
            .to_string(),
        "0x49b980475527f989936ddc8afd1e045612cd567238bb567dbd99b48ad15860dc"
            .to_string(),
    ]
}

pub const RECEIPTS_ROOT_1 : &str = "0x937e08f03388b32d7c776e7a02371b930d71e3ec096d495230b6735e7f9b20ae";

pub const RECEIPTS_ROOT_2 : &str = "0x4c9bb7d6a6c74445c15e5915262c49c69cd14b3e19620302f2c10303fef1e392";

pub const SAMPLE_TX_HASH: &str = "0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6";

pub const SAMPLE_BLOCK_HASH: &str = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

pub fn get_sample_receipts(tx_hashes: Vec<String>) -> Vec<Receipt> {
    tx_hashes
        .iter()
        .map(|hash_string|
             format!("{}{}", SAMPLE_RECEIPT_JSONS_1_PATH, hash_string)
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

pub fn get_sample_trie_with_sample_receipts(tx_hashes: Vec<String>) -> Trie {
    let index = 0;
    let receipts = get_sample_receipts(tx_hashes);
    let trie = Trie::get_new_trie().unwrap();
    let key_value_tuples = get_rlp_encoded_receipts_and_nibble_tuples(
        &receipts
    ).unwrap();
    put_in_trie_recursively(
        trie,
        key_value_tuples,
        index
    ).unwrap()
}

pub fn get_sample_leaf_node() -> Node {
    let path_bytes = vec![0x12, 0x34, 0x56];
    let path_nibbles = get_nibbles_from_bytes(path_bytes.clone());
    let value = hex::decode("c0ffee".to_string()).unwrap();
    Node::get_new_leaf_node(path_nibbles, value)
        .unwrap()
}

pub fn get_sample_extension_node() -> Node {
    let path_bytes = vec![0xc0, 0xff, 0xee];
    let path_nibbles = get_nibbles_from_bytes(path_bytes);
    let value = hex::decode(
        "1d237c84432c78d82886cb7d6549c179ca51ebf3b324d2a3fa01af6a563a9377".to_string()
    ).unwrap();
    Node::get_new_extension_node(path_nibbles, value)
        .unwrap()
}

pub fn get_sample_branch_node() -> Node {
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

pub fn get_thing_to_put_in_database() -> Bytes {
    "Provable".as_bytes().to_owned()
}

pub fn get_expected_key_of_thing_in_database() -> H256 {
    H256::zero()
}

pub fn get_valid_tx_hash_hex() -> String {
    SAMPLE_TX_HASH.to_string()
}

pub fn get_valid_block_hash_hex() -> String {
    SAMPLE_BLOCK_HASH.to_string()
}

pub fn get_valid_block_hash_h256() -> Result<H256> {
    convert_hex_to_h256(get_valid_block_hash_hex())
}

pub fn get_valid_tx_hash_h256() -> Result<H256> {
     convert_hex_to_h256(get_valid_tx_hash_hex())
}

pub fn get_valid_initial_state() -> Result<State> {
    State::init(
        get_valid_tx_hash_h256()?,
        get_valid_tx_hash_hex(),
        true
    )
}

pub fn get_valid_state_with_endpoint() -> Result<State> {
    get_valid_initial_state()
        .and_then(|state|
            State::set_endpoint_in_state(state, WORKING_ENDPOINT.to_string())
        )
}

pub fn get_valid_state_with_receipts_trie_and_index(
    tx_hashes: Vec<String>
) -> Result<State> {
    get_valid_initial_state()
        .and_then(|state| state.set_index_in_state(14))
        .and_then(|state|
            state.set_receipts_trie_in_state(
                get_sample_trie_with_sample_receipts(tx_hashes)
            )
        )
}

pub fn get_valid_state_with_receipts_trie_index_and_branch(
    tx_hashes: Vec<String>
) -> Result<State> {
    get_valid_state_with_receipts_trie_and_index(tx_hashes)
        .and_then(get_branch_from_trie_and_put_in_state)
}

pub fn get_expected_block() -> Block {
    let string = fs::read_to_string(SAMPLE_BLOCK_JSON_PATH).unwrap();
    let res = deserialize_to_block_rpc_response(string).unwrap();
    deserialize_block_json_to_block_struct(res.result).unwrap()
}

pub fn get_expected_receipt() -> Receipt {
    let string = fs::read_to_string(SAMPLE_RECEIPT_JSON_PATH).unwrap();
    let res = deserialize_to_receipt_rpc_response(string).unwrap();
    deserialize_receipt_json_to_receipt_struct(res.result).unwrap()
}

pub fn get_expected_log() -> Log {
    // TODO: Implement == for logs!
    get_expected_receipt().logs[0].clone()
}

pub fn assert_log_is_correct(log: Log) {
    let sample_log = get_expected_log();
    assert!(sample_log.address == log.address);
    assert!(sample_log.topics.len() == log.topics.len());
    assert!(sample_log.data == log.data);
}

pub fn assert_block_is_correct(block: Block) {
    // TODO: Implement == for blocks!
    let sample_block = get_expected_block();
    assert!(block.author == sample_block.author);
    assert!(block.difficulty == sample_block.difficulty);
    assert!(block.extra_data == sample_block.extra_data);
    assert!(block.gas_limit == sample_block.gas_limit);
    assert!(block.gas_used == sample_block.gas_used);
    assert!(block.hash == sample_block.hash);
    assert!(block.miner == sample_block.miner);
    assert!(block.mix_hash == sample_block.mix_hash);
    assert!(block.nonce == sample_block.nonce);
    assert!(block.number == sample_block.number);
    assert!(block.parent_hash == sample_block.parent_hash);
    assert!(block.receipts_root == sample_block.receipts_root);
    assert!(block.seal_fields.0 == sample_block.seal_fields.0);
    assert!(block.seal_fields.1 == sample_block.seal_fields.1);
    assert!(block.sha3_uncles == sample_block.sha3_uncles);
    assert!(block.size == sample_block.size);
    assert!(block.state_root == sample_block.state_root);
    assert!(block.timestamp == sample_block.timestamp);
    assert!(block.total_difficulty == sample_block.total_difficulty);
    assert!(block.transactions.len() == sample_block.transactions.len());
    assert!(block.transactions_root == sample_block.transactions_root);
    assert!(block.uncles.len() == sample_block.uncles.len());
}

pub fn assert_receipt_is_correct(receipt: Receipt) {
    // TODO: Implement == for receipts
    let sample_receipt = get_expected_receipt();
    assert!(receipt.to == sample_receipt.to);
    assert!(receipt.from == sample_receipt.from);
    assert!(receipt.status == sample_receipt.status);
    assert!(receipt.block_hash == sample_receipt.block_hash);
    assert!(receipt.transaction_hash == sample_receipt.transaction_hash);
    assert!(receipt.cumulative_gas_used == sample_receipt.cumulative_gas_used);
    assert!(receipt.block_number == sample_receipt.block_number);
    assert!(receipt.transaction_index == sample_receipt.transaction_index);
    assert!(receipt.contract_address == sample_receipt.contract_address);
    assert!(receipt.root == sample_receipt.root);
    assert!(receipt.logs.len() == sample_receipt.logs.len());
}

pub fn read_env_file() -> Result<String> {
    Ok(fs::read_to_string(&DOT_ENV_PATH)?)
}

pub fn write_env_file(endpoint_url: Option<&str>) -> Result<()> {
    let url = endpoint_url.unwrap_or(DEFAULT_ENDPOINT);
    let data = format!("ENDPOINT=\"{}\"", url);
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn delete_env_file() -> Result<()> {
    Ok(fs::remove_file(&DOT_ENV_PATH)?)
}

pub fn restore_env_file(data: String) -> Result<()> {
    Ok(fs::write(&DOT_ENV_PATH, data)?)
}

pub fn get_database_with_thing_in_it() -> Result<Database> {
    let mut database: Database = std::collections::HashMap::new();
    database.insert(
        get_expected_key_of_thing_in_database(),
        "Provable".as_bytes().to_owned()
    );
    Ok(database)
}

pub fn get_sample_proof() -> String {
"f91626f871a0fb5e0d429924a0287196102cda8544cfbbb0949d7ae9e6a2ebbdfe4f6e3c94eba0441d343ee56af21fb1a8e12802f9b91f415cb8dd5dcb47f880c4171846e11d69808080808080a0e58215be848c1293dd381210359d84485553000a82b67410406d183b42adbbdd8080808080808080f901f180a0d9673d2d9fc051cd0c137eb6e8e6fa792ca02465188c3408c86625d76f6396a0a068a36b8233852762f709f47f3bf8f49f7617b2768f432c4c53901158466f32cba0a2cb63340b3750a6f8e423d841f0f6ecf1ee905e2c5429215f90abab8cf981c4a0d5400a85346120207d40c40c706329212cad6256d6ac3c98b0a608382b94e49ba0086498ee145780c9a471de9095b478ca1555985afb5af85e9417240c42d1761ca0f919c75704da25ee06c61ec00a09b45e96233400846e210f1556e05e69b4026fa0ff852e16453c79cd5cb47453cc4b813f67ceeec2fba52f8493f34ce4e11f36dfa0e28a542ee13340426eae878242c5f6ae00d32d8428520d0e4d43c941c8501b5ca0690514b2df03293f0a4af4c5c1f7f4e14f597498fcafaef0294f7d9275fafb93a0fe6a59f583d64752de4922fa78290d52214d53dfdb8398ea622458045d1bf790a0b8c0730f4a260ecaff702f95300731b3b0bde28caf3591eb56c376e173655179a0c1fc1ea14cf024f0c58ae16906be6f3db05cee425ebfaded1402b248e979cecaa047b8d8cd77fbba9406f8be39009cd3de8681294ac06321d81ecefcabe2a50f5fa015cabab29394775f3844179f7200ac6f368a89a819ca370b29ef4e01fe1bd5f5a0fbfb53995b5a638a7f32d344fc04d26db19c1e0953d2b3128da43f9343c1e23080f913bc20b913b8f913b50183717d1db90100000000000000000000100000020000042000328000000002000000800000000000000000000100000002200008800000120000880804000040008081002000000000000000000000480004084008000080000404004000400000000880420000000200000220040000000000000809200000000000810400000000100000000000000000401000000000000040020400002000010100000000000000000000000200000000002000000000001040000000000104000000000002200000200110000500028902000000000000000020000000000000020008000800020000200000302000000a2000088002000000000100040000000000400400200040100000f912aaf87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a0e1fffcc4923d04b559f4d29a8bfc6cda04eb5b0d3c460751c2402c5c5cc9109ca0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da000000000000000000000000000000000000000000000000000be1571569ebfdaf89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a07fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df87a949ae49c0d7f8f9ef4b864e004fe86ac8294e20950f842a075f33ed68675112c77094e7c5b073890598be1d23e27cd7f6907b4a7d98ac619a000000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b60495a00000000000000000000000000000000000000000000000001ee7fa454bdf5b6df8fb9457f8160e1c59d16c01bbe181fd94db4e56b60495f842a0ea9415385bae08fe9f6dc457b02577166790cde83bb18cc340aac6cb81b824dea00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950b8a0000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000001ee7fa454bdf5b6d000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d0000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950f89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa000000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8fa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a000000000000000000000000000000000000000000000000000000000172b812cf8fb9463825c174ab367968ec60f061753d3bbd36a0d8ff842a0ea9415385bae08fe9f6dc457b02577166790cde83bb18cc340aac6cb81b824dea00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950b8a0000000000000000000000000eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000000000000000000000000000000000000172b812c0000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950f89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009ae49c0d7f8f9ef4b864e004fe86ac8294e20950a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da000000000000000000000000000000000000000000000000000000000172b812cf8799452166528fcc12681af996e409ee3a421a4e128a3e1a0f838f6ddc89706878e3c3e698e9b5cbfbf2c0e3d3dcd0bd2e00f1ccf313e0185b84000000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8f0000000000000000000000000000000000000000000000006261b3899e23cbe6f9019c949ae49c0d7f8f9ef4b864e004fe86ac8294e20950f842a0d30ca399cb43507ecec6a629a35cf45eb98cda550c27696dcb0d8c4a3873ce6ca00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56db90140000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000000000000000000000000000000000000172b812c0000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56d0000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000057f8160e1c59d16c01bbe181fd94db4e56b6049500000000000000000000000063825c174ab367968ec60f061753d3bbd36a0d8f00000000000000000000000000000000000000000000000000000000000001200000000000000000000000000000000000000000000000000000000000000000f8db94818e6fecd516ecc3849daf6845e3ec868087b755f842a01849bd6a030a1bca28b83437fd3de96f3d27a5d172fa7e9c78e7b61468928a39a00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56db880000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000001ee7fa454bdf5b6d00000000000000000000000000000000000000000000000000000000172b812cf89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000000be1571569ebfdaf89b94a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48f863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000009b3eb3b22dc2c29e878d7766276a86a8395fb56da00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca000000000000000000000000000000000000000000000000000000000172b812cf89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000011927d1af6564000f87994f55186cc537e7067ea616f2aae007b4427a120c8e1a09c2c6ec1cb8ee2fe8d5549d7d071a1a8f76ec3cc057d7c46f118247b0e5e8572b840000000000000000000000000d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91c00000000000000000000000000000000000000000000000011927d1af6564000f9015c9473df03b5436c84cf9d5a758fb756928dceaf19d7f842a0c7fce5271a7dcbf20bd48128dcbf6f2df01bceda67919e43870de3be7f1b0690a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392bb90100000000000000000000000000d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91c00000000000000000000000000000000000000000000000011927d1af6564000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000001fa60fb6a27e1b47f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca000000000000000000000000000000000000000000000000011927d1af6541593f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba0000000000000000000000000882d80d3a191859d64477eb78cca46599307ec1ca0fffffffffffffffffffffffffffffffffffffffffffffff67d19c841ccf2e46cf89b949ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951df863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000005be139fa43fdc0e583ac0e4fab48e5e451fa6575a00000000000000000000000000000000000000000000000001460fce85296abc0f87994f55186cc537e7067ea616f2aae007b4427a120c8e1a09c2c6ec1cb8ee2fe8d5549d7d071a1a8f76ec3cc057d7c46f118247b0e5e8572b8400000000000000000000000009ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951d0000000000000000000000000000000000000000000000001460fce85296abc0f89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a0ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3efa0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba00000000000000000000000005b67871c3a857de81a1ca0f9f7945e5670d986dca00000000000000000000000000000000000000000000000000000000000022a6df89b94d14d4e7eb9b36ae1ac0efd5e0833bf517eafd91cf863a08c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba0000000000000000000000000882d80d3a191859d64477eb78cca46599307ec1ca0fffffffffffffffffffffffffffffffffffffffffffffff67d19c841ccf0b9fff87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f842a07fcf532c15f0a6db0bd6d0e038bea71d30d808c7d98cb3bf7268a95bf5081b65a0000000000000000000000000d4240987d6f92b06c8b5068b1e4006a97c47392ba000000000000000000000000000000000000000000000000000be1571569ebfdaf8dc94d4240987d6f92b06c8b5068b1e4006a97c47392bf863a000293d5012632fad25e327fa894460c60bef74241d2f04c42802f4b2212f66aaa00000000000000000000000009ea463ec4ce9e9e5bc9cfd0187c4ac3a70dd951da00000000000000000000000005be139fa43fdc0e583ac0e4fab48e5e451fa6575b860000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc20000000000000000000000000000000000000000000000001460fce85296abc000000000000000000000000000000000000000000000000000be1571569ebfda".to_string()
}

mod tests {
    use hex;
    use std::fs;
    use super::*;
    use crate::state::State;
    use crate::errors::AppError;
    use crate::validate_tx_hash::validate_tx_hash;
    use crate::utils::{
        dot_env_file_exists,
        get_not_in_state_err,
    };

    #[test]
    fn should_get_expected_block_correctly() {
        let result = get_expected_block();
        assert_block_is_correct(result);
    }

    #[test]
    fn should_get_expected_log_correctly() {
        let result = get_expected_log();
        assert_log_is_correct(result);
    }

    #[test]
    fn should_get_expected_receipt_correctly() {
        let result = get_expected_receipt();
        assert_receipt_is_correct(result);
    }

    #[test]
    fn should_get_valid_tx_hash_as_hex() {
        let result = get_valid_tx_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex tx hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_block_hash_as_hex() {
        let result = get_valid_block_hash_hex();
        match validate_tx_hash(result) {
            Ok(_) => assert!(true),
            Err(_) => panic!("Hex block hash should be valid!")
        }
    }

    #[test]
    fn should_get_valid_tx_hash_as_h256() {
        let result = get_valid_tx_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_tx_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_block_hash_as_h256() {
        let result = get_valid_block_hash_h256()
            .unwrap();
        let result_bytes = result.as_bytes();
        let result_hex = format!("0x{}", hex::encode(result_bytes));
        let expected_result = get_valid_block_hash_hex();
        assert!(result_hex == expected_result)
    }

    #[test]
    fn should_get_valid_intial_state_correctly() {
        let expected_verbosity = true;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_initial_state()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        assert!(result.verbose == expected_verbosity);
        match State::get_endpoint_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("endpoint")),
            _ => panic!("Intial state should not have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }

    #[test]
    fn should_get_valid_state_with_endpoint_correctly() {
        let expected_verbosity = true;
        let expected_endpoint = WORKING_ENDPOINT;
        let expected_tx_hash = get_valid_tx_hash_h256()
            .unwrap();
        let result = get_valid_state_with_endpoint()
            .unwrap();
        assert!(result.tx_hash == expected_tx_hash);
        assert!(result.verbose == expected_verbosity);
        match State::get_endpoint_from_state(&result) {
            Ok(endpoint) => assert!(endpoint == expected_endpoint),
            _ => panic!("Intial w/ endpoint should have endpoint set!")
        }
        match State::get_block_from_state(&result) {
            Err(AppError::Custom(e)) =>
                assert!(e == get_not_in_state_err("block")),
            _ => panic!("Intial state should not have endpoint set!")
        }
    }

    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            assert!(file == original_file);
        } else {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_read_existing_env_file_correctly() {
        if dot_env_file_exists() {
            let file = read_env_file().unwrap();
            assert!(file.contains("ENDPOINT"))
        }
    }


    #[test]
    #[serial]
    fn should_delete_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            restore_env_file(original_file.clone()).unwrap();
            let file = read_env_file().unwrap();
            assert!(file == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_write_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_exists() {
        if dot_env_file_exists() {
            let original_file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(original_file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == original_file)
        }
    }

    #[test]
    #[serial]
    fn should_restore_env_file_correctly_if_it_does_not_exist() {
        if !dot_env_file_exists() {
            write_env_file(None).unwrap();
            assert!(dot_env_file_exists());
            let file = read_env_file().unwrap();
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
            restore_env_file(file.clone()).unwrap();
            assert!(dot_env_file_exists());
            let result = read_env_file().unwrap();
            assert!(result == file);
            delete_env_file().unwrap();
            assert!(!dot_env_file_exists());
        }
    }
}
