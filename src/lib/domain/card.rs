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
                excerpt: "Since late 2020, I've been re-learning to code...",
            },
            Card {
                id: 20220409,
                title: "Rust: Constants",
                date: "2022-04-09",
                excerpt: "The journey to proficiency in Rust continues...",
            },
            Card {
                id: 20220412,
                title: "Rust: Pouring the Footings",
                date: "2022-04-12",
                excerpt: "Before I continue my series of articles...",
            },
            Card {
                id: 20220414,
                title: "Rust: Single Value Data Types",
                date: "2022-04-14",
                excerpt: "Today I continue my campaign about...",
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
