use crate::game_state;

// Payload format: <direction (0-359)>,<nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    println!("Recieved \"rotate\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 3 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get().clone();

    let mut index=0;
    loop {
        if index >= _game_state.players.len() {
            return axum::http::StatusCode::UNAUTHORIZED;
        }

        if _game_state.players[index].nick == splitted_payload[1] {
            break;
        }

        index+=1;
    }
    if _game_state.players[index].token != splitted_payload[2] {
        return axum::http::StatusCode::FORBIDDEN;
    }

    let parsed_direction = splitted_payload[0].parse::<u16>();
    if parsed_direction.is_err() {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    _game_state.players[index].direction = parsed_direction.unwrap();
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
