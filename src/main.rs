pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

use std::collections::HashMap;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::{env, io};
use actix_web::{web, App, HttpServer, middleware, HttpResponse, Error};
use diesel::prelude::*;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn main() -> io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not create pool.");
    HttpServer::new(move || {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
        App::new()
            .data(pool.clone())
            .data(tera)
            .service(web::resource("/").route(web::get().to(index)))
            .wrap(middleware::Logger::default())
        })
    .bind("127.0.0.1:8080")?
    .run()
}

fn index(query: web::Query<HashMap<String, String>>, pool: web::Data<Pool>, tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        use self::schema::users::dsl::*;
        let mut ctx = tera::Context::new();

    } else {
        templ.render("index.html", &tera::Context.new()).map_err(|_| error::ErrorInternalServiceError("Template Error"))?
    }
    let connection  = pool.get().unwrap();
    let query =  users.select(usernames)
        .filter(username.eq(info.into_inner()))
        .load::<String>(&connection).unwrap();
   if query.len() == 1 {
       
   } else {
        HttpResponse::InternalServerError().json("error")
   }
}
