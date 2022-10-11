// data for the index template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Card {
    pub title: &'static str,
    pub date: &'static str,
    pub content: &'static str,
}

impl Card {
    pub fn get_cards() -> Vec<Card> {
        let cards = vec![
            Card {
                title: "First Post",
                date: "2022-10-09",
                content: "Every site has to begin somewhere.",
            },
            Card {
                title: "Second Post",
                date: "2022-10-10",
                content: "Getting up and running with Rocket is easy...",
            },
            Card {
                title: "Third Post",
                date: "2022-10-11",
                content: "OMG it took me 2 hours to render these cards..."
            }
        ];
        cards
    }
}
