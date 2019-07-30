use std::result;
use crate::types::Block;
use crate::errors::AppError;

type Result<T> = result::Result<T, AppError>;

pub struct State {
    block: Option<Block>,
    endpoint: Option<String>,
}

impl State {
    pub fn get_initial_state(self) -> Result<State> {
        Ok(
            State {
                block: None,
                endpoint: None,
            }
        )
    }

    pub fn add_block_to_state(mut self, block: Block) -> Result<State> {
        self.block = Some(block);
        Ok(self)
    }

    pub fn add_endpoint_to_state(mut self, endpoint: String) -> Result<State> {
        self.endpoint= Some(endpoint);
        Ok(self)
    }

    pub fn get_block_from_state(self) -> Result<Block> {
        Ok(self.block?)
    }

    pub fn get_endpoint_from_state(self) -> Result<String> {
        Ok(self.endpoint?)
    }
}
