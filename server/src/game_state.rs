#[derive(Clone,Debug)]
pub struct PlayerData {
    pub token: String,
    pub last_keep_alive: std::time::Instant,
    pub nick: String,
    pub wish_direction: Option<u16>,
    pub direction: u16,
    pub x: i32,
    pub y: i32
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
