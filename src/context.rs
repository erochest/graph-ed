use diesel::pg::PgConnection;
use juniper;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub struct Context {
    pub pool: Pool,
}

impl juniper::Context for Context {}
