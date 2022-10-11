// data for the index template

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Card {
    title: &'static str,
    date: &'static str,
    content: &'static str,
}

impl Card {
    pub fn get_cards() -> Json<Vec<Card>> {
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
        ];
        Json(cards)
    }
}
