#[macro_use]
extern crate diesel;
#[macro_use]
extern crate sailfish_macros;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger};
use actix_files;
use dotenv::dotenv;

mod db;
mod api;
mod models;
mod schema;
mod images;


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

    let pool = db::init_pool(&database_url).expect("Faled to create db pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
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
        .bind("0.0.0.0:8080")?
        .bind("[::]:8080")?
        .run()
        .await
}
