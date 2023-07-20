// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $player_texture:expr,
        $grass_texture:expr,
        $forward_pressed:expr,
        $right_pressed:expr,
        $backward_pressed:expr,
        $left_pressed:expr
    ) => {
        // EVENTS
        let mut should_send_run = false;

        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => $forward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => $right_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => $backward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => $left_pressed = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => $forward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => $right_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => $backward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => $left_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } => should_send_run = true,
                _ => {}
            }
        }

        // OTHER EVERY-FRAME STUFF
        // Getting current to_send_data
        let mut to_send_data = data::to_send_data::get();

        // Movement
        if should_send_run {
            to_send_data.should_send_run = true;
        }

        if $forward_pressed && $right_pressed && $left_pressed && $backward_pressed {}
        else if $forward_pressed && $right_pressed && $left_pressed {
            to_send_data.move_direction = Some('6');
        } else if $forward_pressed && $backward_pressed && $left_pressed { 
            to_send_data.move_direction = Some('4');
        } else if $forward_pressed && $backward_pressed && $right_pressed { 
            to_send_data.move_direction = Some('0');
        } else if $backward_pressed && $left_pressed && $right_pressed {
            to_send_data.move_direction = Some('2');
        } else if $forward_pressed && $right_pressed {
            to_send_data.move_direction = Some('7');
        } else if $forward_pressed && $left_pressed {
            to_send_data.move_direction = Some('5');
        } else if $backward_pressed && $right_pressed {
            to_send_data.move_direction = Some('1');
        } else if $backward_pressed && $left_pressed {
            to_send_data.move_direction = Some('3');
        } else if $forward_pressed {
            to_send_data.move_direction = Some('6');
        } else if $right_pressed {
            to_send_data.move_direction = Some('0');
        } else if $left_pressed {
            to_send_data.move_direction = Some('4');
        } else if $backward_pressed {
            to_send_data.move_direction = Some('2');
        }

        // Setting to_send_data
        data::to_send_data::set(to_send_data);

        // Getting the game state
        let game_state = data::game_state::get();
        //println!("{:#?}", game_state);

        // Finding our player and getting the camera x & y offsets for the camera scrolling effect.
        let mut index = 0;
        let mut our_player = 0;
        let mut did_find = false;
        loop {
            if index == game_state.players.len() {
                break;
            }
            if game_state.players[index].nick == data::credentials::get().0 { 
                did_find = true;
                our_player = index;
                break;
            }
            index += 1;
        }
        if !did_find {
            continue;
        }
        let camera_x_offset = game_state.players[our_player].x as i32 - 610;
        let camera_y_offset = game_state.players[our_player].y as i32 - 330;

        // Rendering the grass (has to be first, serves as a background to the whole scene, and is used instead of canvas.clear())
        $canvas.copy(&$grass_texture, None, Some(sdl2::rect::Rect::new((43 - (game_state.players[our_player].x as i32 % 43)) - 86, (43 - (game_state.players[our_player].y as i32 % 43)) - 86, 1376, 817))).unwrap();

        // Rendering the players
        for player in &game_state.players {
            $canvas.copy(&$player_texture, None, Some(sdl2::rect::Rect::new((player.x as i32) - camera_x_offset, (player.y as i32) - camera_y_offset, 60, 60))).unwrap();
        }

        // Updating the screen
        $canvas.present();
    };
} 

pub(crate) use frame;
