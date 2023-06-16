use crate::game_state;

pub async fn handle() -> (axum::http::StatusCode, axum::Json<JsonGameState>) {
    println!("Recieved \"get_game_state\"!");

    let _game_state = game_state::get();

    let mut json_player_data: std::vec::Vec<JsonPlayerData> = std::vec::Vec::new();
    for i in _game_state.players {
        json_player_data.push(JsonPlayerData {
            nick: i.nick,
            direction: i.direction,
            x: i.x,
            y: i.y
        });
    }

    return (axum::http::StatusCode::OK, axum::Json(JsonGameState {
        players: json_player_data
    }));
}

#[derive(serde::Serialize)]
pub struct JsonPlayerData {
    pub nick: String,
    pub direction: u16,
    pub x: f64,
    pub y: f64
}

#[derive(serde::Serialize)]
pub struct JsonGameState {
    pub players: std::vec::Vec<JsonPlayerData>
}
