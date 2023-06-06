use crate::config;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"get_name\"!");
    return (axum::http::StatusCode::OK, config::get().clone().server_name);
}
