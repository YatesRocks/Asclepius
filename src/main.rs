#![allow(unused)]
#![allow(non_snake_case)]

use rocket::form::Form;
use rocket::response::Redirect;
use rocket_async_compression::CachedCompression;
#[macro_use]
extern crate rocket;

mod backend;

#[derive(FromForm)]
struct User {
    email: String,
    // pfsh what's security anyway
    password: String,
}

#[post("/auth", data = "<user>")]
fn authenticate(user: Form<User>) -> Redirect {
    if user.email == "demo@demo.com".to_string() && user.password == "password".to_string() {
        Redirect::to("dashboard.html")
    } else {
        Redirect::to("/")
    }
}

#[launch]
fn rocket() -> _ {
    let fs = rocket::fs::FileServer::from("static");
    let compress = CachedCompression::path_suffix_fairing(vec![
        ".js".to_owned(),
        ".html".to_owned(),
        ".css".to_owned(),
    ]);

    rocket::build()
        .mount("/", fs)
        .mount("/", routes![authenticate])
        .attach(compress)
}
