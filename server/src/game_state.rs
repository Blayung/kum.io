#[derive(Clone,Debug)]
pub struct PlayerData {
    pub token: String,
    pub last_keep_alive: std::time::Instant,
    pub nick: String,
    pub x: i32,
    pub y: i32
}

#[derive(Clone,Debug)]
pub struct GameState {
    pub players: std::vec::Vec<PlayerData>
}

lazy_static::lazy_static! {
    static ref GAME_STATE: mut_static::MutStatic<GameState> = mut_static::MutStatic::new();
}

pub fn get_game_state() -> GameState {
    return GAME_STATE.read().unwrap().as_ref().clone();
}

pub fn set_game_state(game_state: GameState) -> mut_static::error::Result<()> {
    return GAME_STATE.set(game_state);
}

pub fn init() {
    set_game_state(
        GameState {
            players: std::vec::Vec::new()
        }
    ).unwrap();
}
