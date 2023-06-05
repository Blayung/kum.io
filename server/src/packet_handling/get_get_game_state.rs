use crate::game_state;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"get_get_game_state\"!");
    return (axum::http::StatusCode::OK, format!("{:#?}",game_state::get_game_state()));
}
