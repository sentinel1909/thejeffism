// httpd.rs main application binary

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::{context, Template};

use thejeffism_lib::domain::about::AboutContext;
use thejeffism_lib::domain::card::Card;
use thejeffism_lib::domain::projects::ProjectsContext;

#[get("/")]
fn index() -> Template {
    let rendered_cards = Card::get_cards();
    Template::render(
        "index",
        context! {
            title: &rendered_cards.0,
            date: &rendered_cards.0,
            content: &rendered_cards.0
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", AboutContext::new())
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", ProjectsContext::new())
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index, about, projects])
}
