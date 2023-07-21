// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $player_texture:expr,
        $grass_texture:expr
    ) => {
        // Getting to_send_data
        let mut to_send_data = data::to_send_data::get();

        // Events
        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => to_send_data.mov_forward = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => to_send_data.mov_backward = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => to_send_data.mov_left = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => to_send_data.mov_right = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } => to_send_data.mov_run = true,
                _ => {}
            }
        }

        // Setting to_send_data
        data::to_send_data::set(to_send_data);

        // Drawing
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
        let camera_x_offset = game_state.players[our_player].x as i32 - 605;
        let camera_y_offset = game_state.players[our_player].y as i32 - 325;

        // Rendering the grass (has to be first, serves as a background to the whole scene, and is used instead of canvas.clear())
        $canvas.copy(&$grass_texture, None, Some(sdl2::rect::Rect::new((43 - (game_state.players[our_player].x as i32 % 43)) - 86, (43 - (game_state.players[our_player].y as i32 % 43)) - 86, 1376, 817))).unwrap();

        // Rendering the players
        for player in &game_state.players {
            text::render_dynamic_text(&mut $canvas, &$texture_creator, &$sdl_ttf_font, &player.nick, sdl2::pixels::Color::RGB(255,255,255), Some(sdl2::pixels::Color::RGBA(0,0,0,100)), (player.x as i32) - camera_x_offset, (player.y as i32) - camera_y_offset - 30, 25, 0);
            $canvas.copy(&$player_texture, None, Some(sdl2::rect::Rect::new((player.x as i32) - camera_x_offset, (player.y as i32) - camera_y_offset, 70, 70))).unwrap();
        }

        // Updating the screen
        $canvas.present();
    };
} 

pub(crate) use frame;
