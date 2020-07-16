use actix_web::{web, Responder};
use crate::db;
use crate::models;


use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "hello.stpl")]
struct PostList {
    posts: Vec<models::Post>
}


pub async fn index(
    pool: web::Data<db::LitePool>
) -> impl Responder {
    let posts = db::get_posts(&pool)
        .expect("Something weird happened");
    let context = PostList {
        posts,
    };
    context.render_once().unwrap()
}
