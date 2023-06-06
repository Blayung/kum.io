use crate::game_state;
use rand::distributions::DistString;

pub async fn handle(nick: String) -> (axum::http::StatusCode, String) {
    println!("Recieved \"post_register\"!");

    let mut _game_state = game_state::get_game_state().clone();

    let mut is_bad=false;
    for i in &_game_state.players {
        if i.nick == nick {
            is_bad=true;
            break;
        }
    }
    if is_bad {
        return (axum::http::StatusCode::BAD_REQUEST, "This nick is already taken.".to_string());
    }

    let token=rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32);

    _game_state.players.push(
        game_state::PlayerData {
            token: (*token).to_string(),
            last_keep_alive: std::time::Instant::now(),
            nick: nick,
            x: 0,
            y: 0
        }
    );

    game_state::set_game_state(_game_state);

    return (axum::http::StatusCode::OK, token);
}
