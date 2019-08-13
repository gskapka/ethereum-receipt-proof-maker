use hash_db::HashDB;
use crate::state::State;
use ethereum_types::H256;
use keccak_hasher::KeccakHasher;
use crate::types::{
    Bytes,
    Node,
    Result,
    Database,
};
use crate::constants::EMPTY_NODE;
use memory_db::{
    MemoryDB,
    HashKey,
};

pub fn get_new_database() -> Result<Database> {
    Ok(MemoryDB::<KeccakHasher, HashKey<_>, Bytes>::default())
}

pub fn put_thing_in_database(
    mut database: Database,
    node: Node,
    thing: Bytes
) -> Result<(H256, Database)> {
    let key = H256(database.insert(node, &thing));
    Ok((key, database))
}

pub fn remove_thing_from_database(
    mut database: Database,
    node: Node,
    key: H256,
) -> Result<Database> {
    database.remove(key.as_fixed_bytes(), node);
    Ok(database)
}

// TODO: Make state getter for DB so we don't have to clone it? Less pure but faster?
pub fn put_thing_in_database_in_state(
    state: State,
    node: Node,
    thing: Bytes
) -> Result<State> {
    put_thing_in_database(state.database.clone(), node, thing)
        .and_then(|(_, database)| State::update_database_in_state(state, database))
}

// TODO: Ibid!
pub fn remove_thing_from_database_in_state(
    state: State,
    node: Node,
    key: H256
) -> Result<State> {
    remove_thing_from_database(state.database.clone(), node, key)
        .and_then(|database| State::update_database_in_state(state, database))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_valid_initial_state,
        get_thing_to_put_in_database,
        get_expected_key_of_thing_in_database,
    };

    #[test]
    fn should_get_memory_database_instance() {
        get_new_database().unwrap();
    }

    #[test]
    fn should_insert_thing_in_database() {
        let database = get_new_database()
            .unwrap();
        let expected_result = get_thing_to_put_in_database();
        let (key, returned_database) = put_thing_in_database(
            database,
            EMPTY_NODE,
            expected_result.clone()
        ).unwrap();
        assert!(returned_database.contains(key.as_fixed_bytes(), EMPTY_NODE));
        let result = returned_database.get(key.as_fixed_bytes(), EMPTY_NODE);
        match result {
            None => panic!("Should be a result from database get fxn!"),
            Some(bytes) => assert!(bytes == expected_result)
        }
    }

    #[test]
    fn should_insert_thing_in_database_in_state() {
        let expected_key = get_expected_key_of_thing_in_database();
        let state = get_valid_initial_state()
            .unwrap();
        let expected_result = get_thing_to_put_in_database();
        let returned_state = put_thing_in_database_in_state(
            state,
            EMPTY_NODE,
            expected_result.clone()
        ).unwrap();
        assert!(returned_state
            .database
            .contains(
                expected_key.as_fixed_bytes(),
                EMPTY_NODE
            )
        );
        let result = returned_state
            .database
            .get(
                expected_key.as_fixed_bytes(),
                EMPTY_NODE
            );
        match result {
            None => panic!("Should be a result from database get fxn!"),
            Some(bytes) => assert!(bytes == expected_result)
        }
    }

    #[test]
    fn should_remove_thing_from_database() {
        let database = get_new_database()
            .unwrap();
        let expected_result = get_thing_to_put_in_database();
        let (key, returned_database) = put_thing_in_database(
            database,
            EMPTY_NODE,
            expected_result.clone()
        ).unwrap();
        assert!(returned_database.contains(key.as_fixed_bytes(), EMPTY_NODE));
        let final_database = remove_thing_from_database(
            returned_database,
            EMPTY_NODE,
            key
        ).unwrap();
        assert!(!final_database.contains(key.as_fixed_bytes(), EMPTY_NODE))
    }

    #[test]
    fn should_remove_thing_from_database_in_state() {
        let expected_key = get_expected_key_of_thing_in_database();
        let state = get_valid_initial_state()
            .unwrap();
        let expected_result = get_thing_to_put_in_database();
        let returned_state = put_thing_in_database_in_state(
            state,
            EMPTY_NODE,
            expected_result.clone()
        ).unwrap();
        assert!(returned_state
            .database
            .contains(
                expected_key.as_fixed_bytes(),
                EMPTY_NODE
            )
        );
        let final_state = remove_thing_from_database_in_state(
            returned_state,
            EMPTY_NODE,
            expected_key
        ).unwrap();
        assert!(!final_state
            .database
            .contains(
                expected_key.as_fixed_bytes(),
                EMPTY_NODE
            )
        );
    }
}
