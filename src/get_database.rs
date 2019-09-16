use ethereum_types::H256;
use crate::types::{
    Bytes,
    Result,
    Database,
};

pub fn get_new_database() -> Result<Database> {
    Ok(std::collections::HashMap::new())
}

pub fn put_thing_in_database(
    mut database: Database,
    key: H256,
    value: Bytes
) -> Result<Database> {
    database.insert(key, value);
    Ok(database)
}

pub fn remove_thing_from_database(
    mut database: Database,
    key: &H256
) -> Result<Database> {
    match database.remove(&key) {
        Some(_) => Ok(database),
        None => Ok(database)
    }
}


pub fn get_thing_from_database(
    database: &Database,
    key: &H256,
) -> Option<Bytes> {
    match database.get(&key) {
        Some(thing) => Some(thing.to_vec()),
        None => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_thing_to_put_in_database,
        get_database_with_thing_in_it,
        get_expected_key_of_thing_in_database,
    };

    #[test]
    fn should_get_new_empty_database() {
        let database = get_new_database()
            .unwrap();
        assert!(database.is_empty())
    }

    #[test]
    fn should_insert_thing_in_database() {
        let database = get_new_database()
            .unwrap();
        let expected_result = get_thing_to_put_in_database();
        put_thing_in_database(
            database,
            get_expected_key_of_thing_in_database(),
            expected_result.clone()
        ).unwrap();
    }

    #[test]
    fn should_get_thing_from_database() {
        let expected_thing = get_thing_to_put_in_database();
        let database = get_database_with_thing_in_it()
            .unwrap();
        let key = get_expected_key_of_thing_in_database();
        let result = get_thing_from_database(&database, &key)
            .unwrap();
        assert!(result == expected_thing);
    }

    #[test]
    fn should_remove_thing_from_database() {
        let key = get_expected_key_of_thing_in_database();
        let database = get_new_database()
            .unwrap();
        let updated_database = put_thing_in_database(
            database,
            key.clone(),
            get_thing_to_put_in_database(),
        ).unwrap();
        assert!(updated_database.contains_key(&key));
        let result = remove_thing_from_database(
            updated_database,
            &key,
        ).unwrap();
        assert!(!result.contains_key(&key));
    }
}
