// data for the index template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Card {
    pub id: u32,
    pub title: &'static str,
    pub date: &'static str,
    pub content: &'static str,
}

impl Card {
    pub fn get_cards() -> Vec<Card> {
        let cards = vec![
            Card {
                id: 20221009,
                title: "First Post",
                date: "2022-10-09",
                content: "Every site has to begin somewhere...",
            },
            Card {
                id: 20221010,
                title: "Second Post",
                date: "2022-10-10",
                content: "Getting up and running with Rocket is easy...",
            },
            Card {
                id: 20221011,
                title: "Third Post",
                date: "2022-10-11",
                content: "OMG it took me 2 hours to render these cards...",
            },
            Card {
                id: 20221012,
                title: "Fourth Post",
                date: "2022-10-12",
                content: "Trying to get markdown rendered to a posts page...",
            },
            Card {
                id: 20221013,
                title: "Fifth Post",
                date: "2022-10-13",
                content: "Just filling in space...",
            },
            Card {
                id: 20221014,
                title: "Sixth Post",
                date: "2022-10-14",
                content: "And now the grid is complete...",
            },
        ];
        cards
    }

    pub fn get_card_id(&self) -> u32 {
        self.id
    }
}
