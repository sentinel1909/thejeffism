// data for the index template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct IndexContext {
    message: &'static str,
}

impl IndexContext {
    pub fn new() -> Self {
        IndexContext {
            message: "thejeffism",
        }
    }
}
