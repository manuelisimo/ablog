use chrono::NaiveDateTime;
// use chrono::

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub created_at: NaiveDateTime,
}