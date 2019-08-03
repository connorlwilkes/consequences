#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    turn_number: u8,
    final_result: String,
    turns: Vec<Turn>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Turn {
    player: String,
    round: u8,
    answer: String,
}

impl Game {
    pub fn new() -> Game {
        Game {
            turn_number: 0,
            final_result: String::new(),
            turns: vec![],
        }
    }
//
//    pub fn startGame(&self) {
//        // for each player in self.lobby:
//        //  send game info
//        //  send first round info
//        //
//    }
}