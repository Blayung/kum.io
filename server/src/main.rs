mod config;
mod game_state;
mod packet_handling;

#[tokio::main]
async fn main() {
    let axum_app = axum::Router::new()
        .route("/register", axum::routing::post(packet_handling::post_register::handle))
        .route("/get_game_state", axum::routing::get(packet_handling::get_get_game_state::handle))
        .route("/get_name", axum::routing::get(packet_handling::get_get_name::handle));

    let addr = std::net::SocketAddr::from(([config::get_config().ip[0], config::get_config().ip[1], config::get_config().ip[2], config::get_config().ip[3]], config::get_config().port));
    axum::Server::bind(&addr).serve(axum_app.into_make_service()).await.unwrap();
}
