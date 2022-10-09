// httpd.rs main application binary

#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

use thejeffism_lib::domain::index::IndexContext;

#[get("/")]
fn index() -> Template {
    Template::render("index", IndexContext::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![index])
}
