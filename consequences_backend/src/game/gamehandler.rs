use actix_web::{web, HttpResponse};
use r2d2_redis::redis;
use actix_web::middleware::identity::Identity;
use self::redis::Commands;
use crate::appdata::AppData;
use crate::appdata::RedisPool;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyInfo { pub lobby_name: String }

pub fn create_lobby_handler(lobby_info: web::Json<LobbyInfo>, id: Identity, data: web::Data<AppData>) -> HttpResponse {
    // TODO - make this async
    // I should check here if this is a valid cookie - TODO Write a function to check
    match id.identity() {
        Some(user) => create_lobby(lobby_info.into_inner().lobby_name, user, data.redis_pool()),
        _ => HttpResponse::BadRequest().json("Not logged in"),
    }
}

fn create_lobby(lobby_name: String, owner: String, redis_pool: &RedisPool) -> HttpResponse {
    // TODO - Check if the player is already in another lobby
    let redis_connection = redis_pool.get().unwrap();
    //Check lobby doesn't already exist
    let check_result = redis_connection.exists(&lobby_name).unwrap();
    if check_result {
        HttpResponse::Conflict().json(format!("{} already exists", lobby_name))
    } else {
        // Assume that add was successful for now
        let _result: u64 = redis_connection.sadd(&lobby_name, &owner).unwrap();
        HttpResponse::Created().finish()
    }
}

pub fn join_lobby(lobby_info: web::Json<LobbyInfo>, id: Identity, data: web::Data<AppData>) -> HttpResponse {
    unimplemented!()
    // TODO - match on the identity
    // TODO - check exisistence of lobby
    // TODO - add user to lobby
    // TODO - check the number of users in lobby, if max (5) reached - do not allow anymore to join
}

pub fn get_lobbies() {
    unimplemented!()
    // TODO - check if user is logged in etc - write a function to check this???
    // TODO - get a list of all lobbies that exist and return them in json
}

pub fn start_lobby() {
    unimplemented!()
    // TODO - for all users in the lobby send out a signal that game has started
    // TODO - create a db backend (postgresql) with the game info
}