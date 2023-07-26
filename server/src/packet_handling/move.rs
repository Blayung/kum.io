use crate::game_state;
use crate::logging;

// Payload format: <direction (01234567)>,<is running (1/0)>,<nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    logging::debug("Recieved \"move\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 3 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get();

    let mut player = 0;
    loop {
        if player >= _game_state.players.len() {
            return axum::http::StatusCode::UNAUTHORIZED;
        }
        player += 1;
        if _game_state.players[player].nick == splitted_payload[1] {
            break;
        }
    }
    if _game_state.players[player].token != splitted_payload[2] {
        return axum::http::StatusCode::FORBIDDEN;
    }

    let parsed_move_direction = splitted_payload[0].parse::<u8>();
    if parsed_move_direction.is_err() {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    _game_state.players[player].next_move_direction = Some(parsed_move_direction.unwrap());
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
