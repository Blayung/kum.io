macro_rules! spawn {
    () => {
        logging::extra("Spawning the tick thread...");
        std::thread::spawn(|| {
            let mut tick_start: std::time::Instant;

            loop {
                tick_start=std::time::Instant::now();
                
                // Getting the game state
                let mut _game_state=game_state::get();

                // Deleting old chat messages
                _game_state.chat_messages.retain(|i| i.2.elapsed().as_secs() < 21);

                // Looping over players
                let mut player=0;
                loop {
                    if player >= _game_state.players.len() {
                        break;
                    }

                    if _game_state.players[player].last_keep_alive.elapsed().as_secs() > 20 {
                        // Kicking players when no keep alive packets are sent
                        logging::_info(format!("We have stopped recieving communications from player \"{}\"! Disconnecting...", _game_state.players[player].nick));
                        _game_state.players.remove(player);
                    } else {
                        // MOVEMENT
                        let next_move_direction = _game_state.players[player].next_move_direction;
                        if next_move_direction.is_some() {
                            //
                            // To calculate the diagonal values (im not used to such kind of math):
                            //
                            //  normal_val
                            // ------------
                            //      √2
                            //
                            
                            match next_move_direction.unwrap() {
                                0 => _game_state.players[player].x += 7.0,
                                1 => {
                                    _game_state.players[player].x += 4.94974746831;
                                    _game_state.players[player].y += 4.94974746831;
                                },
                                2 => _game_state.players[player].y += 7.0,
                                3 => {
                                    _game_state.players[player].x -= 4.94974746831;
                                    _game_state.players[player].y += 4.94974746831;
                                },
                                4 => _game_state.players[player].x -= 7.0,
                                5 => {
                                    _game_state.players[player].x -= 4.94974746831;
                                    _game_state.players[player].y -= 4.94974746831;
                                },
                                6 => _game_state.players[player].y -= 7.0,
                                7 => {
                                    _game_state.players[player].x += 4.94974746831;
                                    _game_state.players[player].y -= 4.94974746831;
                                },

                                8 => _game_state.players[player].x += 14.0,
                                9 => {
                                    _game_state.players[player].x += 9.89949493661;
                                    _game_state.players[player].y += 9.89949493661;
                                },
                                10 => _game_state.players[player].y += 14.0,
                                11 => {
                                    _game_state.players[player].x -= 9.89949493661;
                                    _game_state.players[player].y += 9.89949493661;
                                },
                                12 => _game_state.players[player].x -= 14.0,
                                13 => {
                                    _game_state.players[player].x -= 9.89949493661;
                                    _game_state.players[player].y -= 9.89949493661;
                                },
                                14 => _game_state.players[player].y -= 14.0,
                                15 => {
                                    _game_state.players[player].x += 9.89949493661;
                                    _game_state.players[player].y -= 9.89949493661;
                                },

                                _ => {}
                            }

                            _game_state.players[player].next_move_direction = None;
                        }
                    }

                    player += 1;
                }

                // Setting the game state
                game_state::set(_game_state);

                // TPS limit
                std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
            }
        });
    }
}

pub(crate) use spawn;
