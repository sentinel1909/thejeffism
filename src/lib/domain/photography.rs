// data for the photography template

use serde::{Deserialize, Serialize};

use crate::time::get_current_year;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PhotographyContext {
    content: &'static str,
    copyright_year: i32,
}

impl PhotographyContext {
    pub fn new() -> Self {
        let year = get_current_year();
        PhotographyContext {
            content: "This is my photography page...",
            copyright_year: year,
        }
    }
}

impl Default for PhotographyContext {
    fn default() -> Self {
        Self::new()
    }
}
