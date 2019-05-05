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
use actix_web::{web, Error as ActixError, HttpRequest, http::header, App, HttpServer, HttpResponse, middleware};
use actix_session::{CookieSession, Session, UserSession};
use futures::Future;
use dotenv::dotenv;
use futures::future::IntoFuture;
use actix_web::cookie::Cookie;
use actix_web::error;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService, Identity};
use self::models::NewUser;
use core::borrow::Borrow;

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
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth")
                    .secure(false),
            ))
            .service(web::resource("/login").route(web::post().to_async(login)))
    })
        .bind("127.0.0.1:8080")?
        .run()
}

fn login(item: web::Json<User>, data: web::Data<AppData>, req: HttpRequest, id: Identity) -> impl Future<Item=HttpResponse, Error=ActixError> {
    println!("{:?}", req);
    web::block(move || insert_user(item.into_inner().name, &data.pool)).then(move |res| match res {
        Ok(result) => {
            id.remember(String::from(result.user.username.as_str()));
            match result.already_present {
                true => Ok(HttpResponse::Ok().json(result.user)),
                false => Ok(HttpResponse::Created().json(result.user))
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            Ok(HttpResponse::InternalServerError().json("Database Error"))
        }
    })
}

struct UserSelectResult {
    user: models::User,
    already_present: bool,
}

fn insert_user(nm: String, pool: &Pool) -> Result<UserSelectResult, diesel::result::Error> {
    use self::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();
    let user = NewUser {
        username: &nm,
        id: None,
    };
    match users.filter(username.eq(user.username)).first(conn) {
        Ok(result) => Ok(UserSelectResult {
            user: result,
            already_present: true,
        }),
        Err(diesel::NotFound) => {
            match diesel::insert_into(users)
                .values(&user)
                .get_result::<models::User>(conn) {
                Ok(res) => Ok(UserSelectResult {
                    user: res,
                    already_present: false,
                }),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
