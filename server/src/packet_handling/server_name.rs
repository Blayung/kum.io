use crate::config;
use crate::logging;

pub async fn handle() -> (axum::http::StatusCode, String) {
    logging::debug("Recieved \"server_name\"!");
    return (axum::http::StatusCode::OK, config::get().to_owned().server_name);
}
