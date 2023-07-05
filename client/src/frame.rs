use crate::data;
use std::str::FromStr;

pub enum ToDrawObj {
    Rectangle(sdl2::rect::Rect, sdl2::pixels::Color),
    DynamicText {
        text: String,
        color: sdl2::pixels::Color,
        bg_color: Option<sdl2::pixels::Color>,
        x: i32,
        y: i32,
        size: u32
    },
}

pub enum FrameReturnData {
    DoNothing,
    Quit,
    DrawAndSleep(std::vec::Vec<ToDrawObj>, std::time::Duration),
    Draw(std::vec::Vec<ToDrawObj>)
}

pub fn frame(event_poll_iter: sdl2::event::EventPollIterator<'_>) -> FrameReturnData {
    let mut local_data = data::local_data::get();

    if local_data.game_stage == 0 { // Typing the ip in
        // Events
        for event in event_poll_iter {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return FrameReturnData::Quit,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => if local_data.input.len()<21 { local_data.input += "0"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => if local_data.input.len()<21 { local_data.input += "1"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => if local_data.input.len()<21 { local_data.input += "2"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => if local_data.input.len()<21 { local_data.input += "3"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => if local_data.input.len()<21 { local_data.input += "4"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => if local_data.input.len()<21 { local_data.input += "5"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => if local_data.input.len()<21 { local_data.input += "6"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => if local_data.input.len()<21 { local_data.input += "7"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => if local_data.input.len()<21 { local_data.input += "8"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => if local_data.input.len()<21 { local_data.input += "9"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpPeriod), .. } => if local_data.input.len()<21 { local_data.input += "." },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Semicolon), .. } => if local_data.input.len()<21 { local_data.input += ":"; },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => if local_data.input.len()>0 { local_data.input.pop(); },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                    if std::net::SocketAddr::from_str(&local_data.input).is_ok() {
                        if data::http_client::get().get("http://".to_owned() + &local_data.input + "/server_name").send().is_ok() {
                            data::server_ip::init( "http://".to_owned() + &local_data.input );
                            local_data.input="fungi".to_owned();
                            local_data.game_stage = 1;
                        } else {
                            //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            //canvas.clear();
                            //canvas.copy(&server_conn_err_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                            //canvas.present();
                            //std::thread::sleep(std::time::Duration::new(3,0));
                            return FrameReturnData::DrawAndSleep(
                                std::vec::Vec::from([ToDrawObj::DynamicText {
                                    text: "Couldn't connect to server!".to_owned(),
                                    color: sdl2::pixels::Color::RGB(255,255,255),
                                    bg_color: None,
                                    x: 0,
                                    y: 50,
                                    size: 30
                                }]), 
                                std::time::Duration::new(3,0)
                            );
                        }
                    } else {
                        //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                        //canvas.clear();
                        //canvas.copy(&invalid_ip_texture, None, Some(sdl2::rect::Rect::new(50, 50, 275, 50))).unwrap();
                        //canvas.present();
                        //std::thread::sleep(std::time::Duration::new(3,0));
                        return FrameReturnData::DrawAndSleep(
                            std::vec::Vec::from([ToDrawObj::DynamicText {
                                text: "Invalid IP!".to_owned(),
                                color: sdl2::pixels::Color::RGB(255,255,255),
                                bg_color: None,
                                x: 50,
                                y: 50,
                                size: 50
                            }]), 
                            std::time::Duration::new(3,0)
                        );
                    }
                },
                _ => continue
            }
            break;
        }

        // Drawing to the screen
        //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        //canvas.clear();

        local_data.flickering_cursor += 1;
        if local_data.flickering_cursor > 10 {
            local_data.flickering_cursor = 0;
        }

        if local_data.flickering_cursor < 6 {
            //local_data.input.push('_');
            //canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&local_data.input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*local_data.input.len()).try_into().unwrap(), 30))).unwrap();
            //local_data.input.pop();
            return FrameReturnData::Draw(
                std::vec::Vec::from([ToDrawObj::DynamicText {
                    text: local_data.input + "_",
                    color: sdl2::pixels::Color::RGB(255,255,255),
                    bg_color: None,
                    x: 50,
                    y: 50,
                    size: 30
                }])
            );
        } else if local_data.input.len() != 0 {
            //canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&local_data.input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*local_data.input.len()).try_into().unwrap(), 30))).unwrap();
            return FrameReturnData::Draw(
                std::vec::Vec::from([ToDrawObj::DynamicText {
                    text: local_data.input,
                    color: sdl2::pixels::Color::RGB(255,255,255),
                    bg_color: None,
                    x: 50,
                    y: 50,
                    size: 30
                }])
            );
        }

        //canvas.present();
    }
    else if local_data.game_stage == 1 // Typing the nick in
    {
        // Events
        local_data.letter_pressed = None;

        for event in event_poll_iter {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return FrameReturnData::Quit,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => local_data.letter_pressed = Some('0'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => local_data.letter_pressed = Some('1'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => local_data.letter_pressed = Some('2'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => local_data.letter_pressed = Some('3'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => local_data.letter_pressed = Some('4'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => local_data.letter_pressed = Some('5'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => local_data.letter_pressed = Some('6'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => local_data.letter_pressed = Some('7'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => local_data.letter_pressed = Some('8'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => local_data.letter_pressed = Some('9'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), .. } => local_data.letter_pressed = Some('q'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } => local_data.letter_pressed = Some('w'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::E), .. } => local_data.letter_pressed = Some('e'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::R), .. } => local_data.letter_pressed = Some('r'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::T), .. } => local_data.letter_pressed = Some('t'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Y), .. } => local_data.letter_pressed = Some('y'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::U), .. } => local_data.letter_pressed = Some('u'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::I), .. } => local_data.letter_pressed = Some('i'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::O), .. } => local_data.letter_pressed = Some('o'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::P), .. } => local_data.letter_pressed = Some('p'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } => local_data.letter_pressed = Some('a'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } => local_data.letter_pressed = Some('s'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } => local_data.letter_pressed = Some('d'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), .. } => local_data.letter_pressed = Some('f'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::G), .. } => local_data.letter_pressed = Some('g'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), .. } => local_data.letter_pressed = Some('h'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::J), .. } => local_data.letter_pressed = Some('j'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::K), .. } => local_data.letter_pressed = Some('k'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::L), .. } => local_data.letter_pressed = Some('l'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Z), .. } => local_data.letter_pressed = Some('z'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::X), .. } => local_data.letter_pressed = Some('x'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::C), .. } => local_data.letter_pressed = Some('c'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::V), .. } => local_data.letter_pressed = Some('v'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::B), .. } => local_data.letter_pressed = Some('b'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::N), .. } => local_data.letter_pressed = Some('n'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::M), .. } => local_data.letter_pressed = Some('m'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Minus), .. } => local_data.letter_pressed = Some('-'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Space), .. } => local_data.letter_pressed = Some(' '),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => local_data.letter_pressed = Some('.'),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpMinus), .. } => local_data.letter_pressed = Some(','),
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => local_data.shift_pressed = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::LShift), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::RShift), .. } => local_data.shift_pressed = false,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => {
                    if local_data.input.len() != 0 {
                        let response=data::http_client::get().post(data::server_ip::get().to_owned()+"/register").body(local_data.input.clone()).send().unwrap();
                        if response.status().is_success() {
                            data::credentials::init((local_data.input.clone(),response.text().unwrap()));

                            local_data.game_stage = 2;

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
                            //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
                            //canvas.clear();
                            //canvas.copy(&nick_taken_texture, None, Some(sdl2::rect::Rect::new(0, 50, 405, 30))).unwrap();
                            //canvas.present();
                            //std::thread::sleep(std::time::Duration::new(3,0));
                            return FrameReturnData::DrawAndSleep(
                                std::vec::Vec::from([ToDrawObj::DynamicText {
                                    text: "This nick is already taken!".to_owned(),
                                    color: sdl2::pixels::Color::RGB(255,255,255),
                                    bg_color: None,
                                    x: 0,
                                    y: 50,
                                    size: 30
                                }]), 
                                std::time::Duration::new(3,0)
                            );
                        }
                    }
                },
                _ => {}
            }
        }

        if local_data.letter_pressed.is_some() {
            let letter = local_data.letter_pressed.unwrap();
            if letter == '.' {
                if local_data.input.len() > 0 {
                    local_data.input.pop();
                }
            } else if local_data.input.len() < 20 {
                if letter == ',' {
                    local_data.input.push('-');
                } else {
                    if local_data.shift_pressed {
                        if letter == '-' {
                            local_data.input.push('_');
                        } else {
                            local_data.input.push(letter.to_ascii_uppercase());
                        }
                    } else {
                        local_data.input.push(letter);
                    }
                }
            }
        }

        // Drawing to the screen
        //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        //canvas.clear();

        local_data.flickering_cursor += 1;
        if local_data.flickering_cursor > 10 {
            local_data.flickering_cursor = 0;
        }

        if local_data.flickering_cursor < 6 {
            //local_data.input.push('_');
            //canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&local_data.input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*local_data.input.len()).try_into().unwrap(), 30))).unwrap();
            //local_data.input.pop();
            return FrameReturnData::Draw(
                std::vec::Vec::from([ToDrawObj::DynamicText {
                    text: local_data.input + "_",
                    color: sdl2::pixels::Color::RGB(255,255,255),
                    bg_color: None,
                    x: 50,
                    y: 50,
                    size: 30
                }])
            );
        } else if local_data.input.len() != 0 {
            //canvas.copy(&texture_creator.create_texture_from_surface(sdl_ttf_font.render(&local_data.input).blended(sdl2::pixels::Color::RGB(255,255,255)).unwrap()).unwrap(), None, Some(sdl2::rect::Rect::new(50, 50, (15*local_data.input.len()).try_into().unwrap(), 30))).unwrap();
            return FrameReturnData::Draw(
                std::vec::Vec::from([ToDrawObj::DynamicText {
                    text: local_data.input,
                    color: sdl2::pixels::Color::RGB(255,255,255),
                    bg_color: None,
                    x: 50,
                    y: 50,
                    size: 30
                }])
            );
        }

        //canvas.present();
    }
    else if local_data.game_stage == 2 // Main game!!! 
    {
        // Events
        for event in event_poll_iter {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return FrameReturnData::Quit,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => local_data.forward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => local_data.right_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => local_data.backward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => local_data.left_pressed = true,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => local_data.forward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => local_data.right_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => local_data.backward_pressed = false,
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => local_data.left_pressed = false,
                _ => {}
            }
        }

        // Every-frame pre-drawing stuff
        let mut to_send_data = data::to_send_data::get();

        if local_data.forward_pressed && local_data.right_pressed && local_data.left_pressed && local_data.backward_pressed {}
        else if local_data.forward_pressed && local_data.right_pressed && local_data.left_pressed {
            to_send_data.move_direction = Some('6');
            data::to_send_data::set(to_send_data);
        } else if local_data.forward_pressed && local_data.backward_pressed && local_data.left_pressed { 
            to_send_data.move_direction = Some('4');
            data::to_send_data::set(to_send_data);
        } else if local_data.forward_pressed && local_data.backward_pressed && local_data.right_pressed { 
            to_send_data.move_direction = Some('0');
            data::to_send_data::set(to_send_data);
        } else if local_data.backward_pressed && local_data.left_pressed && local_data.right_pressed {
            to_send_data.move_direction = Some('2');
            data::to_send_data::set(to_send_data);
        } else if local_data.forward_pressed && local_data.right_pressed {
            to_send_data.move_direction = Some('7');
            data::to_send_data::set(to_send_data);
        } else if local_data.forward_pressed && local_data.left_pressed {
            to_send_data.move_direction = Some('5');
            data::to_send_data::set(to_send_data);
        } else if local_data.backward_pressed && local_data.right_pressed {
            to_send_data.move_direction = Some('1');
            data::to_send_data::set(to_send_data);
        } else if local_data.backward_pressed && local_data.left_pressed {
            to_send_data.move_direction = Some('3');
            data::to_send_data::set(to_send_data);
        } else if local_data.forward_pressed {
            to_send_data.move_direction = Some('6');
            data::to_send_data::set(to_send_data);
        } else if local_data.right_pressed {
            to_send_data.move_direction = Some('0');
            data::to_send_data::set(to_send_data);
        } else if local_data.left_pressed {
            to_send_data.move_direction = Some('4');
            data::to_send_data::set(to_send_data);
        } else if local_data.backward_pressed {
            to_send_data.move_direction = Some('2');
            data::to_send_data::set(to_send_data);
        }

        println!("{:#?}", data::game_state::get());
        println!("{:#?}", data::to_send_data::get());

        // Drawing to the screen
        //canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        //canvas.clear();

        //canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        //for player in data::game_state::get().players {
        //    canvas.fill_rect(sdl2::rect::Rect::new(player.x, player.y, 50, 50)).unwrap();
        //}

        //canvas.present();
        let mut to_return=FrameReturnData::Draw(std::vec::Vec::new());
        for player in data::game_state::get().players {
            match to_return {
                FrameReturnData::Draw(to_draw_objects) => {
                    to_draw_objects.push(ToDrawObj::Rectangle(sdl2::rect::Rect::new(player.x, player.y, 50, 50), sdl2::pixels::Color::RGB(255, 255, 255)));
                },
                _ => {}
            }
        }
        return to_return;
    }

    data::local_data::set(local_data);
    return FrameReturnData::DoNothing;
}
