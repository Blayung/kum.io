use crate::game_state;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"get_game_state\"!");
    // Only temporary - should be converted into json, not formatted with rust's default formatter,    
    // Debug only - shouldn't return tokens
    return (axum::http::StatusCode::OK, format!("{:#?}",game_state::get()));
}
