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

// Should use Mutex in the future
static mut GAME_STATE: GameState = GameState {
    players: std::vec::Vec::new()
};

pub fn get() -> &'static GameState {
    unsafe {
        return &GAME_STATE;
    }
}

pub fn set(game_state: GameState) {
    unsafe {
        GAME_STATE = game_state;
    }
}
