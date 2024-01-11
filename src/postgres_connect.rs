use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .min_idle(Some(1))
        .max_size(3)
        .build(manager)
        .expect("Failed to create connection pool");
    return pool;
}
