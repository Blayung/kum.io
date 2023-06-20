use crate::config;

pub async fn handle() -> (axum::http::StatusCode, String) {
    println!("Recieved \"server_name\"!");
    return (axum::http::StatusCode::OK, config::get().to_owned().server_name);
}
