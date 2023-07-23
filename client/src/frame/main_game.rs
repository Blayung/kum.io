// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $player_texture:expr,
        $grass_texture:expr,
        $server_name:expr,
        $server_name_len:expr,
        $debug_menu:expr,
        $last_elapsed:expr
    ) => {
        // Getting to_send_data
        let mut to_send_data = data::to_send_data::get();

        // Events
        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => to_send_data.mov_forward ^= true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => to_send_data.mov_backward ^= true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => to_send_data.mov_left ^= true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => to_send_data.mov_right ^= true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } => to_send_data.mov_run ^= true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F3), repeat: false, .. } => $debug_menu ^= true,
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
        $canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,150));
        for player in &game_state.players {
            let x = (player.x as i32) - camera_x_offset;
            let y = (player.y as i32) - camera_y_offset;
            let text_width = 12 * (player.nick.len() as u32);
            let text_x = x + ((70 - text_width as i32) / 2);
            let text_y = y - 30;
            $canvas.fill_rect(sdl2::rect::Rect::new(text_x, text_y, text_width, 24)).unwrap();
            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&player.nick).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(text_x, text_y, text_width, 24))).unwrap();
            $canvas.copy(&$player_texture, None, Some(sdl2::rect::Rect::new(x, y, 70, 70))).unwrap();
        }

        // Rendering the debug menu
        if $debug_menu {
            let fps;
            let fps_delay = $last_elapsed.as_nanos();
            if fps_delay == 0 {
                fps = "∞".to_owned();
            } else {
                fps = (1000000000/fps_delay).to_string();
            }

            let ctps;
            let ctps_delay = data::ctps_elapsed::get().as_nanos();
            if ctps_delay == 0 {
                ctps = "∞".to_owned();
            } else {
                ctps = (1000000000/ctps_delay).to_string();
            }

            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&("FPS/CTPS (120/20): ".to_owned() + &fps + "/" + &ctps)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 5, (fps.len() as u32 + ctps.len() as u32 + 19) * 10, 20))).unwrap();

            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&("Server name: ".to_owned() + &$server_name)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 25, $server_name_len, 20))).unwrap();

            let x = &game_state.players[our_player].x.to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&("X: ".to_owned() + x)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 45, (x.len() as u32 + 4) * 10, 20))).unwrap();
            let y = &game_state.players[our_player].y.to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&("Y: ".to_owned() + y)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 65, (y.len() as u32 + 4) * 10, 20))).unwrap();
        }

        // Updating the screen
        $canvas.present();
    };
} 

pub(crate) use frame;
