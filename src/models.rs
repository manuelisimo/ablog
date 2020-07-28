use chrono::NaiveDateTime;
// use chrono::

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub intro: String,
    pub web_name: String,
    pub banner: Option<i32>,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub published_at: Option<NaiveDateTime>,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub file_name: Option<String>,
    pub web_name: String,
    pub path: String,
    pub created_at: NaiveDateTime,
}