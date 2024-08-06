#[macro_use]
extern crate rocket;

use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        name: "Rocket with Tera"
    })
}

#[get("/add")]
fn add() -> Template {
    Template::render("add", context! {
        title: "Add a Rust job"
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, add])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;