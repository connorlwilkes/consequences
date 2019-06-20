use actix_web::{web, HttpResponse, App};
use r2d2_redis::redis;
use actix_web::middleware::identity::Identity;
use self::redis::Commands;
use crate::appdata::AppData;
use crate::appdata::RedisPool;
use super::lobby::Lobby;
use serde::private::ser::constrain;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyInfo { pub lobby_name: String }

pub fn create_lobby_handler(lobby_info: web::Json<LobbyInfo>, id: Identity, data: web::Data<AppData>) -> HttpResponse {
    // TODO - make this async
    // TODO Write a function to check if cookie is valid
    match id.identity() {
        Some(user) => create_lobby(lobby_info.into_inner().lobby_name, user, data.redis_pool()),
        _ => HttpResponse::BadRequest().json("Not logged in"),
    }
}

fn create_lobby(lobby_name: String, owner: String, redis_pool: &RedisPool) -> HttpResponse {
    // TODO - Check if the player is already in another lobby
    let redis_connection = redis_pool.get().unwrap();
    let check_result = redis_connection.exists(&lobby_name).unwrap();
    if check_result {
        HttpResponse::Conflict().json(format!("{} already exists", lobby_name))
    } else {
        let _result: u64 = redis_connection.sadd(format!("{}:members", &lobby_name), &owner).unwrap();
        let _result: u64 = redis_connection.hset(&lobby_name, "users", format!("{}:members", &lobby_name)).unwrap();
        let _result: u64 = redis_connection.sadd("lobbies", &lobby_name).unwrap();
        HttpResponse::Created().finish()
    }
}

pub fn join_lobby(lobby_info: web::Json<LobbyInfo>, id: Identity, data: web::Data<AppData>) -> HttpResponse {
    // TODO - Check if the player is already in another lobby
    match id.identity() {
        Some(user) => {
            let conn = data.redis_pool().get().unwrap();
            let lobby_name = lobby_info.into_inner().lobby_name.clone();
            let check_result = conn.exists(&lobby_name).unwrap();
            if check_result {
                let result: u64 = conn.sadd(lobby_name, user).unwrap();
                HttpResponse::Ok().finish()
            } else {
                HttpResponse::NoContent().finish()
            }
        },
        _ => HttpResponse::BadRequest().json("Not logged in"),
    }
}

pub fn get_lobbies(data: web::Data<AppData>) -> HttpResponse {
    let conn = data.redis_pool().get().unwrap();
    let results: Vec<String> = conn.smembers("lobbies").unwrap();
    HttpResponse::Ok().json(results)
}

pub fn start_lobby() {
    unimplemented!()
    // TODO - for all users in the lobby send out a signal that game has started
    // TODO - create a db backend (postgresql) with the game info
}

pub fn get_lobby_info(lobby_info: web::Json<LobbyInfo>, data: web::Data<AppData>) {
    let conn = data.redis_pool().get().unwrap();
    Lobby::get_from_redis(&conn, lobby_info.into_inner().lobby_name);
}