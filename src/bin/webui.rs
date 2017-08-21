#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel_demo;
extern crate diesel;
extern crate rocket;

use diesel_demo::*;
use self::models::*;
use diesel::prelude::*;


#[cfg(test)] mod tests;


#[get("/")]
fn hello() -> String {
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let postcount = format!("Displaying {} posts", results.len());
    let mut current_posts = String::from("");
    for post in results {
        current_posts.push_str(&post.title);
        current_posts.push_str(&"-----------\n".to_string());
        current_posts.push_str(&post.body);
    }
    
    let content = format!("{}.\n{}", postcount, current_posts);

    content
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
