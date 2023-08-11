use crate::game_state;
use crate::logging;

pub async fn handle() -> (axum::http::StatusCode, axum::Json<ClientGameState>) {
    logging::debug("Recieved \"game_state\"!");

    let _game_state = game_state::get();

    let mut client_player_data: std::vec::Vec<ClientPlayerData> = std::vec::Vec::new();
    for player in _game_state.players {
        client_player_data.push(ClientPlayerData {
            nick: player.nick,
            direction: player.direction,
            x: player.x,
            y: player.y
        });
    }

    let mut client_chat_messages: std::vec::Vec<ClientChatMessage> = std::vec::Vec::new();
    for message in _game_state.chat_messages {
        client_chat_messages.push(ClientChatMessage {
            nick: message.0,
            message: message.1
        });
    }

    return (axum::http::StatusCode::OK, axum::Json(ClientGameState {
        players: client_player_data,
        chat_messages: client_chat_messages
    }));
}

#[derive(serde::Serialize)]
pub struct ClientPlayerData {
    pub nick: String,
    pub direction: u16,
    pub x: f64,
    pub y: f64
}

#[derive(serde::Serialize)]
pub struct ClientChatMessage {
    pub nick: String,
    pub message: String
}

#[derive(serde::Serialize)]
pub struct ClientGameState {
    pub players: std::vec::Vec<ClientPlayerData>,
    pub chat_messages: std::vec::Vec<ClientChatMessage>
}
