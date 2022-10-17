// data for the projects template

use serde::{Deserialize, Serialize};

use crate::time::get_current_year;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct WritingContext {
    content: &'static str,
    copyright_year: i32,
}

impl WritingContext {
    pub fn new() -> Self {
        let year = get_current_year();
        WritingContext {
            content: "This page will feature other writing that I've done.",
            copyright_year: year,
        }
    }
}

impl Default for WritingContext {
    fn default() -> Self {
        Self::new()
    }
}
