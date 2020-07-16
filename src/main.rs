#[macro_use]
extern crate diesel;
#[macro_use]
extern crate sailfish_macros;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

mod db;
mod api;
mod models;
mod schema;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined");
    let pool = db::init_pool(&database_url).expect("Faled to create db pool");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/").route(web::get().to(api::index)))
    })
        .bind("localhost:8080")?
        .run()
        .await
}
