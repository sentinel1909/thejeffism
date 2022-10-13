// data for the projects template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ProjectsContext {
    content: &'static str,
}

impl ProjectsContext {
    pub fn new() -> Self {
        ProjectsContext {
            content: "This site is my first major project with Rust. Will write more about it in the coming days.",
        }
    }
}

impl Default for ProjectsContext {
    fn default() -> Self {
        Self::new()
    }
}
