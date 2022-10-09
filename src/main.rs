#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, from Rocket! Thanks to railway.app!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
