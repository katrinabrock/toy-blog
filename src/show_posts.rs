extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use diesel::prelude::*;
use self::rocket_contrib::Template;
use models::Post;

#[derive(Debug, Serialize)]
pub struct Context {
    pub postcount: String,
    pub posts: Vec<Post>,
}


pub fn show_all() -> Template {
    use schema::posts::dsl::*;

    let connection = super::establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    let postcount = format!("Displaying {} posts", results.len());
    let context = Context{ postcount: postcount, posts: results};
    Template::render("index", context)
}
