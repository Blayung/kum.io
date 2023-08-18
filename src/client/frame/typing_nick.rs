// FRAME FOR TYPING THE NICK IN SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $font:expr,
        $text_input:expr,
        $ver_info_texture:expr,
        $nick_taken_texture:expr,
        $game_stage:expr,
        $input:expr,
        $cursor:expr,
        $flickering_cursor:expr,
        $start_of_error_display:expr,
        $error_displayed:expr
    ) => {
        if $error_displayed == 0 {
            // Events
            let mut letter_pressed = None;

            for event in $event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,

                    sdl2::event::Event::TextInput { text, .. } =>
                        if $input.len()<20 && $cursor<20 {
                            $input.insert_str($cursor as usize, &text);
                        }

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => 
                        if $cursor>0 {
                            $input.remove($cursor as usize - 1);
                            $cursor -= 1;
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Delete), .. } =>
                        if ($cursor as usize) < $input.len() {
                            $input.remove($cursor as usize);
                        },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => if $cursor>0 { $cursor -= 1 },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => if $cursor<$input.len() as u8 { $cursor += 1 },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                        if $input.len() != 0 {
                            let response=data::http_client::get().post(data::server_ip::get().to_owned()+"/register").body($input.clone()).send().unwrap();
                            if response.status().is_success() {
                                data::credentials::init(($input.clone(),response.text().unwrap()));

                                $game_stage = 2;
                                $text_input.stop();

                                keep_alive_thread::spawn!();
                                every_tick_thread::spawn!();
                            } else {
                                $error_displayed = 1;
                                $start_of_error_display = std::time::Instant::now();
                            }
                        }
                    },
                    _ => {}
                }
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
                $canvas.copy(&$texture_creator.create_texture_from_surface($font.render(&$input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, 15 * ($input.len() as u32), 30))).unwrap();
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
                    1 => $canvas.copy(&$nick_taken_texture, None, Some(sdl2::rect::Rect::new(50, 50, 405, 30))).unwrap(),
                    _ => {}
                }

                $canvas.copy(&$ver_info_texture, None, Some(sdl2::rect::Rect::new(925, 695, 350, 20))).unwrap();

                $canvas.present();
            }
        }
    };
} 

pub(crate) use frame;
