use crate::game_state;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"debug_game_state\"!");
    return (axum::http::StatusCode::OK, format!("{:#?}", game_state::get()));
}
