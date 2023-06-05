use std::io::Write;

#[derive(serde::Deserialize,Clone)]
#[allow(non_snake_case)]
pub struct Config {
    pub ip: std::vec::Vec<u8>,
    pub port: u16,
    pub server_name: String,
    pub CONFIG_VERSION_DO_NOT_TOUCH: i32
}

const CURRENT_CONFIG_VER:i32 = 0;

lazy_static::lazy_static! {
    static ref CONFIG: mut_static::MutStatic<Config> = mut_static::MutStatic::new();
}

pub fn get_config() -> Config {
    return CONFIG.read().unwrap().as_ref().clone();
}

pub fn init() {
    let raw_config = std::fs::read_to_string("config.json5").unwrap_or_else(|_|{
        std::fs::File::create("config.json5").unwrap().write_all(b"{
    \"ip\": [0,0,0,0], // Do not touch the ip if you do not know what it does.
    \"port\": 8080,
    \"server_name\": \"The best kum.io server in the world\",
    \"CONFIG_VERSION_DO_NOT_TOUCH\": 0 // DO NOT TOUCH IT UNLESS YOU REALLY KNOW WHAT YOU'RE DOING!
}").unwrap();
        return std::fs::read_to_string("config.json5").unwrap();
    });

    let config: Config = json5::from_str(&raw_config).unwrap();

    if config.CONFIG_VERSION_DO_NOT_TOUCH != CURRENT_CONFIG_VER {
        panic!("An error occured while reading the config: The config's version doesn't match! This is probably because of a config from a previous/newer version of the server being read. If you really want to use that config, please change the CONFIG_VERSION_DO_NOT_TOUCH field to {} (the config version of this version of the server).",CURRENT_CONFIG_VER);
    }

    if config.ip.len()!=4 {
        panic!("An error occured while reading the config: There are four elements required in ip. Example: \"ip\": [0,0,0,0]");
    }

    CONFIG.set(config).unwrap();
}
