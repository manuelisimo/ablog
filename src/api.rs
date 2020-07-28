use actix_web::{web, Responder, HttpResponse, Error, error};
use serde::Deserialize;
use sailfish::TemplateOnce;
use comrak::{markdown_to_html, ComrakOptions};
use crate::db;
use crate::models;


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

#[derive(Deserialize)]
pub struct PostParams {
    post_web_name: String,
}

#[derive(TemplateOnce)]
#[template(path = "post.stpl")]
struct PostTemplate {
    post: models::Post,
}

pub async fn post(
    pool: web::Data::<db::LitePool>,
    info: web::Path<PostParams>,
) -> Result<HttpResponse, Error> {
    let mut post = db::get_post(info.post_web_name.clone(), &pool)
        .map_err(|e| error::ErrorNotFound(e.to_string()))?;
    post.body = markdown_to_html(&post.body, &ComrakOptions::default());

    let context = PostTemplate {
        post,
    };
    Ok(HttpResponse::Ok().body(context.render_once().unwrap()))
}
