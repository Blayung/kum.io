// FRAME FOR TYPING THE NICK IN SCREEN
macro_rules! frame {
    (
        $main_loop:lifetime,
        $canvas:expr,
        $event_pump:expr,
        $texture_creator:expr,
        $sdl_ttf_font:expr,
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

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => letter_pressed = Some(' '),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpMinus), .. } => letter_pressed = Some('-'),

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => letter_pressed = Some('0'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => letter_pressed = Some('1'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => letter_pressed = Some('2'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => letter_pressed = Some('3'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => letter_pressed = Some('4'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => letter_pressed = Some('5'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => letter_pressed = Some('6'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => letter_pressed = Some('7'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => letter_pressed = Some('8'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => letter_pressed = Some('9'),

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('q');
                        } else {
                            letter_pressed = Some('Q');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('w');
                        } else {
                            letter_pressed = Some('W');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::E), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('e');
                        } else {
                            letter_pressed = Some('E');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('r');
                        } else {
                            letter_pressed = Some('R');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::T), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('t');
                        } else {
                            letter_pressed = Some('T');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Y), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('y');
                        } else {
                            letter_pressed = Some('Y');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::U), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('u');
                        } else {
                            letter_pressed = Some('U');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::I), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('i');
                        } else {
                            letter_pressed = Some('I');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::O), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('o');
                        } else {
                            letter_pressed = Some('O');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('p');
                        } else {
                            letter_pressed = Some('P');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('a');
                        } else {
                            letter_pressed = Some('A');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('s');
                        } else {
                            letter_pressed = Some('S');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('d');
                        } else {
                            letter_pressed = Some('D');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('f');
                        } else {
                            letter_pressed = Some('F');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('g');
                        } else {
                            letter_pressed = Some('G');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('h');
                        } else {
                            letter_pressed = Some('H');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::J), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('j');
                        } else {
                            letter_pressed = Some('J');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::K), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('k');
                        } else {
                            letter_pressed = Some('K');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::L), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('l');
                        } else {
                            letter_pressed = Some('L');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Z), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('z');
                        } else {
                            letter_pressed = Some('Z');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::X), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('x');
                        } else {
                            letter_pressed = Some('X');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('c');
                        } else {
                            letter_pressed = Some('C');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::V), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('v');
                        } else {
                            letter_pressed = Some('V');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::B), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('b');
                        } else {
                            letter_pressed = Some('B');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::N), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('n');
                        } else {
                            letter_pressed = Some('N');
                        },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::M), keymod, .. } =>
                        if ((keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD)) && keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) || (!keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) && !keymod.contains(sdl2::keyboard::Mod::CAPSMOD)) {
                            letter_pressed = Some('m');
                        } else {
                            letter_pressed = Some('M');
                        },

                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), keymod, .. } => 
                        if keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD) || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD) {
                            letter_pressed = Some('_');
                        } else {
                            letter_pressed = Some('-');
                        },

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

            if letter_pressed.is_some() {
                if $input.len()<20 && $cursor<20 {
                    $input.insert($cursor as usize, letter_pressed.unwrap());
                    $cursor += 1;
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
