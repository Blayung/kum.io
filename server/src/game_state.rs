#[derive(Clone,Debug)]
pub struct PlayerData {
    nick: String,
    x: i32,
    y: i32,
}

#[derive(Clone,Debug)]
pub struct GameState {
    players: std::vec::Vec<(String, u16, PlayerData)>
}

lazy_static::lazy_static! {
    static ref GAME_STATE: mut_static::MutStatic<GameState> = mut_static::MutStatic::new();
}

pub fn get_game_state() -> GameState {
    return GAME_STATE.read().unwrap().as_ref().clone();
}

pub fn init() {
    GAME_STATE.set(
        GameState {
            players: std::vec::Vec::new()
        }
    ).unwrap();
}
