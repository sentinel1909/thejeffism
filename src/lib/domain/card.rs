// data for the index template

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct Card {
    pub id: u32,
    pub title: &'static str,
    pub date: &'static str,
    pub excerpt: &'static str,
}

impl Card {
    pub fn get_cards() -> Vec<Card> {
        let cards = vec![
            Card {
                id: 20220405,
                title: "Rust: The Journey Begins",
                date: "2022-04-05",
                excerpt: "Since the end of 2020, I've been on a mission to re-learn to code...",
            },
            Card {
                id: 20221010,
                title: "Second Post",
                date: "2022-10-10",
                excerpt: "Getting up and running with Rocket is easy...",
            },
            Card {
                id: 20221011,
                title: "Third Post",
                date: "2022-10-11",
                excerpt: "OMG it took me 2 hours to render these cards...",
            },
            Card {
                id: 20221012,
                title: "Fourth Post",
                date: "2022-10-12",
                excerpt: "Trying to get markdown rendered to a posts page...",
            },
            Card {
                id: 20221013,
                title: "Fifth Post",
                date: "2022-10-13",
                excerpt: "Just filling in space...",
            },
            Card {
                id: 20221014,
                title: "Sixth Post",
                date: "2022-10-14",
                excerpt: "And now the grid is complete...",
            },
        ];
        cards
    }
}
