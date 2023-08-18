// FRAME FOR THE MAIN GAME SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $font:expr,
        $text_input:expr,
        $player_texture:expr,
        $grass_texture:expr,
        $ver_info_texture:expr,
        $input:expr,
        $cursor:expr,
        $flickering_cursor:expr,
        $server_name:expr,
        $server_name_len:expr,
        $debug_menu:expr,
        $last_elapsed:expr,
        $is_going_forward:expr,
        $is_going_backward:expr,
        $is_going_left:expr,
        $is_going_right:expr,
        $is_running:expr
    ) => {
        // Getting to_send_data
        let mut to_send_data = data::to_send_data::get();

        // Events
        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break $main_loop,

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => if $text_input.is_active() { $text_input.stop(); } else { break $main_loop; },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), repeat: false, .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), repeat: false, .. } =>
                    if $text_input.is_active() {
                        // <sending the message here>
                        $text_input.stop();
                    } else { $text_input.start(); },

                sdl2::event::Event::TextInput { text, .. } =>
                    if $input.len()<20 && $cursor<20 && text.len() == 1 {
                        let first_char = text.chars().next().unwrap();
                        if "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 -_".contains(first_char) {
                            $input.insert($cursor as usize, first_char);
                            $cursor += 1;
                        }
                    }

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => 
                    if $text_input.is_active() && $cursor>0 {
                        $input.remove($cursor as usize - 1);
                        $cursor -= 1;
                    },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Delete), .. } =>
                    if $text_input.is_active() && ($cursor as usize) < $input.len() {
                        $input.remove($cursor as usize);
                    },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => if $text_input.is_active() && $cursor>0 { $cursor -= 1 },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => if $text_input.is_active() && $cursor<$input.len() as u8 { $cursor += 1 },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => if !$text_input.is_active() { $is_going_forward = true; },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), repeat: false, .. } => if !$text_input.is_active() { $is_going_forward = false },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => if !$text_input.is_active() { $is_going_backward = true },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), repeat: false, .. } => if !$text_input.is_active() { $is_going_backward = false },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => if !$text_input.is_active() { $is_going_left = true },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), repeat: false, .. } => if !$text_input.is_active() { $is_going_left = false },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => if !$text_input.is_active() { $is_going_right = true },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), repeat: false, .. } => if !$text_input.is_active() { $is_going_right = false },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } => if !$text_input.is_active() { $is_running = true },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), repeat: false, .. } => if !$text_input.is_active() { $is_running = false },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F3), repeat: false, .. } => $debug_menu ^= true,

                sdl2::event::Event::MouseMotion { x, y, .. } => {
                    let dy = (y as f64 - 360.0);
                    if dy < 0.0 {
                        to_send_data.rotate = Some(((dy.atan2(x as f64 - 640.0) * 180.0 / std::f64::consts::PI) + 360.0) as u16);
                    } else {
                        to_send_data.rotate = Some((dy.atan2(x as f64 - 640.0) * 180.0 / std::f64::consts::PI) as u16);
                    }
                }

                _ => {}
            }
        }

        // Movement
        if $is_going_forward && $is_going_right && $is_going_left && $is_going_backward {}
        else if $is_going_forward && $is_going_right && $is_going_left {
            if $is_running {
                to_send_data.move_direction = Some(14);
            } else {
                to_send_data.move_direction = Some(6);
            }
        } else if $is_going_forward && $is_going_backward && $is_going_left {
            if $is_running {
                to_send_data.move_direction = Some(12);
            } else {
                to_send_data.move_direction = Some(4);
            }
        } else if $is_going_forward && $is_going_backward && $is_going_right {
            if $is_running {
                to_send_data.move_direction = Some(8);
            } else {
                to_send_data.move_direction = Some(0);
            }
        } else if $is_going_backward && $is_going_left && $is_going_right {
            if $is_running {
                to_send_data.move_direction = Some(10);
            } else {
                to_send_data.move_direction = Some(2);
            }
        } else if $is_going_forward && $is_going_right {
            if $is_running {
                to_send_data.move_direction = Some(15);
            } else {
                to_send_data.move_direction = Some(7);
            }
        } else if $is_going_forward && $is_going_left {
            if $is_running {
                to_send_data.move_direction = Some(13);
            } else {
                to_send_data.move_direction = Some(5);
            }
        } else if $is_going_backward && $is_going_right {
            if $is_running {
                to_send_data.move_direction = Some(9);
            } else {
                to_send_data.move_direction = Some(1);
            }
        } else if $is_going_backward && $is_going_left {
            if $is_running {
                to_send_data.move_direction = Some(11);
            } else {
                to_send_data.move_direction = Some(3);
            }
        } else if $is_going_forward {
            if $is_running {
                to_send_data.move_direction = Some(14);
            } else {
                to_send_data.move_direction = Some(6);
            }
        } else if $is_going_right {
            if $is_running {
                to_send_data.move_direction = Some(8);
            } else {
                to_send_data.move_direction = Some(0);
            }
        } else if $is_going_left {
            if $is_running {
                to_send_data.move_direction = Some(12);
            } else {
                to_send_data.move_direction = Some(4);
            }
        } else if $is_going_backward {
            if $is_running {
                to_send_data.move_direction = Some(10);
            } else {
                to_send_data.move_direction = Some(2);
            }
        }

        // Setting to_send_data
        data::to_send_data::set(to_send_data);

        // Getting the game state
        let game_state = data::game_state::get();
        //println!("{:#?}", game_state); 
        
        // Finding our player
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

        // Drawing
        // Rendering the grass (has to be first, serves as a background to the whole scene, and is used instead of canvas.clear())
        $canvas.copy(&$grass_texture, None, Some(sdl2::rect::Rect::new((43 - (game_state.players[our_player].x as i32 % 43)) - 86, (43 - (game_state.players[our_player].y as i32 % 43)) - 86, 1376, 817))).unwrap();

        // Getting the camera x & y offsets for the camera scrolling effect.
        let camera_x_offset = game_state.players[our_player].x as i32 - 605;
        let camera_y_offset = game_state.players[our_player].y as i32 - 325;

        // Rendering the players
        $canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,150));
        for player in &game_state.players {
            let x = (player.x as i32) - camera_x_offset;
            let y = (player.y as i32) - camera_y_offset;
            let text_width = 12 * (player.nick.len() as u32);
            let text_x = x + ((70 - text_width as i32) / 2);
            let text_y = y - 30;
            $canvas.fill_rect(sdl2::rect::Rect::new(text_x - 1, text_y, text_width + 2, 24)).unwrap();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&player.nick).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(text_x, text_y, text_width, 24))).unwrap();
            $canvas.copy_ex(&$player_texture, None, Some(sdl2::rect::Rect::new(x, y, 70, 70)), player.direction as f64, sdl2::rect::Point::new(35, 35), false, false).unwrap();
        }

        // Rendering the chat
        let mut message = 0;
        loop {
            if message == game_state.chat_messages.len() {
                break;
            }
            let width = ( 3 + game_state.chat_messages[message].nick.len() as u32 + game_state.chat_messages[message].message.len() as u32 ) * 12;
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("<".to_owned() + &game_state.chat_messages[message].nick + "> " + &game_state.chat_messages[message].message)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(1275 - width as i32, 340 - (24 * (game_state.chat_messages.len() - message) as i32), width, 24))).unwrap();
            message += 1;
        }

        if $text_input.is_active() {
            $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(5, 5, 350, 20))).unwrap();
        }

        // Rendering the debug menu
        if $debug_menu {
            $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(5, 5, 350, 20))).unwrap();

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

            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("FPS/CTPS (120/20): ".to_owned() + &fps + "/" + &ctps)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 25, (fps.len() as u32 + ctps.len() as u32 + 19) * 10, 20))).unwrap();

            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Server name: ".to_owned() + &$server_name)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 45, $server_name_len, 20))).unwrap();

            let x = &game_state.players[our_player].x.to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("X: ".to_owned() + x)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 65, (x.len() as u32 + 4) * 10, 20))).unwrap();
            let y = &game_state.players[our_player].y.to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Y: ".to_owned() + y)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 85, (y.len() as u32 + 4) * 10, 20))).unwrap();

            let direction = &game_state.players[our_player].direction.to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Direction: ".to_owned() + direction)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 105, (direction.len() as u32 + 11) * 10, 20))).unwrap();

            let alive_chat_messages_str = &game_state.chat_messages.len().to_string();
            $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&("Alive chat messages: ".to_owned() + alive_chat_messages_str)).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(5, 125, (alive_chat_messages_str.len() as u32 + 21) * 10, 20))).unwrap();
        }

        // Updating the screen
        $canvas.present();
    };
} 

pub(crate) use frame;
