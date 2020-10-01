use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use crate::models::Post;
use crate::models::Image;
use crate::diesel::prelude::*;
use crate::error::BlogError;

pub type LitePool = Pool<ConnectionManager<SqliteConnection>>;
type LitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<LitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection(pool: &LitePool) -> Result<LitePooledConnection, BlogError> {
    pool.get()
        .map_err(|e| BlogError::build(e.to_string(), 500))
}

pub fn get_posts(pool: &LitePool) -> Result<Vec<Post>, BlogError> {
    use crate::schema::post::dsl::*;
    let connection = get_connection(pool)?;

    post.filter(published.eq(true))
        .limit(20)
        .load::<Post>(&connection)
        .map_err(|e| e.into())
}

pub fn get_post(post_web_name: String, pool: &LitePool) -> Result<Post, BlogError> {
    use crate::schema::post::dsl::*;
    let connection = get_connection(pool)?;

    post.filter(web_name.eq(post_web_name))
        .get_result::<Post>(&connection)
        .map_err(|e| e.into())
}

pub fn get_image_record(image_id: i32, pool: &LitePool) -> Result<Image, BlogError> {
    use crate::schema::image::dsl::*;
    let connection = get_connection(pool)?;

    image.filter(id.eq(image_id))
        .get_result::<Image>(&connection)
        .map_err(|e| e.into())
}
