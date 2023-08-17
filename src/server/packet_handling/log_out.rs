use crate::game_state;
use crate::logging;

// Payload format: <nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    logging::debug("Recieved \"log_out\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 2 {
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

    _game_state.players.remove(player);
    game_state::set(_game_state);

    logging::_info(format!("Player \"{}\" left the game!", splitted_payload[0]));

    return axum::http::StatusCode::OK;
}
