use self::game::Game;

static MAX_ROUNDS: u8 = 8;


struct Lobby<'a> {
    id: u32,
    name: String,
    owner: LobbyPlayer,
    players: Vec<&'a LobbyPlayer>,
    round_number: u8,
    game: Game,

}

#[derive(Debug, Clone)]
struct LobbyPlayer {
    username: String,
    plays: Vec<String>,
}

impl Lobby {

    pub fn new(id: u32, name: String, owner: LobbyPlayer) -> Lobby {
        let mut players = Vec::new();
        players.push(&owner);
        Lobby {
            name,
            id,
            owner,
            players,
            round_number: 0,
        }
    }


}