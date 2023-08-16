// FRAME FOR TYPING THE IP IN SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $ver_info_texture:expr,
        $server_conn_err_texture:expr,
        $invalid_ip_texture:expr,
        $game_stage:expr,
        $input:expr,
        $cursor:expr,
        $flickering_cursor:expr,
        $start_of_error_display:expr,
        $error_displayed:expr,
        $server_name:expr,
        $server_name_len:expr
    ) => {
        if $error_displayed == 0 {
            // Events
            for event in $event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '0'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '1'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '2'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '3'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '4'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '5'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '6'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '7'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '8'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '9'); $cursor += 1; },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpPeriod), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, '.'); $cursor += 1; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Semicolon), .. } => if $input.len()<21 && $cursor<21 { $input.insert($cursor as usize, ':'); $cursor += 1; },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => if $cursor>0 { $input.remove($cursor as usize - 1); $cursor -= 1 },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Delete), .. } => if ($cursor as usize)<$input.len() { $input.remove($cursor as usize); },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => if $cursor>0 { $cursor -= 1 },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => if $cursor<$input.len() as u8 { $cursor += 1 },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                        if std::net::SocketAddr::from_str(&$input).is_ok() {
                            let server_name = data::http_client::get().get("http://".to_owned() + &$input + "/server_name").send();
                            if server_name.is_ok() {
                                $server_name = server_name.unwrap().text().unwrap().clone();
                                $server_name_len = ($server_name.len() as u32 + 13) * 10;
                                data::server_ip::init( "http://".to_owned() + &$input );
                                $input = "fungi".to_owned();
                                $cursor = 5;
                                $game_stage = 1;
                            } else {
                                $error_displayed = 1;
                                $start_of_error_display = std::time::Instant::now();
                            }
                        } else {
                            $error_displayed = 2;
                            $start_of_error_display = std::time::Instant::now();
                        }
                    },
                    _ => continue
                }
                break;
            }

            // Drawing to the screen
            $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            $canvas.clear();

            $flickering_cursor += 1;
            if $flickering_cursor > 20 {
                $flickering_cursor = 0;
            } 

            if $flickering_cursor < 11 {
                $canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
                $canvas.fill_rect(sdl2::rect::Rect::new(50+(15*($cursor as i32)), 73, 15, 2)).unwrap();
            }

            if $input.len() > 0 {
                $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&$input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, 15 * ($input.len() as u32), 30))).unwrap();
            }

            $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(925, 695, 350, 20))).unwrap();

            $canvas.present();
        } else {
            // Displaying errors
            for event in $event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                    _ => {}
                }
            }

            if $start_of_error_display.elapsed().as_millis() > 2500 {
                $error_displayed = 0;
            } else {
                $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                $canvas.clear();

                match $error_displayed {
                    1 => $canvas.copy(&$server_conn_err_texture, None, Some(sdl2::rect::Rect::new(50, 50, 405, 30))).unwrap(),
                    2 => $canvas.copy(&$invalid_ip_texture, None, Some(sdl2::rect::Rect::new(50, 50, 165, 30))).unwrap(),
                    _ => {}
                }

                $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(925, 695, 350, 20))).unwrap();

                $canvas.present();
            }
        }
    };
} 

pub(crate) use frame;
