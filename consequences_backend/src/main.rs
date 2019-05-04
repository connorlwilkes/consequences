pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate dotenv;
extern crate actix_web;
extern crate env_logger;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel::prelude::*;
use std::{env, io};
use actix_web::{web, Error as ActixError, http::header, App, HttpServer, HttpResponse, middleware};
use futures::Future;
use dotenv::dotenv;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

struct AppData {
    pool: Pool,
}

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
            .data(AppData {
                pool: pool.clone()
            })
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::cors::Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::resource("/login").route(web::post().to_async(add_user)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}

fn add_user(item: web::Json<User>, data: web::Data<AppData>) -> impl Future<Item=HttpResponse, Error=ActixError> {
    web::block(move || insert_user(item.into_inner().name, &data.pool)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn insert_user(nm: String, pool: &Pool) -> Result<models::User, diesel::result::Error> {
    use self::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();
    diesel::insert_into(users).values(username.eq(nm.clone())).execute(conn)?;
    let mut items = users.filter(username.eq(nm)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())
}
