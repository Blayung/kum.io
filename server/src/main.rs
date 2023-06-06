mod config;
mod game_state;
mod packet_handling;

const TICKRATE: u8 = 20;

#[tokio::main]
async fn main() {
    config::init();

    let axum_app = axum::Router::new()
        .route("/register", axum::routing::post(packet_handling::register::handle))
        .route("/get_game_state", axum::routing::get(packet_handling::get_game_state::handle))
        .route("/get_name", axum::routing::get(packet_handling::get_name::handle));

    let addr = std::net::SocketAddr::from(([config::get().ip[0], config::get().ip[1], config::get().ip[2], config::get().ip[3]], config::get().port));

    // A thread supposed to execute the every-tick code
    std::thread::spawn(|| {
        let mut tick_start: std::time::Instant;

        loop {
            tick_start=std::time::Instant::now();
            
            let mut _game_state=game_state::get().clone();

            _game_state.players.retain(|i| i.last_keep_alive.elapsed().as_secs() < 20);

            let mut i=0;
            loop {
                if i >= _game_state.players.len() {
                    break;
                }
                if _game_state.players[index].wish_direction.is_some() {
                    _game_state.players[index].direction = _game_state.players[index].wish_direction.unwrap();
                    _game_state.players[index].wish_direction = None;
                }
                i+=1;
            }
            
            // A LOT OF COMPLICATED MATH TO FIGURE OUT THE MOVEMENT IN HERE

            game_state::set(_game_state);

            std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / TICKRATE as u32).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    axum::Server::bind(&addr).serve(axum_app.into_make_service()).await.unwrap();
}
