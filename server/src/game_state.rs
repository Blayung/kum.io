#[derive(Clone,Debug)]
pub struct PlayerData {
    pub token: String,
    pub last_keep_alive: std::time::Instant,
    pub nick: String,
    pub next_move_direction: Option<u8>,
    pub direction: u16,
    pub x: f64,
    pub y: f64
}

#[derive(Clone,Debug)]
pub struct GameState {
    pub players: std::vec::Vec<PlayerData>
}

static GAME_STATE: std::sync::RwLock<GameState> = std::sync::RwLock::new(GameState {
    players: std::vec::Vec::new()
});

pub fn get() -> GameState {
    return GAME_STATE.try_read().unwrap().clone();
}

pub fn set(game_state: GameState) {
    let mut _game_state = GAME_STATE.write().unwrap();
    *_game_state = game_state;
}
