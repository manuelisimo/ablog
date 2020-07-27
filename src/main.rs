#[macro_use]
extern crate diesel;
#[macro_use]
extern crate sailfish_macros;
#[macro_use]
extern crate log;


use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger};
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
    let pool = db::init_pool(&database_url).expect("Faled to create db pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(api::index)))
            .service(web::resource("/image/{id}").route(web::get().to(images::get_image)))
    })
        .bind("localhost:8080")?
        .run()
        .await
}
