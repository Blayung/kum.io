macro_rules! spawn {
    () => {
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

                            let multiplier;
                            if _game_state.players[index].is_running {
                                multiplier = 2;
                            } else {
                                multiplier = 1;
                            }

                            if next_move_direction == 0 {
                                _game_state.players[index].x += 8 * multiplier;
                            }
                            else if next_move_direction == 1 {
                                _game_state.players[index].x += 8 * multiplier;
                                _game_state.players[index].y += 8 * multiplier;
                            }
                            else if next_move_direction == 2 {
                                _game_state.players[index].y += 8 * multiplier;
                            }
                            else if next_move_direction == 3 {
                                _game_state.players[index].x -= 8 * multiplier;
                                _game_state.players[index].y += 8 * multiplier;
                            }
                            else if next_move_direction == 4 {
                                _game_state.players[index].x -= 8 * multiplier;
                            }
                            else if next_move_direction == 5 {
                                _game_state.players[index].x -= 8 * multiplier;
                                _game_state.players[index].y -= 8 * multiplier;
                            }
                            else if next_move_direction == 6 {
                                _game_state.players[index].y -= 8 * multiplier;
                            }
                            else if next_move_direction == 7 {
                                _game_state.players[index].x += 8 * multiplier;
                                _game_state.players[index].y -= 8 * multiplier;
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
    }
}

pub(crate) use spawn;
