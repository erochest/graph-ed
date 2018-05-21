#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate juniper;
extern crate juniper_rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[cfg(test)]
extern crate spectral;
extern crate uuid;

use rocket::response::content;
use rocket::State;
use std::env;

mod context;
mod graph;
mod models;
mod schema;

use context::Context;

#[get("/")]
fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
fn get_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<graph::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

#[post("/graphql", data = "<request>")]
fn post_graphql_handler(
    context: State<Context>,
    request: juniper_rocket::GraphQLRequest,
    schema: State<graph::Schema>,
) -> juniper_rocket::GraphQLResponse {
    request.execute(&schema, &context)
}

fn main() {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = context::create_db_pool(&database_url);
    let context = Context { pool };

    rocket::ignite()
        .manage(context)
        .manage(graph::Schema::new(graph::Query, graph::Mutation))
        .mount(
            "/",
            routes![graphiql, get_graphql_handler, post_graphql_handler],
        )
        .launch();
}
