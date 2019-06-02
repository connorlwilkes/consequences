use actix_web::{web, HttpResponse};
use actix_web::middleware::identity::Identity;
use crate::appdata::AppData;
use r2d2_redis::{RedisConnectionManager, redis};
use crate::appdata::RedisPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyInfo { name: String }

struct CreateError { error_info: String }

pub fn create_lobby_handler(lobby_info: web::Json<LobbyInfo>, id: Identity, data: AppData) -> HttpResponse {
    // I should check here if this is a valid cookie - TODO Write a function to check
    match id.identity() {
        Some(id) => HttpResponse::Created().json(1),
        _ => HttpResponse::BadRequest().json("Not logged in"),
    }
}

fn create_lobby(lobby_name: String, redis_pool: RedisPool ) -> Result<HttpResponse, CreateError> {
    let redis_connection = redis_pool.get().unwrap();
    //Check lobby doesn't already exist
    let reply = redis::cmd("")
}