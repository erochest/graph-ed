use dataloader::BatchFn;
use dataloader::BatchFuture;
use diesel::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use juniper;
use r2d2;
use r2d2_diesel::ConnectionManager;
use futures::Future;
use futures::future::ok;

use crate::models::Node;
use crate::schema::nodes;

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

impl BatchFn<i32, Node> for Context {
    type Error = String;

    fn load(&self, keys: &[i32]) -> BatchFuture<Node, Self::Error> {
        self.pool.get()
            .map_err(|err| format!("Error getting connection pool: {}", &err))
            .and_then(|connection|
                nodes::table
                    .filter(nodes::dsl::id.eq(any(keys)))
                    .load::<Node>(&*connection)
                    .map_err(|err| format!("Unable to load nodes: {}", &err)))
            .boxed()
    }
}
