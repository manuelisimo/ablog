use actix_web::{web, Responder, HttpResponse, Error};
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
) -> Result<HttpResponse, Error> {
    let posts = db::get_posts(&pool)
        .expect("Something weird happened");
    let context = PostList {
        posts,
    };

    Ok(HttpResponse::Ok().body(context.render_once().unwrap()))
}
