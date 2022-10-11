// data for the about template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AboutContext {
    content: &'static str,
}

impl AboutContext {
    pub fn new() -> Self {
        AboutContext {
            content: "Eventually, I will tell you all about me.",
        }
    }
}

impl Default for AboutContext {
    fn default() -> Self {
        Self::new()
    }
}
