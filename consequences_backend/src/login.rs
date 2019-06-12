use super::models;
use super::appdata::{AppData, DatabasePool, RedisPool};

use actix_web::middleware::identity::Identity;
use actix_web::{web, Error as ActixError, HttpResponse};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::{Future, future::ok};
use r2d2_redis::redis;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub fn login(item: web::Json<User>, data: web::Data<AppData>, id: Identity) -> Box<dyn Future<Item=HttpResponse, Error=ActixError>> {
    match id.identity() {
        Some(id) => Box::new(ok(HttpResponse::Conflict().json(&id))),
        None => Box::new(login_helper(item.into_inner().name, data.database_pool().clone(), id, data.redis_pool())),
    }
}

fn login_helper(name: String, pool: DatabasePool, id: Identity, redis_pool: &RedisPool) -> impl Future<Item=HttpResponse, Error=ActixError> {
    web::block(move || insert_user(name, &pool)).then(move |res| match res {
        Ok(result) => {
            id.remember(String::from(result.user.username.as_str()));
            redis_pool.get().unwrap().
            match result.already_present {
                // TODO - Seperate signup from login
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

fn insert_user(nm: String, pool: &DatabasePool) -> Result<UserSelectResult, diesel::result::Error> {
    use super::schema::users::dsl::*;
    let conn: &PgConnection = &pool.get().unwrap();
    let user = models::NewUser {
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
