#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate serde_derive;
extern crate diesel_demo;
extern crate rocket;
extern crate rocket_contrib;


use self::rocket_contrib::Template;
use rocket::request::{Form};
use rocket::response::{Flash, Redirect};
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use diesel_demo::models::NewPost;
use diesel::prelude::*;
use diesel_demo::*;
use diesel_demo::controllers::*;
use diesel_demo::views::show_all;
use diesel_demo::models::Post;


#[cfg(test)] mod tests;

#[get("/<path..>", rank = 5)]
fn all(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}

#[get("/")]
fn show() -> Template {
    let context = show_all();
    Template::render("index", context)
}

#[post("/", data="<post_form>")]
fn new_post(post_form: Form<NewPost>) -> Flash<Redirect>{
    let connection = establish_connection();

	let post_form = post_form.into_inner();

    create_post(&connection, post_form.title, post_form.body);
	Flash::success(Redirect::to("/"), "")
}

#[post("/<id>")]
fn publish(id: i32) -> Flash<Redirect>{

    let connection = establish_connection();
    
    publish_post(connection, id);

	Flash::success(Redirect::to("../.."), "")
}

#[post("/<id>")]
fn delete(id: i32) -> Flash<Redirect>{
    let connection = establish_connection();
    delete_post(connection, id);
	Flash::success(Redirect::to("../.."), "")
}



fn main() {
    rocket::ignite()
        .mount("/", routes![show, new_post, all])
        .mount("/publish/", routes![publish])
        .mount("/delete/", routes![delete])
        .attach(Template::fairing())
        .launch();
}
