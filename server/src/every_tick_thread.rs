macro_rules! spawn {
    () => {
        logging::extra("Spawning the tick thread...");
        std::thread::spawn(|| {
            let mut tick_start: std::time::Instant;

            loop {
                tick_start=std::time::Instant::now();
                
                // Getting the game state
                let mut _game_state=game_state::get();

                // Looping over players
                let mut index=0;
                loop {
                    if index >= _game_state.players.len() {
                        break;
                    }

                    if _game_state.players[index].last_keep_alive.elapsed().as_secs() > 20 {
                        // Kicking players when no keep alive packets are sent
                        logging::_info(format!("We have stopped recieving communications from player \"{}\"! Disconnecting...", _game_state.players[index].nick));
                        _game_state.players.remove(index);
                    } else {
                        // MOVEMENT
                        if _game_state.players[index].next_move_direction.is_some() {
                            let next_move_direction = _game_state.players[index].next_move_direction.unwrap();

                            // To calculate the diagonal values (im not used to such kind of math):
                            //
                            //  normal_val
                            // ------------
                            //      √2
                            //

                            match next_move_direction {
                                0 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x += 14.0;
                                    } else {
                                        _game_state.players[index].x += 7.0;
                                    }
                                },
                                1 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x += 9.89949493661;
                                        _game_state.players[index].y += 9.89949493661;
                                    } else {
                                        _game_state.players[index].x += 4.94974746831;
                                        _game_state.players[index].y += 4.94974746831;
                                    }
                                },
                                2 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].y += 14.0;
                                    } else {
                                        _game_state.players[index].y += 7.0;
                                    }
                                },
                                3 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x -= 9.89949493661;
                                        _game_state.players[index].y += 9.89949493661;
                                    } else {
                                        _game_state.players[index].x -= 4.94974746831;
                                        _game_state.players[index].y += 4.94974746831;
                                    }
                                },
                                4 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x -= 14.0;
                                    } else {
                                        _game_state.players[index].x -= 7.0;
                                    }
                                },
                                5 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x -= 9.89949493661;
                                        _game_state.players[index].y -= 9.89949493661;
                                    } else {
                                        _game_state.players[index].x -= 4.94974746831;
                                        _game_state.players[index].y -= 4.94974746831;
                                    }
                                },
                                6 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].y -= 14.0;
                                    } else {
                                        _game_state.players[index].y -= 7.0;
                                    }
                                },
                                7 => {
                                    if _game_state.players[index].is_running {
                                        _game_state.players[index].x += 9.89949493661;
                                        _game_state.players[index].y -= 9.89949493661;
                                    } else {
                                        _game_state.players[index].x += 4.94974746831;
                                        _game_state.players[index].y -= 4.94974746831;
                                    }
                                },
                                _ => {}
                            }

                            _game_state.players[index].next_move_direction = None;
                        }
                    }

                    index+=1;
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
