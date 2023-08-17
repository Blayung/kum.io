use crate::logging::unwrap_res;

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
pub struct ChatMessage {
    pub nick: String,
    pub message: String,
    pub recieve_moment: std::time::Instant
}

#[derive(Clone,Debug)]
pub struct GameState {
    pub players: std::vec::Vec<PlayerData>,
    pub chat_messages: std::vec::Vec<ChatMessage>
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
    return unwrap_res(game_state).clone();
}

pub fn set(game_state: GameState) {
    *unwrap_res(GAME_STATE.write()) = game_state;
}
