// use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use crate::models::Post;
use crate::diesel::prelude::*;


pub type LitePool = Pool<ConnectionManager<SqliteConnection>>;
type LitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_url: &str) -> Result<LitePool, PoolError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_connection(pool: &LitePool) -> Result<LitePooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn get_posts(pool: &LitePool) -> Result<Vec<Post>, &'static str> {
    use crate::schema::post::dsl::*;
    let connection = get_connection(pool).expect("Shit happens");

    post.filter(published.eq(true))
        .limit(20)
        .load::<Post>(&connection)
        .map_err(|_| "Things went wrong trying to get posts")
}
