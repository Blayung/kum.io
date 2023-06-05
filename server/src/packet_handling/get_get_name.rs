use crate::config;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"get_get_name\"!");
    return (axum::http::StatusCode::OK, config::get_config().server_name);
}
