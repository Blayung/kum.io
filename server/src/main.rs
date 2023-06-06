mod config;
mod game_state;
mod packet_handling;

#[tokio::main]
async fn main() {
    config::init();

    let axum_app = axum::Router::new()
        .route("/register", axum::routing::post(packet_handling::register::handle))
        .route("/get_game_state", axum::routing::get(packet_handling::get_game_state::handle))
        .route("/get_name", axum::routing::get(packet_handling::get_name::handle));

    let addr = std::net::SocketAddr::from(([config::get().ip[0], config::get().ip[1], config::get().ip[2], config::get().ip[3]], config::get().port));

    axum::Server::bind(&addr).serve(axum_app.into_make_service()).await.unwrap();
}
