use crate::game_state;

pub async fn handle(nick: String) -> (axum::http::StatusCode, String) {
    println!("Recieved \"post_register\"!");
    return (axum::http::StatusCode::OK, nick + &format!("{:#?}",game_state::get_game_state()));
}
