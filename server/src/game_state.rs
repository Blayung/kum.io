#[derive(Clone,Debug)]
pub struct PlayerData {
    pub token: String,
    pub last_keep_alive: std::time::Instant,
    pub nick: String,
    pub is_running: bool,
    pub is_going_forward: bool,
    pub is_going_backward: bool,
    pub is_going_left: bool,
    pub is_going_right: bool,
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
    let mut game_state;
    loop {
        game_state = GAME_STATE.try_read();
        if game_state.is_ok() {
            break;
        }
    }
    return game_state.unwrap().clone();
}

pub fn set(game_state: GameState) {
    *GAME_STATE.write().unwrap() = game_state;
}
