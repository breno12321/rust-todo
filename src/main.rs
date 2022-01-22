#[macro_use]
extern crate log;

#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate diesel;

extern crate dotenv;

mod db;

use actix_web::{middleware, web, App, HttpServer};
use std::env;

mod constants;
mod item;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    info!("Starting server on address");
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("/items")
                .service(item::get_items)
                .service(item::get_item)
                .service(item::delete_item)
                .service(item::create_item),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
