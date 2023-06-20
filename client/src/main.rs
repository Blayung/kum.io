mod data;

const SCREEN_HEIGHT: u32 = 500;
const SCREEN_WIDTH: u32 = 500;

pub fn main() {
    data::server_ip::init(String::from("http://localhost:8888"));
    data::credentials::init((String::new(),String::new()));

    // SDL2 Vars
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    data::http_client::init(); // Http client init

    // Logging in should be handled in here...
    
    // Keep alive thread
    std::thread::spawn(|| {
        let mut keep_alive_start: std::time::Instant;
        loop {
            keep_alive_start = std::time::Instant::now();
            data::http_client::get().post(data::server_ip::get()+"/keep_alive").body(data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
            std::thread::sleep(std::time::Duration::new(20, 0).checked_sub(keep_alive_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    // Every-tick thread
    std::thread::spawn(|| {
        let mut tick_start: std::time::Instant;
        loop {
            tick_start = std::time::Instant::now();
        
            data::game_state::set(data::http_client::get().get(data::server_ip::get()+"/game_state").send().unwrap().json().unwrap());

            let to_send_data = data::to_send_data::get();

            if to_send_data.move_direction.is_some() {
                data::http_client::get().post(data::server_ip::get()+"/move").body(to_send_data.move_direction.unwrap().to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();
            }
            data::http_client::get().post(data::server_ip::get()+"/rotate").body(to_send_data.direction.to_string() + "," + &data::credentials::get().0 + "," + &data::credentials::get().1).send().unwrap();

            data::to_send_data::set(data::to_send_data::ToSendData {
                move_direction: None,
                direction: 0
            });
            
            std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    // Main loop's vars
    let mut forward_pressed: bool;
    let mut right_pressed: bool;
    let mut backward_pressed: bool;
    let mut left_pressed: bool;

    let mut frame_start: std::time::Instant; 

    // Main loop
    'main_loop: loop {
        frame_start = std::time::Instant::now();

        // Events
        forward_pressed = false;
        right_pressed = false;
        backward_pressed = false;
        left_pressed = false;

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => break 'main_loop,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::W), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), .. } => forward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::D), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), .. } => right_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::S), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), .. } => backward_pressed = true,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::A), .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), .. } => left_pressed = true,
                _ => {}
            }
        }

        let mut to_send_data = data::to_send_data::get();
        if forward_pressed && right_pressed && left_pressed {
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

        // Every-frame stuff
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
}
