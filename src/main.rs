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
use actix_web::{web, Error as ActixError,  App, HttpServer, HttpResponse, middleware};
use futures::Future;
use std::sync::{Arc, Mutex};
use dotenv::dotenv;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

struct AppData {
    user_counter: Arc<Mutex<u32>>,
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
            pool: pool.clone(),
            user_counter: Arc::new(Mutex::new(0))
        })
        .wrap(middleware::Logger::default())
        .service(web::resource("/login").route(web::post().to_async(add_user)))
        })
        .bind("127.0.0.1:8080")?
        .run()
}

fn add_user(item: web::Json<User>, data: web::Data<AppData>) -> impl Future<Item = HttpResponse, Error = ActixError> {
    web::block(move || insert_user(item.into_inner().name, &data.pool, &data.user_counter)).then(|res| match res {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
}

fn insert_user(nm: String, pool: &Pool, user_counter: &Arc<Mutex<u32>> ) -> Result<models::User, diesel::result::Error> { 
    println!{"Adding {}", nm.as_str()};
    use self::schema::users::dsl::*;
    let mut user_counter = user_counter.lock().unwrap();
    *user_counter += 1;
    let new_user = models::NewUser {
        id: *user_counter as i32,
        username: nm.as_str(),
    };
    let conn: &PgConnection = &pool.get().unwrap();

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    let mut items = users.filter(id.eq(&id)).load::<models::User>(conn)?;
    Ok(items.pop().unwrap())    
}
