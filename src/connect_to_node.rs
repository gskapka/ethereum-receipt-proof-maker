use crate::state::State;
use crate::types::Result;
use crate::get_block::get_block_by_number;

pub fn connect_to_node(state: State) -> Result<State> {
    if state.verbose { println!("\n✔ Connecting to node..."); }
    get_block_by_number(State::get_endpoint_from_state(&state)?, "latest")
        .and_then(|block| {
            if state.verbose {
                println!(
                    "✔ Connection successful! Latest block number: {:?}",
                    block.number
                );
            }
            Ok(state)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_dummy_initial_state;

    #[test]
    fn should_connect_to_node_and_return_state_when_endpoint_works() {
        let working_endpoint = "https://rpc.slock.it/mainnet".to_string();
        let state = State::set_endpoint_in_state(
            get_dummy_initial_state().unwrap(),
            working_endpoint
        ).unwrap();
        match connect_to_node(state) {
            Ok(returned_state) => assert!(returned_state.verbose),
            Err(_) => panic!("Should connect to node w/ working endpoint!")
        }
    }

    #[test]
    fn should_fail_to_connect_to_node_to_non_working_endpoint() {
        let non_working_endpoint = "non-working-endpoint".to_string();
        let state = State::set_endpoint_in_state(
            get_dummy_initial_state().unwrap(),
            non_working_endpoint
        ).unwrap();
        match connect_to_node(state) {
            Ok(_) => panic!("Should not connect to non-working endpoint!"),
            Err(_) => assert!(true)
        }
    }
}
