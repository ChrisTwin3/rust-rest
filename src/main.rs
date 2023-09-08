#[macro_use]
extern crate actix_web;

use std::{env, io};
use actix_web::{middleware, App, HttpServer};

mod api;
mod response;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(api::get_users)
            .service(api::create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}