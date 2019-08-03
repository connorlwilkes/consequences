use actix_web::{web, HttpResponse, App};
use r2d2_redis::redis;
use actix_identity::Identity;
use self::redis::Commands;
use crate::appdata::AppData;
use crate::appdata::RedisPool;
use super::game_logic::Lobby;
use serde::private::ser::constrain;
use actix_web::web::to;
use std::convert::identity;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyInfo { pub lobby_name: String }

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub lobby_name: String,
    pub game_name: String,
    pub response: String,
}

pub fn create_lobby_handler(lobby_info: web::Json<LobbyInfo>, id: Identity, data: web::Data<AppData>) -> HttpResponse {
    // TODO - make this async
    // TODO Write a function to check if cookie is valid
    match id.identity() {
        Some(user) => create_lobby(lobby_info.into_inner().lobby_name, user, data.redis_pool()),
        _ => HttpResponse::BadRequest().body("Not logged in"),
    }
}

fn create_lobby(lobby_name: String, owner: String, redis_pool: &RedisPool) -> HttpResponse {
    // TODO - Check if the player is already in another lobby
    let redis_connection = redis_pool.get().unwrap();
    let check_result = redis_connection.exists(&lobby_name).unwrap();
    if check_result {
        HttpResponse::Conflict().body(format!("{} already exists", lobby_name))
    } else {
        //TODO - Pipeline these commands/Transaction? hmset would help also
        let _result: u64 = redis_connection.zadd(format!("{}:members", &lobby_name), &owner, 1).unwrap();
        let _result: u64 = redis_connection.hset(&lobby_name, "users", format!("{}:members", &lobby_name)).unwrap();
        let _result: u64 = redis_connection.hset(&lobby_name, "owner", &owner).unwrap();
        let _result: u64 = redis_connection.hset(&lobby_name, "player_count", 1).unwrap();
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
                let player_count: u8 = conn.hincr(&lobby_name, "player_count", 1).unwrap();
                let result: u64 = conn.zadd(format!("{}:members", &lobby_name), user, player_count).unwrap();
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
    //TODO: Get info on players for each of the lobbies?
    let results: Vec<String> = conn.smembers("lobbies").unwrap();
    HttpResponse::Ok().json(results)
}

pub fn get_lobby_info(lobby_info: web::Json<LobbyInfo>, data: web::Data<AppData>, id: Identity) -> HttpResponse {
    let conn = data.redis_pool().get().unwrap();
    let lobby = Lobby::get_from_redis(&conn, lobby_info.into_inner().lobby_name);
    let to_return = web::Json(lobby);
    HttpResponse::Ok().json(to_return.into_inner())
}

pub fn start_game(lobby_info: web::Json<LobbyInfo>, data: web::Data<AppData>, id: Identity, redis_pool: &RedisPool) -> HttpResponse {
    let conn = data.redis_pool().get().unwrap();
    let lobby_name = lobby_info.into_inner().lobby_name;
    let mut lobby = Lobby::get_from_redis(&conn, lobby_name.clone());
    if id.identity().unwrap() == lobby.owner && lobby.players.len() > 1 {
        let redis_connection = redis_pool.get().unwrap();
        let game_name = format!("{}:game", &lobby_name);
        let _result: u64 = redis_connection.hset(&lobby_name, "game", &game_name).unwrap();
        let _result: u64 = redis_connection.hset(&game_name, "round", "1").unwrap();
        for i in 1..=lobby.players.len() {
            let _result: u64 = redis_connection.hset(&game_name, format!("consequence{}", i), "").unwrap();
        }
        let to_return = web::Json(lobby);
        lobby = Lobby::get_from_redis(&conn, lobby_name.clone());
        HttpResponse::Ok().json(to_return.into_inner())
    } else {
        HttpResponse::BadRequest().body("Not enough players")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameInfoResponse {
    pub lobby_name: String,
    pub stories: Option<Vec<String>>,
    pub done: bool,
}

pub fn process_round_result(game_info: web::Json<GameInfo>, data: web::Data<AppData>, id: Identity, redis_pool: &RedisPool) -> HttpResponse {
    let conn = data.redis_pool().get().unwrap();
    let message = game_info.into_inner().response;
    let lobby_name = game_info.into_inner().lobby_name;
    let game_name = format!("{}:game", &lobby_name);
    let player_number: u8 = conn.zscore(id.identity().unwrap());
    let round_number: u8 = conn.hget(&game_name, "round");
    let player_count: u8 = conn.hget(&lobby_name, "player_count");
    let mut to_return = GameInfoResponse {
        lobby_name: lobby_name.clone(),
        stories: None,
        done: false,
    };
    let temp = (player_number - 1) + round_number;
    match temp % player_count {
        0 => {
            let mut consequence: String = conn.hget(&game_name, format!("consequence{}", player_number)).unwrap();
            consequence.push_str(message.as_str());
            let _result: u8 = conn.hset(&game_name, format!("consequence{}", player_number), consequence).unwrap();
        },
        _ => {
            let nmbr = temp % player_count;
            let mut consequence: String = conn.hget(&game_name, format!("consequence{}", nmbr)).unwrap();
            consequence.push_str(message.as_str());
            let _result: u8 = conn.hset(&game_name, format!("consequence{}", nmbr), consequence).unwrap();
        }
    }
    if round_number == 8 {
        let lobby = Lobby::get_from_redis(&conn, lobby_name.clone());
        let mut stories = Vec::new();
        for i in 1..=lobby.players.len() {
            let consequence: String = conn.hget(&game_name, format!("consequence{}", player_number)).unwrap();
            stories.push(consequence);
        }
        to_return.stories = Some(stories);
        to_return.done = true;
    }
    HttpResponse::Ok().json(web::Json(to_return).into_inner())
}