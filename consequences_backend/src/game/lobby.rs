use crate::game::game::Game;

static MAX_ROUNDS: u8 = 8;


pub struct Lobby<'a> {
    id: u32,
    name: String,
    owner: &'a LobbyPlayer,
    players: Vec<&'a LobbyPlayer>,
    round_number: u8,
    game: Option<Game>,
}

#[derive(Debug, Clone)]
pub struct LobbyPlayer {
    username: String,
    plays: Vec<String>,
}

impl<'a> Lobby<'a> {

    pub fn new(id: u32, name: String, owner: &'a LobbyPlayer) -> Lobby<'a> {
        let mut players = Vec::new();
        players.push(owner);
        Lobby {
            name,
            id,
            owner,
            players,
            round_number: 0,
            game: None,
        }
    }


}