// use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use crate::models::Post;
use crate::models::Image;
use crate::diesel::prelude::*;
use std::error::Error;


pub type LitePool = Pool<ConnectionManager<SqliteConnection>>;
type LitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<LitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection(pool: &LitePool) -> Result<LitePooledConnection, PoolError> {
    pool.get()
}

pub fn get_posts(pool: &LitePool) -> Result<Vec<Post>, String> {
    use crate::schema::post::dsl::*;
    let connection = get_connection(pool).map_err(|e| e.to_string())?;

    post.filter(published.eq(true))
        .limit(20)
        .load::<Post>(&connection)
        .map_err(|_| "Things went wrong trying to get posts".to_string())
}

pub fn get_image_record(image_id: i32, pool: &LitePool) -> Result<Image, String> {
    use crate::schema::image::dsl::*;
    let connection = get_connection(pool).map_err(|e| e.to_string())?;

    image.filter(id.eq(image_id))
        .get_result::<Image>(&connection)
        .map_err(|_| "Error querying images".to_string())
}
