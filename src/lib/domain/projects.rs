// data for the projects template

use serde::{Deserialize, Serialize};

use crate::time::get_current_year;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ProjectsContext {
    content: &'static str,
    copyright_year: i32,
}

impl ProjectsContext {
    pub fn new() -> Self {
        let year = get_current_year();
        ProjectsContext {
            content: "This site is my first major project with Rust. Will write more about it in the coming days.",
            copyright_year: year,
        }
    }
}

impl Default for ProjectsContext {
    fn default() -> Self {
        Self::new()
    }
}
