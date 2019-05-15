use crate::models;
use crate::app_data::{AppData, Pool};

use actix_web::middleware::identity::Identity;
use actix_web::{web, Error as ActixError, HttpRequest, HttpResponse};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use futures::Future;


#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    name: String,
}

pub fn login(item: web::Json<User>, data: web::Data<AppData>, req: HttpRequest, id: Identity) -> impl Future<Item=HttpResponse, Error=ActixError> {
    println!("{:?}", req);
    id.forget();
    println!("{}", format!("Hello {}", id.identity().unwrap_or("Anonymous".to_owned())));
    web::block(move || insert_user(item.into_inner().name, data.pool())).then(move |res| match res {
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
    use crate::schema::users::dsl::*;
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