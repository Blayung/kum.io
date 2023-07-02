mod data;

use std::str::FromStr;

const SCREEN_HEIGHT: u32 = 500;
const SCREEN_WIDTH: u32 = 500;

pub fn main() {
    // Init
    data::http_client::init();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let sdl_ttf_context = sdl2::ttf::init().unwrap();
    let sdl_ttf_font = sdl_ttf_context.load_font(std::path::Path::new("monospace.medium.ttf"), 128).unwrap();

    // The main loop
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut frame_start: std::time::Instant; 

    let mut game_stage: u8 = 0;

    let mut input = "193.107.8.49:8888".to_owned();
    let mut flickering_cursor: u8 = 0;
    let mut letter_pressed: Option<char>;
    let mut shift_pressed = false;
    let server_conn_err_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Couldn't connect to server!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let invalid_ip_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("Invalid IP!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();
    let nick_taken_texture = texture_creator.create_texture_from_surface(sdl_ttf_font.render("This nick is already taken!").blended(sdl2::pixels::Color::RGB(255,0,0)).unwrap()).unwrap();

    let mut forward_pressed = false;
    let mut right_pressed = false;
    let mut backward_pressed = false;
    let mut left_pressed = false;

    'main_loop: loop {
        frame_start = std::time::Instant::now();

        if game_stage == 0 { // Typing the ip in
            // Events
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main_loop,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => if input.len()<21 { input += "0"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => if input.len()<21 { input += "1"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => if input.len()<21 { input += "2"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => if input.len()<21 { input += "3"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => if input.len()<21 { input += "4"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => if input.len()<21 { input += "5"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => if input.len()<21 { input += "6"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => if input.len()<21 { input += "7"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => if input.len()<21 { input += "8"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => if input.len()<21 { input += "9"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpPeriod), .. } => if input.len()<21 { input += "." },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Semicolon), .. } => if input.len()<21 { input += ":"; },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => if input.len()>0 { input.pop(); },
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                        if std::net::SocketAddr::from_str(&input).is_ok() {
                            if data::http_client::get().get("http://".to_owned() + &input + "/server_name").send().is_ok() {
                                data::server_ip::init( "http://".to_owned() + &input );
                                input="fungi".to_owned();
                                game_stage = 1;
                            } else {
                                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                                canvas.clear();
                                canvas.copy(&server_conn_err_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                                canvas.present();
                                std::thread::sleep(std::time::Duration::new(3,0));
                            }
                        } else {
                            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            canvas.clear();
                            canvas.copy(&invalid_ip_texture, None, Some(sdl2::rect::Rect::new(50, 50, 275, 50))).unwrap();
                            canvas.present();
                            std::thread::sleep(std::time::Duration::new(3,0));
                        }
                    },
                    _ => continue
                }
                break;
            }

            // Drawing to the screen
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            flickering_cursor += 1;
            if flickering_cursor > 10 {
                flickering_cursor = 0;
            }

            if flickering_cursor < 6 {
                input.push('_');
                canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*input.len()).try_into().unwrap(), 30))).unwrap();
                input.pop();
            } else if input.len() != 0 {
                canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*input.len()).try_into().unwrap(), 30))).unwrap();
            }

            canvas.present();
        }
        else if game_stage == 1 // Typing the nick in
        {
            // Events
            letter_pressed = None;

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main_loop,
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
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => letter_pressed = Some('q'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } => letter_pressed = Some('w'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::E), .. } => letter_pressed = Some('e'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => letter_pressed = Some('r'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::T), .. } => letter_pressed = Some('t'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Y), .. } => letter_pressed = Some('y'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::U), .. } => letter_pressed = Some('u'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::I), .. } => letter_pressed = Some('i'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::O), .. } => letter_pressed = Some('o'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), .. } => letter_pressed = Some('p'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } => letter_pressed = Some('a'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => letter_pressed = Some('s'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => letter_pressed = Some('d'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => letter_pressed = Some('f'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => letter_pressed = Some('g'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), .. } => letter_pressed = Some('h'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::J), .. } => letter_pressed = Some('j'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::K), .. } => letter_pressed = Some('k'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::L), .. } => letter_pressed = Some('l'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Z), .. } => letter_pressed = Some('z'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::X), .. } => letter_pressed = Some('x'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => letter_pressed = Some('c'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::V), .. } => letter_pressed = Some('v'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::B), .. } => letter_pressed = Some('b'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::N), .. } => letter_pressed = Some('n'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::M), .. } => letter_pressed = Some('m'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), .. } => letter_pressed = Some('-'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => letter_pressed = Some(' '),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => letter_pressed = Some('.'),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpMinus), .. } => letter_pressed = Some(','),
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => shift_pressed = true,
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => shift_pressed = false,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                        if input.len() != 0 {
                            let response=data::http_client::get().post(data::server_ip::get().to_owned()+"/register").body(input.clone()).send().unwrap();
                            if response.status().is_success() {
                                data::credentials::init((input.clone(),response.text().unwrap()));

                                game_stage = 2;

                                // Keep alive thread
                                std::thread::spawn(|| {
                                    let mut keep_alive_start: std::time::Instant;
                                    loop {
                                        keep_alive_start = std::time::Instant::now();
                                        data::http_client::get().post(data::server_ip::get().to_owned()+"/keep_alive").body(data::credentials::get().0.to_owned() + "," + &data::credentials::get().1).send().unwrap();
                                        std::thread::sleep(std::time::Duration::new(20, 0).checked_sub(keep_alive_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
                                    }
                                });

                                // Every-tick thread
                                std::thread::spawn(|| {
                                    let mut tick_start: std::time::Instant;
                                    loop {
                                        tick_start = std::time::Instant::now();
                                    
                                        data::game_state::update();

                                        let to_send_data = data::to_send_data::get();

                                        if to_send_data.move_direction.is_some() {
                                            data::http_client::get().post(data::server_ip::get().to_owned()+"/move").body(to_send_data.move_direction.unwrap().to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
                                        }
                                        if to_send_data.direction.is_some() {
                                            data::http_client::get().post(data::server_ip::get().to_owned()+"/rotate").body(to_send_data.direction.unwrap().to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
                                        }

                                        data::to_send_data::set(data::to_send_data::ToSendData {
                                            move_direction: None,
                                            direction: None
                                        });
                                        
                                        std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
                                    }
                                });
                            } else {
                                canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                                canvas.clear();
                                canvas.copy(&nick_taken_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                                canvas.present();
                                std::thread::sleep(std::time::Duration::new(3,0));
                            }
                        }
                    },
                    _ => {}
                }
            }

            if letter_pressed.is_some() {
                let letter = letter_pressed.unwrap();
                if letter == '.' {
                    if input.len() > 0 {
                        input.pop();
                    }
                } else if input.len() < 20 {
                    if letter == ',' {
                        input.push('-');
                    } else {
                        if shift_pressed {
                            if letter == '-' {
                                input.push('_');
                            } else {
                                input.push(letter.to_ascii_uppercase());
                            }
                        } else {
                            input.push(letter);
                        }
                    }
                }
            }

            // Drawing to the screen
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            flickering_cursor += 1;
            if flickering_cursor > 10 {
                flickering_cursor = 0;
            }

            if flickering_cursor < 6 {
                input.push('_');
                canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*input.len()).try_into().unwrap(), 30))).unwrap();
                input.pop();
            } else if input.len() != 0 {
                canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*input.len()).try_into().unwrap(), 30))).unwrap();
            }

            canvas.present();
        }
        else if game_stage == 2 // Main game!!! 
        {
            // Events
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main_loop,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => forward_pressed = true,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => right_pressed = true,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => backward_pressed = true,
                    sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => left_pressed = true,
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => forward_pressed = false,
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => right_pressed = false,
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => backward_pressed = false,
                    sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => left_pressed = false,
                    _ => {}
                }
            }

            // Every-frame pre-drawing stuff
            let mut to_send_data = data::to_send_data::get();

            if forward_pressed && right_pressed && left_pressed && backward_pressed {}
            else if forward_pressed && right_pressed && left_pressed {
                to_send_data.move_direction = Some('6');
                data::to_send_data::set(to_send_data);
            } else if forward_pressed && backward_pressed && left_pressed { 
                to_send_data.move_direction = Some('4');
                data::to_send_data::set(to_send_data);
            } else if forward_pressed && backward_pressed && right_pressed { 
                to_send_data.move_direction = Some('0');
                data::to_send_data::set(to_send_data);
            } else if backward_pressed && left_pressed && right_pressed {
                to_send_data.move_direction = Some('2');
                data::to_send_data::set(to_send_data);
            } else if forward_pressed && right_pressed {
                to_send_data.move_direction = Some('7');
                data::to_send_data::set(to_send_data);
            } else if forward_pressed && left_pressed {
                to_send_data.move_direction = Some('5');
                data::to_send_data::set(to_send_data);
            } else if backward_pressed && right_pressed {
                to_send_data.move_direction = Some('1');
                data::to_send_data::set(to_send_data);
            } else if backward_pressed && left_pressed {
                to_send_data.move_direction = Some('3');
                data::to_send_data::set(to_send_data);
            } else if forward_pressed {
                to_send_data.move_direction = Some('6');
                data::to_send_data::set(to_send_data);
            } else if right_pressed {
                to_send_data.move_direction = Some('0');
                data::to_send_data::set(to_send_data);
            } else if left_pressed {
                to_send_data.move_direction = Some('4');
                data::to_send_data::set(to_send_data);
            } else if backward_pressed {
                to_send_data.move_direction = Some('2');
                data::to_send_data::set(to_send_data);
            }

            println!("{:#?}", data::game_state::get());
            println!("{:#?}", data::to_send_data::get());

            // Drawing to the screen
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
            for player in data::game_state::get().players {
                canvas.fill_rect(sdl2::rect::Rect::new(player.x, player.y, 50, 50)).unwrap();
            }

            canvas.present();
        }

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 16666666).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }

    // Logging out
    data::http_client::get().post(data::server_ip::get().to_owned() + "/log_out").body((&data::credentials::get().0).to_owned().to_owned() + "," + &data::credentials::get().1).send().unwrap();
}
