use schema::posts;
extern crate serde;
extern crate diesel;


#[derive(Queryable, Debug, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, FromForm)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
