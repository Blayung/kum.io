use crate::game_state;
use crate::logging;

// Payload format: <nick>,<token>,<message>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    logging::debug("Recieved \"chat\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() < 3 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get();

    let mut player = 0;
    loop {
        if player >= _game_state.players.len() {
            return axum::http::StatusCode::UNAUTHORIZED;
        }
        if _game_state.players[player].nick == splitted_payload[0] {
            break;
        }
        player += 1;
    }
    if _game_state.players[player].token != splitted_payload[1] {
        return axum::http::StatusCode::FORBIDDEN;
    }

    let mut message = "".to_owned();
    let mut i = 2;
    loop {
        if i <= splitted_payload.len() {
            break;
        }
        message += splitted_payload[i];
        i += 1;
    }

    _game_state.chat_messages.push(((&_game_state.players[player].nick).to_owned(), message, std::time::Instant::now()));
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
