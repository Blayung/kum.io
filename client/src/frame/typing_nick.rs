// FRAME FOR TYPING THE NICK IN SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
        $nick_taken_texture:expr,
        $game_stage:expr,
        $input:expr,
        $cursor:expr,
        $flickering_cursor:expr,
        $letter_pressed:expr,
        $shift_pressed:expr
    ) => {
        // Events
        $letter_pressed = None;

        for event in $event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break $main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => $letter_pressed = Some('0'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => $letter_pressed = Some('1'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => $letter_pressed = Some('2'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => $letter_pressed = Some('3'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => $letter_pressed = Some('4'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => $letter_pressed = Some('5'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => $letter_pressed = Some('6'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => $letter_pressed = Some('7'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => $letter_pressed = Some('8'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => $letter_pressed = Some('9'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => $letter_pressed = Some('q'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } => $letter_pressed = Some('w'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::E), .. } => $letter_pressed = Some('e'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => $letter_pressed = Some('r'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::T), .. } => $letter_pressed = Some('t'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Y), .. } => $letter_pressed = Some('y'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::U), .. } => $letter_pressed = Some('u'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::I), .. } => $letter_pressed = Some('i'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::O), .. } => $letter_pressed = Some('o'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), .. } => $letter_pressed = Some('p'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } => $letter_pressed = Some('a'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => $letter_pressed = Some('s'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => $letter_pressed = Some('d'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => $letter_pressed = Some('f'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => $letter_pressed = Some('g'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), .. } => $letter_pressed = Some('h'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::J), .. } => $letter_pressed = Some('j'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::K), .. } => $letter_pressed = Some('k'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::L), .. } => $letter_pressed = Some('l'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Z), .. } => $letter_pressed = Some('z'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::X), .. } => $letter_pressed = Some('x'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => $letter_pressed = Some('c'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::V), .. } => $letter_pressed = Some('v'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::B), .. } => $letter_pressed = Some('b'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::N), .. } => $letter_pressed = Some('n'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::M), .. } => $letter_pressed = Some('m'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), .. } => $letter_pressed = Some('-'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => $letter_pressed = Some(' '),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => $letter_pressed = Some('.'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Delete), .. } => $letter_pressed = Some('/'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpMinus), .. } => $letter_pressed = Some(','),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => if $cursor>0 { $cursor -= 1 },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => if $cursor<$input.len() as u8 { $cursor += 1 },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => $shift_pressed = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => $shift_pressed = false,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                    if $input.len() != 0 {
                        let response=data::http_client::get().post(data::server_ip::get().to_owned()+"/register").body($input.clone()).send().unwrap();
                        if response.status().is_success() {
                            data::credentials::init(($input.clone(),response.text().unwrap()));

                            $game_stage = 2;

                            keep_alive_thread::spawn!();
                            every_tick_thread::spawn!();
                        } else {
                            $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            $canvas.clear();
                            $canvas.copy(&$nick_taken_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                            $canvas.present();
                            std::thread::sleep(std::time::Duration::new(3,0));
                        }
                    }
                },
                _ => {}
            }
        }

        if $letter_pressed.is_some() {
            let letter = $letter_pressed.unwrap();
            if letter == '.' {
                if $cursor>0 {
                    $input.remove($cursor as usize - 1);
                    $cursor -= 1;
                }
            } else if letter == '/' {
                if $cursor>0 && ($cursor as usize)<$input.len() {
                    $input.remove($cursor as usize);
                }
            } else if $input.len()<20 && $cursor<20 {
                if letter == ',' {
                    $input.insert($cursor as usize, '-');
                } else {
                    if $shift_pressed {
                        if letter == '-' {
                            $input.insert($cursor as usize, '_');
                        } else {
                            $input.insert($cursor as usize, letter.to_ascii_uppercase());
                        }
                    } else {
                        $input.insert($cursor as usize, letter);
                    }
                }
                $cursor += 1;
            }
        }

        // Drawing to the screen
        $canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        $canvas.clear();

        $flickering_cursor += 1;
        if $flickering_cursor > 10 {
            $flickering_cursor = 0;
        }

        if $flickering_cursor < 6 {
            $canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            $canvas.fill_rect(sdl2::rect::Rect::new(50+(15*($cursor as i32)), 73, 15, 2)).unwrap();
        }

        $canvas.copy(&$texture_creator.create_texture_from_surface($sdl_ttf_font.render(&$input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*$input.len()).try_into().unwrap(), 30))).unwrap();

        $canvas.present();
    };
} 

pub(crate) use frame;
