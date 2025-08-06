use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::env;

pub mod models;
pub mod repo;
pub mod schema;

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_pool() -> anyhow::Result<Pool> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);

    let pool = Pool::builder().build(config).await?;

    Ok(pool)
}
