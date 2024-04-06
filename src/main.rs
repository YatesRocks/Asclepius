#![allow(unused)]
#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;

mod backend;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hbs::index])
        .attach(Template::fairing())
}
