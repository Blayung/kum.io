use crate::game_state;
use crate::config;
use crate::logging;

pub async fn handle() -> (axum::http::StatusCode, String) {
    logging::debug("Recieved \"debug_game_state\"!");
    if config::get().enable_debug_game_state {
        return (axum::http::StatusCode::OK, format!("{:#?}", game_state::get()));
    } else {
        return (axum::http::StatusCode::IM_A_TEAPOT, "I'm a teapot!".to_owned());
    }
}
