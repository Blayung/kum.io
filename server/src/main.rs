mod config;
mod game_state;
mod packet_handling;
mod logging;
mod every_tick_thread;

use logging::unwrap_res;
use logging::unwrap_opt;

#[tokio::main]
async fn main() {
    logging::info("Welcome to the wonderful server of kum.io!");

    config::init();

    let axum_app = axum::Router::new()
        .route("/register", axum::routing::post(packet_handling::register::handle))
        .route("/keep_alive", axum::routing::post(packet_handling::keep_alive::handle))
        .route("/game_state", axum::routing::get(packet_handling::game_state::handle))
        .route("/server_name", axum::routing::get(packet_handling::server_name::handle))
        .route("/move", axum::routing::post(packet_handling::r#move::handle))
        .route("/rotate", axum::routing::post(packet_handling::rotate::handle))
        .route("/chat", axum::routing::post(packet_handling::chat::handle))
        .route("/log_out", axum::routing::post(packet_handling::log_out::handle));

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(tower_http::cors::Any);

    let ip_address = std::net::SocketAddr::from(([config::get().ip[0], config::get().ip[1], config::get().ip[2], config::get().ip[3]], config::get().port));

    every_tick_thread::spawn!();

    logging::_extra(format!("Starting the http server on {}...", &ip_address.to_string()));
    unwrap_res(axum::Server::bind(&ip_address).serve(axum_app.layer(cors_layer).into_make_service()).await);
}
