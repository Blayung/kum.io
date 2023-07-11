use crate::game_state;
use crate::logging;
use rand::distributions::DistString;

const ALLOWED_NICK_CHARS: &str="abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 -_";

pub async fn handle(nick: String) -> (axum::http::StatusCode, String) {
    logging::debug("Recieved \"register\"!");

    if nick.len() < 1 || nick.len() > 20 {
        return (axum::http::StatusCode::BAD_REQUEST, String::from("0 The nick's length has to be >0 and <21."));
    }

    let mut is_bad=false;
    for i in nick.chars() {
        if !ALLOWED_NICK_CHARS.contains(i) {
            is_bad=true;
            break;
        }
    }
    if is_bad {
        return (axum::http::StatusCode::BAD_REQUEST, String::from("1 The nick can contain only english alphabet upper and lower case letters, numbers, the space, a dash (or a minus sign), and a floor character."));
    }

    let mut _game_state = game_state::get();

    is_bad=false;
    for i in &_game_state.players {
        if i.nick == nick {
            is_bad=true;
            break;
        }
    }
    if is_bad {
        return (axum::http::StatusCode::BAD_REQUEST, String::from("2 This nick is already taken."));
    }

    let token=rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    _game_state.players.push(
        game_state::PlayerData {
            token: (*token).to_owned(),
            last_keep_alive: std::time::Instant::now(),
            nick: nick.clone(),
            next_move_direction: None,
            direction: 0,
            x: 0,
            y: 0
        }
    );

    game_state::set(_game_state);

    logging::_info(format!("Player \"{}\" joined the game!", nick));

    return (axum::http::StatusCode::CREATED, token);
}
