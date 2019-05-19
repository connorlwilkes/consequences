pub mod schema;
pub mod models;
pub mod login;
pub mod app_data;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::{io, env};
use actix_web::{App, HttpServer, middleware, web, http::header};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use dotenv::dotenv;
use app_data::AppData;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv().ok();
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");

    HttpServer::new(move || {
        App::new()
            .data(AppData::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::cors::Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .secure(false),
            ))
            .service(web::resource("/login").route(web::post().to_async(login::login)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}