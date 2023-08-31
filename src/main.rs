use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod db;
mod employees;
mod error_handlers;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let mut listenfd: ListenFd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default())
            .configure(employees::init_routes)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let port: String = env::var("PORT").expect("Please set port in .env");
            let host = format!("localhost:{port}");
            server.bind(host)?
        }
    };
    server.run().await
}
