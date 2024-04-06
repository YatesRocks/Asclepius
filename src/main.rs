#![allow(unused)]
#![allow(non_snake_case)]
#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod backend;
mod hbs;
#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hbs::index])
        .attach(Template::fairing())
}
