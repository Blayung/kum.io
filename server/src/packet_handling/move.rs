use crate::game_state;

// Payload format: <direction (01234567)>,<nick>,<token>
pub async fn handle(payload: String) -> axum::http::StatusCode {
    println!("Recieved \"move\"!");

    let splitted_payload = payload.split(",").collect::<Vec<&str>>();
    if splitted_payload.len() != 3 {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    let mut _game_state = game_state::get();

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

    let direction: u8;
    if splitted_payload[0]=="0" {
        direction=0;
    }
    else if splitted_payload[0]=="1" {
        direction=1;
    }
    else if splitted_payload[0]=="2" {
        direction=2;
    }
    else if splitted_payload[0]=="3" {
        direction=3;
    }
    else if splitted_payload[0]=="4" {
        direction=4;
    }
    else if splitted_payload[0]=="5" {
        direction=5;
    }
    else if splitted_payload[0]=="6" {
        direction=6;
    }
    else if splitted_payload[0]=="7" {
        direction=7;
    }
    else {
        return axum::http::StatusCode::BAD_REQUEST;
    }

    _game_state.players[index].next_move_direction = Some(direction);
    game_state::set(_game_state);

    return axum::http::StatusCode::OK;
}
