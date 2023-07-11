mod config;
mod game_state;
mod packet_handling;
mod logging;

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
        .route("/log_out", axum::routing::post(packet_handling::log_out::handle))
        .route("/rotate", axum::routing::post(packet_handling::rotate::handle));

    let cors_layer = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST])
        .allow_origin(tower_http::cors::Any);

    let ip_address = std::net::SocketAddr::from(([config::get().ip[0], config::get().ip[1], config::get().ip[2], config::get().ip[3]], config::get().port));

    // A thread supposed to execute the every-tick code
    logging::extra("Spawning the tick thread...");
    std::thread::spawn(|| {
        let mut tick_start: std::time::Instant;

        loop {
            tick_start=std::time::Instant::now();
            
            let mut _game_state=game_state::get();

            let mut index=0;
            loop {
                if index >= _game_state.players.len() {
                    break;
                }

                if _game_state.players[index].last_keep_alive.elapsed().as_secs() > 20 {
                    logging::_info(format!("We have stopped recieving communications from player \"{}\"! Disconnecting...", _game_state.players[index].nick));
                    _game_state.players.remove(index);
                } else {
                    if _game_state.players[index].next_move_direction.is_some() {
                        let next_move_direction = _game_state.players[index].next_move_direction.unwrap();

                        if next_move_direction == 0 {
                            _game_state.players[index].x += 8;
                        }
                        else if next_move_direction == 1 {
                            _game_state.players[index].x += 4;
                            _game_state.players[index].y += 4;
                        }
                        else if next_move_direction == 2 {
                            _game_state.players[index].y += 8;
                        }
                        else if next_move_direction == 3 {
                            _game_state.players[index].x -= 4;
                            _game_state.players[index].y += 4;
                        }
                        else if next_move_direction == 4 {
                            _game_state.players[index].x -= 8;
                        }
                        else if next_move_direction == 5 {
                            _game_state.players[index].x -= 4;
                            _game_state.players[index].y -= 4;
                        }
                        else if next_move_direction == 6 {
                            _game_state.players[index].y -= 8;
                        }
                        else if next_move_direction == 7 {
                            _game_state.players[index].x += 4;
                            _game_state.players[index].y -= 4;
                        }

                        _game_state.players[index].next_move_direction = None;
                    }
                }

                index+=1;
            }

            game_state::set(_game_state);

            std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    logging::_extra(format!("Starting the http server on {}...", &ip_address.to_string()));
    axum::Server::bind(&ip_address).serve(axum_app.layer(cors_layer).into_make_service()).await.unwrap();
}
