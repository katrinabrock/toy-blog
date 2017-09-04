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
use diesel_demo::models::NewPost;
use diesel::prelude::*;
use diesel_demo::*;
use diesel_demo::models::Post;


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

    create_post(&connection, post_form.title, post_form.body);
	Flash::success(Redirect::to("/"), "")
}

#[post("/<id>")]
fn publish(id: i32) -> Flash<Redirect>{
    use diesel_demo::schema::posts::dsl::{posts, published};

    let connection = establish_connection();

    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
	Flash::success(Redirect::to("../.."), "")
}

#[post("/<id>")]
fn delete(id: i32) -> Flash<Redirect>{
    use diesel_demo::schema::posts::dsl::posts;
    let connection = establish_connection();
    diesel::delete(posts.find(id))
        .execute(&connection)
        .expect("Error deleting posts");
	Flash::success(Redirect::to("../.."), "")
}



fn main() {
    rocket::ignite()
        .mount("/", routes![show_all, new_post, all])
        .mount("/publish/", routes![publish])
        .mount("/delete/", routes![delete])
        .attach(Template::fairing())
        .launch();
}
