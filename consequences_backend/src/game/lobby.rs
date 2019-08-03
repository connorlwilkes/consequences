use crate::game::game::Game;
use r2d2_redis::RedisConnectionManager;
use r2d2::PooledConnection;
use itertools::{Tuples, Itertools};
use self::redis::Commands;
use r2d2_redis::redis;

static MAX_ROUNDS: u8 = 8;

type RedisConnection = PooledConnection<RedisConnectionManager>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lobby {
    name: String,
    owner: String,
    players: Vec<String>,
    round_number: u8,
    game: Option<Game>,
}

impl Lobby {

    pub fn new(name: String, owner: String) -> Lobby {
        let mut players = Vec::new();
        players.push(owner.clone());
        Lobby {
            name,
            owner,
            players,
            round_number: 0,
            game: None,
        }
    }

    pub fn get_from_redis(conn: &RedisConnection, lobby_name: String) -> Lobby {
        let fields: Vec<String> = conn.hgetall(lobby_name.clone()).unwrap();
        let mut name = lobby_name.clone();
        let mut owner = String::new();
        let mut players = Vec::new();
        for (field, value) in fields.into_iter().tuples() {
                match field {
                    ref field if field.contains("users") => {
                        players = conn.smembers(value).unwrap();
                    }
                    ref field if field.contains("owner") => {
                        owner = value
                    }
                    _ => {}
                }
        }
        Lobby {
            name,
            owner,
            players,
            round_number: 0,
            game: None,
        }
    }

    pub fn add_to_redis(&self, fields_to_add: Vec<String>, ) {
        unimplemented!()
//        for &field in fields_to_add {
//            match field {
//
//            }
//        }
    }
}