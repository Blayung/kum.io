mod config;
mod game_state;
mod packet_handling;
mod logging;
mod every_tick_thread;

#[tokio::main]
async fn main() {
    logging::info("Welcome to the wonderful server of kum.io!");

    config::init();

    let axum_app = axum::Router::new()
        .route("/register", axum::routing::post(packet_handling::register::handle))
        .route("/keep_alive", axum::routing::post(packet_handling::keep_alive::handle))
        .route("/game_state", axum::routing::get(packet_handling::game_state::handle))
        .route("/server_name", axum::routing::get(packet_handling::server_name::handle))
        .route("/mov_run", axum::routing::post(packet_handling::mov_run::handle))
        .route("/mov_forward", axum::routing::post(packet_handling::mov_forward::handle))
        .route("/mov_backward", axum::routing::post(packet_handling::mov_backward::handle))
        .route("/mov_left", axum::routing::post(packet_handling::mov_left::handle))
        .route("/mov_right", axum::routing::post(packet_handling::mov_right::handle))
        .route("/mov_rotate", axum::routing::post(packet_handling::mov_rotate::handle))
        .route("/log_out", axum::routing::post(packet_handling::log_out::handle));

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(tower_http::cors::Any);

    let ip_address = std::net::SocketAddr::from(([config::get().ip[0], config::get().ip[1], config::get().ip[2], config::get().ip[3]], config::get().port));

    every_tick_thread::spawn!();

    logging::_extra(format!("Starting the http server on {}...", &ip_address.to_string()));
    axum::Server::bind(&ip_address).serve(axum_app.layer(cors_layer).into_make_service()).await.unwrap();
}
