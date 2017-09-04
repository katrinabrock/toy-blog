extern crate diesel;

use schema::posts::dsl::{posts, published};
use models::*;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub fn create_post(conn: &PgConnection, title: String, body: String) -> Post {
    use schema::posts;

    let new_post = NewPost {
        title: title,
        body: body,
    };

    diesel::insert(&new_post).into(posts::table)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn publish_post(conn: PgConnection, id: i32) -> Post {
    diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&conn)
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post(conn: PgConnection, id: i32) -> usize {
    diesel::delete(posts.find(id))
        .execute(&conn)
        .expect("Error deleting posts")
}
