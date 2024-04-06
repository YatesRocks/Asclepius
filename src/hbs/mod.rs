use rocket::response::Redirect;
use rocket::Request;

use rocket_dyn_templates::{context, handlebars, Template};

use self::handlebars::{Handlebars, JsonRender};

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}
