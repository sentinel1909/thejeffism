// data for the about template

use serde::{Deserialize, Serialize};

use crate::time::get_current_year;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct AboutContext {
    content: &'static str,
    copyright_year: i32,
}

impl AboutContext {
    pub fn new() -> Self {
        let year = get_current_year();
        AboutContext {
            content: "Eventually, I will tell you all about me.",
            copyright_year: year,
        }
    }
}

impl Default for AboutContext {
    fn default() -> Self {
        Self::new()
    }
}
