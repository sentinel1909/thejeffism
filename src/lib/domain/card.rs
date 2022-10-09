// data for the index template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CardContext {
    title: &'static str,
    date: &'static str,
    content: &'static str,
}

impl CardContext {
    pub fn new() -> Self {
        CardContext {
            title: "First Post",
            date: "2022-10-09",
            content: "Every site has to begin somewhere."
        }
    }
}
