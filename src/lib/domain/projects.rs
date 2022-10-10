// data for the projects template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct ProjectsContext {
    content: &'static str,
}

impl ProjectsContext {
    pub fn new() -> Self {
        ProjectsContext {
            content: "The projects I'm working on..."
        }
    }
}