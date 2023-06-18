pub mod game_state {
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
}

pub mod to_send {
    use std::ops::Deref;

    #[derive(Clone,Debug)]
    pub struct ToSendData {
        pub move_direction: Option<char>,
        pub direction: u16
    }

    static TO_SEND_DATA: std::sync::RwLock<ToSendData> = std::sync::RwLock::new(ToSendData {
        move_direction: None,
        direction: 0
    });

    pub fn reset() {
        set(ToSendData {
            move_direction: None,
            direction: 0
        });
    }

    pub fn get() -> ToSendData {
        return TO_SEND_DATA.try_read().unwrap().deref().clone();
    }

    pub fn set(to_send_data: ToSendData) {
        let mut _to_send_data = TO_SEND_DATA.write().unwrap();
        *_to_send_data = to_send_data;
    }
}

pub mod http_client {
    static HTTP_CLIENT: std::sync::OnceLock<reqwest::blocking::Client> = std::sync::OnceLock::new();

    pub fn init() {
        HTTP_CLIENT.set(reqwest::blocking::Client::new()).unwrap();
    }

    pub fn get() -> reqwest::blocking::Client {
        return HTTP_CLIENT.get().unwrap().clone();
    }
}
