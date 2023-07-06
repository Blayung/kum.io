// FRAME FOR TYPING THE IP IN SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $server_conn_err_texture:expr,
        $invalid_ip_texture:expr,
        $game_stage:expr,
        $input:expr,
        $flickering_cursor:expr
    ) => {
        // Events
        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => if $input.len()<21 { $input += "0"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => if $input.len()<21 { $input += "1"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => if $input.len()<21 { $input += "2"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => if $input.len()<21 { $input += "3"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => if $input.len()<21 { $input += "4"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => if $input.len()<21 { $input += "5"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => if $input.len()<21 { $input += "6"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => if $input.len()<21 { $input += "7"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => if $input.len()<21 { $input += "8"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => if $input.len()<21 { $input += "9"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpPeriod), .. } => if $input.len()<21 { $input += "." },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Semicolon), .. } => if $input.len()<21 { $input += ":"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => if $input.len()>0 { $input.pop(); },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                    if std::net::SocketAddr::from_str(&$input).is_ok() {
                        if data::http_client::get().get("http://".to_owned() + &$input + "/server_name").send().is_ok() {
                            data::server_ip::init( "http://".to_owned() + &$input );
                            $input="fungi".to_owned();
                            $game_stage = 1;
                        } else {
                            $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            $canvas.clear();
                            $canvas.copy(&$server_conn_err_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                            $canvas.present();
                            std::thread::sleep(std::time::Duration::new(3,0));
                        }
                    } else {
                        $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                        $canvas.clear();
                        $canvas.copy(&$invalid_ip_texture, None, Some(sdl2::rect::Rect::new(50, 50, 275, 50))).unwrap();
                        $canvas.present();
                        std::thread::sleep(std::time::Duration::new(3,0));
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
        if $flickering_cursor > 10 {
            $flickering_cursor = 0;
        }

        if $flickering_cursor < 6 {
            $input.push('_');
            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&$input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*$input.len()).try_into().unwrap(), 30))).unwrap();
            $input.pop();
        } else if $input.len() != 0 {
            $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&$input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*$input.len()).try_into().unwrap(), 30))).unwrap();
        }

        $canvas.present();
    };
} 

pub(crate) use frame;
