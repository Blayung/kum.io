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
    pub players: std::vec::Vec<PlayerData>,
    pub chat_messages: std::vec::Vec<(String,String,std::time::Instant)>
}

static GAME_STATE: std::sync::RwLock<GameState> = std::sync::RwLock::new(GameState {
    players: std::vec::Vec::new(),
    chat_messages: std::vec::Vec::new()
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
