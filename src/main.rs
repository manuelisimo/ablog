#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate sailfish_macros;

use actix_web::{web, App, HttpServer, middleware, http};
use actix_web::middleware::{Logger};
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use actix_files;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslAcceptorBuilder};
use std::io::Error;
use dotenv::dotenv;

mod db;
mod api;
mod models;
mod schema;
mod images;
mod error;

embed_migrations!("./migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined");
    let static_path = std::env::var("STATIC_PATH")
        .expect("STATIC_PATH must be defined");
    let static_dir = std::env::var("STATIC_DIR")
        .expect("STATIC_DIR must be defined");
    let pool = db::init_pool(&database_url)
        .expect("Failed to create db pool");

    let connection = pool.get()
        .expect("Could not connect to database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Could not run database migrations");

    HttpServer::new(move || {
        let error_handlers = ErrorHandlers::new()
            .handler(http::StatusCode::INTERNAL_SERVER_ERROR, api::internal_server_error);

        App::new()
            .data(pool.clone())
            .wrap(RedirectSchemeBuilder::new().build())
            .wrap(Logger::default())
            .wrap(error_handlers)
            .wrap(middleware::Compress::default())
            .service(web::resource("/")
                .route(web::get().to(api::index)))
            .service(web::resource("/post/{post_web_name}")
                .route(web::get().to(api::post)))
            .service(web::resource("/image/{id}")
                .route(web::get().to(images::get_image)))
            .service(web::resource("/fortune")
                .route(web::get().to(api::fortune)))
            .service(web::resource("/favicon.ico", )
                .route(web::get().to(api::favicon)))
            .service(actix_files::Files::new(&static_path, &static_dir))
    })
        .keep_alive(75)
        .bind("0.0.0.0:80")?
        // TODO: Troubleshoot docker ip6
        // .bind("[::]:80")?
        .bind_openssl("0.0.0.0:443", build_builder()?)?
        // TODO: Troubleshoot docker ip6
        // .bind_openssl("[::]:443", build_builder()?)?
        .run()
        .await
}

pub fn build_builder()
    -> Result<SslAcceptorBuilder, Error> {
    let tls_private_key = std::env::var("TLS_PRIVATE_KEY")
        .expect("TLS_PRIVATE_KEY must be defined");
    let tls_certificate = std::env::var("TLS_CERTIFICATE")
        .expect("TLS_CERTIFICATE must be defined");
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(tls_private_key, SslFiletype::PEM)?;
    builder.set_certificate_chain_file(tls_certificate)?;
    Ok(builder)
}
