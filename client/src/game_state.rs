use std::ops::Deref;

#[derive(serde::Deserialize,Clone,Debug)]
pub struct PlayerData {
    pub nick: String,
    pub direction: u16,
    pub x: f64,
    pub y: f64
}

#[derive(serde::Deserialize,Clone,Debug)]
pub struct GameState {
    pub players: std::vec::Vec<PlayerData>
}

static GAME_STATE: std::sync::RwLock<GameState> = std::sync::RwLock::new(GameState {
    players: std::vec::Vec::new()
});

pub fn get() -> GameState {
    return GAME_STATE.try_read().unwrap().deref().clone();
}

pub fn set(game_state: GameState) {
    let mut _game_state = GAME_STATE.write().unwrap();
    *_game_state = game_state;
}
