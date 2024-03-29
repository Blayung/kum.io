// Server data, that we download from the server each tick.
pub mod game_state {
    use crate::data;

    #[derive(serde::Deserialize,Clone,Debug)]
    pub struct PlayerData {
        pub nick: String,
        pub direction: u16,
        pub x: f64,
        pub y: f64
    }

    #[derive(serde::Deserialize,Clone,Debug)]
    pub struct ChatMessage {
        pub nick: String,
        pub message: String
    }

    #[derive(serde::Deserialize,Clone,Debug)]
    pub struct GameState {
        pub players: std::vec::Vec<PlayerData>,
        pub chat_messages: std::vec::Vec<ChatMessage>
    }

    static GAME_STATE: std::sync::RwLock<GameState> = std::sync::RwLock::new(GameState {
        players: std::vec::Vec::new(),
        chat_messages: std::vec::Vec::new()
    });

    pub fn get() -> GameState {
        return GAME_STATE.read().unwrap().clone();
    }

    pub fn update() {
        let new_game_state = data::http_client::get().get(data::server_ip::get().to_owned()+"/game_state").send().unwrap().json().unwrap();
        let mut game_state;
        loop {
            game_state = GAME_STATE.try_write();
            if game_state.is_ok() {
                break;
            }
        }
        *game_state.unwrap() = new_game_state;
    }
}

// Data, that will be recieved by the every tick thread, sent to the server, and reset.
pub mod to_send_data {
    #[derive(Clone,Debug)]
    pub struct ToSendData {
        pub move_direction: Option<u8>,
        pub rotate: Option<u16>
    }

    static TO_SEND_DATA: std::sync::RwLock<ToSendData> = std::sync::RwLock::new(ToSendData {
        move_direction: None,
        rotate: None
    });

    pub fn get() -> ToSendData {
        let mut to_send_data;
        loop {
            to_send_data = TO_SEND_DATA.try_read();
            if to_send_data.is_ok() {
                break;
            }
        }
        return to_send_data.unwrap().clone();
    }

    pub fn set(to_send_data: ToSendData) {
        *TO_SEND_DATA.write().unwrap() = to_send_data;
    }
}

// Client ticks per second (Time between last two ticks)
pub mod ctps_elapsed {
    static mut CTPS_ELAPSED: std::time::Duration = std::time::Duration::ZERO;

    pub fn get() -> &'static std::time::Duration {
        unsafe {
            return &CTPS_ELAPSED;
        }
    }

    pub fn set(ctps_elapsed: std::time::Duration) {
        unsafe {
            CTPS_ELAPSED = ctps_elapsed;
        }
    }
}

// The http client, that's accessed in different threads.
pub mod http_client {
    static HTTP_CLIENT: std::sync::OnceLock<reqwest::blocking::Client> = std::sync::OnceLock::new();

    pub fn init() {
        HTTP_CLIENT.set(reqwest::blocking::Client::new()).unwrap();
    }

    pub fn get() -> &'static reqwest::blocking::Client {
        return HTTP_CLIENT.get().unwrap();
    }
}

// The server's ip, that's accessed in different threads.
pub mod server_ip {
    static SERVER_IP: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    pub fn init(ip: String) {
        SERVER_IP.set(ip).unwrap();
    }

    pub fn get() -> &'static String {
        return SERVER_IP.get().unwrap();
    }
}

// The player's credentials, that are accessed in different threads.
pub mod credentials {
    static CREDENTIALS: std::sync::OnceLock<(String,String)> = std::sync::OnceLock::new();

    pub fn init(credentials: (String,String)) {
        println!("{:?}", credentials);
        CREDENTIALS.set(credentials).unwrap();
    }

    pub fn get() -> &'static (String,String) {
        return CREDENTIALS.get().unwrap();
    }
}
