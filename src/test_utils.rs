#![cfg(test)]
#![allow(unused_imports)]

use std::fs;
use crate::state::State;
use ethereum_types::H256;
use crate::trie_nodes::Node;
use crate::nibble_utils::get_nibbles_from_bytes;
use crate::get_block::deserialize_block_json_to_block_struct;
use crate::get_receipt::deserialize_receipt_json_to_receipt_struct;
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
    put_thing_in_database
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

pub const WORKING_ENDPOINT: &str = "http://localhost:8545";

pub const SAMPLE_BLOCK_JSON_PATH: &str = "./test_utils/sample_block_json";

pub const SAMPLE_RECEIPT_JSON_PATH: &str = "./test_utils/sample_receipt_json";

pub const SAMPLE_RECEIPT_JSONS_PATH: &str = "./test_utils/sample_receipt_jsons/";

pub const SAMPLE_RECECIPT_TX_HASHES: [&str; 25] = [
    "0xee6b2afff6a61686199965dd64d56ec613213b48bb4620e71e0176a881d3b0dc",
    "0xf2df2d51c0b5187e32363ec5dbcfe2e0bb8b8cb70a6708ffc0095d9db53ffda9",
    "0xab8078c9aa8720c5f9206bd2673f25f359d8a01b62212da99ff3b53c1ca3d440",
    "0x0ab2a8d425c3a55855717ce37b0831f644ae8afe496b269b347690ab4f393e3e",
    "0x5af4923b95627fdc57c6573d16e6fa0df716a98063a1027d9733e3eed2cbc24b",
    "0x93c8c513ad5a3eed0150166861c76010254efedbe4951ccb4d02f81cc0f85369",
    "0xe35e3b404ccd568df46ed52ce421998b83063ee1ee1420b36a90288121d5dcc1",
    "0xcdc5a5c943c62a489a04045dbe0e10eda34e3a7162ca6fb0e618b6590ca72ae1",
    "0xe805f3c56e99d3dbbf3bc0fd93f440fd8c9dae1f7876153f96449da523ea21f0",
    "0x4250ff983d0907f560003873c6a916e319a85a111f26127fb2ad459a296e0ce8",
    "0x8cedbb955a7c090ea993591ea541adfe1383f3b2391b74526ef481729b32aa7f",
    "0x8bbcf4950d5924a739114ca0c2bc6f2be118651ccd0dc9028f74f500198ecc06",
    "0x5f023c49e60c14763f5fe72cf6df2666aa4d311e6897ce408301a7246dc17bda",
    "0xbbebd7bbb8797b8790e4f91a0ee49080c4456b8f95c27af8562f70dda40be67a",
    "0x640cb533d56a7e215c6a81aa1cf988c1e7ba479e70a571b974fa811ab2d41796",
    "0xa067162103a794e23234844ff4c8951853488cbafb3e138df2a8ce24968fd394",
    "0xf9ca12a74c3454fcf7e23f5287a057c3605e2aec13fee03a3e03b4774b5faf38",
    "0x20d2a35a89b01589489f142f4881acf8e419308f99c30c791a1bb1f3035b949e",
    "0x40a07797beb2b5247a832e62deff7b631f415a5e6c559eae621d40bc7c33e8bd",
    "0x852cce56dcd2d00c22fab9143d59e5e2a547f0d3390e500f351124b922e7903d",
    "0x164207a34902693be57ccc4b6c2860eb781db2aba1a6e2ed93473a9dd516a542",
    "0x9b8063fe52a38566d5279e8ee9fa3c23c17557b339ea55a7ea1100b44f436434",
    "0x5272da6bc5a763d93e2023a1cd80ad97a112d4a8af0e8e0629c5e7d6e5eddb9d",
    "0x4d2c712ffbc54f8970a4377c03cc7ca8b6d58f8af2181282954b9b16f860cda2",
    "0x49b980475527f989936ddc8afd1e045612cd567238bb567dbd99b48ad15860dc",
];

pub const TRIE_ROOTS_AS_RECEIPTS_ADDED: [&str; 25] = [
    "0x056b23fbba480696b65fe5a59b8f2148a1299103c4f57df839233af2cf4ca2d2",
    "0xd95b673818fa493deec414e01e610d97ee287c9421c8eff4102b1647c1a184e4",
    "0xe3bb586a5131b6981306066443ff124f39a286ae5245e7d8f8b44ceab6ec97d8",
    "0x6c3b482de04166e0bfe33b8b0610bc364a0b4ae443aec117a4fa404d43889ae0",
    "0x8677cc8a49e122ab080d151cfaef160ddc7cdd9335a13933b5eb1bc2272e62f9",
    "0x74b986b96dd1cfe4e6d09f39e5ac0acae0dd4a1b2c1c381df393e0189b5ea98f",
    "0x6871131e7e0e36f6f066dab6f98cf94afd7052d7627cd10334fbd37a9b819348",
    "0x28f747d31d5a8306077ea741399733b9f990d1adfaf9d32fdc8c4565f9972b4b",
    "0xca5cd46d084f6a63f70bc7c1d5599e27663b987d42167ac75a33ddffd587b552",
    "0xbb492016685e8c0d3c37315179aa5638751888e403917ce32261904ca0d240e1",
    "0x359c3d18443fd2034af0cd4ace7102744f55e6b14b4b792298354e6c4b6e383b",
    "0xf952addc41512328019e67336654beb5e1aedc645fabf446b9b335b91a183fce",
    "0x42807e3c9b647e07ea2d936ebf073c028e403de100bfc9fe2a1cdbf48affceef",
    "0x3485a3111458fbd7937b9157ab5e648a5ab834884ad517d8273c234d1a0e4778",
    "0x334e75c4f3dfaf7b8ee9c5c7a0e3b2d3670dcc3a104fd2eb8a6b0e68b4bb41db",
    "0x83fc3d1a4bcd51c9da701579402e7ec2fe80e5ca442a03e72bda961f64635858",
    "0xd77c74d1f3f1ea49b67081ac8b08e87db224aa01afcd43abfa22ab9ea5d7b417",
    "0xd527a26793636e6e0c111dd7c838913bf899ea17159644aed5dda24644836d45",
    "0x4744360a8f2313bd5bcbad7b8ecf46e4b52f5309bf6f41db75b90b98a65d9a32",
    "0x487f5eea40fb04c56d1d5e29ca9aae0bc635d1571501fb612bc301b44446c31b",
    "0xbfbf8d438b60b4e7c658c62ad2ef5953b9495bff40dae9240e6d06b647a2a61d",
    "0x150f0532393cfd782327fef7a8de23d83aedd5e7ffc0fe8dd63b59e5c16d7838",
    "0x65974b8374dc3713c05cb59445cb3601c9da051ed4aae7873c2d2c40f18d6e6c",
    "0xf16fb14b6203337434e31002ce32316b74ea2dd764c3b4ad1bd22b7151a14e55",
    "0x937e08f03388b32d7c776e7a02371b930d71e3ec096d495230b6735e7f9b20ae",
];

pub const RECEIPTS_ROOT : &str = TRIE_ROOTS_AS_RECEIPTS_ADDED[24];

pub const SAMPLE_TX_HASH: &str = "0xd6f577a93332e015438fcca4e73f538b1829acbd7eb0cf9ee5a0a73ff2752cc6";

pub const SAMPLE_BLOCK_HASH: &str = "0x1ddd540f36ea0ed23e732c1709a46c31ba047b98f1d99e623f1644154311fe10";

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
