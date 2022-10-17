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
                title: "Coming Soon",
                date: "xxxx-xx-xx",
                excerpt: "Nothing to see yet...",
            },
            Card {
                id: 20221011,
                title: "Coming Soon",
                date: "xxxx-xx-xx",
                excerpt: "Nothing to see yet...",
            },
            Card {
                id: 20221012,
                title: "Coming Soon",
                date: "xxxx-xx-xx",
                excerpt: "Nothing to see yet...",
            },
            Card {
                id: 20221013,
                title: "Coming Soon",
                date: "xxxx-xx-xx",
                excerpt: "Nothing to see yet...",
            },
            Card {
                id: 20221014,
                title: "Coming Soon",
                date: "xxxx-xx-xx",
                excerpt: "Nothing to see yet...",
            },
        ];
        cards
    }
}
