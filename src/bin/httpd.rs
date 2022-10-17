// httpd.rs main application binary

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_dyn_templates::{context, Template};

use thejeffism_lib::domain::about::AboutContext;
use thejeffism_lib::domain::card::Card;
use thejeffism_lib::domain::music::MusicContext;
use thejeffism_lib::domain::photography::PhotographyContext;
use thejeffism_lib::domain::post::get_html;
use thejeffism_lib::domain::projects::ProjectsContext;
use thejeffism_lib::domain::writing::WritingContext;
use thejeffism_lib::time::get_current_year;

#[get("/")]
fn index() -> Template {
    let rendered_cards = Card::get_cards();
    let year = get_current_year();
    Template::render(
        "index",
        context! {
            views: rendered_cards,
            copyright_year: year
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", AboutContext::new())
}

#[get("/music")]
fn music() -> Template {
    Template::render("music", MusicContext::new())
}

#[get("/projects")]
fn projects() -> Template {
    Template::render("projects", ProjectsContext::new())
}

#[get("/photography")]
fn photography() -> Template {
    Template::render("photography", PhotographyContext::new())
}

#[get("/writing")]
fn writing() -> Template {
    Template::render("writing", WritingContext::new())
}

#[get("/api/health_check")]
fn api_health_check() -> (Status, &'static str) {
    (Status::Ok, "200 OK")
}

#[get("/api/posts")]
fn api_posts() -> Json<Vec<Card>> {
    Json(Card::get_cards())
}

#[get("/posts/<postid>")]
fn posts(postid: u32) -> Template {
    let content = get_html(postid);
    let year = get_current_year();
    match content {
        Ok(html) => Template::render(
            postid.to_string(),
            context! { value: html, copyright_year: year },
        ),
        Err(_) => Template::render(
            "post",
            context! {value: "Unable to render from markdown file"},
        ),
    }
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 8080))
        .merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                about,
                projects,
                music,
                photography,
                posts,
                writing,
                api_health_check,
                api_posts,
            ],
        )
}
