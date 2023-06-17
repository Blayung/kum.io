mod game_state;

const SCREEN_HEIGHT: u32 = 500;
const SCREEN_WIDTH: u32 = 500;

pub fn main() {
    // Normal vars
    let mut server_ip = "http://localhost:8888".to_string();
    let mut credentials: (String, String) = ("".to_string(),"".to_string());

    let mut forward_pressed: bool;
    let mut right_pressed: bool;
    let mut backward_pressed: bool;
    let mut left_pressed: bool;
    let mut direction = '0';

    let http_client = reqwest::blocking::Client::new();

    let mut frame_start: std::time::Instant;

    // SDL2 Vars
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Kum.io client", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().opengl().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Logging in should be handled in here...
    
    // Keep alive thread
    std::thread::spawn(|| {
        let mut start: std::time::Instant;
        loop {
            start = std::time::Instant::now();
            http_client.post(server_ip+"/keep_alive").body(credentials.0 + "," + &credentials.1).send().unwrap();
            std::thread::sleep(std::time::Duration::new(20, 0).checked_sub(start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

    // Every-tick thread
    std::thread::spawn(|| {
        let mut tick_start: std::time::Instant;
        loop {
            tick_start = std::time::Instant::now();
        
            game_state::set(http_client.get(server_ip+"/get_game_state").send().unwrap().json().unwrap());
            http_client.post(server_ip+"/move").body(direction + "," + credentials.0 + "," + &credentials.1).send().unwrap();
        
            std::thread::sleep(std::time::Duration::new(0, 50000000).checked_sub(tick_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
        }
    });

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

        if forward_pressed && right_pressed && left_pressed {
            direction = '6';
        } else if forward_pressed && backward_pressed && left_pressed { 
            direction = '4';
        } else if forward_pressed && backward_pressed && right_pressed { 
            direction = '0';
        } else if backward_pressed && left_pressed && right_pressed {
            direction = '2';
        } else if forward_pressed && right_pressed {
            direction = '7';
        } else if forward_pressed && left_pressed {
            direction = '5';
        } else if backward_pressed && right_pressed {
            direction = '1';
        } else if backward_pressed && left_pressed {
            direction = '3';
        } else if forward_pressed {
            direction = '6';
        } else if right_pressed {
            direction = '0';
        } else if left_pressed {
            direction = '4';
        } else if backward_pressed {
            direction = '2';
        }

        // Every-frame stuff
        println!("{:#?}", game_state::get());

        // Drawing to the screen
        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        for player in game_state::get().players {
            canvas.fill_rect(sdl2::rect::Rect::new(player.x as i32, player.y as i32, 50, 50)).unwrap();
        }

        canvas.present();

        // FPS Limit
        std::thread::sleep(std::time::Duration::new(0, 16666666).checked_sub(frame_start.elapsed()).unwrap_or(std::time::Duration::ZERO));
    }
}
