use crate::game_state;

// Payload format: <nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    println!("Recieved \"log_out\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 2 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get().clone();

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

    _game_state.players.remove(index);
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
