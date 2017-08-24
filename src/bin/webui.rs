#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;

use diesel_demo::*;
use self::models::*;
use diesel::prelude::*;
use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


#[cfg(test)] mod tests;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[derive(Debug, Serialize)]
pub struct Context {
    pub postcount: String,
    pub posts: Vec<Post>,
}


#[get("/")]
fn hello() -> Template {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let postcount = format!("Displaying {} posts", results.len());
    let context = Context{ postcount: postcount, posts: results};
    Template::render("index", context)
}


fn main() {
    rocket::ignite()
        .mount("/", routes![hello, all])
        .attach(Template::fairing())
        .launch();
}
