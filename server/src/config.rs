use std::io::Write;
use crate::logging;

#[derive(serde::Deserialize,Clone,Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub ip: std::vec::Vec<u8>,
    pub port: u16,
    pub server_name: String,
    pub log_level: u8,
    pub enable_debug_game_state: bool,
    pub CONFIG_VERSION_DO_NOT_TOUCH: i32
}

const CURRENT_CONFIG_VER:i32 = 0;

static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();

pub fn get() -> &'static Config {
    return CONFIG.get().unwrap();
}

pub fn init() {
    let raw_config = std::fs::read_to_string("config.json5").unwrap_or_else(|_| {
        std::fs::File::create("config.json5").unwrap().write_all(b"{
    \"ip\": [0,0,0,0], // Do not touch the ip if you do not know what it does.
    \"port\": 8888,
    \"server_name\": \"The best kum.io server in the world\",
    \"log_level\": 0, // 0 -> normal, 1 -> extra, 2 -> debug
    \"enable_debug_game_state\": false, // Always make sure to disable it in production. (For ordinary users: NEVER TRY TO TURN IT ON!)
    \"CONFIG_VERSION_DO_NOT_TOUCH\": 0
}").unwrap();
        return std::fs::read_to_string("config.json5").unwrap();
    });

    let config: Config = json5::from_str(&raw_config).unwrap();

    if config.CONFIG_VERSION_DO_NOT_TOUCH != CURRENT_CONFIG_VER {
        logging::_fatal(format!("An error occured while reading the config: The config's version doesn't match! This is probably because of a config from a previous/newer version of the server being read. If you really want to use that config, please change the CONFIG_VERSION_DO_NOT_TOUCH field to {ccv}. Example: \"CONFIG_VERSION_DO_NOT_TOUCH\": {ccv}", ccv=CURRENT_CONFIG_VER));
    }

    if config.ip.len()!=4 {
        logging::fatal("An error occured while reading the config: There are four elements required in ip. Example: \"ip\": [0,0,0,0]");
    }

    if config.log_level != 0 && config.log_level != 1 && config.log_level != 2 {
        logging::fatal("An error occured while reading the config: log_level has to be set to either 0, 1 or 2. Example: \"log_level\": 0");
    }

    CONFIG.set(config).unwrap();
}
