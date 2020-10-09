use std::process::Command;
use std::fs::File;
use actix_web::{web, HttpResponse, Error, error, dev};
use actix_files::NamedFile;
use serde::{Serialize, Deserialize};
use sailfish::TemplateOnce;
use comrak::{markdown_to_html, ComrakOptions};
use crate::db;
use crate::models;
use crate::error::BlogError;
use actix_web::middleware::errhandlers::ErrorHandlerResponse;


#[derive(TemplateOnce)]
#[template(path = "landing.html")]
struct PostList {
    posts: Vec<models::Post>
}

pub async fn index(
    pool: web::Data<db::LitePool>
) -> Result<HttpResponse, BlogError> {
    let posts = db::get_posts(&pool)?;
    let context = PostList {
        posts,
    };

    Ok(HttpResponse::Ok().body(context.render_once()?))
}

#[derive(Deserialize)]
pub struct PostParams {
    post_web_name: String,
}

#[derive(TemplateOnce)]
#[template(path = "post.html")]
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

#[derive(Serialize)]
struct Fortune {
    fortune: String,
}

/**
 * This was necessary!
 */
pub async fn fortune() -> Result<HttpResponse, Error> {
    let fortune_bin = std::env::var("FORTUNE")
        .map_err(error::ErrorInternalServerError)?;
    let command_output =  Command::new(fortune_bin)
        .args(&[
            "25%",
            "art",
            "25%",
            "science",
            "25%",
            "ascii-art",
            "25%",
            "linuxcookie",
        ])
        .output()?;
    let text_output = std::str::from_utf8(&command_output.stdout)?;
    let fortune = Fortune {
        fortune: text_output.to_string()
    };

    Ok(HttpResponse::Ok().json(fortune))
}

pub async fn favicon() -> Result<NamedFile, Error> {
    let static_dir = std::env::var("STATIC_DIR")
        .map_err(error::ErrorInternalServerError)?;
    let favicon = File::open(format!("{}favicon.ico", static_dir))?;

    actix_files::NamedFile::from_file(favicon, "favicon.ico")
        .map_err(error::ErrorInternalServerError)
}

pub fn not_found<B>(res: dev::ServiceResponse<B>,) -> Result<ErrorHandlerResponse<B>, Error> {
    let response = NamedFile::open("static/errors/404.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(res.into_response(response.into_body()),))
}

pub fn bad_request<B>(res: dev::ServiceResponse<B>,) -> Result<ErrorHandlerResponse<B>, Error> {
    let response = NamedFile::open("static/errors/400.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;
    Ok(ErrorHandlerResponse::Response(res.into_response(response.into_body()),))
}

pub fn internal_server_error<B>(res: dev::ServiceResponse<B>,)
    -> Result<ErrorHandlerResponse<B>, Error> {
    let response = NamedFile::open("static/errors/500.html")?
        .set_status_code(res.status())
        .into_response(res.request())?;

    Ok(ErrorHandlerResponse::Response(res.into_response(response.into_body()),))
}
