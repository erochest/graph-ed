extern crate dotenv;
extern crate graph_ed;

use std::env;

fn main() {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    graph_ed::run(&database_url);
}
