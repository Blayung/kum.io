use crate::game_state;
use crate::logging;

pub async fn handle() -> (axum::http::StatusCode, axum::Json<JsonGameState>) {
    logging::debug("Recieved \"game_state\"!");

    let _game_state = game_state::get();

    let mut json_player_data: std::vec::Vec<JsonPlayerData> = std::vec::Vec::new();
    for player in _game_state.players {
        json_player_data.push(JsonPlayerData {
            nick: player.nick,
            direction: player.direction,
            x: player.x,
            y: player.y
        });
    }

    let mut json_chat_messages: std::vec::Vec<JsonChatMessage> = std::vec::Vec::new();
    for message in _game_state.chat_messages {
        json_chat_messages.push(JsonChatMessage {
            nick: message.0,
            message: message.1
        });
    }

    return (axum::http::StatusCode::OK, axum::Json(JsonGameState {
        players: json_player_data,
        chat_messages: json_chat_messages
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
pub struct JsonChatMessage {
    pub nick: String,
    pub message: String
}

#[derive(serde::Serialize)]
pub struct JsonGameState {
    pub players: std::vec::Vec<JsonPlayerData>,
    pub chat_messages: std::vec::Vec<JsonChatMessage>
}
