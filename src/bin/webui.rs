#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


#[cfg(test)] mod tests;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/")]
fn show_all() -> Template {
    diesel_demo::show_posts::show_all()
}


fn main() {
    rocket::ignite()
        .mount("/", routes![show_all, all])
        .attach(Template::fairing())
        .launch();
}
