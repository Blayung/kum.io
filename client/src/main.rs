mod data;

//use std::str::FromStr;

const SCREEN_HEIGHT: u32 = 500;
const SCREEN_WIDTH: u32 = 500;

pub fn main() {
    // Init
    data::http_client::init();
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Logging in's main loop
    data::server_ip::init(String::from("http://localhost:8888"));
    data::credentials::init(("blay".to_owned(),data::http_client::get().post(data::server_ip::get().to_owned()+"/register").body("blay").send().unwrap().text().unwrap()));
    /*
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut ip = "193.107.8.49".to_owned();
    let mut nick = "kummie".to_owned();

    let mut frame_start: std::time::Instant;

    'logging_in_loop: loop {
        frame_start = std::time::Instant::now();

        // Events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'logging_in_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num0), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp0), .. } => { if ip.len()<21 { ip += "0"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num1), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp1), .. } => { if ip.len()<21 { ip += "1"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp2), .. } => { if ip.len()<21 { ip += "2"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num3), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp3), .. } => { if ip.len()<21 { ip += "3"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num4), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp4), .. } => { if ip.len()<21 { ip += "4"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num5), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp5), .. } => { if ip.len()<21 { ip += "5"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num6), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp6), .. } => { if ip.len()<21 { ip += "6"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num7), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp7), .. } => { if ip.len()<21 { ip += "7"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num8), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp8), .. } => { if ip.len()<21 { ip += "8"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Num9), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Kp9), .. } => { if ip.len()<21 { ip += "9"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Period), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpPeriod), .. } => { if ip.len()<21 { ip += "." } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Semicolon), .. } => { if ip.len()<21 { ip += ":"; } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Backspace), .. } => { if ip.len()>0 { ip.pop(); } },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Return2), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::KpEnter), .. } => { if std::net::IpAddr::from_str(&ip).is_ok() { data::server_ip::init( "http://".to_owned() + &ip ); } else { /* go cry */ } },
                _ => { continue; }
            }
            break;
        }

        println!("{}",ip);

        // Drawing to the screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for player in data::game_state::get().players {
            canvas.fill_rect(sdl2::rect::Rect::new(player.x as i32, player.y as i32, 50, 50)).unwrap();
        }

        canvas.present();

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 33333333).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }
    */
    
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

    // The main loop
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut forward_pressed = false;
    let mut right_pressed = false;
    let mut backward_pressed = false;
    let mut left_pressed = false;

    let mut frame_start: std::time::Instant; 

    'main_loop: loop {
        frame_start = std::time::Instant::now();

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

        // Every-frame pre-drawing stuff
        println!("{:#?}", data::game_state::get());
        println!("{:#?}", data::to_send_data::get());

        // Drawing to the screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for player in data::game_state::get().players {
            canvas.fill_rect(sdl2::rect::Rect::new(player.x as i32, player.y as i32, 50, 50)).unwrap();
        }

        canvas.present();

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 16666666).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }

    // Logging out
    data::http_client::get().post(data::server_ip::get().to_owned() + "/log_out").body((&data::credentials::get().0).to_owned().to_owned() + "," + &data::credentials::get().1).send().unwrap();
}
