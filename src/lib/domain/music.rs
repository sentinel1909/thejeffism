// data for the music template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct MusicContext {
    content: &'static str,
}

impl MusicContext {
    pub fn new() -> Self {
        MusicContext {
            content: "A page devoted to spotlighting my favourite music",
        }
    }
}

impl Default for MusicContext {
    fn default() -> Self {
        Self::new()
    }
}
