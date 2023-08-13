use std::io::Write;
use crate::logging;
use crate::logging::unwrap_res;
use crate::logging::unwrap_opt;

#[derive(serde::Deserialize,Clone,Debug)]
pub struct Config {
    pub ip: std::vec::Vec<u8>,
    pub port: u16,
    pub server_name: String,
    pub log_level: u8,
    pub config_version: i32
}

const CURRENT_CONFIG_VER:i32 = 0;

static CONFIG: std::sync::OnceLock<Config> = std::sync::OnceLock::new();

pub fn get() -> &'static Config {
    return unwrap_opt(CONFIG.get());
}

pub fn init() {
    let raw_config = std::fs::read_to_string("config.json5").unwrap_or_else(|_| {
        unwrap_res(std::fs::File::create("config.json5")).write_all(b"{
    \"ip\": [0,0,0,0], // Do not touch the ip if you do not know what it does.
    \"port\": 8888,
    \"server_name\": \"The best kum.io server in the world\",
    \"log_level\": 0, // 0 -> normal, 1 -> extra, 2 -> debug
    \"config_version\": 0 // If you get an error when launching the server, then the config syntax has probably changed, and you rather shouldn't change that value - instead - just delete that file and rewrite it. (but it won't hurt to try!)
}").unwrap();
        return unwrap_res(std::fs::read_to_string("config.json5"));
    });

    let config: Config = unwrap_res(json5::from_str(&raw_config));

    if config.config_version != CURRENT_CONFIG_VER {
        logging::_fatal(format!("An error occured while reading the config: The config's version doesn't match! This is probably because of a config from a previous/newer version of the server being read. If you really want to use that config, please change the config_version field to {ccv}. Example: \"config_version\": {ccv}", ccv=CURRENT_CONFIG_VER));
    }

    if config.ip.len()!=4 {
        logging::fatal("An error occured while reading the config: There are four elements required in ip. Example: \"ip\": [0,0,0,0]");
    }

    if config.log_level != 0 && config.log_level != 1 && config.log_level != 2 {
        logging::fatal("An error occured while reading the config: log_level has to be set to either 0, 1 or 2. Example: \"log_level\": 0");
    }

    unwrap_res(CONFIG.set(config));
}
