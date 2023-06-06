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

static mut GAME_STATE: GameState =
    GameState {
        players: std::vec::Vec::new()
    };

pub fn get_game_state() -> &'static GameState {
    unsafe {
        return &GAME_STATE;
    }
}

pub fn set_game_state(game_state: GameState) {
    unsafe {
        GAME_STATE = game_state;
        println!("{:#?}",GAME_STATE);
    }
}
