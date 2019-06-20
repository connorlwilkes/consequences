pub mod schema;
pub mod models;
pub mod login;
pub mod appdata;
pub mod index;
pub mod game;
pub mod authentication;
pub mod redis_session;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate actix_web;
extern crate env_logger;
extern crate futures;

use std::io;
use actix_web::{App, HttpServer, middleware, web, http::header};
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use appdata::AppData;

fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .data(AppData::new())
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::cors::Cors::new()
                    .allowed_origin("127.0.0.1:62026")
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
            .service(web::resource("/create-lobby").route(web::post().to(game::gamehandler::create_lobby_handler)))
            .service(web::resource("/join-lobby").route(web::post().to(game::gamehandler::join_lobby)))
            .service(web::resource("/lobby-info").route(web::post().to(game::gamehandler::get_lobby_info)))
            .service(web::resource("/").route(web::get().to(index::check)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}