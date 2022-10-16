// data for the music template

use serde::{Deserialize, Serialize};

use crate::time::get_current_year;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MusicContext {
    content: &'static str,
    copyright_year: i32,
}

impl MusicContext {
    pub fn new() -> Self {
        let year = get_current_year();
        MusicContext {
            content: "A page devoted to spotlighting my favourite music.",
            copyright_year: year,
        }
    }
}

impl Default for MusicContext {
    fn default() -> Self {
        Self::new()
    }
}
