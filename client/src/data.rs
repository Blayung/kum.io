// Server-sided data, that is used to draw stuff in the main game. We get it by accessing the /game_state path of the server, and update it each tick.
pub mod game_state {
    use crate::data;

    #[derive(serde::Deserialize,Clone,Debug)]
    pub struct PlayerData {
        pub nick: String,
        pub direction: u16,
        pub x: i32,
        pub y: i32
    }

    #[derive(serde::Deserialize,Clone,Debug)]
    pub struct GameState {
        pub players: std::vec::Vec<PlayerData>
    }

    static GAME_STATE: std::sync::RwLock<GameState> = std::sync::RwLock::new(GameState {
        players: std::vec::Vec::new()
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

// Data, that has to be passed to the every-tick thread, that then sends all of it to the server
pub mod to_send_data {
    #[derive(Clone,Debug)]
    pub struct ToSendData {
        pub move_direction: Option<char>,
        pub direction: Option<u16>
    }

    static TO_SEND_DATA: std::sync::RwLock<ToSendData> = std::sync::RwLock::new(ToSendData {
        move_direction: None,
        direction: None
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

// Client-sided data, that has to be passed between different frames
pub mod local_data {
    #[derive(Clone,Debug)]
    pub struct LocalData {
        pub game_stage: u8,

        pub input: String,
        pub flickering_cursor: u8,
        pub letter_pressed: Option<char>,
        pub shift_pressed: bool,
        /*
        let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
        let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
        let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
        */

        pub forward_pressed: bool,
        pub right_pressed: bool,
        pub backward_pressed: bool,
        pub left_pressed: bool,
    }

    static mut LOCAL_DATA: LocalData = LocalData {
        game_stage: 0,

        input: "193.107.8.49:8888".to_owned(),
        flickering_cursor: 0,
        letter_pressed: None,
        shift_pressed: false,

        forward_pressed: false,
        right_pressed: false,
        backward_pressed: false,
        left_pressed: false,
    };

    pub fn get() -> LocalData {
        unsafe {
            return LOCAL_DATA.clone();
        }
    }

    pub fn set(local_data: LocalData) {
        unsafe {
            LOCAL_DATA = local_data;
        }
    }
}

// The http client accessed in different threads
pub mod http_client {
    static HTTP_CLIENT: std::sync::OnceLock<reqwest::blocking::Client> = std::sync::OnceLock::new();

    pub fn init() {
        HTTP_CLIENT.set(reqwest::blocking::Client::new()).unwrap();
    }

    pub fn get() -> &'static reqwest::blocking::Client {
        return HTTP_CLIENT.get().unwrap();
    }
}

// The server's ip accessed in different threads
pub mod server_ip {
    static SERVER_IP: std::sync::OnceLock<String> = std::sync::OnceLock::new();

    pub fn init(ip: String) {
        SERVER_IP.set(ip).unwrap();
    }

    pub fn get() -> &'static String {
        return SERVER_IP.get().unwrap();
    }
}

// The player's credentials accessed in different threads
pub mod credentials {
    static CREDENTIALS: std::sync::OnceLock<(String,String)> = std::sync::OnceLock::new();

    pub fn init(credentials: (String,String)) {
        CREDENTIALS.set(credentials).unwrap();
    }

    pub fn get() -> &'static (String,String) {
        return CREDENTIALS.get().unwrap();
    }
}
