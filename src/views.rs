extern crate rocket_contrib;
extern crate serde;

use diesel::prelude::*;
use models::{Post, Context};


pub fn show_all() -> Context {
    use schema::posts::dsl::*;

    let connection = super::establish_connection();
    let results = posts 
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let postcount = format!("Displaying {} posts", results.len());

    Context{ postcount: postcount, posts: results}
}
