#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::Template;
use rocket::request::{Form};
use rocket::response::{Flash, Redirect};
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use diesel_demo::{establish_connection, create_post};
use diesel_demo::models::NewPost;


#[cfg(test)] mod tests;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/")]
fn show_all() -> Template {
    diesel_demo::show_posts::show_all()
}

#[post("/", data="<post_form>")]
fn new_post(post_form: Form<NewPost>) -> Flash<Redirect>{
    let connection = establish_connection();

	let post_form = post_form.into_inner();

    let post = create_post(&connection, post_form.title, post_form.body);
	Flash::success(Redirect::to("/"), "")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![show_all, new_post, all])
        .attach(Template::fairing())
        .launch();
}
