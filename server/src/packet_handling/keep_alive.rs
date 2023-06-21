use crate::game_state;
use crate::logging;

// Payload format: <nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    logging::debug("Recieved \"keep_alive\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 2 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get();

    let mut index=0;
    loop {
        if index >= _game_state.players.len() {
            return axum::http::StatusCode::UNAUTHORIZED;
        }

        if _game_state.players[index].nick == splitted_payload[0] {
            break;
        }

        index+=1;
    }
    if _game_state.players[index].token != splitted_payload[1] {
        return axum::http::StatusCode::FORBIDDEN;
    }

    _game_state.players[index].last_keep_alive = std::time::Instant::now();
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
