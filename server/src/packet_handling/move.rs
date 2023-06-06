use crate::game_state;

// Payload format: <pressed buttons in wsad order represented as ones and zeroes (example for pressed "w" and "d": 1001)>,<nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    println!("Recieved \"move\"!");

    let splitted_payload = payload.split(",");
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

    let parsed_wish_direction = splitted_payload[0].parse::<u16>();
    if parsed_wish_direction.is_err() {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    _game_state.players[index].wish_direction = Some(parsed_wish_direction.unwrap());
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
